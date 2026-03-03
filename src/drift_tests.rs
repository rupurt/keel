//! Drift detection tests for dual-truth pairs in the keel codebase.
//!
//! When a system has two representations of the same truth, they will drift.
//! These tests catch structural divergence the moment it happens.

use std::collections::HashSet;

/// Extract YAML frontmatter keys from a template string.
fn extract_frontmatter_keys(template: &str) -> HashSet<String> {
    let without_prefix = template
        .strip_prefix("---\n")
        .expect("template should start with ---");
    let frontmatter = without_prefix
        .split("\n---")
        .next()
        .expect("template should have closing ---");

    frontmatter
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| line.split(':').next())
        .map(|key| key.trim().to_string())
        .collect()
}

/// Extract table header columns from a markdown table line.
fn extract_table_columns(content: &str) -> Vec<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('|') && !trimmed.contains("---") {
            return trimmed
                .split('|')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }
    vec![]
}

/// Get serialized field names from a serde Serialize type.
fn serde_field_names<T: serde::Serialize>(value: &T) -> HashSet<String> {
    let yaml = serde_yaml::to_value(value).unwrap();
    match yaml {
        serde_yaml::Value::Mapping(map) => map
            .keys()
            .filter_map(|k| k.as_str().map(|s| s.to_string()))
            .collect(),
        _ => panic!("Expected a YAML mapping"),
    }
}

/// Extract the content between BEGIN GENERATED and END GENERATED markers.
fn extract_generated_section(content: &str) -> &str {
    let begin_marker = "<!-- BEGIN GENERATED -->";
    let end_marker = "<!-- END GENERATED -->";
    let start = content
        .find(begin_marker)
        .expect("content should have BEGIN GENERATED marker")
        + begin_marker.len();
    let end = content
        .find(end_marker)
        .expect("content should have END GENERATED marker");
    &content[start..end]
}

// ============================================================
// Story 1vpv06QV6: Serde round-trip and Display agreement tests
// ============================================================

mod serde_round_trips {
    use crate::domain::model::{AdrStatus, BearingStatus, StoryState, StoryType, VoyageState};

    /// All Stage variants — exhaustive match forces compile error when variants change.
    fn all_stages() -> Vec<StoryState> {
        let all = [
            StoryState::Backlog,
            StoryState::InProgress,
            StoryState::NeedsHumanVerification,
            StoryState::Done,
            StoryState::Rejected,
            StoryState::Icebox,
        ];
        for v in &all {
            match v {
                StoryState::Backlog
                | StoryState::InProgress
                | StoryState::NeedsHumanVerification
                | StoryState::Done
                | StoryState::Rejected
                | StoryState::Icebox => {}
            }
        }
        all.to_vec()
    }

    fn all_story_types() -> Vec<StoryType> {
        let all = [
            StoryType::Feat,
            StoryType::Bug,
            StoryType::Chore,
            StoryType::Refactor,
            StoryType::Fix,
            StoryType::Docs,
        ];
        for v in &all {
            match v {
                StoryType::Feat
                | StoryType::Bug
                | StoryType::Chore
                | StoryType::Refactor
                | StoryType::Fix
                | StoryType::Docs => {}
            }
        }
        all.to_vec()
    }

    fn all_bearing_statuses() -> Vec<BearingStatus> {
        let all = [
            BearingStatus::Exploring,
            BearingStatus::Evaluating,
            BearingStatus::Ready,
            BearingStatus::Laid,
            BearingStatus::Parked,
            BearingStatus::Declined,
        ];
        for v in &all {
            match v {
                BearingStatus::Exploring
                | BearingStatus::Evaluating
                | BearingStatus::Ready
                | BearingStatus::Laid
                | BearingStatus::Parked
                | BearingStatus::Declined => {}
            }
        }
        all.to_vec()
    }

    fn all_statuses() -> Vec<VoyageState> {
        let all = [
            VoyageState::Draft,
            VoyageState::Planned,
            VoyageState::InProgress,
            VoyageState::Done,
        ];
        for v in &all {
            match v {
                VoyageState::Draft
                | VoyageState::Planned
                | VoyageState::InProgress
                | VoyageState::Done => {}
            }
        }
        all.to_vec()
    }

