//! Verification unit tests

use crate::infrastructure::verification::comparator::compare;
use crate::infrastructure::verification::executor::execute;
use crate::infrastructure::verification::parser::{
    Comparison, RequirementPhase, parse_ac_references, parse_verify_annotations,
};
use std::path::Path;
use std::time::Duration;

#[test]
fn parses_simple_verify_annotation() {
    let content = "Check this <!-- verify: cargo test -->";
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 1);
    assert_eq!(annotations[0].command, Some("cargo test".to_string()));
    assert!(matches!(annotations[0].comparison, Comparison::Success));
}

#[test]
fn parses_equality_comparison() {
    let content = "Check value <!-- verify: echo 0 == 0 -->";
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 1);
    if let Comparison::Equals(val) = &annotations[0].comparison {
        assert_eq!(val, "0");
    } else {
        panic!("Expected Equals comparison");
    }
}

#[test]
fn parses_contains_comparison() {
    let content = "Check output <!-- verify: echo 1.0.0 ~= \"1.0\" -->";
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 1);
    if let Comparison::Contains(val) = &annotations[0].comparison {
        assert_eq!(val, "1.0");
    } else {
        panic!("Expected Contains comparison");
    }
}

#[test]
fn parses_manual_verification() {
    let content = "Check manually <!-- verify: manual -->";
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 1);
    assert!(matches!(annotations[0].comparison, Comparison::Manual));
}

#[test]
fn parses_multiple_annotations() {
    let content = r#"
- [ ] AC1 <!-- verify: cmd1 -->
- [ ] AC2 <!-- verify: cmd2 == ok -->
"#;
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 2);
    assert_eq!(annotations[0].command, Some("cmd1".to_string()));
    assert_eq!(annotations[1].command, Some("cmd2".to_string()));
}

#[test]
fn parses_requirement_start_phase() {
    let content = "Start req <!-- verify: cmd, SRS-01:start -->";
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 1);
    let req = annotations[0].requirement.as_ref().unwrap();
    assert_eq!(req.id, "SRS-01");
    assert!(matches!(req.phase, RequirementPhase::Start));
}

#[test]
fn parses_requirement_continues_phase() {
    let content = "Continue req <!-- verify: manual, SRS-01:continues -->";
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 1);
    let req = annotations[0].requirement.as_ref().unwrap();
    assert_eq!(req.id, "SRS-01");
    assert!(matches!(req.phase, RequirementPhase::Continues));
}

#[test]
fn parses_requirement_end_phase() {
    let content = "End req <!-- verify: cmd, SRS-01:end -->";
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 1);
    let req = annotations[0].requirement.as_ref().unwrap();
    assert_eq!(req.id, "SRS-01");
    assert!(matches!(req.phase, RequirementPhase::End));
}

#[test]
fn parses_requirement_start_end_phase() {
    let content = "One-shot req <!-- verify: cmd, SRS-01:start:end -->";
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 1);
    let req = annotations[0].requirement.as_ref().unwrap();
    assert_eq!(req.id, "SRS-01");
    assert!(matches!(req.phase, RequirementPhase::StartEnd));
}

#[test]
fn parses_requirement_with_contains_comparison() {
    let content = "Contains req <!-- verify: cmd ~= \"done\", SRS-01:end -->";
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 1);
    if let Comparison::Contains(val) = &annotations[0].comparison {
        assert_eq!(val, "done");
    } else {
        panic!("Expected Contains");
    }
    let req = annotations[0].requirement.as_ref().unwrap();
    assert_eq!(req.id, "SRS-01");
    assert!(matches!(req.phase, RequirementPhase::End));
}

#[test]
fn parses_requirement_with_equals_comparison() {
    let content = "Equals req <!-- verify: cmd == 1, SRS-01:start -->";
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 1);
    if let Comparison::Equals(val) = &annotations[0].comparison {
        assert_eq!(val, "1");
    } else {
        panic!("Expected Equals");
    }
    let req = annotations[0].requirement.as_ref().unwrap();
    assert_eq!(req.id, "SRS-01");
    assert!(matches!(req.phase, RequirementPhase::Start));
}

#[test]
fn parses_nfr_requirement_phase() {
    let content = "NFR check <!-- verify: manual, SRS-NFR-01:start:end -->";
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 1);
    let req = annotations[0].requirement.as_ref().unwrap();
    assert_eq!(req.id, "SRS-NFR-01");
    assert!(matches!(req.phase, RequirementPhase::StartEnd));
}

#[test]
fn ac_ref_coexists_with_requirement_phase() {
    let content = "[SRS-01/AC-01] req <!-- verify: cmd, SRS-01:start -->";
    let annotations = parse_verify_annotations(content);
    assert_eq!(annotations.len(), 1);
    let req = annotations[0].requirement.as_ref().unwrap();
    assert_eq!(req.id, "SRS-01");
}

