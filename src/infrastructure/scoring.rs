//! EV (Expected Value) scoring system for bearings
//!
//! Calculates a composite score based on impact, confidence, effort, and risk factors
//! parsed from `ASSESSMENT.md`, and can blend in evidence-quality signals derived from
//! cited `EVIDENCE.md` source records.

use anyhow::{Context, Result, anyhow, bail};
use regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::Path;
use std::sync::LazyLock;

use crate::infrastructure::bearing_evidence::{
    EvidenceRecord, EvidenceStrength, parse_evidence_records,
};
use crate::infrastructure::config::ModeWeights;
use crate::infrastructure::markdown_sections::{extract_section, parse_markdown_list_items};

static CITATION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[(SRC-[A-Z0-9-]+)\]").unwrap());

/// Assessment factors for EV scoring (values 1-5)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AssessmentFactors {
    pub impact: Option<u8>,
    pub confidence: Option<u8>,
    pub effort: Option<u8>,
    pub risk: Option<u8>,
}

impl AssessmentFactors {
    /// Check if all required factors are present and valid
    pub fn is_complete(&self) -> bool {
        self.impact.is_some()
            && self.confidence.is_some()
            && self.effort.is_some()
            && self.risk.is_some()
    }

    /// Get a list of missing factors
    pub fn missing_factors(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if self.impact.is_none() {
            missing.push("impact");
        }
        if self.confidence.is_none() {
            missing.push("confidence");
        }
        if self.effort.is_none() {
            missing.push("effort");
        }
        if self.risk.is_none() {
            missing.push("risk");
        }
        missing
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CitedAssessmentItem {
    pub text: String,
    pub citations: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AssessmentDocument {
    pub factors: AssessmentFactors,
    pub findings: Vec<CitedAssessmentItem>,
    pub opportunity_cost: Option<CitedAssessmentItem>,
    pub dependencies: Vec<CitedAssessmentItem>,
    pub alternatives: Vec<CitedAssessmentItem>,
    pub recommendation: Option<CitedAssessmentItem>,
}

pub type AssessmentAnalysis = AssessmentDocument;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EvidenceQualitySummary {
    pub cited_source_count: usize,
    pub source_class_count: usize,
    pub authority_score: f64,
    pub freshness_score: f64,
    pub contradiction_count: usize,
    pub breadth_score: f64,
    pub gap_penalty: f64,
    pub contradiction_penalty: f64,
}

/// EV score result
#[derive(Debug, Clone)]
pub struct EvScore {
    /// Raw score before weights
    #[allow(dead_code)] // Available for detailed score breakdown display
    pub raw_score: f64,
    /// Weighted score
    pub weighted_score: f64,
    /// Evidence-derived multiplier applied to the weighted score
    #[allow(dead_code)] // Available for detailed score breakdown display
    pub evidence_multiplier: f64,
    /// Evidence-quality metrics used to ground the score
    pub evidence_quality: EvidenceQualitySummary,
    /// The factors used
    #[allow(dead_code)] // Available for detailed score breakdown display
    pub factors: AssessmentFactors,
}

impl EvScore {
    /// Format score for display
    #[allow(dead_code)] // Available for CLI score display
    pub fn display(&self) -> String {
        format!("{:.2}", self.weighted_score)
    }
}

/// Calculate EV score from authored factors and mode weights.
///
/// Formula: score = (impact × confidence) / (effort + risk_penalty)
/// Where risk_penalty = (risk - 1) / 2 (so risk 1 = 0, risk 5 = 2)
pub fn calculate_score(factors: &AssessmentFactors, weights: &ModeWeights) -> Result<EvScore> {
    let impact = factors
        .impact
        .ok_or_else(|| anyhow!("Missing impact factor"))? as f64;
    let confidence = factors
        .confidence
        .ok_or_else(|| anyhow!("Missing confidence factor"))? as f64;
    let effort = factors
        .effort
        .ok_or_else(|| anyhow!("Missing effort factor"))? as f64;
    let risk = factors.risk.ok_or_else(|| anyhow!("Missing risk factor"))? as f64;

    // Risk penalty: converts 1-5 risk to 0-2 penalty
    let risk_penalty = (risk - 1.0) / 2.0;

    // Raw formula: (impact × confidence) / (effort + risk_penalty)
    let raw_numerator = impact * confidence;
    let raw_denominator = effort + risk_penalty;
    let raw_score = if raw_denominator > 0.0 {
        raw_numerator / raw_denominator
    } else {
        raw_numerator
    };

    // Weighted formula: (impact*w1 × confidence*w2) / (effort*w3 + risk_penalty*w4)
    let weighted_numerator =
        (impact * weights.impact_weight) * (confidence * weights.confidence_weight);
    let weighted_denominator =
        (effort * weights.effort_weight) + (risk_penalty * weights.risk_weight);
    let weighted_score = if weighted_denominator > 0.0 {
        weighted_numerator / weighted_denominator
    } else {
        weighted_numerator
    };

    Ok(EvScore {
        raw_score,
        weighted_score,
        evidence_multiplier: 1.0,
        evidence_quality: EvidenceQualitySummary::default(),
        factors: factors.clone(),
    })
}

/// Blend evidence-quality signals into the weighted EV score.
pub fn calculate_evidence_backed_score(
    factors: &AssessmentFactors,
    signals: &EvidenceQualitySummary,
    weights: &ModeWeights,
) -> Result<EvScore> {
    let mut score = calculate_score(factors, weights)?;
    let evidence_multiplier = evidence_multiplier(signals);
    score.weighted_score *= evidence_multiplier;
    score.evidence_multiplier = evidence_multiplier;
    score.evidence_quality = signals.clone();
    Ok(score)
}

/// Parse a full assessment document including cited sections.
pub fn parse_assessment_document(content: &str) -> AssessmentDocument {
    AssessmentDocument {
        factors: parse_assessment(content),
        findings: parse_cited_list_section(content, "### Findings"),
        opportunity_cost: parse_cited_paragraph(content, "### Opportunity Cost"),
        dependencies: parse_cited_list_section(content, "### Dependencies"),
        alternatives: parse_cited_list_section(content, "### Alternatives Considered"),
        recommendation: parse_cited_recommendation(content),
    }
}

/// Parse assessment factors from `ASSESSMENT.md` content.
///
/// Looks for a markdown table with columns for Factor and Value/Score.
pub fn parse_assessment(content: &str) -> AssessmentFactors {
    let mut factors = AssessmentFactors::default();

    for line in content.lines() {
        let line = line.trim();

        if !line.starts_with('|') || line.contains("---") {
            continue;
        }

        let cells: Vec<&str> = line
            .split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if cells.len() < 2 {
            continue;
        }

        let factor_name = cells[0].to_lowercase();
        let value_str = cells[1];

        if let Some(value) = extract_score(value_str).filter(|v| (1..=5).contains(v)) {
            if factor_name.contains("impact") {
                factors.impact = Some(value);
            } else if factor_name.contains("confidence") {
                factors.confidence = Some(value);
            } else if factor_name.contains("effort") {
                factors.effort = Some(value);
            } else if factor_name.contains("risk") {
                factors.risk = Some(value);
            }
        }
    }

    factors
}

/// Validate that assessment conclusions cite canonical evidence records.
pub fn validate_assessment_citations(
    document: &AssessmentDocument,
    evidence_records: &[EvidenceRecord],
) -> Vec<String> {
    let known_ids: HashSet<String> = evidence_records
        .iter()
        .map(|record| record.id.clone())
        .collect();
    let mut errors = Vec::new();

    validate_cited_items("Findings", &document.findings, &known_ids, &mut errors);
    validate_cited_items(
        "Dependencies",
        &document.dependencies,
        &known_ids,
        &mut errors,
    );
    validate_cited_items(
        "Alternatives Considered",
        &document.alternatives,
        &known_ids,
        &mut errors,
    );

    match &document.recommendation {
        Some(recommendation) => {
            if recommendation.citations.is_empty() {
                errors.push("Recommendation must cite at least one evidence source".to_string());
            }
            push_unknown_citation_errors(
                "Recommendation",
                &recommendation.citations,
                &known_ids,
                &mut errors,
            );
        }
        None => errors.push("Recommendation must select one cited option".to_string()),
    }

    errors.sort();
    errors.dedup();
    errors
}

/// Derive evidence-quality signals from a cited assessment document and source records.
pub fn derive_evidence_quality(
    document: &AssessmentDocument,
    evidence_records: &[EvidenceRecord],
) -> Result<EvidenceQualitySummary> {
    let validation_errors = validate_assessment_citations(document, evidence_records);
    if !validation_errors.is_empty() {
        bail!(validation_errors.join("; "));
    }

    let record_map: HashMap<&str, &EvidenceRecord> = evidence_records
        .iter()
        .map(|record| (record.id.as_str(), record))
        .collect();

    let findings_ids = collect_citation_ids(&document.findings);
    let dependency_ids = collect_citation_ids(&document.dependencies);
    let alternative_ids = collect_citation_ids(&document.alternatives);
    let recommendation_ids = document
        .recommendation
        .as_ref()
        .map(|item| dedup_citations(&item.citations))
        .unwrap_or_default();
    let opportunity_cost_ids = document
        .opportunity_cost
        .as_ref()
        .map(|item| dedup_citations(&item.citations))
        .unwrap_or_default();

    let support_ids = merge_citation_sets(&[
        findings_ids.clone(),
        dependency_ids.clone(),
        recommendation_ids.clone(),
        opportunity_cost_ids.clone(),
    ]);
    let all_ids = merge_citation_sets(&[support_ids.clone(), alternative_ids.clone()]);

    let support_records = resolve_records(&record_map, &support_ids)?;
    let recommendation_records = resolve_records(&record_map, &recommendation_ids)?;
    let alternative_records = resolve_records(&record_map, &alternative_ids)?;

    let required_sections = [
        !document.findings.is_empty(),
        !document.dependencies.is_empty(),
        !document.alternatives.is_empty(),
        document.recommendation.is_some(),
    ];
    let section_coverage = required_sections.iter().filter(|covered| **covered).count() as f64
        / required_sections.len() as f64;

    let cited_source_count = all_ids.len();
    let source_class_count = all_ids
        .iter()
        .filter_map(|id| record_map.get(id.as_str()))
        .map(|record| format!("{:?}", record.class))
        .collect::<BTreeSet<_>>()
        .len();
    let authority_score = mean_strength(&support_records, |record| &record.authority);
    let freshness_score = mean_strength(&support_records, |record| &record.freshness);
    let contradiction_penalty =
        contradiction_penalty(&recommendation_records, &alternative_records);
    Ok(EvidenceQualitySummary {
        cited_source_count,
        source_class_count,
        authority_score,
        freshness_score,
        contradiction_count: usize::from(contradiction_penalty > 0.0),
        breadth_score: (cited_source_count.min(4) as f64) / 4.0,
        gap_penalty: 1.0 - section_coverage,
        contradiction_penalty,
    })
}

/// Load and parse assessment analysis from an `ASSESSMENT.md` file.
pub fn load_assessment(path: &Path) -> Result<AssessmentAnalysis> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read assessment file: {}", path.display()))?;

    Ok(parse_assessment_document(&content))
}

/// Load the full assessment document from disk.
pub fn load_assessment_document(path: &Path) -> Result<AssessmentDocument> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read assessment file: {}", path.display()))?;

    Ok(parse_assessment_document(&content))
}

/// Load an evidence-backed EV score from authored assessment and evidence documents.
pub fn load_bearing_score(
    assessment_path: &Path,
    evidence_path: &Path,
    weights: &ModeWeights,
) -> Result<EvScore> {
    let assessment = load_assessment(assessment_path)?;
    let evidence_content = std::fs::read_to_string(evidence_path)
        .with_context(|| format!("Failed to read evidence file: {}", evidence_path.display()))?;
    let evidence_records =
        parse_evidence_records(&evidence_content).map_err(|errors| anyhow!(errors.join("; ")))?;
    let signals = derive_evidence_quality(&assessment, &evidence_records)?;
    calculate_evidence_backed_score(&assessment.factors, &signals, weights)
}

fn parse_cited_list_section(content: &str, heading: &str) -> Vec<CitedAssessmentItem> {
    extract_section(content, heading)
        .map(|section| {
            parse_markdown_list_items(&section)
                .into_iter()
                .map(|item| parse_cited_item(&item))
                .collect()
        })
        .unwrap_or_default()
}

fn parse_cited_paragraph(content: &str, heading: &str) -> Option<CitedAssessmentItem> {
    let section = extract_section(content, heading)?;
    if section.trim().is_empty() {
        return None;
    }

    Some(parse_cited_item(section.trim()))
}

fn parse_cited_recommendation(content: &str) -> Option<CitedAssessmentItem> {
    let section = extract_section(content, "## Recommendation")?;

    for line in section.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed
            .strip_prefix("[x] ")
            .or_else(|| trimmed.strip_prefix("[X] "))
            .or_else(|| trimmed.strip_prefix("- [x] "))
            .or_else(|| trimmed.strip_prefix("- [X] "))
        {
            return Some(parse_cited_item(value.trim()));
        }
    }

