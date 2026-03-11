//! Canonical parsing and validation for bearing `EVIDENCE.md` source records.

use std::cmp::Ordering;
use std::collections::HashSet;
use std::str::FromStr;

use chrono::NaiveDate;

use crate::infrastructure::markdown_sections::extract_section;

pub const CANONICAL_SOURCE_HEADERS: &[&str] = &[
    "ID",
    "Class",
    "Provenance",
    "Location",
    "Observed / Published",
    "Retrieved",
    "Authority",
    "Freshness",
    "Notes",
];

const SCAFFOLD_PROVENANCE: &str = "manual:web-search";
const SCAFFOLD_LOCATION: &str = "https://example.com";
const SCAFFOLD_NOTES: &str = "Replace with the concrete claim this source supports";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvidenceSourceClass {
    Web,
    Academic,
    Social,
    Manual,
}

impl EvidenceSourceClass {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Web => "web",
            Self::Academic => "academic",
            Self::Social => "social",
            Self::Manual => "manual",
        }
    }
}

impl std::fmt::Display for EvidenceSourceClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for EvidenceSourceClass {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "web" => Ok(Self::Web),
            "academic" => Ok(Self::Academic),
            "social" => Ok(Self::Social),
            "manual" => Ok(Self::Manual),
            other => Err(format!(
                "invalid source class '{other}' (expected web, academic, social, or manual)"
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvidenceStrength {
    Low,
    Medium,
    High,
}

impl EvidenceStrength {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

impl std::fmt::Display for EvidenceStrength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for EvidenceStrength {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            other => Err(format!(
                "invalid evidence strength '{other}' (expected low, medium, or high)"
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRecord {
    pub id: String,
    pub class: EvidenceSourceClass,
    pub provenance: String,
    pub location: String,
    pub observed_or_published_at: NaiveDate,
    pub retrieved_at: NaiveDate,
    pub authority: EvidenceStrength,
    pub freshness: EvidenceStrength,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedEvidenceRecords {
    records: Vec<EvidenceRecord>,
    saw_scaffold_row: bool,
}

/// Parse and normalize canonical evidence records from a full `EVIDENCE.md` document.
pub fn parse_evidence_records(content: &str) -> Result<Vec<EvidenceRecord>, Vec<String>> {
    parse_evidence_records_internal(content).map(|parsed| parsed.records)
}

/// Render evidence records as the canonical `## Sources` markdown table.
pub fn render_evidence_records_table(records: &[EvidenceRecord]) -> String {
    let mut sorted = records.to_vec();
    sorted.sort_by(evidence_record_ordering);

    let mut lines = vec![
        format!("| {} |", CANONICAL_SOURCE_HEADERS.join(" | ")),
        "|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|"
            .to_string(),
    ];

    for record in sorted {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            sanitize_markdown_cell(&record.id),
            sanitize_markdown_cell(record.class.as_str()),
            sanitize_markdown_cell(&record.provenance),
            sanitize_markdown_cell(&record.location),
            record.observed_or_published_at.format("%Y-%m-%d"),
            record.retrieved_at.format("%Y-%m-%d"),
            sanitize_markdown_cell(record.authority.as_str()),
            sanitize_markdown_cell(record.freshness.as_str()),
            sanitize_markdown_cell(&record.notes),
        ));
    }

    lines.join("\n")
}

/// Validate a full `EVIDENCE.md` document and return actionable error messages.
pub fn validate_evidence_document(content: &str) -> Vec<String> {
    match parse_evidence_records_internal(content) {
        Ok(parsed) => {
            let mut errors = Vec::new();
            if parsed.saw_scaffold_row {
                errors.push(
                    "Sources table still contains scaffold/example values; replace the template row with authored evidence"
                        .to_string(),
                );
            }
            if parsed.records.is_empty() {
                errors.push(
                    "Sources table must include at least one authored evidence record".to_string(),
                );
            }
            errors
        }
        Err(errors) => errors,
    }
}

fn parse_evidence_records_internal(content: &str) -> Result<ParsedEvidenceRecords, Vec<String>> {
    let sources = extract_section(content, "## Sources")
        .filter(|section| !section.trim().is_empty())
        .ok_or_else(|| vec!["missing required '## Sources' section".to_string()])?;

    let table_lines: Vec<&str> = sources
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('|'))
        .collect();

    if table_lines.len() < 2 {
        return Err(vec![
            "Sources section must include the canonical markdown table".to_string(),
        ]);
    }

    let header = parse_markdown_row(table_lines[0]).ok_or_else(|| {
        vec!["Sources table header is malformed; expected canonical columns".to_string()]
    })?;
    if header
        .iter()
        .map(String::as_str)
        .ne(CANONICAL_SOURCE_HEADERS.iter().copied())
    {
        return Err(vec![format!(
            "Sources table must use canonical columns: {}",
            CANONICAL_SOURCE_HEADERS.join(" | ")
        )]);
    }

    if !is_separator_row(table_lines[1]) {
        return Err(vec![
            "Sources table separator row is malformed; keep the canonical markdown table shape"
                .to_string(),
        ]);
    }

    let mut errors = Vec::new();
    let mut records = Vec::new();
    let mut seen_ids = HashSet::new();
    let mut saw_scaffold_row = false;

    for (row_index, row) in table_lines.iter().enumerate().skip(2) {
        if row.trim().is_empty() {
            continue;
        }

        let Some(cells) = parse_markdown_row(row) else {
            errors.push(format!(
                "Sources row {} is malformed; expected {} columns",
                row_index - 1,
                CANONICAL_SOURCE_HEADERS.len()
            ));
            continue;
        };

        if cells.len() != CANONICAL_SOURCE_HEADERS.len() {
            errors.push(format!(
                "Sources row {} is malformed; expected {} columns and found {}",
                row_index - 1,
                CANONICAL_SOURCE_HEADERS.len(),
                cells.len()
            ));
            continue;
        }

        if is_scaffold_row(&cells) {
            saw_scaffold_row = true;
            continue;
        }

        match parse_record(&cells) {
            Ok(record) => {
                if !seen_ids.insert(record.id.clone()) {
                    errors.push(format!("duplicate evidence source id '{}'", record.id));
                    continue;
                }
                records.push(record);
            }
            Err(message) => errors.push(format!("Sources row {}: {message}", row_index - 1)),
        }
    }

    if errors.is_empty() {
        records.sort_by(evidence_record_ordering);
        Ok(ParsedEvidenceRecords {
            records,
            saw_scaffold_row,
        })
    } else {
        errors.sort();
        errors.dedup();
        Err(errors)
    }
}

fn parse_record(cells: &[String]) -> Result<EvidenceRecord, String> {
    let id = cells[0].trim().to_string();
    if !is_canonical_source_id(&id) {
        return Err(format!(
            "invalid source id '{id}' (expected canonical form SRC-01)"
        ));
    }

    let class = EvidenceSourceClass::from_str(&cells[1])?;

    let provenance = required_cell(&cells[2], "provenance")?;
    let location = required_cell(&cells[3], "location")?;
    let observed_or_published_at = parse_date(&cells[4], "observed/published date")?;
    let retrieved_at = parse_date(&cells[5], "retrieved date")?;
    let authority = EvidenceStrength::from_str(&cells[6])?;
    let freshness = EvidenceStrength::from_str(&cells[7])?;
    let notes = required_cell(&cells[8], "notes")?;

    Ok(EvidenceRecord {
        id,
        class,
        provenance,
        location,
        observed_or_published_at,
        retrieved_at,
        authority,
        freshness,
        notes,
    })
}

fn parse_markdown_row(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        return None;
    }

    Some(
        trimmed[1..trimmed.len() - 1]
            .split('|')
            .map(|cell| cell.trim().to_string())
            .collect(),
    )
}

fn is_separator_row(line: &str) -> bool {
    parse_markdown_row(line).is_some_and(|cells| {
        !cells.is_empty()
            && cells.iter().all(|cell| {
                !cell.is_empty()
                    && cell
                        .chars()
                        .all(|ch| ch == '-' || ch == ':' || ch.is_whitespace())
            })
    })
}

fn is_scaffold_row(cells: &[String]) -> bool {
    cells.first().is_some_and(|id| id == "SRC-01")
        && cells
            .get(1)
            .is_some_and(|class| class.eq_ignore_ascii_case("web"))
        && cells
            .get(2)
            .is_some_and(|provenance| provenance == SCAFFOLD_PROVENANCE)
        && cells
            .get(3)
            .is_some_and(|location| location == SCAFFOLD_LOCATION)
        && cells.get(8).is_some_and(|notes| notes == SCAFFOLD_NOTES)
}

fn required_cell(value: &str, label: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(format!("missing required {label}"))
    } else {
        Ok(trimmed.to_string())
    }
}

fn parse_date(value: &str, label: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(value.trim(), "%Y-%m-%d")
        .map_err(|_| format!("{label} must use YYYY-MM-DD"))
}

fn is_canonical_source_id(value: &str) -> bool {
    let Some(suffix) = value.strip_prefix("SRC-") else {
        return false;
    };
    !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit())
}

fn source_id_sort_key(value: &str) -> (u32, &str) {
    let numeric = value
        .strip_prefix("SRC-")
        .and_then(|suffix| suffix.parse::<u32>().ok())
        .unwrap_or(u32::MAX);
    (numeric, value)
}

fn evidence_record_ordering(left: &EvidenceRecord, right: &EvidenceRecord) -> Ordering {
    source_id_sort_key(&left.id)
        .cmp(&source_id_sort_key(&right.id))
        .then_with(|| left.class.cmp(&right.class))
        .then_with(|| left.provenance.cmp(&right.provenance))
        .then_with(|| left.location.cmp(&right.location))
        .then_with(|| left.notes.cmp(&right.notes))
}

fn sanitize_markdown_cell(value: &str) -> String {
    value
        .lines()
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .replace('|', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_EVIDENCE: &str = r#"# Evidence

## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-02 | SOCIAL | provider:hn | https://news.ycombinator.com/item?id=1 | 2026-03-02 | 2026-03-04 | medium | high | Operators are asking for evidence-backed research outputs. |
| SRC-01 | academic | provider:arxiv | https://arxiv.org/abs/1234.5678 | 2026-02-15 | 2026-03-03 | HIGH | medium | Prior art supports lightweight local ranking. |
"#;

    #[test]
    fn evidence_record_schema_is_canonical() {
        assert!(crate::infrastructure::templates::bearing::EVIDENCE.contains(
            "| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |"
        ));

        let records = parse_evidence_records(VALID_EVIDENCE).unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].id, "SRC-01");
        assert_eq!(records[0].class, EvidenceSourceClass::Academic);
        assert_eq!(records[0].provenance, "provider:arxiv");
        assert_eq!(records[0].location, "https://arxiv.org/abs/1234.5678");
        assert_eq!(
            records[0].observed_or_published_at,
            NaiveDate::from_ymd_opt(2026, 2, 15).unwrap()
        );
        assert_eq!(
            records[0].retrieved_at,
            NaiveDate::from_ymd_opt(2026, 3, 3).unwrap()
        );
        assert_eq!(records[0].authority, EvidenceStrength::High);
        assert_eq!(records[0].freshness, EvidenceStrength::Medium);
    }

    #[test]
    fn evidence_parser_rejects_malformed_records() {
        let malformed = r#"# Evidence

## Sources

| ID | Type | Source | Observed / Published | Authority | Notes |
|----|------|--------|----------------------|-----------|-------|
| SRC-01 | web | https://example.com | 2026-01-01 | medium | Replace with the concrete claim this source supports |
"#;

        let errors = validate_evidence_document(malformed);

        assert!(
            errors
                .iter()
                .any(|message| message.contains("canonical columns"))
        );

        let temp = crate::test_helpers::TestBoardBuilder::new()
            .bearing(crate::test_helpers::TestBearing::new("BRG-01").has_evidence(true))
            .build();
        let evidence_path = temp.path().join("bearings/BRG-01/EVIDENCE.md");
        std::fs::write(
            &evidence_path,
            crate::infrastructure::templates::bearing::EVIDENCE
                .replace("{{id}}", "BRG-01")
                .replace("{{title}}", "Test Bearing"),
        )
        .unwrap();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let doctor_errors =
            crate::infrastructure::validation::bearings::check_bearing_content_sections(&board);

        assert!(
            doctor_errors
                .iter()
                .any(|problem| { problem.message.contains("scaffold/example values") })
        );
    }

    #[test]
    fn evidence_record_parsing_is_deterministic() {
        let variant_a = VALID_EVIDENCE;
        let variant_b = r#"# Evidence

## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | ACADEMIC | provider:arxiv | https://arxiv.org/abs/1234.5678 | 2026-02-15 | 2026-03-03 | high | MEDIUM | Prior art supports lightweight local ranking. |
| SRC-02 | social | provider:hn | https://news.ycombinator.com/item?id=1 | 2026-03-02 | 2026-03-04 | Medium | High | Operators are asking for evidence-backed research outputs. |
"#;

        assert_eq!(
            parse_evidence_records(variant_a).unwrap(),
            parse_evidence_records(variant_b).unwrap()
        );
    }

    #[test]
    fn parse_evidence_records_ignores_template_row_for_capture_workflows() {
        let scaffold = crate::infrastructure::template_rendering::render(
            crate::infrastructure::templates::bearing::EVIDENCE,
            &[("id", "BRG-01"), ("title", "Test Bearing")],
        );

        let records = parse_evidence_records(&scaffold).unwrap();

        assert!(records.is_empty());
        assert!(
            validate_evidence_document(&scaffold)
                .iter()
                .any(|message| message.contains("scaffold/example values"))
        );
    }

    #[test]
    fn render_evidence_records_table_uses_canonical_headers() {
        let records = parse_evidence_records(VALID_EVIDENCE).unwrap();

        let rendered = render_evidence_records_table(&records);

        assert!(rendered.starts_with(
            "| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |"
        ));
        assert!(rendered.contains("| SRC-01 | academic | provider:arxiv |"));
        assert!(rendered.contains("| SRC-02 | social | provider:hn |"));
    }
}
