//! Canonical bearing decision-readiness evaluation.
//!
//! This keeps doctor, lifecycle gates, and bearing projections on one shared
//! evidence-quality contract instead of re-deriving readiness from file
//! presence alone.

use std::fs;
use std::path::Path;

use crate::domain::model::Bearing;
use crate::infrastructure::bearing_evidence::{parse_evidence_records, validate_evidence_document};
use crate::infrastructure::config::ModeWeights;
use crate::infrastructure::scoring::{
    EvScore, EvidenceQualitySummary, calculate_evidence_backed_score, load_assessment_document,
    validate_assessment_citations,
};

const MIN_CITED_SOURCES: usize = 2;
const MIN_AUTHORITY_SCORE: f64 = 0.60;
const MIN_FRESHNESS_SCORE: f64 = 0.60;

#[derive(Debug, Clone)]
pub struct BearingReadinessReport {
    pub issues: Vec<BearingReadinessIssue>,
    pub evidence_quality: Option<EvidenceQualitySummary>,
    pub score: Option<EvScore>,
}

impl BearingReadinessReport {
    pub fn is_decision_ready(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn short_status(&self) -> String {
        self.issues
            .first()
            .map(BearingReadinessIssue::short_label)
            .unwrap_or_else(|| "decision-ready".to_string())
    }

    pub fn primary_recovery_command(&self, bearing_id: &str) -> Option<String> {
        self.issues
            .first()
            .map(|issue| issue.recovery_command(bearing_id))
    }

    pub fn problem_messages(&self, bearing_id: &str) -> Vec<String> {
        self.issues
            .iter()
            .map(|issue| issue.detail_message(bearing_id))
            .collect()
    }

    pub fn next_action_category(&self) -> &'static str {
        self.issues
            .first()
            .map(BearingReadinessIssue::next_action_category)
            .unwrap_or("lay")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BearingReadinessIssue {
    MissingEvidence,
    InvalidEvidence(String),
    MissingAssessment,
    InvalidAssessment(String),
    IncompleteFactors(Vec<String>),
    InsufficientCoverage { cited_sources: usize },
    WeakAuthority { authority_score: f64 },
    WeakFreshness { freshness_score: f64 },
    ContradictedRecommendation { contradiction_penalty: f64 },
}

impl BearingReadinessIssue {
    pub fn short_label(&self) -> String {
        match self {
            Self::MissingEvidence => "need evidence".to_string(),
            Self::InvalidEvidence(_) => "repair evidence".to_string(),
            Self::MissingAssessment => "need assessment".to_string(),
            Self::InvalidAssessment(_) => "repair citations".to_string(),
            Self::IncompleteFactors(_) => "complete scoring".to_string(),
            Self::InsufficientCoverage { .. } => "grow evidence".to_string(),
            Self::WeakAuthority { .. } => "raise authority".to_string(),
            Self::WeakFreshness { .. } => "refresh evidence".to_string(),
            Self::ContradictedRecommendation { .. } => "resolve conflict".to_string(),
        }
    }

    pub fn next_action_category(&self) -> &'static str {
        match self {
            Self::MissingEvidence
            | Self::InvalidEvidence(_)
            | Self::InsufficientCoverage { .. }
            | Self::WeakAuthority { .. }
            | Self::WeakFreshness { .. } => "research",
            Self::MissingAssessment
            | Self::InvalidAssessment(_)
            | Self::IncompleteFactors(_)
            | Self::ContradictedRecommendation { .. } => "assess",
        }
    }

    pub fn recovery_command(&self, bearing_id: &str) -> String {
        match self {
            Self::MissingEvidence => format!("keel bearing research {bearing_id}"),
            Self::MissingAssessment => format!("keel bearing assess {bearing_id}"),
            Self::InvalidEvidence(_)
            | Self::InsufficientCoverage { .. }
            | Self::WeakAuthority { .. }
            | Self::WeakFreshness { .. } => {
                format!("keel bearing file {bearing_id} EVIDENCE")
            }
            Self::InvalidAssessment(_)
            | Self::IncompleteFactors(_)
            | Self::ContradictedRecommendation { .. } => {
                format!("keel bearing file {bearing_id} ASSESSMENT")
            }
        }
    }

    pub fn detail_message(&self, bearing_id: &str) -> String {
        match self {
            Self::MissingEvidence => format!(
                "bearing '{bearing_id}' is not decision-ready: missing EVIDENCE.md; run `keel bearing research {bearing_id}` first"
            ),
            Self::InvalidEvidence(message) => format!(
                "bearing '{bearing_id}' is not decision-ready: evidence contract error: {message}; repair `EVIDENCE.md` via `keel bearing file {bearing_id} EVIDENCE`"
            ),
            Self::MissingAssessment => format!(
                "bearing '{bearing_id}' is not decision-ready: missing ASSESSMENT.md; run `keel bearing assess {bearing_id}` first"
            ),
            Self::InvalidAssessment(message) => format!(
                "bearing '{bearing_id}' is not decision-ready: assessment contract error: {message}; repair `ASSESSMENT.md` via `keel bearing file {bearing_id} ASSESSMENT`"
            ),
            Self::IncompleteFactors(missing) => format!(
                "bearing '{bearing_id}' is not decision-ready: ASSESSMENT.md is missing scoring factors: {}; complete the factor table via `keel bearing file {bearing_id} ASSESSMENT`",
                missing.join(", ")
            ),
            Self::InsufficientCoverage { cited_sources } => format!(
                "bearing '{bearing_id}' is not decision-ready: recommendation coverage is too narrow ({cited_sources} cited source{}; need at least {MIN_CITED_SOURCES}); expand `EVIDENCE.md` via `keel bearing file {bearing_id} EVIDENCE`",
                if *cited_sources == 1 { "" } else { "s" }
            ),
            Self::WeakAuthority { authority_score } => format!(
                "bearing '{bearing_id}' is not decision-ready: evidence authority is too weak ({authority_score:.2}; need >= {MIN_AUTHORITY_SCORE:.2}); capture stronger sources in `EVIDENCE.md` via `keel bearing file {bearing_id} EVIDENCE`"
            ),
            Self::WeakFreshness { freshness_score } => format!(
                "bearing '{bearing_id}' is not decision-ready: evidence freshness is too weak ({freshness_score:.2}; need >= {MIN_FRESHNESS_SCORE:.2}); refresh `EVIDENCE.md` via `keel bearing file {bearing_id} EVIDENCE`"
            ),
            Self::ContradictedRecommendation {
                contradiction_penalty,
            } => format!(
                "bearing '{bearing_id}' is not decision-ready: the recommendation is contradicted by stronger alternative evidence (penalty {contradiction_penalty:.2}); repair `ASSESSMENT.md` via `keel bearing file {bearing_id} ASSESSMENT`"
            ),
        }
    }
}

pub fn evaluate_bearing_readiness(
    board_dir: &Path,
    bearing: &Bearing,
    weights: Option<&ModeWeights>,
) -> BearingReadinessReport {
    let bearing_dir = board_dir.join("bearings").join(bearing.id());
    let evidence_path = bearing_dir.join("EVIDENCE.md");
    let assessment_path = bearing_dir.join("ASSESSMENT.md");

    let mut issues = Vec::new();
    let mut evidence_quality = None;
    let mut score = None;

    let evidence_records = if evidence_path.exists() {
        match fs::read_to_string(&evidence_path) {
            Ok(content) => {
                let validation_errors = validate_evidence_document(&content);
                if !validation_errors.is_empty() {
                    issues.extend(
                        validation_errors
                            .into_iter()
                            .map(BearingReadinessIssue::InvalidEvidence),
                    );
                    None
                } else {
                    match parse_evidence_records(&content) {
                        Ok(records) => Some(records),
                        Err(errors) => {
                            issues.extend(
                                errors
                                    .into_iter()
                                    .map(BearingReadinessIssue::InvalidEvidence),
                            );
                            None
                        }
                    }
                }
            }
            Err(error) => {
                issues.push(BearingReadinessIssue::InvalidEvidence(format!(
                    "cannot read EVIDENCE.md: {error}"
                )));
                None
            }
        }
    } else {
        issues.push(BearingReadinessIssue::MissingEvidence);
        None
    };

    let assessment = if assessment_path.exists() {
        match load_assessment_document(&assessment_path) {
            Ok(document) => Some(document),
            Err(error) => {
                issues.push(BearingReadinessIssue::InvalidAssessment(error.to_string()));
                None
            }
        }
    } else {
        issues.push(BearingReadinessIssue::MissingAssessment);
        None
    };

    if let Some(assessment) = &assessment {
        let missing = assessment
            .factors
            .missing_factors()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            issues.push(BearingReadinessIssue::IncompleteFactors(missing));
        }

        if let Some(records) = &evidence_records {
            let citation_errors = validate_assessment_citations(assessment, records);
            if !citation_errors.is_empty() {
                issues.extend(
                    citation_errors
                        .into_iter()
                        .map(BearingReadinessIssue::InvalidAssessment),
                );
            } else {
                match crate::infrastructure::scoring::derive_evidence_quality(assessment, records) {
                    Ok(signals) => {
                        if signals.cited_source_count < MIN_CITED_SOURCES {
                            issues.push(BearingReadinessIssue::InsufficientCoverage {
                                cited_sources: signals.cited_source_count,
                            });
                        }
                        if signals.authority_score < MIN_AUTHORITY_SCORE {
                            issues.push(BearingReadinessIssue::WeakAuthority {
                                authority_score: signals.authority_score,
                            });
                        }
                        if signals.freshness_score < MIN_FRESHNESS_SCORE {
                            issues.push(BearingReadinessIssue::WeakFreshness {
                                freshness_score: signals.freshness_score,
                            });
                        }
                        if signals.contradiction_penalty > 0.0 {
                            issues.push(BearingReadinessIssue::ContradictedRecommendation {
                                contradiction_penalty: signals.contradiction_penalty,
                            });
                        }
                        if let Some(weights) = weights
                            && assessment.factors.is_complete()
                        {
                            score = calculate_evidence_backed_score(
                                &assessment.factors,
                                &signals,
                                weights,
                            )
                            .ok();
                        }
                        evidence_quality = Some(signals);
                    }
                    Err(error) => {
                        issues.push(BearingReadinessIssue::InvalidAssessment(error.to_string()));
                    }
                }
            }
        }
    }

