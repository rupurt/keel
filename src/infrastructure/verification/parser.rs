use regex::Regex;
use std::sync::LazyLock;

// Compile each regex once on first use, then reuse forever.
static AC_REF_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[(SRS-[A-Z0-9-]+)/AC-(\d+)\]").unwrap());
static AC_REF_STRIP_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[(SRS-[A-Z0-9-]+)/AC-(\d+)\]\s*").unwrap());
static COMMENT_AC_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<!--\s*\[(SRS-[A-Z0-9-]+)/AC-(\d+)\]\s*verify:").unwrap());
static REQ_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(SRS-[A-Z0-9-]+):(start:end|start|continues|end)").unwrap());
static CONTAINS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(.+?)\s+(?:contains|~=)\s+(.+)$"#).unwrap());
static EQUALS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(.+?)\s+==\s+(.+)$").unwrap());
static PROOF_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"proof:\s*(\S+)\s*$").unwrap());

/// Comparison type for verify annotations
#[derive(Debug, Clone, PartialEq)]
pub enum Comparison {
    /// Command must exit with code 0
    Success,
    /// stdout must equal this value (trimmed)
    Equals(String),
    /// stdout must contain this substring
    Contains(String),
    /// Requires human verification
    Manual,
}

/// Phase of a requirement's implementation
#[derive(Debug, Clone, PartialEq)]
pub enum RequirementPhase {
    /// First AC implementing this requirement
    Start,
    /// Intermediate AC (neither first nor last)
    Continues,
    /// Final AC completing this requirement
    End,
    /// One-shot: single AC fully implements requirement
    StartEnd,
}

/// Reference to an SRS requirement with implementation phase
#[derive(Debug, Clone, PartialEq)]
pub struct RequirementRef {
    /// The requirement ID (e.g., "SRS-01")
    pub id: String,
    /// The phase of implementation
    pub phase: RequirementPhase,
}

/// A reference to an acceptance criterion within an SRS requirement
#[derive(Debug, Clone, PartialEq)]
pub struct AcReference {
    /// The SRS requirement ID (e.g., "SRS-01", "SRS-NFR-01")
    pub srs_id: String,
    /// The acceptance criterion number (e.g., 1 for AC-01)
    pub ac_num: u32,
}

/// A parsed verify annotation from acceptance criteria
#[derive(Debug, Clone)]
pub struct VerifyAnnotation {
    /// The acceptance criterion text
    pub criterion: String,
    /// The command to run (None for manual)
    pub command: Option<String>,
    /// How to compare the output
    pub comparison: Comparison,
    /// Optional requirement traceability
    pub requirement: Option<RequirementRef>,
    /// Optional proof artifact filename
    pub proof: Option<String>,
    /// Optional AC reference parsed from [SRS-XX/AC-YY] marker (used by traceability matrix)
    #[allow(dead_code)] // Read by future verification report commands
    pub ac_ref: Option<AcReference>,
}

/// Parse all `[SRS-XX/AC-YY]` markers from story content (used by traceability matrix)
pub fn parse_ac_references(content: &str) -> Vec<AcReference> {
    let mut refs = Vec::new();

    for caps in AC_REF_REGEX.captures_iter(content) {
        let srs_id = caps.get(1).unwrap().as_str().to_string();
        let ac_num: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        refs.push(AcReference { srs_id, ac_num });
    }

    refs
}

/// Extract an [SRS-XX/AC-YY] marker from text, returning the reference and the remaining text
fn extract_ac_reference(text: &str) -> (Option<AcReference>, String) {
    if let Some(caps) = AC_REF_STRIP_REGEX.captures(text) {
        let srs_id = caps.get(1).unwrap().as_str().to_string();
        let ac_num: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let remaining = AC_REF_STRIP_REGEX.replace(text, "").trim().to_string();
        (Some(AcReference { srs_id, ac_num }), remaining)
    } else {
        (None, text.to_string())
    }
}

