//! Embedded templates for board entity creation
//!
//! Templates are embedded at compile time using `include_str!()`.
//! They use `{{placeholder}}` syntax for variable substitution.

/// Epic templates
pub mod epic {
    /// Epic README template
    pub const README: &str = include_str!("../../templates/epic/[name]/README.md");
    /// Epic PRD template
    pub const PRD: &str = include_str!("../../templates/epic/[name]/PRD.md");
    /// Press Release template (Working Backwards)
    pub const PRESS_RELEASE: &str = include_str!("../../templates/epic/[name]/PRESS_RELEASE.md");
}

/// Voyage templates
pub mod voyage {
    /// Voyage README template
    pub const README: &str = include_str!("../../templates/epic/[name]/voyages/[name]/README.md");
    /// Voyage SRS template
    pub const SRS: &str = include_str!("../../templates/epic/[name]/voyages/[name]/SRS.md");
    /// Voyage SDD template
    pub const SDD: &str = include_str!("../../templates/epic/[name]/voyages/[name]/SDD.md");
    /// Voyage Report template
    pub const REPORT: &str = include_str!("../../templates/voyage/VOYAGE_REPORT.md");
    /// Voyage Compliance Report template
    pub const COMPLIANCE: &str = include_str!("../../templates/voyage/COMPLIANCE_REPORT.md");
}

/// Story templates
pub mod story {
    /// Story template
    pub const STORY: &str = include_str!("../../templates/stories/[id]/README.md");
    /// Story reflection template
    pub const REFLECT: &str = include_str!("../../templates/stories/[id]/REFLECT.md");
}

/// Bearing templates
pub mod bearing {
    /// Bearing README template
    pub const README: &str = include_str!("../../templates/bearings/README.md");
    /// Bearing BRIEF template
    pub const BRIEF: &str = include_str!("../../templates/bearings/BRIEF.md");
    /// Bearing SURVEY template
    pub const SURVEY: &str = include_str!("../../templates/bearings/SURVEY.md");
    /// Bearing ASSESSMENT template
    pub const ASSESSMENT: &str = include_str!("../../templates/bearings/ASSESSMENT.md");
}

/// ADR templates
pub mod adr {
    /// ADR template
    pub const ADR: &str = include_str!("../../templates/adrs/ADR.md");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    const CLI_OWNED_TOKENS: &[&str] = &[
        "applies_to",
        "context",
        "epic",
        "goal",
        "problem",
        "knowledge_example_id",
        "title",
        "type",
    ];
    const SYSTEM_OWNED_TOKENS: &[&str] = &[
        "created_at",
        "decided_at",
        "id",
        "index",
        "status",
        "updated_at",
    ];
    const GENERATED_TOKENS: &[&str] = &[
        "done_count",
        "epic_id",
        "matrix",
        "narrative",
        "total_count",
    ];

    fn extract_tokens(template: &str) -> BTreeSet<String> {
        let mut tokens = BTreeSet::new();
        let mut cursor = template;

        while let Some(start) = cursor.find("{{") {
            let after_start = &cursor[start + 2..];
            let Some(end) = after_start.find("}}") else {
                break;
            };

            let token = &after_start[..end];
            if !token.trim().is_empty() {
                tokens.insert(token.to_string());
            }

            cursor = &after_start[end + 2..];
        }

        tokens
    }

    fn token_set(items: &[&str]) -> BTreeSet<String> {
        items.iter().map(|token| (*token).to_string()).collect()
    }

    #[test]
    fn epic_readme_contains_placeholders() {
        assert!(epic::README.contains("{{id}}"));
        assert!(epic::README.contains("{{title}}"));
        assert!(epic::README.contains("{{created_at}}"));
    }

    #[test]
    fn epic_readme_uses_created_at_field() {
        assert!(
            epic::README.contains("created_at:"),
            "Epic README should use created_at field"
        );
        assert!(
            !epic::README.contains("created:"),
            "Epic README should not use created field (renamed to created_at)"
        );
    }