    BearingReadinessReport {
        issues,
        evidence_quality,
        score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::BearingStatus;
    use crate::infrastructure::config::ModeWeights;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBearing, TestBoardBuilder};
    use std::fs;

    fn readiness_board(
        status: &str,
        evidence: &str,
        assessment: &str,
    ) -> (tempfile::TempDir, Bearing) {
        let temp = TestBoardBuilder::new()
            .bearing(
                TestBearing::new("1w5H2Bq9L")
                    .status(status)
                    .has_evidence(true)
                    .has_assessment(true),
            )
            .build();
        let bearing_dir = temp.path().join("bearings/1w5H2Bq9L");
        fs::write(bearing_dir.join("EVIDENCE.md"), evidence).unwrap();
        fs::write(bearing_dir.join("ASSESSMENT.md"), assessment).unwrap();
        let board = load_board(temp.path()).unwrap();
        let bearing = board.bearings.get("1w5H2Bq9L").unwrap().clone();
        (temp, bearing)
    }

    fn strong_evidence() -> &'static str {
        r#"
## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | academic | manual:prior-art-review | https://example.com/paper | 2026-02-01 | 2026-03-07 | high | high | Prior art supports the direction. |
| SRC-02 | web | manual:official-doc | https://example.com/docs | 2026-03-01 | 2026-03-08 | high | high | Official docs confirm feasibility. |
"#
    }

    fn narrow_evidence() -> &'static str {
        r#"
## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | web | manual:official-doc | https://example.com/docs | 2026-03-01 | 2026-03-08 | high | high | One strong source supports the direction. |
"#
    }

    fn stronger_alternative_evidence() -> &'static str {
        r#"
## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | web | manual:official-doc | https://example.com/docs | 2026-02-25 | 2026-03-08 | medium | high | Support for proceeding is credible but narrower than the alternative. |
| SRC-02 | academic | manual:prior-art-review | https://example.com/paper | 2026-03-01 | 2026-03-08 | high | high | The simpler alternative is better supported. |
"#
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


## Findings
- Delivery teams need source-backed recommendations before converting research into roadmap work [SRC-01][SRC-02]

## Opportunity Cost
Deferring this leaves roadmap work underspecified [SRC-02]

## Dependencies
- Evidence capture must produce canonical source records first [SRC-02]

## Alternatives Considered
- Keep factor-only scoring and trust operator judgment alone [SRC-01]

## Recommendation

[x] Proceed → convert to epic [SRC-01][SRC-02]
[ ] Park → revisit later [SRC-02]
[ ] Decline → document learnings [SRC-01]
"#
    }

    fn contradicted_assessment() -> &'static str {
        r#"
# Assessment

| Factor | Score |
|--------|-------|
| Impact | 4 |
| Confidence | 4 |
| Effort | 2 |
| Risk | 2 |