    None
}

fn parse_cited_item(value: &str) -> CitedAssessmentItem {
    CitedAssessmentItem {
        text: strip_citations(value),
        citations: dedup_citations(&extract_citations(value)),
    }
}

fn extract_citations(value: &str) -> Vec<String> {
    CITATION_REGEX
        .captures_iter(value)
        .filter_map(|captures| captures.get(1).map(|capture| capture.as_str().to_string()))
        .collect()
}

fn dedup_citations(citations: &[String]) -> Vec<String> {
    let mut ordered = BTreeSet::new();
    for citation in citations {
        ordered.insert(citation.clone());
    }
    ordered.into_iter().collect()
}

fn strip_citations(value: &str) -> String {
    CITATION_REGEX
        .replace_all(value, "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn validate_cited_items(
    section: &str,
    items: &[CitedAssessmentItem],
    known_ids: &HashSet<String>,
    errors: &mut Vec<String>,
) {
    if items.is_empty() {
        errors.push(format!("{section} must contain at least one cited item"));
        return;
    }

    for item in items {
        if item.citations.is_empty() {
            errors.push(format!(
                "{section} item '{}' is missing evidence citations",
                item.text
            ));
            continue;
        }

        push_unknown_citation_errors(section, &item.citations, known_ids, errors);
    }
}

fn push_unknown_citation_errors(
    section: &str,
    citations: &[String],
    known_ids: &HashSet<String>,
    errors: &mut Vec<String>,
) {
    for citation in citations {
        if !known_ids.contains(citation) {
            errors.push(format!(
                "{section} references unknown evidence source '{}'",
                citation
            ));
        }
    }
}

fn collect_citation_ids(items: &[CitedAssessmentItem]) -> Vec<String> {
    let mut ids = Vec::new();
    for item in items {
        ids.extend(item.citations.clone());
    }
    dedup_citations(&ids)
}

fn merge_citation_sets(groups: &[Vec<String>]) -> Vec<String> {
    let mut all = Vec::new();
    for group in groups {
        all.extend(group.clone());
    }
    dedup_citations(&all)
}

fn resolve_records<'a>(
    record_map: &HashMap<&'a str, &'a EvidenceRecord>,
    citation_ids: &[String],
) -> Result<Vec<&'a EvidenceRecord>> {
    let mut records = Vec::new();
    for citation in citation_ids {
        let Some(record) = record_map.get(citation.as_str()) else {
            bail!("unknown evidence source '{}'", citation);
        };
        records.push(*record);
    }
    Ok(records)
}