    #[test]
    fn story_reflect_contains_created_at_placeholder() {
        assert!(story::REFLECT.contains("{{created_at}}"));
        assert!(story::REFLECT.contains("created_at:"));
    }

    #[test]
    fn epic_readme_has_generated_section_markers() {
        // Epic README needs markers for auto-generated voyages section
        assert!(
            epic::README.contains("<!-- BEGIN GENERATED -->"),
            "Epic README should have BEGIN GENERATED marker for voyages"
        );
        assert!(
            epic::README.contains("<!-- END GENERATED -->"),
            "Epic README should have END GENERATED marker for voyages"
        );
    }

    #[test]
    fn epic_readme_has_voyages_section() {
        assert!(
            epic::README.contains("## Voyages"),
            "Epic README should have Voyages section header"
        );
        assert!(
            !epic::README.contains("## Milestones"),
            "Epic README should not use Milestones (renamed to Voyages)"
        );
    }

    #[test]
    fn epic_readme_has_initial_progress() {
        // New epics should show progress even before generation runs
        assert!(
            epic::README.contains("**Progress:**"),
            "Epic README should have initial progress line"
        );
        assert!(
            epic::README.contains("0/0 stories done"),
            "Epic README should show 0/0 stories done initially"
        );
    }

    #[test]
    fn epic_readme_has_voyage_table_template() {
        // New epics should have voyage table matching generator format
        assert!(
            epic::README.contains("| Voyage | Status | Stories |"),
            "Epic README should have voyage table header"
        );
        assert!(
            epic::README.contains("|--------|--------|---------|"),
            "Epic README should have voyage table separator"
        );
        assert!(
            epic::README.contains("voyage-id"),
            "Epic README should have example voyage row"
        );
    }

    #[test]
    fn epic_prd_contains_title_placeholder() {
        assert!(epic::PRD.contains("{{title}}"));
    }

    #[test]
    fn epic_readme_contains_problem_placeholder() {
        assert!(
            epic::README.contains("{{problem}}"),
            "Epic README should have problem placeholder for epic context"
        );
    }

    #[test]
    fn epic_prd_contains_problem_placeholder() {
        assert!(
            epic::PRD.contains("{{problem}}"),
            "Epic PRD should have problem placeholder for epic context"
        );
    }

    #[test]
    fn epic_prd_has_verification_strategy_section() {
        assert!(
            epic::PRD.contains("## Verification Strategy"),
            "PRD should include a verification strategy section"
        );
    }

    #[test]
    fn epic_prd_does_not_embed_voyage_breakdown() {
        assert!(
            !epic::PRD.contains("## Voyages"),
            "PRD should not include a voyages section"
        );
    }

    #[test]
    fn epic_prd_has_no_default_todo_placeholders() {
        assert!(
            !epic::PRD.contains("TODO:"),
            "Epic PRD template should not ship with TODO placeholders"
        );
        assert!(
            !epic::PRD.contains("{{TODO:"),
            "Epic PRD template should not ship with TODO token placeholders"
        );
    }

    #[test]
    fn epic_press_release_has_no_default_todo_placeholders() {
        assert!(
            !epic::PRESS_RELEASE.contains("TODO:"),
            "Epic press release template should not ship with TODO placeholders"
        );
        assert!(
            !epic::PRESS_RELEASE.contains("{{TODO:"),
            "Epic press release template should not ship with TODO token placeholders"
        );
    }

    #[test]
    fn voyage_readme_contains_placeholders() {
        assert!(voyage::README.contains("{{id}}"));
        assert!(voyage::README.contains("{{title}}"));
        assert!(voyage::README.contains("{{created_at}}"));
    }

    #[test]
    fn voyage_readme_uses_created_at_field() {
        assert!(
            voyage::README.contains("created_at:"),
            "Voyage README should use created_at field"
        );
        assert!(
            !voyage::README.contains("\ncreated:"),
            "Voyage README should not use created field (renamed to created_at)"
        );
    }

    #[test]
    fn voyage_readme_has_epic_field() {
        assert!(
            voyage::README.contains("epic:"),
            "Voyage README should have epic field in frontmatter"
        );
    }