## Findings
- Delivery teams need source-backed recommendations before converting research into roadmap work [SRC-01]

## Opportunity Cost
Deferring this leaves roadmap work underspecified [SRC-01]

## Dependencies
- Evidence capture must produce canonical source records first [SRC-01]

## Alternatives Considered
- Keep factor-only scoring and trust operator judgment alone [SRC-02]

## Recommendation

[x] Proceed → convert to epic [SRC-01]
[ ] Park → revisit later [SRC-02]
[ ] Decline → document learnings [SRC-02]
"#
    }

    fn single_source_assessment() -> &'static str {
        r#"
# Assessment

| Factor | Score |
|--------|-------|
| Impact | 4 |
| Confidence | 4 |
| Effort | 2 |
| Risk | 2 |


## Findings
- Delivery teams need source-backed recommendations before converting research into roadmap work [SRC-01]

## Opportunity Cost
Deferring this leaves roadmap work underspecified [SRC-01]

## Dependencies
- Evidence capture must produce canonical source records first [SRC-01]

## Alternatives Considered
- Keep factor-only scoring and trust operator judgment alone [SRC-01]

## Recommendation

[x] Proceed → convert to epic [SRC-01]
[ ] Park → revisit later [SRC-01]
[ ] Decline → document learnings [SRC-01]
"#
    }

    #[test]
    fn readiness_requires_quality_thresholds() {
        let (temp, bearing) =
            readiness_board("ready", narrow_evidence(), single_source_assessment());
        let report =
            evaluate_bearing_readiness(temp.path(), &bearing, Some(&ModeWeights::constrained()));

        assert!(!report.is_decision_ready());
        assert!(
            report
                .issues
                .iter()
                .any(|issue| matches!(issue, BearingReadinessIssue::InsufficientCoverage { .. }))
        );
        assert_eq!(report.short_status(), "grow evidence");
        assert_eq!(
            report.primary_recovery_command("1w5H2Bq9L").as_deref(),
            Some("keel bearing file 1w5H2Bq9L EVIDENCE")
        );
    }

    #[test]
    fn readiness_detects_contradicted_recommendations() {
        let (temp, bearing) = readiness_board(
            "ready",
            stronger_alternative_evidence(),
            contradicted_assessment(),
        );
        let report =
            evaluate_bearing_readiness(temp.path(), &bearing, Some(&ModeWeights::growth()));

        assert!(!report.is_decision_ready());
        assert!(report.issues.iter().any(|issue| matches!(
            issue,
            BearingReadinessIssue::ContradictedRecommendation { .. }
        )));
        assert_eq!(report.short_status(), "resolve conflict");
        assert_eq!(
            report.primary_recovery_command("1w5H2Bq9L").as_deref(),
            Some("keel bearing file 1w5H2Bq9L ASSESSMENT")
        );
    }

    #[test]
    fn readiness_marks_strong_research_as_decision_ready() {
        let (temp, bearing) = readiness_board("ready", strong_evidence(), cited_assessment());
        let report =
            evaluate_bearing_readiness(temp.path(), &bearing, Some(&ModeWeights::growth()));

        assert!(bearing.status() == BearingStatus::Ready);
        assert!(report.is_decision_ready());
        assert_eq!(report.short_status(), "decision-ready");
        assert!(report.score.is_some());
        assert!(report.evidence_quality.is_some());
    }
}