#[test]
fn parses_single_ac_reference() {
    let content = "- [ ] [SRS-01/AC-01] do something";
    let refs = parse_ac_references(content);
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].srs_id, "SRS-01");
    assert_eq!(refs[0].ac_num, 1);
}

#[test]
fn parses_multiple_ac_references() {
    let content = r#"
- [ ] [SRS-01/AC-01] first
- [x] [SRS-02/AC-05] second
"#;
    let refs = parse_ac_references(content);
    assert_eq!(refs.len(), 2);
    assert_eq!(refs[0].srs_id, "SRS-01");
    assert_eq!(refs[1].srs_id, "SRS-02");
}

#[test]
fn parse_ac_references_with_nfr_ids() {
    let content = "[SRS-NFR-01/AC-01] performance";
    let refs = parse_ac_references(content);
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].srs_id, "SRS-NFR-01");
}

#[test]
fn parse_ac_references_with_double_digit_numbers() {
    let content = "[SRS-10/AC-12] complex";
    let refs = parse_ac_references(content);
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].srs_id, "SRS-10");
    assert_eq!(refs[0].ac_num, 12);
}

#[test]
fn comparator_success_passes_on_exit_zero() {
    let result = compare(&Comparison::Success, 0, "any output", "");
    assert!(result.passed);
}

#[test]
fn comparator_success_fails_on_nonzero_exit() {
    let result = compare(&Comparison::Success, 1, "", "error");
    assert!(!result.passed);
}

#[test]
fn comparator_equals_passes_on_exact_match() {
    let result = compare(&Comparison::Equals("hello".to_string()), 0, "hello", "");
    assert!(result.passed);
}

#[test]
fn comparator_equals_trims_whitespace() {
    let result = compare(
        &Comparison::Equals("hello".to_string()),
        0,
        "  hello  \n",
        "",
    );
    assert!(result.passed);
}

#[test]
fn comparator_equals_fails_on_mismatch() {
    let result = compare(&Comparison::Equals("hello".to_string()), 0, "world", "");
    assert!(!result.passed);
}

#[test]
fn comparator_contains_passes_on_substring() {
    let result = compare(
        &Comparison::Contains("world".to_string()),
        0,
        "hello world!",
        "",
    );
    assert!(result.passed);
}

#[test]
fn comparator_contains_fails_when_not_found() {
    let result = compare(&Comparison::Contains("foo".to_string()), 0, "bar baz", "");
    assert!(!result.passed);
}

#[test]
fn comparator_manual_always_pending() {
    let result = compare(&Comparison::Manual, 0, "", "");
    assert!(!result.passed);
    assert!(result.requires_human_review);
}

#[test]
fn executor_executes_simple_command() {
    let result = execute("echo hello", Path::new("."), Duration::from_secs(5));
    assert!(result.is_ok());
    let r = result.unwrap();
    assert_eq!(r.exit_code, 0);
    assert_eq!(r.stdout.trim(), "hello");
}

#[test]
fn executor_captures_exit_code() {
    let result = execute("exit 42", Path::new("."), Duration::from_secs(5));
    let r = result.unwrap();
    assert_eq!(r.exit_code, 42);
}

#[test]
fn executor_captures_stderr() {
    let result = execute("echo error >&2", Path::new("."), Duration::from_secs(5));
    let r = result.unwrap();
    assert!(r.stderr.contains("error"));
}

#[test]
fn executor_respects_working_directory() {
    let tmp = tempfile::tempdir().unwrap();
    let tmp_path = tmp.path();
    let result = execute("pwd", tmp_path, Duration::from_secs(5));
    let r = result.unwrap();

    let pwd = r.stdout.trim();
    let tmp_str = tmp_path.to_string_lossy();
    assert!(pwd.contains(&*tmp_str) || tmp_str.contains(pwd));
}

#[test]
fn test_verify_all_executes_all_linked_proofs() {
    let temp = crate::test_helpers::TestBoardBuilder::new()
        .story(
            crate::test_helpers::TestStory::new("S1")
                .body("## Acceptance Criteria\n\n- [x] AC 1 <!-- verify: echo 'S1-AC1-PASS', SRS-01:start -->")
        )
        .story(
            crate::test_helpers::TestStory::new("S2")
                .body("## Acceptance Criteria\n\n- [x] AC 1 <!-- verify: echo 'S2-AC1-PASS', SRS-02:start -->")
        )
        .build();

    // This function doesn't exist yet
    let results = super::verify_all(temp.path()).unwrap();

    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|r| r.story_id == "S1"));
    assert!(results.iter().any(|r| r.story_id == "S2"));
}