fn mean_strength(
    records: &[&EvidenceRecord],
    mapper: impl Fn(&EvidenceRecord) -> &EvidenceStrength,
) -> f64 {
    if records.is_empty() {
        return 0.0;
    }

    records
        .iter()
        .map(|record| strength_value(mapper(record)))
        .sum::<f64>()
        / records.len() as f64
}

fn strength_value(strength: &EvidenceStrength) -> f64 {
    match strength {
        EvidenceStrength::Low => 0.35,
        EvidenceStrength::Medium => 0.65,
        EvidenceStrength::High => 1.0,
    }
}

fn contradiction_penalty(
    recommendation_records: &[&EvidenceRecord],
    alternative_records: &[&EvidenceRecord],
) -> f64 {
    if recommendation_records.is_empty() || alternative_records.is_empty() {
        return 0.0;
    }

    let recommendation_strength = average_support_strength(recommendation_records);
    let alternative_strength = average_support_strength(alternative_records);
    if alternative_strength > recommendation_strength {
        (alternative_strength - recommendation_strength).min(1.0)
    } else {
        0.0
    }
}

fn average_support_strength(records: &[&EvidenceRecord]) -> f64 {
    if records.is_empty() {
        return 0.0;
    }

    records
        .iter()
        .map(|record| (strength_value(&record.authority) + strength_value(&record.freshness)) / 2.0)
        .sum::<f64>()
        / records.len() as f64
}