    fn all_adr_statuses() -> Vec<AdrStatus> {
        let all = [
            AdrStatus::Proposed,
            AdrStatus::Accepted,
            AdrStatus::Rejected,
            AdrStatus::Deprecated,
            AdrStatus::Superseded,
        ];
        for v in &all {
            match v {
                AdrStatus::Proposed
                | AdrStatus::Accepted
                | AdrStatus::Rejected
                | AdrStatus::Deprecated
                | AdrStatus::Superseded => {}
            }
        }
        all.to_vec()
    }

    #[test]
    fn stage_round_trip() {
        for variant in all_stages() {
            let yaml = serde_yaml::to_string(&variant).unwrap();
            let back: StoryState = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(variant, back, "Stage round-trip failed for {:?}", variant);
        }
    }

    #[test]
    fn story_type_round_trip() {
        for variant in all_story_types() {
            let yaml = serde_yaml::to_string(&variant).unwrap();
            let back: StoryType = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(
                variant, back,
                "StoryType round-trip failed for {:?}",
                variant
            );
        }
    }

    #[test]
    fn bearing_status_round_trip() {
        for variant in all_bearing_statuses() {
            let yaml = serde_yaml::to_string(&variant).unwrap();
            let back: BearingStatus = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(
                variant, back,
                "BearingStatus round-trip failed for {:?}",
                variant
            );
        }
    }

    #[test]
    fn status_round_trip() {
        for variant in all_statuses() {
            let yaml = serde_yaml::to_string(&variant).unwrap();
            let back: VoyageState = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(variant, back, "Status round-trip failed for {:?}", variant);
        }
    }

    #[test]
    fn adr_status_round_trip() {
        for variant in all_adr_statuses() {
            let yaml = serde_yaml::to_string(&variant).unwrap();
            let back: AdrStatus = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(
                variant, back,
                "AdrStatus round-trip failed for {:?}",
                variant
            );
        }
    }

    #[test]
    fn display_matches_serde() {
        for variant in all_stages() {
            let display = format!("{}", variant);
            let serde = serde_yaml::to_string(&variant).unwrap().trim().to_string();
            assert_eq!(
                display, serde,
                "Stage Display/serde mismatch for {:?}",
                variant
            );
        }
        for variant in all_story_types() {
            let display = format!("{}", variant);
            let serde = serde_yaml::to_string(&variant).unwrap().trim().to_string();
            assert_eq!(
                display, serde,
                "StoryType Display/serde mismatch for {:?}",
                variant
            );
        }
        for variant in all_bearing_statuses() {
            let display = format!("{}", variant);
            let serde = serde_yaml::to_string(&variant).unwrap().trim().to_string();
            assert_eq!(
                display, serde,
                "BearingStatus Display/serde mismatch for {:?}",
                variant
            );
        }
        for variant in all_statuses() {
            let display = format!("{}", variant);
            let serde = serde_yaml::to_string(&variant).unwrap().trim().to_string();
            assert_eq!(
                display, serde,
                "Status Display/serde mismatch for {:?}",
                variant
            );
        }
        for variant in all_adr_statuses() {
            let display = format!("{}", variant);
            let serde = serde_yaml::to_string(&variant).unwrap().trim().to_string();
            assert_eq!(
                display, serde,
                "AdrStatus Display/serde mismatch for {:?}",
                variant
            );
        }
    }
}

// ============================================================
// Story 1vpv0B9GD: Template-struct field agreement tests
// ============================================================

mod template_struct_fields {
    use super::*;
    use chrono::NaiveDate;

    use crate::domain::model::{
        AdrFrontmatter, AdrStatus, EpicFrontmatter, StoryFrontmatter, StoryState, StoryType,
        VoyageFrontmatter, VoyageState,
    };

