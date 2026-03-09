//! Canonical research capture workflow for bearing evidence.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::NaiveDate;

use crate::infrastructure::bearing_evidence::{
    EvidenceRecord, EvidenceSourceClass, EvidenceStrength, parse_evidence_records,
    render_evidence_records_table,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResearchCaptureRequest {
    pub class: EvidenceSourceClass,
    pub provider: Option<String>,
    pub location: String,
    pub observed_or_published_at: NaiveDate,
    pub retrieved_at: NaiveDate,
    pub authority: EvidenceStrength,
    pub freshness: EvidenceStrength,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResearchCaptureResult {
    pub appended_records: Vec<EvidenceRecord>,
    pub all_records: Vec<EvidenceRecord>,
}

/// Capture one or more evidence sources into an `EVIDENCE.md` file.
pub fn capture_research_evidence_file(
    evidence_path: &Path,
    requests: &[ResearchCaptureRequest],
) -> Result<ResearchCaptureResult> {
    let content = fs::read_to_string(evidence_path)
        .with_context(|| format!("Failed to read {}", evidence_path.display()))?;
    let (updated, result) = capture_research_evidence_document(&content, requests)?;
    fs::write(evidence_path, updated)
        .with_context(|| format!("Failed to write {}", evidence_path.display()))?;
    Ok(result)
}

/// Normalize one canonical capture batch into `EVIDENCE.md` content.
pub fn capture_research_evidence_document(
    content: &str,
    requests: &[ResearchCaptureRequest],
) -> Result<(String, ResearchCaptureResult)> {
    if requests.is_empty() {
        return Err(anyhow!("at least one research capture request is required"));
    }

    let mut records =
        parse_evidence_records(content).map_err(|errors| anyhow!(errors.join("\n")))?;
    let next_id = next_source_index(&records) + 1;
    let appended_records = requests
        .iter()
        .enumerate()
        .map(|(offset, request)| request_to_record(next_id + offset as u32, request))
        .collect::<Vec<_>>();
    records.extend(appended_records.iter().cloned());

    let updated = replace_heading_section(
        content,
        "## Sources",
        &render_evidence_records_table(&records),
    )?;
    let all_records =
        parse_evidence_records(&updated).map_err(|errors| anyhow!(errors.join("\n")))?;

    Ok((
        updated,
        ResearchCaptureResult {
            appended_records,
            all_records,
        },
    ))
}

fn request_to_record(index: u32, request: &ResearchCaptureRequest) -> EvidenceRecord {
    EvidenceRecord {
        id: format!("SRC-{index:02}"),
        class: request.class.clone(),
        provenance: normalize_provenance(request.provider.as_deref(), &request.class),
        location: sanitize_field(&request.location),
        observed_or_published_at: request.observed_or_published_at,
        retrieved_at: request.retrieved_at,
        authority: request.authority.clone(),
        freshness: request.freshness.clone(),
        notes: sanitize_field(&request.notes),
    }
}

fn normalize_provenance(provider: Option<&str>, class: &EvidenceSourceClass) -> String {
    let Some(provider) = provider
        .map(str::trim)
        .filter(|provider| !provider.is_empty())
    else {
        return format!("provider:{class}");
    };

    if provider.contains(':') {
        provider.to_string()
    } else {
        format!("provider:{provider}")
    }
}

fn sanitize_field(value: &str) -> String {
    value
        .lines()
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .replace('|', "/")
}

fn next_source_index(records: &[EvidenceRecord]) -> u32 {
    records
        .iter()
        .filter_map(|record| {
            record
                .id
                .strip_prefix("SRC-")
                .and_then(|suffix| suffix.parse::<u32>().ok())
        })
        .max()
        .unwrap_or(0)
}

fn replace_heading_section(content: &str, heading: &str, new_body: &str) -> Result<String> {
    let mut result = String::new();
    let heading_level = heading.chars().take_while(|ch| *ch == '#').count();
    let lines: Vec<&str> = content.lines().collect();
    let had_trailing_newline = content.ends_with('\n');
    let mut replaced = false;
    let mut index = 0usize;

    while index < lines.len() {
        let line = lines[index];
        if line.starts_with(heading) {
            replaced = true;
            result.push_str(line);
            result.push('\n');
            result.push('\n');
            result.push_str(new_body.trim_end());
            result.push('\n');
            index += 1;
            while index < lines.len() {
                let candidate = lines[index];
                if candidate.starts_with('#') {
                    let level = candidate.chars().take_while(|ch| *ch == '#').count();
                    if level <= heading_level {
                        break;
                    }
                }
                index += 1;
            }
            continue;
        }

        result.push_str(line);
        if index < lines.len() - 1 || had_trailing_newline {
            result.push('\n');
        }
        index += 1;
    }

    if replaced {
        Ok(result)
    } else {
        Err(anyhow!("missing required '## Sources' section"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bearing_evidence::parse_evidence_records;

    fn evidence_template() -> String {
        crate::infrastructure::template_rendering::render(
            crate::infrastructure::templates::bearing::EVIDENCE,
            &[("id", "BRG-01"), ("title", "Test Bearing")],
        )
    }

    fn request(
        class: EvidenceSourceClass,
        provider: Option<&str>,
        location: &str,
        notes: &str,
    ) -> ResearchCaptureRequest {
        ResearchCaptureRequest {
            class,
            provider: provider.map(str::to_string),
            location: location.to_string(),
            observed_or_published_at: NaiveDate::from_ymd_opt(2026, 3, 8).unwrap(),
            retrieved_at: NaiveDate::from_ymd_opt(2026, 3, 9).unwrap(),
            authority: EvidenceStrength::High,
            freshness: EvidenceStrength::Medium,
            notes: notes.to_string(),
        }
    }

    #[test]
    fn research_capture_persists_provenance_for_all_signal_classes() {
        let (updated, result) = capture_research_evidence_document(
            &evidence_template(),
            &[
                request(
                    EvidenceSourceClass::Web,
                    Some("web-search"),
                    "https://example.com/web",
                    "Web finding",
                ),
                request(
                    EvidenceSourceClass::Academic,
                    Some("arxiv"),
                    "https://arxiv.org/abs/1234.5678",
                    "Academic finding",
                ),
                request(
                    EvidenceSourceClass::Social,
                    Some("social-trends"),
                    "https://news.ycombinator.com/item?id=42",
                    "Social finding",
                ),
                request(
                    EvidenceSourceClass::Manual,
                    Some("manual:internal-note"),
                    "docs/internal/research.md",
                    "Manual finding",
                ),
            ],
        )
        .unwrap();

        assert_eq!(result.appended_records.len(), 4);
        assert_eq!(
            result
                .appended_records
                .iter()
                .map(|record| record.id.as_str())
                .collect::<Vec<_>>(),
            vec!["SRC-01", "SRC-02", "SRC-03", "SRC-04"]
        );
        assert_eq!(result.appended_records[0].provenance, "provider:web-search");
        assert_eq!(result.appended_records[1].provenance, "provider:arxiv");
        assert_eq!(
            result.appended_records[2].provenance,
            "provider:social-trends"
        );
        assert_eq!(
            result.appended_records[3].provenance,
            "manual:internal-note"
        );

        let records = parse_evidence_records(&updated).unwrap();
        assert_eq!(records, result.all_records);
        assert!(!updated.contains("Replace with the concrete claim this source supports"));
    }
}