/// Parse acceptance criteria and extract verify annotations
pub fn parse_verify_annotations(content: &str) -> Vec<VerifyAnnotation> {
    let mut annotations = Vec::new();

    for line in content.lines() {
        if let Some(start) = line.find("<!--") {
            let rest = &line[start + 4..];
            if let Some(end) = rest.find("-->") {
                let comment_text = rest[..end].trim();

                // Check if it's a verify annotation
                if let Some(verify_start) = comment_text.find("verify:") {
                    let verify_content = comment_text[verify_start + 7..].trim();
                    let (command, comparison, requirement, proof) =
                        parse_verify_content(verify_content);

                    // For the criterion text, take everything before the comment
                    let raw_criterion = line[..start].trim().to_string();
                    let raw_criterion = raw_criterion
                        .trim_start_matches("- [x]")
                        .trim_start_matches("- [X]")
                        .trim_start_matches("- [ ]")
                        .trim()
                        .to_string();

                    let (ac_ref, criterion) = extract_ac_reference(&raw_criterion);

                    // If not in criterion text, try inside the comment
                    let ac_ref = if ac_ref.is_some() {
                        ac_ref
                    } else if let Some(comment_caps) = COMMENT_AC_REGEX.captures(line) {
                        let srs_id = comment_caps.get(1).unwrap().as_str().to_string();
                        let ac_num: u32 = comment_caps.get(2).unwrap().as_str().parse().unwrap();
                        Some(AcReference { srs_id, ac_num })
                    } else {
                        None
                    };

                    annotations.push(VerifyAnnotation {
                        criterion,
                        command,
                        comparison,
                        requirement,
                        proof,
                        ac_ref,
                    });
                }
            }
        }
    }

    annotations
}

/// Parse the content inside <!-- verify: ... -->
fn parse_verify_content(
    content: &str,
) -> (
    Option<String>,
    Comparison,
    Option<RequirementRef>,
    Option<String>,
) {
    // Strictly split by comma.
    let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

    let mut command_part = None;
    let mut requirement = None;
    let mut proof = None;

    for part in parts {
        if part.is_empty() {
            continue;
        }

        // Try requirement ref (must contain a colon and start with SRS-)
        if part.starts_with("SRS-")
            && part.contains(':')
            && let Some(caps) = REQ_REGEX.captures(part)
        {
            let id = caps.get(1).unwrap().as_str().to_string();
            let phase_str = caps.get(2).unwrap().as_str();
            let phase = match phase_str {
                "start" => RequirementPhase::Start,
                "continues" => RequirementPhase::Continues,
                "end" => RequirementPhase::End,
                "start:end" => RequirementPhase::StartEnd,
                _ => unreachable!(),
            };
            requirement = Some(RequirementRef { id, phase });
            continue;
        }

        // Try proof
        if let Some(stripped) = part.strip_prefix("proof:") {
            proof = Some(stripped.trim().to_string());
            continue;
        }

        // Otherwise, it's the command/comparison part.
        // Only take the first non-requirement, non-proof part as the command.
        if command_part.is_none() {
            command_part = Some(part);
        }
    }

    let command_str = command_part.unwrap_or("");
    if command_str.is_empty() {
        return (None, Comparison::Success, requirement, proof);
    }

    // Handle manual
    if command_str == "manual" {
        return (None, Comparison::Manual, requirement, proof);
    }

    // Handle contains "text"
    if let Some(caps) = CONTAINS_REGEX.captures(command_str) {
        let command = caps.get(1).unwrap().as_str().trim().to_string();
        let expected = caps
            .get(2)
            .unwrap()
            .as_str()
            .trim()
            .trim_matches('"')
            .to_string();
        return (
            Some(command),
            Comparison::Contains(expected),
            requirement,
            proof,
        );
    }

    // Handle == value
    if let Some(caps) = EQUALS_REGEX.captures(command_str) {
        let command = caps.get(1).unwrap().as_str().trim().to_string();
        let expected = caps.get(2).unwrap().as_str().trim().to_string();
        return (
            Some(command),
            Comparison::Equals(expected),
            requirement,
            proof,
        );
    }

    // Default: bare command (success check)
    (
        Some(command_str.to_string()),
        Comparison::Success,
        requirement,
        proof,
    )
}