    fn populated_story() -> StoryFrontmatter {
        let dt = NaiveDate::from_ymd_opt(2026, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        StoryFrontmatter {
            id: "t".to_string(),
            title: "t".to_string(),
            story_type: StoryType::Feat,
            status: StoryState::Backlog,
            scope: Some("s".to_string()),
            milestone: Some("m".to_string()),
            created_at: Some(dt),
            updated_at: Some(dt),
            started_at: Some(dt),
            completed_at: Some(dt),
            submitted_at: Some(dt),
            index: Some(1),
            governed_by: vec!["g".to_string()],
            role: Some("r".to_string()),
        }
    }

    fn populated_voyage() -> VoyageFrontmatter {
        let dt = NaiveDate::from_ymd_opt(2026, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        VoyageFrontmatter {
            id: "t".to_string(),
            title: "t".to_string(),
            goal: None,
            status: VoyageState::Planned,
            epic: Some("e".to_string()),
            index: None,
            created_at: Some(dt),
            updated_at: Some(dt),
            started_at: Some(dt),
            completed_at: Some(dt),
        }
    }

    fn populated_epic() -> EpicFrontmatter {
        let dt = NaiveDate::from_ymd_opt(2026, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        EpicFrontmatter {
            id: "t".to_string(),
            title: "t".to_string(),
            description: Some("d".to_string()),
            bearing: None,
            index: None,
            created_at: Some(dt),
        }
    }

    fn populated_adr() -> AdrFrontmatter {
        let d = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        AdrFrontmatter {
            id: "t".to_string(),
            title: "t".to_string(),
            status: AdrStatus::Proposed,
            context: Some("c".to_string()),
            applies_to: vec!["a".to_string()],
            supersedes: vec!["s".to_string()],
            superseded_by: Some("sb".to_string()),
            rejection_reason: Some("rr".to_string()),
            deprecation_reason: Some("dr".to_string()),
            decided_at: Some(d.and_hms_opt(12, 0, 0).unwrap()),
            index: Some(1),
        }
    }

    #[test]
    fn story_template_fields_match_struct() {
        let template_keys =
            extract_frontmatter_keys(crate::infrastructure::templates::story::STORY);
        let struct_fields = serde_field_names(&populated_story());
        let missing: Vec<_> = template_keys
            .iter()
            .filter(|k| !struct_fields.contains(k.as_str()))
            .collect();
        assert!(
            missing.is_empty(),
            "Story template has keys not in struct: {:?}\nTemplate keys: {:?}\nStruct fields: {:?}",
            missing,
            template_keys,
            struct_fields
        );
    }

    #[test]
    fn voyage_template_fields_match_struct() {
        let template_keys =
            extract_frontmatter_keys(crate::infrastructure::templates::voyage::README);
        let struct_fields = serde_field_names(&populated_voyage());
        let missing: Vec<_> = template_keys
            .iter()
            .filter(|k| !struct_fields.contains(k.as_str()))
            .collect();
        assert!(
            missing.is_empty(),
            "Voyage template has keys not in struct: {:?}\nTemplate keys: {:?}\nStruct fields: {:?}",
            missing,
            template_keys,
            struct_fields
        );
    }

    #[test]
    fn epic_template_fields_match_struct() {
        let template_keys =
            extract_frontmatter_keys(crate::infrastructure::templates::epic::README);
        let struct_fields = serde_field_names(&populated_epic());
        let missing: Vec<_> = template_keys
            .iter()
            .filter(|k| !struct_fields.contains(k.as_str()))
            .collect();
        assert!(
            missing.is_empty(),
            "Epic template has keys not in struct: {:?}\nTemplate keys: {:?}\nStruct fields: {:?}",
            missing,
            template_keys,
            struct_fields
        );
    }

    #[test]
    fn adr_template_fields_match_struct() {
        let template_keys = extract_frontmatter_keys(crate::infrastructure::templates::adr::ADR);
        let struct_fields = serde_field_names(&populated_adr());
        let missing: Vec<_> = template_keys
            .iter()
            .filter(|k| !struct_fields.contains(k.as_str()))
            .collect();
        assert!(
            missing.is_empty(),
            "ADR template has keys not in struct: {:?}\nTemplate keys: {:?}\nStruct fields: {:?}",
            missing,
            template_keys,
            struct_fields
        );
    }
}

// ============================================================
// Story 1vpv0CKS8: Template-generator output agreement tests
// ============================================================

mod template_generator {
    use super::*;
    use crate::infrastructure::generate::epic_readme::generate_epic_readme;
    use crate::infrastructure::generate::voyage_readme::generate_voyage_readme;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;

    fn create_test_board() -> tempfile::TempDir {
        let temp = tempfile::TempDir::new().unwrap();
        let root = temp.path();

        fs::create_dir_all(root.join("stories")).unwrap();
        fs::create_dir_all(root.join("epics/test-epic/voyages/01-first")).unwrap();

        fs::write(
            root.join("epics/test-epic/README.md"),
            "---\nid: test-epic\ntitle: Test Epic\n---\n\n# Test Epic\n\n> Goal\n\n## Voyages\n\n<!-- BEGIN GENERATED -->\n<!-- END GENERATED -->\n",
        ).unwrap();

        fs::write(
            root.join("epics/test-epic/voyages/01-first/README.md"),
            "---\nid: 01-first\ntitle: First Voyage\nstatus: in-progress\nepic: test-epic\n---\n\n# First Voyage\n\n> Goal\n\n## Stories\n\n<!-- BEGIN GENERATED -->\n<!-- END GENERATED -->\n",
        ).unwrap();

        fs::write(
            root.join("stories/[FEAT][0001]-story.md"),
            "---\nid: FEAT0001\ntitle: Test Story\ntype: feat\nstatus: in-progress\nscope: test-epic/01-first\n---\n",
        ).unwrap();

        temp
    }

    #[test]
    fn epic_progress_line_format() {
        let template_section =
            extract_generated_section(crate::infrastructure::templates::epic::README);
        assert!(
            template_section.contains("**Progress:**"),
            "Template should have progress line"
        );
        assert!(
            template_section.contains("voyages complete,"),
            "Template progress should mention voyages complete"
        );
        assert!(
            template_section.contains("stories done"),
            "Template progress should mention stories done"
        );

        let temp = create_test_board();
        let board = load_board(temp.path()).unwrap();
        let epic = board.epics.get("test-epic").unwrap();
        let generated = generate_epic_readme(&board, epic);

        assert!(
            generated.contains("**Progress:**"),
            "Generated should have progress line"
        );
        assert!(
            generated.contains("voyages complete,"),
            "Generated progress should mention voyages complete"
        );
        assert!(
            generated.contains("stories done"),
            "Generated progress should mention stories done"
        );
    }

    #[test]
    fn epic_table_headers_match() {
        let template_section =
            extract_generated_section(crate::infrastructure::templates::epic::README);
        let template_columns = extract_table_columns(template_section);

        let temp = create_test_board();
        let board = load_board(temp.path()).unwrap();
        let epic = board.epics.get("test-epic").unwrap();
        let generated = generate_epic_readme(&board, epic);
        let generated_columns = extract_table_columns(&generated);

        assert_eq!(
            template_columns, generated_columns,
            "Epic table columns drift: template={:?}, generator={:?}",
            template_columns, generated_columns
        );
    }

    #[test]
    fn voyage_table_headers_match() {
        let template_section =
            extract_generated_section(crate::infrastructure::templates::voyage::README);
        let template_columns = extract_table_columns(template_section);

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic"))
            .story(TestStory::new("S1").scope("test-epic/01-first"))
            .build();
        let board = load_board(temp.path()).unwrap();
        let voyage = board.voyages.get("01-first").unwrap();
        let generated = generate_voyage_readme(&board, voyage);
        let generated_columns = extract_table_columns(&generated);

        if template_columns != generated_columns {
            println!("TEMPLATE SECTION:\n{}", template_section);
            println!("GENERATED SECTION:\n{}", generated);
        }

        assert_eq!(
            template_columns, generated_columns,
            "Voyage table columns drift: template={:?}, generator={:?}",
            template_columns, generated_columns
        );
    }

    #[test]
    fn generator_column_structure() {
        // Verify specific column expectations
        let epic_section =
            extract_generated_section(crate::infrastructure::templates::epic::README);
        let epic_cols = extract_table_columns(epic_section);
        assert_eq!(
            epic_cols,
            vec!["Voyage", "Status", "Stories"],
            "Epic table should have Voyage, Status, Stories columns"
        );

        let voyage_section =
            extract_generated_section(crate::infrastructure::templates::voyage::README);
        let voyage_cols = extract_table_columns(voyage_section);
        assert_eq!(
            voyage_cols,
            vec!["Title", "Type", "Status"],
            "Voyage table should have Title, Type, Status columns"
        );
    }
}

// ============================================================
// Story 1vxH84K5a: Token bucket contract drift checks
// ============================================================

mod token_bucket_contract {
    use std::collections::BTreeSet;

    const CLI_OWNED_TOKENS: &[&str] = &["applies_to", "context", "epic", "goal", "title", "type"];
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

    fn token_set(tokens: &[&str]) -> BTreeSet<String> {
        tokens.iter().map(|token| (*token).to_string()).collect()
    }

    fn arg_ids_for_new_subcommand(root: &str) -> BTreeSet<String> {
        let mut cli = crate::build_cli();
        let root_cmd = cli
            .find_subcommand_mut(root)
            .unwrap_or_else(|| panic!("missing root subcommand: {root}"));
        let new_cmd = root_cmd
            .find_subcommand_mut("new")
            .unwrap_or_else(|| panic!("missing `new` subcommand for: {root}"));

        new_cmd
            .get_arguments()
            .map(|arg| arg.get_id().as_str().to_string())
            .filter(|id| id != "help")
            .collect()
    }

    #[test]
    fn planning_template_tokens_stay_within_cli_or_system_buckets() {
        let known_tokens: BTreeSet<String> = CLI_OWNED_TOKENS
            .iter()
            .chain(SYSTEM_OWNED_TOKENS)
            .map(|token| (*token).to_string())
            .collect();
        let generated_tokens = token_set(GENERATED_TOKENS);

        let templates = [
            (
                "epic README",
                crate::infrastructure::templates::epic::README,
            ),
            ("epic PRD", crate::infrastructure::templates::epic::PRD),
            (
                "epic PRESS_RELEASE",
                crate::infrastructure::templates::epic::PRESS_RELEASE,
            ),
            (
                "voyage README",
                crate::infrastructure::templates::voyage::README,
            ),
            ("voyage SRS", crate::infrastructure::templates::voyage::SRS),
            ("voyage SDD", crate::infrastructure::templates::voyage::SDD),
            (
                "story README",
                crate::infrastructure::templates::story::STORY,
            ),
            (
                "story REFLECT",
                crate::infrastructure::templates::story::REFLECT,
            ),
            (
                "bearing README",
                crate::infrastructure::templates::bearing::README,
            ),
            (
                "bearing BRIEF",
                crate::infrastructure::templates::bearing::BRIEF,
            ),
            (
                "bearing SURVEY",
                crate::infrastructure::templates::bearing::SURVEY,
            ),
            (
                "bearing ASSESSMENT",
                crate::infrastructure::templates::bearing::ASSESSMENT,
            ),
            ("adr", crate::infrastructure::templates::adr::ADR),
        ];

        for (label, template) in templates {
            let tokens = extract_tokens(template);

            let unknown: Vec<String> = tokens.difference(&known_tokens).cloned().collect();
            assert!(
                unknown.is_empty(),
                "{label} contains unknown tokens: {:?}; allowed planning buckets are CLI={:?}, SYSTEM={:?}",
                unknown,
                CLI_OWNED_TOKENS,
                SYSTEM_OWNED_TOKENS
            );

            let out_of_bucket: Vec<String> =
                tokens.intersection(&generated_tokens).cloned().collect();
            assert!(
                out_of_bucket.is_empty(),
                "{label} contains generated-bucket tokens {:?}; generated tokens are only allowed in report templates",
                out_of_bucket
            );
        }
    }

    #[test]
    fn creation_command_new_surfaces_match_cli_owned_token_contract() {
        let expected_epic: BTreeSet<String> =
            ["name", "goal"].into_iter().map(String::from).collect();
        let expected_voyage: BTreeSet<String> = ["name", "epic", "goal"]
            .into_iter()
            .map(String::from)
            .collect();
        let expected_story: BTreeSet<String> =
            ["title", "type"].into_iter().map(String::from).collect();
        let expected_bearing: BTreeSet<String> = ["name"].into_iter().map(String::from).collect();
        let expected_adr: BTreeSet<String> = ["title", "context", "applies-to"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(arg_ids_for_new_subcommand("epic"), expected_epic);
        assert_eq!(arg_ids_for_new_subcommand("voyage"), expected_voyage);
        assert_eq!(arg_ids_for_new_subcommand("story"), expected_story);
        assert_eq!(arg_ids_for_new_subcommand("bearing"), expected_bearing);
        assert_eq!(arg_ids_for_new_subcommand("adr"), expected_adr);
    }

    #[test]
    fn generated_marker_contract_remains_literal() {
        let marker_templates = [
            (
                "epic README",
                crate::infrastructure::templates::epic::README,
                ["<!-- BEGIN GENERATED -->", "<!-- END GENERATED -->"],
            ),
            (
                "voyage README",
                crate::infrastructure::templates::voyage::README,
                ["<!-- BEGIN GENERATED -->", "<!-- END GENERATED -->"],
            ),
            (
                "voyage SRS",
                crate::infrastructure::templates::voyage::SRS,
                [
                    "<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->",
                    "<!-- END FUNCTIONAL_REQUIREMENTS -->",
                ],
            ),
        ];

        for (label, template, markers) in marker_templates {
            for marker in markers {
                assert!(
                    template.contains(marker),
                    "{label} is missing required generated marker: {marker}"
                );
            }
        }
    }
}

// ============================================================
// Story 1vuz97CCg: Queue policy documentation drift checks
// ============================================================

mod queue_policy_docs {
    use crate::domain::policy::queue::{
        FLOW_VERIFY_BLOCK_THRESHOLD, HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD,
    };

    const ARCHITECTURE_DOC: &str = include_str!("../ARCHITECTURE.md");
    const README_DOC: &str = include_str!("../README.md");
    const MAIN_RS: &str = include_str!("main.rs");
    const COMMAND_TREE_RS: &str = include_str!("cli/command_tree.rs");

    #[test]
    fn architecture_thresholds_match_policy_constants() {
        let human_policy_row = format!(
            "`HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD` | {} |",
            HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD
        );
        let flow_policy_row = format!(
            "`FLOW_VERIFY_BLOCK_THRESHOLD` | {} |",
            FLOW_VERIFY_BLOCK_THRESHOLD
        );

        assert!(
            ARCHITECTURE_DOC.contains(&human_policy_row),
            "ARCHITECTURE.md should document HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD row: {}",
            human_policy_row
        );
        assert!(
            ARCHITECTURE_DOC.contains(&flow_policy_row),
            "ARCHITECTURE.md should document FLOW_VERIFY_BLOCK_THRESHOLD row: {}",
            flow_policy_row
        );

        let human_boundary = format!("`>= {}`", HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD);
        let flow_boundary = format!("`> {}`", FLOW_VERIFY_BLOCK_THRESHOLD);

        assert!(
            ARCHITECTURE_DOC.contains(&human_boundary),
            "ARCHITECTURE.md should document human block boundary {}",
            human_boundary
        );
        assert!(
            ARCHITECTURE_DOC.contains(&flow_boundary),
            "ARCHITECTURE.md should document flow block boundary {}",
            flow_boundary
        );
    }

    #[test]
    fn architecture_documents_human_next_decision_contract() {
        assert!(
            ARCHITECTURE_DOC.contains(
                "`decision`, `accept`, `research`, `needs-stories`, `needs-planning`, `blocked`, `empty`"
            ),
            "ARCHITECTURE.md should list the allowed human-mode decision set for `keel next`"
        );
        assert!(
            ARCHITECTURE_DOC.contains("never returns `Work`"),
            "ARCHITECTURE.md should explicitly state that human mode never returns Work"
        );
    }

    #[test]
    fn command_help_docs_describe_next_queue_modes() {
        assert!(
            README_DOC.contains(
                "`keel next` (human mode) only returns human-queue decisions and never returns implementation `Work`."
            ),
            "README.md should document the human-mode queue boundary for `keel next`"
        );
        assert!(
            README_DOC.contains(
                "`keel next --agent` returns implementation work from the agent queue (`in-progress` then `backlog`)."
            ),
            "README.md should document `--agent` implementation queue behavior"
        );
        assert!(
            COMMAND_TREE_RS
                .contains("next        Pull from human queue (default) or agent queue (--agent)"),
            "CLI help text should describe human/agent queue mode behavior for `next`"
        );
    }

    #[test]
    fn docs_and_help_avoid_deprecated_state_terms() {
        for (label, content) in [
            ("ARCHITECTURE.md", ARCHITECTURE_DOC),
            ("README.md", README_DOC),
            ("src/main.rs", MAIN_RS),
            ("src/cli/command_tree.rs", COMMAND_TREE_RS),
        ] {
            assert!(
                !content.contains("ready-for-acceptance"),
                "{label} should use canonical `needs-human-verification` terminology"
            );
            assert!(
                !content.contains("move back to in-progress"),
                "{label} should describe reject as transitioning to `rejected`"
            );
        }
    }
}