    #[test]
    fn voyage_readme_has_documents_section() {
        assert!(
            voyage::README.contains("## Documents"),
            "Voyage README should have Documents section"
        );
        assert!(
            voyage::README.contains("[SRS.md](SRS.md)"),
            "Voyage README should link to SRS.md"
        );
        assert!(
            voyage::README.contains("[SDD.md](SDD.md)"),
            "Voyage README should link to SDD.md"
        );
    }

    #[test]
    fn voyage_readme_has_generated_section() {
        assert!(
            voyage::README.contains("<!-- BEGIN GENERATED -->"),
            "Voyage README should have BEGIN GENERATED marker"
        );
        assert!(
            voyage::README.contains("<!-- END GENERATED -->"),
            "Voyage README should have END GENERATED marker"
        );
    }

    #[test]
    fn voyage_readme_contains_goal_placeholder() {
        assert!(
            voyage::README.contains("{{goal}}"),
            "Voyage README should have goal placeholder for value proposition"
        );
    }

    #[test]
    fn voyage_srs_contains_goal_placeholder() {
        assert!(
            voyage::SRS.contains("{{goal}}"),
            "Voyage SRS should have goal placeholder"
        );
    }

    #[test]
    fn voyage_sdd_contains_goal_placeholder() {
        assert!(
            voyage::SDD.contains("{{goal}}"),
            "Voyage SDD should have goal placeholder"
        );
    }

    #[test]
    fn voyage_srs_contains_placeholders() {
        assert!(voyage::SRS.contains("{{title}}"));
        assert!(voyage::SRS.contains("{{epic}}"));
    }

    #[test]
    fn voyage_srs_has_no_default_todo_placeholders() {
        assert!(
            !voyage::SRS.contains("TODO:"),
            "Voyage SRS template should not ship with TODO placeholders"
        );
        assert!(
            !voyage::SRS.contains("{{TODO:"),
            "Voyage SRS template should not ship with TODO token placeholders"
        );
    }

    #[test]
    fn voyage_sdd_contains_title_placeholder() {
        assert!(voyage::SDD.contains("{{title}}"));
    }

    #[test]
    fn story_template_contains_placeholders() {
        assert!(story::STORY.contains("{{id}}"));
        assert!(story::STORY.contains("{{title}}"));
        assert!(story::STORY.contains("{{type}}"));
        assert!(story::STORY.contains("{{created_at}}"));
        assert!(story::STORY.contains("{{updated_at}}"));
    }

    #[test]
    fn story_reflect_template_contains_placeholders() {
        assert!(story::REFLECT.contains("{{title}}"));
        assert!(story::REFLECT.contains("{{knowledge_example_id}}"));
    }

    #[test]
    fn templates_have_frontmatter() {
        // Epic README has frontmatter
        assert!(epic::README.starts_with("---\n"));
        assert!(epic::README.contains("\n---\n"));

        // Voyage README has frontmatter
        assert!(voyage::README.starts_with("---\n"));
        assert!(voyage::README.contains("\n---\n"));

        // Story has frontmatter
        assert!(story::STORY.starts_with("---\n"));
        assert!(story::STORY.contains("\n---\n"));
    }

    #[test]
    fn bearing_readme_contains_placeholders() {
        assert!(bearing::README.contains("{{id}}"));
        assert!(bearing::README.contains("{{title}}"));
        assert!(bearing::README.contains("{{created_at}}"));
    }

    #[test]
    fn planning_templates_do_not_use_legacy_date_tokens() {
        let planning_templates = [
            epic::README,
            voyage::README,
            story::STORY,
            bearing::README,
            adr::ADR,
        ];

        for template in planning_templates {
            assert!(
                !template.contains("{{date}}"),
                "planning template must not use legacy {{date}} token"
            );
            assert!(
                !template.contains("{{datetime}}"),
                "planning template must not use legacy {{datetime}} token"
            );
        }
    }

