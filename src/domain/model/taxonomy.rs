//! Role taxonomy parser for strings like `engineer/software:infra~methodical@L5#oncall`

/// Represents a parsed role taxonomy string.
///
/// The taxonomy format is: `role[/specialization][:tags][~style][@level][#context]`
///
/// Only `role` is required; all other fields are optional.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RoleTaxonomy {
    /// The base role (required). Example: "engineer"
    pub role: String,
    /// Optional specialization after `/`. Example: "software"
    pub specialization: Option<String>,
    /// Optional comma-separated tags after `:`. Example: ["infra", "security"]
    pub tags: Vec<String>,
    /// Optional style modifier after `~`. Example: "methodical"
    pub style: Option<String>,
    /// Optional level after `@`. Example: "L5"
    pub level: Option<String>,
    /// Optional context after `#`. Example: "oncall"
    pub context: Option<String>,
}

/// Error type for taxonomy parsing failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Role is required but was empty
    EmptyRole,
    /// Invalid character found in input
    InvalidCharacter(char),
    /// Duplicate modifier prefix found
    DuplicateModifier(char),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyRole => write!(f, "role is required"),
            ParseError::InvalidCharacter(c) => write!(f, "invalid character '{}'", c),
            ParseError::DuplicateModifier(c) => write!(f, "duplicate modifier '{}'", c),
        }
    }
}

impl std::error::Error for ParseError {}

impl RoleTaxonomy {
    /// Check if self (actor) can work on a story requiring `story_role`.
    ///
    /// Returns true if the actor has at least all capabilities the story requires
    /// (superset matching: actor ⊇ story).
    pub fn matches(&self, story_role: &RoleTaxonomy) -> bool {
        // Role must match exactly
        if self.role != story_role.role {
            return false;
        }

        // Specialization must match if story requires it
        if let Some(ref spec) = story_role.specialization
            && self.specialization.as_ref() != Some(spec)
        {
            return false;
        }

        // Actor must have ALL story's tags (superset)
        if !story_role.tags.iter().all(|t| self.tags.contains(t)) {
            return false;
        }

        // Style must match if story requires it
        if story_role.style.is_some() && self.style != story_role.style {
            return false;
        }

        // Level must match if story requires it
        if story_role.level.is_some() && self.level != story_role.level {
            return false;
        }

        // Context must match if story requires it
        if story_role.context.is_some() && self.context != story_role.context {
            return false;
        }

        true
    }
}

/// Characters allowed in role, specialization, and modifier values
fn is_valid_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_'
}

/// Parse a taxonomy string into a RoleTaxonomy struct.
///
/// Format: `role[/specialization][:tags][~style][@level][#context]`
pub fn parse(input: &str) -> Result<RoleTaxonomy, ParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(ParseError::EmptyRole);
    }

    // Find the first modifier delimiter to separate role/spec from modifiers
    let modifier_chars = [':', '~', '@', '#'];
    let first_modifier_pos = input
        .char_indices()
        .find(|(_, c)| modifier_chars.contains(c))
        .map(|(i, _)| i);

    // Split into base (role/spec) and modifiers
    let (base, modifiers_str) = match first_modifier_pos {
        Some(pos) => (&input[..pos], &input[pos..]),
        None => (input, ""),
    };

    // Validate base characters (role and specialization)
    for c in base.chars() {
        if c != '/' && !is_valid_identifier_char(c) {
            return Err(ParseError::InvalidCharacter(c));
        }
    }

    // Parse role and optional specialization from base
    let (role, specialization) = if let Some(slash_pos) = base.find('/') {
        let role = &base[..slash_pos];
        let spec = &base[slash_pos + 1..];
        (role.to_string(), Some(spec.to_string()))
    } else {
        (base.to_string(), None)
    };

    // Parse modifiers
    let mut tags = Vec::new();
    let mut style = None;
    let mut level = None;
    let mut context = None;

    // Track which modifiers we've seen to detect duplicates
    let mut seen_modifiers = Vec::new();

    if !modifiers_str.is_empty() {
        let mut remaining = modifiers_str;

        while !remaining.is_empty() {
            let prefix = remaining.chars().next().unwrap();
            remaining = &remaining[1..]; // skip the prefix

            // Check for duplicate modifier
            if seen_modifiers.contains(&prefix) {
                return Err(ParseError::DuplicateModifier(prefix));
            }
            seen_modifiers.push(prefix);

            // Find where this modifier ends (next delimiter or end)
            let end_pos = remaining
                .char_indices()
                .find(|(_, c)| modifier_chars.contains(c))
                .map(|(i, _)| i)
                .unwrap_or(remaining.len());

            let value = &remaining[..end_pos];
            remaining = &remaining[end_pos..];

            // Validate modifier value characters
            for c in value.chars() {
                if c != ',' && !is_valid_identifier_char(c) {
                    return Err(ParseError::InvalidCharacter(c));
                }
            }

            match prefix {
                ':' => {
                    tags = value.split(',').map(|s| s.to_string()).collect();
                }
                '~' => {
                    style = Some(value.to_string());
                }
                '@' => {
                    level = Some(value.to_string());
                }
                '#' => {
                    context = Some(value.to_string());
                }
                _ => {} // ignore unknown
            }
        }
    }

    Ok(RoleTaxonomy {
        role,
        specialization,
        tags,
        style,
        level,
        context,
    })
}