fn evidence_multiplier(signals: &EvidenceQualitySummary) -> f64 {
    let multiplier = 0.6
        + (signals.breadth_score * 0.2)
        + (signals.freshness_score * 0.15)
        + (signals.authority_score * 0.15)
        - (signals.gap_penalty * 0.2)
        - (signals.contradiction_penalty * 0.2);
    multiplier.clamp(0.3, 1.25)
}

/// Extract a score value from a string like "4", "4/5", "High (4)", etc.
fn extract_score(s: &str) -> Option<u8> {
    if let Ok(n) = s.trim().parse::<u8>() {
        return Some(n);
    }

    if let Some(pos) = s.find('/')
        && let Ok(n) = s[..pos].trim().parse::<u8>()
    {
        return Some(n);
    }

    if let Some(start) = s.find('(')
        && let Some(end) = s.find(')')
        && let Ok(n) = s[start + 1..end].trim().parse::<u8>()
    {
        return Some(n);
    }

    s.chars().find_map(|c| c.to_digit(10).map(|d| d as u8))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_factors() -> AssessmentFactors {
        AssessmentFactors {
            impact: Some(4),
            confidence: Some(4),
            effort: Some(2),
            risk: Some(2),
        }
    }

    fn cited_assessment() -> &'static str {
        r#"
# Assessment

| Factor | Score |
|--------|-------|
| Impact | 4 |
| Confidence | 4 |
| Effort | 2 |
| Risk | 2 |

## Analysis

### Findings
- Delivery teams need source-backed recommendations before converting research into roadmap work [SRC-01][SRC-02]

### Opportunity Cost
Deferring this leaves EV scoring disconnected from the evidence contract [SRC-02]

### Dependencies
- Evidence capture must produce canonical source records first [SRC-02]

### Alternatives Considered
- Keep factor-only scoring and trust operator judgment alone [SRC-03]

## Recommendation

[x] Proceed → convert to epic [SRC-01][SRC-02]
[ ] Park → revisit later
[ ] Decline → document learnings
"#
    }

    fn uncited_assessment() -> &'static str {
        r#"