    #[test]
    fn bearing_readme_uses_created_at_field() {
        assert!(
            bearing::README.contains("created_at:"),
            "Bearing README should use created_at field"
        );
    }

    #[test]
    fn bearing_brief_contains_no_frontmatter() {
        assert!(
            !bearing::BRIEF.starts_with("---\n"),
            "Bearing BRIEF should not have frontmatter"
        );
    }

    #[test]
    fn bearing_survey_contains_placeholders() {
        assert!(bearing::SURVEY.contains("{{id}}"));
        assert!(bearing::SURVEY.contains("{{title}}"));
    }

    #[test]
    fn bearing_assessment_contains_placeholders() {
        assert!(bearing::ASSESSMENT.contains("{{id}}"));
        assert!(bearing::ASSESSMENT.contains("{{title}}"));
    }

    #[test]
    fn bearing_assessment_has_scoring_table() {
        assert!(bearing::ASSESSMENT.contains("| Factor | Score"));
        assert!(bearing::ASSESSMENT.contains("| Impact |"));
        assert!(bearing::ASSESSMENT.contains("| Confidence |"));
        assert!(bearing::ASSESSMENT.contains("| Effort |"));
        assert!(bearing::ASSESSMENT.contains("| Risk |"));
    }

    #[test]
    fn template_tokens_match_known_bucket_inventory() {
        let known_tokens: BTreeSet<String> = CLI_OWNED_TOKENS
            .iter()
            .chain(SYSTEM_OWNED_TOKENS)
            .chain(GENERATED_TOKENS)
            .map(|token| (*token).to_string())
            .collect();

        let templates = [
            ("epic README", epic::README),
            ("epic PRD", epic::PRD),
            ("epic PRESS_RELEASE", epic::PRESS_RELEASE),
            ("voyage README", voyage::README),
            ("voyage SRS", voyage::SRS),
            ("voyage SDD", voyage::SDD),
            ("voyage REPORT", voyage::REPORT),
            ("voyage COMPLIANCE", voyage::COMPLIANCE),
            ("story README", story::STORY),
            ("story REFLECT", story::REFLECT),
            ("bearing README", bearing::README),
            ("bearing BRIEF", bearing::BRIEF),
            ("bearing SURVEY", bearing::SURVEY),
            ("bearing ASSESSMENT", bearing::ASSESSMENT),
            ("adr", adr::ADR),
        ];

        for (label, template) in templates {
            let tokens = extract_tokens(template);
            let unknown: Vec<String> = tokens.difference(&known_tokens).cloned().collect();
            assert!(
                unknown.is_empty(),
                "{label} has unknown template tokens: {:?}; allowed buckets are CLI={:?}, SYSTEM={:?}, GENERATED={:?}",
                unknown,
                CLI_OWNED_TOKENS,
                SYSTEM_OWNED_TOKENS,
                GENERATED_TOKENS
            );
        }
    }

    #[test]
    fn planning_templates_reject_generated_bucket_tokens() {
        let generated_tokens = token_set(GENERATED_TOKENS);
        let planning_templates = [
            ("epic README", epic::README),
            ("epic PRD", epic::PRD),
            ("epic PRESS_RELEASE", epic::PRESS_RELEASE),
            ("voyage README", voyage::README),
            ("voyage SRS", voyage::SRS),
            ("voyage SDD", voyage::SDD),
            ("story README", story::STORY),
            ("story REFLECT", story::REFLECT),
            ("bearing README", bearing::README),
            ("bearing BRIEF", bearing::BRIEF),
            ("bearing SURVEY", bearing::SURVEY),
            ("bearing ASSESSMENT", bearing::ASSESSMENT),
            ("adr", adr::ADR),
        ];

        for (label, template) in planning_templates {
            let tokens = extract_tokens(template);
            let out_of_bucket: Vec<String> =
                tokens.intersection(&generated_tokens).cloned().collect();
            assert!(
                out_of_bucket.is_empty(),
                "{label} uses generated-bucket tokens {:?}; planning templates may only use CLI-owned or system-owned tokens",
                out_of_bucket
            );
        }
    }
}