/// Check if an actor with given role taxonomy can work on a story.
///
/// Returns `true` if the story has no role requirement (None) or if the
/// actor's capabilities are a superset of the story's requirements.
/// Returns `false` if the story's role syntax is invalid.
pub fn actor_matches_story(actor: &RoleTaxonomy, story: &crate::domain::model::Story) -> bool {
    match story.required_role() {
        None => true,
        Some(Ok(story_role)) => actor.matches(&story_role),
        Some(Err(_)) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Story 1: Struct field tests =====

    // [SRS-01/AC-01] RoleTaxonomy struct has `role: String` field (required)
    #[test]
    fn has_role_field() {
        let taxonomy = RoleTaxonomy {
            role: "engineer".to_string(),
            ..Default::default()
        };
        assert_eq!(taxonomy.role, "engineer");
    }

    // [SRS-02/AC-01] RoleTaxonomy struct has `specialization: Option<String>` field
    #[test]
    fn has_specialization_field() {
        let taxonomy = RoleTaxonomy {
            role: "engineer".to_string(),
            specialization: Some("software".to_string()),
            ..Default::default()
        };
        assert_eq!(taxonomy.specialization, Some("software".to_string()));
    }

    // [SRS-03/AC-01] RoleTaxonomy struct has `tags: Vec<String>` field
    #[test]
    fn has_tags_field() {
        let taxonomy = RoleTaxonomy {
            role: "engineer".to_string(),
            tags: vec!["infra".to_string(), "security".to_string()],
            ..Default::default()
        };
        assert_eq!(taxonomy.tags, vec!["infra", "security"]);
    }

    // [SRS-04/AC-01] RoleTaxonomy struct has `style: Option<String>` field
    #[test]
    fn has_style_field() {
        let taxonomy = RoleTaxonomy {
            role: "engineer".to_string(),
            style: Some("methodical".to_string()),
            ..Default::default()
        };
        assert_eq!(taxonomy.style, Some("methodical".to_string()));
    }

    // [SRS-05/AC-01] RoleTaxonomy struct has `level: Option<String>` field
    #[test]
    fn has_level_field() {
        let taxonomy = RoleTaxonomy {
            role: "engineer".to_string(),
            level: Some("L5".to_string()),
            ..Default::default()
        };
        assert_eq!(taxonomy.level, Some("L5".to_string()));
    }

    // [SRS-06/AC-01] RoleTaxonomy struct has `context: Option<String>` field
    #[test]
    fn has_context_field() {
        let taxonomy = RoleTaxonomy {
            role: "engineer".to_string(),
            context: Some("oncall".to_string()),
            ..Default::default()
        };
        assert_eq!(taxonomy.context, Some("oncall".to_string()));
    }

    // ===== Story 2: Parse role and specialization =====

    // [SRS-01/AC-02] parse("engineer") returns RoleTaxonomy with role="engineer"
    #[test]
    fn parse_role_only() {
        let result = parse("engineer").unwrap();
        assert_eq!(result.role, "engineer");
        assert_eq!(result.specialization, None);
    }

    // [SRS-02/AC-02] parse("engineer/software") returns role="engineer", specialization="software"
    #[test]
    fn parse_role_with_specialization() {
        let result = parse("engineer/software").unwrap();
        assert_eq!(result.role, "engineer");
        assert_eq!(result.specialization, Some("software".to_string()));
    }

    // [SRS-08/AC-01] parse("") returns an error (role is required)
    #[test]
    fn parse_empty_returns_error() {
        let result = parse("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::EmptyRole);
    }

    // ===== Story 3: Parse optional modifiers =====

    // [SRS-03/AC-02] parse("engineer:infrastructure") extracts tags=["infrastructure"]
    #[test]
    fn parse_single_tag() {
        let result = parse("engineer:infrastructure").unwrap();
        assert_eq!(result.role, "engineer");
        assert_eq!(result.tags, vec!["infrastructure"]);
    }

    // [SRS-03/AC-03] parse("engineer:infra,security") extracts tags=["infra","security"]
    #[test]
    fn parse_multiple_tags() {
        let result = parse("engineer:infra,security").unwrap();
        assert_eq!(result.role, "engineer");
        assert_eq!(result.tags, vec!["infra", "security"]);
    }

    // [SRS-04/AC-02] parse("engineer~methodical") extracts style="methodical"
    #[test]
    fn parse_style() {
        let result = parse("engineer~methodical").unwrap();
        assert_eq!(result.role, "engineer");
        assert_eq!(result.style, Some("methodical".to_string()));
    }

    // [SRS-05/AC-02] parse("engineer@L5") extracts level="L5"
    #[test]
    fn parse_level() {
        let result = parse("engineer@L5").unwrap();
        assert_eq!(result.role, "engineer");
        assert_eq!(result.level, Some("L5".to_string()));
    }

    // [SRS-06/AC-02] parse("engineer#oncall") extracts context="oncall"
    #[test]
    fn parse_context() {
        let result = parse("engineer#oncall").unwrap();
        assert_eq!(result.role, "engineer");
        assert_eq!(result.context, Some("oncall".to_string()));
    }

    // [SRS-07/AC-01] Full string parses correctly
    #[test]
    fn parse_full_taxonomy_string() {
        let result = parse("engineer/software:infra~methodical@L5#oncall").unwrap();
        assert_eq!(result.role, "engineer");
        assert_eq!(result.specialization, Some("software".to_string()));
        assert_eq!(result.tags, vec!["infra"]);
        assert_eq!(result.style, Some("methodical".to_string()));
        assert_eq!(result.level, Some("L5".to_string()));
        assert_eq!(result.context, Some("oncall".to_string()));
    }

    // ===== Story 4: Descriptive error messages =====

    // [SRS-08/AC-02] Empty string error mentions "role is required"
    #[test]
    fn error_message_for_empty_role() {
        let result = parse("");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("role is required"),
            "Error message should mention 'role is required', got: {}",
            err_msg
        );
    }

    // [SRS-08/AC-03] Invalid character error shows the problematic character
    #[test]
    fn error_shows_invalid_character() {
        let result = parse("engineer!invalid");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("'!'"),
            "Error message should show the invalid character '!', got: {}",
            err_msg
        );
    }

    // [SRS-08/AC-04] Duplicate modifier error identifies which modifier
    #[test]
    fn error_shows_duplicate_modifier() {
        let result = parse("engineer:tags1:tags2");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("':'"),
            "Error message should show the duplicate modifier ':', got: {}",
            err_msg
        );
    }

    // ===== RoleTaxonomy::matches() tests =====

    // [SRS-02/AC-01] matches() returns bool
    // [SRS-02/AC-02] Returns true when roles match and story has no other requirements
    #[test]
    fn matches_same_role_only() {
        let actor = parse("engineer").unwrap();
        let story = parse("engineer").unwrap();
        assert!(actor.matches(&story));
    }

    // [SRS-03/AC-01] Returns false when roles differ
    #[test]
    fn matches_fails_different_role() {
        let actor = parse("engineer").unwrap();
        let story = parse("designer").unwrap();
        assert!(!actor.matches(&story));
    }

    // [SRS-04/AC-01] Returns false when story requires specialization actor lacks
    #[test]
    fn matches_fails_missing_specialization() {
        let actor = parse("engineer").unwrap();
        let story = parse("engineer/software").unwrap();
        assert!(!actor.matches(&story));
    }

    // [SRS-05/AC-01] Returns false when story requires tag actor lacks
    #[test]
    fn matches_fails_missing_tag() {
        let actor = parse("engineer:infra").unwrap();
        let story = parse("engineer:infra,security").unwrap();
        assert!(!actor.matches(&story));
    }

    // [SRS-06/AC-01] Returns false when story requires style actor lacks
    #[test]
    fn matches_fails_missing_style() {
        let actor = parse("engineer").unwrap();
        let story = parse("engineer~methodical").unwrap();
        assert!(!actor.matches(&story));
    }

    // [SRS-07/AC-01] Returns false when story requires level actor lacks
    #[test]
    fn matches_fails_missing_level() {
        let actor = parse("engineer").unwrap();
        let story = parse("engineer@L5").unwrap();
        assert!(!actor.matches(&story));
    }

    // [SRS-08/AC-01] Returns false when story requires context actor lacks
    #[test]
    fn matches_fails_missing_context() {
        let actor = parse("engineer").unwrap();
        let story = parse("engineer#oncall").unwrap();
        assert!(!actor.matches(&story));
    }

    // [SRS-09/AC-01] Returns true when actor has MORE capabilities than story requires
    #[test]
    fn matches_actor_superset() {
        let actor = parse("engineer/software:infra,security~methodical@L5#oncall").unwrap();
        let story = parse("engineer/software:infra").unwrap();
        assert!(actor.matches(&story));
    }

    // ===== Story 5: actor_matches_story convenience function =====

    // [SRS-01/AC-01] actor_matches_story(actor, story) -> bool exists
    // [SRS-01/AC-02] Returns true when story has no role requirement
    #[test]
    fn actor_matches_story_no_role_returns_true() {
        use crate::test_helpers::StoryFactory;
        let actor = parse("engineer").unwrap();
        let story = StoryFactory::new("S1").build();
        assert!(actor_matches_story(&actor, &story));
    }

    // [SRS-02/AC-01] Returns true when story role matches actor
    #[test]
    fn actor_matches_story_matching_role() {
        use crate::test_helpers::StoryFactory;
        let actor = parse("engineer/software:infra").unwrap();
        let story = StoryFactory::new("S2").role("engineer/software").build();
        assert!(actor_matches_story(&actor, &story));
    }

    // [SRS-03/AC-01] Returns false when story role doesn't match actor
    #[test]
    fn actor_matches_story_non_matching_role() {
        use crate::test_helpers::StoryFactory;
        let actor = parse("designer").unwrap();
        let story = StoryFactory::new("S3").role("engineer").build();
        assert!(!actor_matches_story(&actor, &story));
    }

    // [SRS-03/AC-02] Returns false when story has invalid role syntax
    #[test]
    fn actor_matches_story_invalid_syntax_returns_false() {
        use crate::test_helpers::StoryFactory;
        let actor = parse("engineer").unwrap();
        let story = StoryFactory::new("S4").role("").build();
        assert!(!actor_matches_story(&actor, &story));
    }
}