# Assessment

| Factor | Score |
|--------|-------|
| Impact | 4 |
| Confidence | 4 |
| Effort | 2 |
| Risk | 2 |

## Analysis

### Findings
- Delivery teams need source-backed recommendations before converting research into roadmap work

### Dependencies
- Evidence capture must produce canonical source records first

### Alternatives Considered
- Keep factor-only scoring and trust operator judgment alone

## Recommendation

[x] Proceed → convert to epic
[ ] Park → revisit later
[ ] Decline → document learnings
"#
    }

    fn strong_evidence() -> &'static str {
        r#"
## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | academic | manual:prior-art-review | https://example.com/paper | 2026-02-01 | 2026-03-07 | high | high | Prior art supports the scoring shift. |
| SRC-02 | web | manual:official-doc | https://example.com/docs | 2026-03-01 | 2026-03-08 | high | high | Existing surfaces can carry citations without new UI primitives. |
| SRC-03 | manual | manual:internal-retro | docs/retro.md | 2026-03-05 | 2026-03-08 | medium | high | Factor-only scoring caused mis-prioritization in practice. |
"#
    }

    fn weak_evidence() -> &'static str {
        r#"
## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | social | manual:community-signal | https://example.com/thread | 2021-02-01 | 2026-03-07 | low | low | One anecdotal thread prefers source-backed recommendations. |
| SRC-02 | social | manual:community-signal | https://example.com/thread-2 | 2020-03-01 | 2026-03-08 | low | low | One anecdotal thread prefers score changes. |
| SRC-03 | social | manual:community-signal | https://example.com/thread-3 | 2020-05-01 | 2026-03-08 | low | low | One anecdotal thread prefers keeping factor-only scoring. |
"#
    }

    fn alternative_heavier_evidence() -> &'static str {
        r#"
## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | social | manual:community-signal | https://example.com/thread | 2021-02-01 | 2026-03-07 | low | low | Support for proceeding is weak. |
| SRC-02 | social | manual:community-signal | https://example.com/thread-2 | 2021-03-01 | 2026-03-07 | low | low | Support for proceeding is weak. |
| SRC-03 | academic | manual:prior-art-review | https://example.com/paper | 2026-03-01 | 2026-03-08 | high | high | Alternative simpler approach is stronger. |
"#
    }

    fn load_records(content: &str) -> Vec<EvidenceRecord> {
        parse_evidence_records(content).unwrap()
    }

    #[test]
    fn parse_assessment_extracts_factors() {
        let content = r#"
# Assessment

| Factor | Score |
|--------|-------|
| Impact | 4 |
| Confidence | 3 |
| Effort | 2 |
| Risk | 1 |
"#;

        let factors = parse_assessment(content);
        assert_eq!(factors.impact, Some(4));
        assert_eq!(factors.confidence, Some(3));
        assert_eq!(factors.effort, Some(2));
        assert_eq!(factors.risk, Some(1));
    }

    #[test]
    fn parse_assessment_handles_fraction_format() {
        let content = r#"
| Factor | Score |
|--------|-------|
| Impact | 4/5 |
| Confidence | 3/5 |
| Effort | 2/5 |
| Risk | 1/5 |
"#;

        let factors = parse_assessment(content);
        assert_eq!(factors.impact, Some(4));
        assert_eq!(factors.confidence, Some(3));
    }

    #[test]
    fn parse_assessment_handles_parentheses_format() {
        let content = r#"
| Factor | Score |
|--------|-------|
| Impact | High (4) |
| Confidence | Medium (3) |
| Effort | Low (2) |
| Risk | Minimal (1) |
"#;

        let factors = parse_assessment(content);
        assert_eq!(factors.impact, Some(4));
        assert_eq!(factors.risk, Some(1));
    }

    #[test]
    fn parse_assessment_handles_missing_factors() {
        let content = r#"
| Factor | Score |
|--------|-------|
| Impact | 4 |
| Effort | 2 |
"#;

        let factors = parse_assessment(content);
        assert_eq!(factors.impact, Some(4));
        assert_eq!(factors.confidence, None);
        assert_eq!(factors.effort, Some(2));
        assert_eq!(factors.risk, None);
        assert!(!factors.is_complete());
        assert_eq!(factors.missing_factors(), vec!["confidence", "risk"]);
    }

    #[test]
    fn calculate_score_uses_formula() {
        let factors = AssessmentFactors {
            impact: Some(4),
            confidence: Some(4),
            effort: Some(2),
            risk: Some(1),
        };

        let weights = ModeWeights {
            impact_weight: 1.0,
            confidence_weight: 1.0,
            effort_weight: 1.0,
            risk_weight: 1.0,
        };

        let score = calculate_score(&factors, &weights).unwrap();

        assert!((score.raw_score - 8.0).abs() < 0.01);
        assert!((score.weighted_score - 8.0).abs() < 0.01);
        assert!((score.evidence_multiplier - 1.0).abs() < 0.01);
    }

    #[test]
    fn calculate_score_applies_risk_penalty() {
        let factors = AssessmentFactors {
            impact: Some(4),
            confidence: Some(4),
            effort: Some(2),
            risk: Some(5),
        };

        let weights = ModeWeights {
            impact_weight: 1.0,
            confidence_weight: 1.0,
            effort_weight: 1.0,
            risk_weight: 1.0,
        };

        let score = calculate_score(&factors, &weights).unwrap();

        assert!((score.raw_score - 4.0).abs() < 0.01);
    }

    #[test]
    fn calculate_score_applies_weights() {
        let factors = AssessmentFactors {
            impact: Some(4),
            confidence: Some(3),
            effort: Some(2),
            risk: Some(1),
        };

        let constrained = ModeWeights::constrained();
        let score = calculate_score(&factors, &constrained).unwrap();

        let growth = ModeWeights::growth();
        let growth_score = calculate_score(&factors, &growth).unwrap();

        assert!(growth_score.weighted_score > score.weighted_score);
    }

    #[test]
    fn calculate_score_errors_on_missing_factors() {
        let factors = AssessmentFactors {
            impact: Some(4),
            confidence: None,
            effort: Some(2),
            risk: Some(1),
        };

        let weights = ModeWeights::constrained();
        let result = calculate_score(&factors, &weights);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("confidence"));
    }

    #[test]
    fn extract_score_handles_various_formats() {
        assert_eq!(extract_score("4"), Some(4));
        assert_eq!(extract_score("  4  "), Some(4));
        assert_eq!(extract_score("4/5"), Some(4));
        assert_eq!(extract_score("High (4)"), Some(4));
        assert_eq!(extract_score("Medium 3"), Some(3));
        assert_eq!(extract_score(""), None);
        assert_eq!(extract_score("None"), None);
    }

    #[test]
    fn factors_is_complete() {
        let complete = AssessmentFactors {
            impact: Some(4),
            confidence: Some(3),
            effort: Some(2),
            risk: Some(1),
        };
        assert!(complete.is_complete());

        let incomplete = AssessmentFactors {
            impact: Some(4),
            confidence: None,
            effort: Some(2),
            risk: Some(1),
        };
        assert!(!incomplete.is_complete());
    }

    #[test]
    fn bearing_assessment_requires_evidence_citations() {
        let uncited = parse_assessment_document(uncited_assessment());
        let uncited_errors =
            validate_assessment_citations(&uncited, &load_records(strong_evidence()));
        assert!(
            uncited_errors
                .iter()
                .any(|error| error.contains("Findings item") || error.contains("Recommendation")),
            "assessment validation should fail when conclusions are uncited: {uncited_errors:#?}"
        );

        let cited = parse_assessment_document(cited_assessment());
        let cited_errors = validate_assessment_citations(&cited, &load_records(strong_evidence()));
        assert!(
            cited_errors.is_empty(),
            "cited assessment should validate cleanly"
        );
    }

    #[test]
    fn bearing_ev_score_uses_evidence_quality_signals() {
        let factors = base_factors();
        let high = derive_evidence_quality(
            &parse_assessment_document(cited_assessment()),
            &load_records(strong_evidence()),
        )
        .unwrap();
        let low = derive_evidence_quality(
            &parse_assessment_document(cited_assessment()),
            &load_records(weak_evidence()),
        )
        .unwrap();

        let weights = ModeWeights::constrained();
        let high_score = calculate_evidence_backed_score(&factors, &high, &weights).unwrap();
        let low_score = calculate_evidence_backed_score(&factors, &low, &weights).unwrap();

        assert!(high.breadth_score >= low.breadth_score);
        assert!(high.authority_score > low.authority_score);
        assert!(high.freshness_score > low.freshness_score);
        assert!(high_score.weighted_score > low_score.weighted_score);
    }

    #[test]
    fn bearing_ev_score_changes_with_evidence_quality() {
        let temp = tempfile::TempDir::new().unwrap();
        let assessment_path = temp.path().join("ASSESSMENT.md");
        let evidence_path = temp.path().join("EVIDENCE.md");
        std::fs::write(&assessment_path, cited_assessment()).unwrap();

        let weights = ModeWeights::growth();

        std::fs::write(&evidence_path, strong_evidence()).unwrap();
        let strong_score = load_bearing_score(&assessment_path, &evidence_path, &weights)
            .unwrap()
            .weighted_score;

        std::fs::write(&evidence_path, weak_evidence()).unwrap();
        let weak_score = load_bearing_score(&assessment_path, &evidence_path, &weights)
            .unwrap()
            .weighted_score;

        std::fs::write(&evidence_path, alternative_heavier_evidence()).unwrap();
        let contradiction_score = load_bearing_score(&assessment_path, &evidence_path, &weights)
            .unwrap()
            .weighted_score;

        assert!(strong_score > weak_score);
        assert!(weak_score > contradiction_score);
    }

    #[test]
    fn bearing_ev_score_is_deterministic() {
        let temp = tempfile::TempDir::new().unwrap();
        let assessment_path = temp.path().join("ASSESSMENT.md");
        let evidence_path = temp.path().join("EVIDENCE.md");
        std::fs::write(&assessment_path, cited_assessment()).unwrap();
        std::fs::write(&evidence_path, strong_evidence()).unwrap();

        let weights = ModeWeights::constrained();
        let first = load_bearing_score(&assessment_path, &evidence_path, &weights)
            .unwrap()
            .weighted_score;

        std::fs::write(
            &evidence_path,
            r#"
## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-03 | manual | manual:internal-retro | docs/retro.md | 2026-03-05 | 2026-03-08 | medium | high | Factor-only scoring caused mis-prioritization in practice. |
| SRC-01 | academic | manual:prior-art-review | https://example.com/paper | 2026-02-01 | 2026-03-07 | high | high | Prior art supports the scoring shift. |
| SRC-02 | web | manual:official-doc | https://example.com/docs | 2026-03-01 | 2026-03-08 | high | high | Existing surfaces can carry citations without new UI primitives. |
"#,
        )
        .unwrap();
        let second = load_bearing_score(&assessment_path, &evidence_path, &weights)
            .unwrap()
            .weighted_score;

        assert!((first - second).abs() < 0.000_001);
    }
}
