use crate::domain::model::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestFm {
    z: String,
    a: String,
    b: u32,
}

#[test]
fn test_serde_yaml_determinism() {
    let fm = TestFm {
        z: "last".to_string(),
        a: "first".to_string(),
        b: 42,
    };
    let s = serde_yaml::to_string(&fm).unwrap();
    // Struct field order should be preserved
    assert_eq!(s, "z: last\na: first\nb: 42\n");
}

#[test]
fn test_mission_serialization_order() {
    let fm = MissionFrontmatter {
        id: "M1".to_string(),
        title: "Mission 1".to_string(),
        status: MissionStatus::Defining,
        created_at: None,
        updated_at: None,
        activated_at: None,
        achieved_at: None,
        verified_at: None,
        watch: None,
        operator_signal: None,
    };
    let s = serde_yaml::to_string(&fm).unwrap();
    assert_eq!(s, "id: M1\ntitle: Mission 1\nstatus: defining\n");
}

#[test]
fn test_epic_serialization_order() {
    let fm = EpicFrontmatter {
        id: "E1".to_string(),
        title: "Epic 1".to_string(),
        description: Some("Desc".to_string()),
        bearing: None,
        mission: None,
        index: Some(1),
        created_at: None,
    };
    let s = serde_yaml::to_string(&fm).unwrap();
    assert_eq!(s, "id: E1\ntitle: Epic 1\ndescription: Desc\nindex: 1\n");
}

#[test]
fn test_voyage_serialization_order() {
    let fm = VoyageFrontmatter {
        id: "V1".to_string(),
        title: "Voyage 1".to_string(),
        epic: Some("E1".to_string()),
        status: VoyageState::Draft,
        operator_signal: None,
        index: Some(1),
        created_at: None,
        started_at: None,
        updated_at: None,
        completed_at: None,
        goal: None,
    };
    let s = serde_yaml::to_string(&fm).unwrap();
    assert_eq!(
        s,
        "id: V1\ntitle: Voyage 1\nstatus: draft\nepic: E1\nindex: 1\n"
    );
}

#[test]
fn test_story_serialization_order() {
    let fm = StoryFrontmatter {
        id: "S1".to_string(),
        title: "Story 1".to_string(),
        story_type: StoryType::Feat,
        status: StoryState::Backlog,
        scope: Some("E1/V1".to_string()),
        milestone: None,
        created_at: None,
        updated_at: None,
        started_at: None,
        completed_at: None,
        submitted_at: None,
        index: Some(1),
        governed_by: vec![],
        blocked_by: vec![],
        role: None,
        operator_signal: None,
    };
    let s = serde_yaml::to_string(&fm).unwrap();
    assert_eq!(
        s,
        "id: S1\ntitle: Story 1\ntype: feat\nstatus: backlog\nscope: E1/V1\nindex: 1\n"
    );
}

#[test]
fn test_adr_serialization_order() {
    let fm = AdrFrontmatter {
        id: "ADR1".to_string(),
        index: Some(1),
        title: "ADR 1".to_string(),
        mission: None,
        status: AdrStatus::Proposed,
        context: None,
        applies_to: vec![],
        supersedes: vec![],
        superseded_by: None,
        decided_at: None,
        deprecation_reason: None,
        rejection_reason: None,
    };
    let s = serde_yaml::to_string(&fm).unwrap();
    assert_eq!(s, "id: ADR1\ntitle: ADR 1\nstatus: proposed\nindex: 1\n");
}

#[test]
fn test_routine_serialization_order() {
    let fm = RoutineFrontmatter {
        id: "R1".to_string(),
        title: "Routine 1".to_string(),
        cadence: serde_yaml::Value::Null,
        target_scope: "E1".to_string(),
        created_at: None,
        updated_at: None,
        operator_signal: None,
    };
    let s = serde_yaml::to_string(&fm).unwrap();
    assert_eq!(
        s,
        "id: R1\ntitle: Routine 1\ncadence: null\ntarget-scope: E1\n"
    );
}

#[test]
fn test_canonical_markdown_formatting() {
    let fm = MissionFrontmatter {
        id: "M1".to_string(),
        title: "Mission 1".to_string(),
        status: MissionStatus::Defining,
        created_at: None,
        updated_at: None,
        activated_at: None,
        achieved_at: None,
        verified_at: None,
        watch: None,
        operator_signal: None,
    };
    let serialized = serde_yaml::to_string(&fm).unwrap();
    let body = "# My Body";

    // Logic from filesystem.rs
    let body = body.trim();
    let updated = if body.is_empty() {
        format!("---\n{}\n---\n", serialized.trim())
    } else {
        format!("---\n{}\n---\n\n{}\n", serialized.trim(), body)
    };

    assert_eq!(
        updated,
        "---\nid: M1\ntitle: Mission 1\nstatus: defining\n---\n\n# My Body\n"
    );
}
