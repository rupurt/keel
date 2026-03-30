//! Mission lifecycle application service

use anyhow::{Result, anyhow};
use std::path::Path;

use crate::domain::state_machine::{
    MissionTransition, evaluate_mission_transition, format_gate_error,
};
use crate::domain::transitions::mission::{execute, mission_transitions};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::markdown_sections::{extract_section, replace_section};
use crate::infrastructure::validation::charter::{self, GoalVerification};

pub struct MissionLifecycleService;

impl MissionLifecycleService {
    pub fn refine(board_dir: &Path, id: &str, answer: Option<&str>) -> Result<()> {
        let board = load_board(board_dir)?;
        let mission = board.require_mission(id)?;
        let mission_dir = mission.path.parent().unwrap();
        let charter_path = mission_dir.join("CHARTER.md");
        let content = std::fs::read_to_string(&charter_path)?;

        if let Some(ans) = answer {
            let updated_content = self::process_answer(&content, ans)?;
            std::fs::write(&charter_path, updated_content)?;
            // Reload content for next question check
            return Self::refine(board_dir, id, None);
        }

        let question = self::get_next_question(&content);
        match question {
            Some(q) => println!("{}", q),
            None => println!("Mission charter is complete and ready for activation."),
        }

        Ok(())
    }

    pub fn pause(board_dir: &Path, id: &str) -> Result<()> {
        execute(board_dir, id, &mission_transitions::PAUSE)?;
        println!("Paused mission: {}", id);
        Ok(())
    }

    pub fn resume(board_dir: &Path, id: &str) -> Result<()> {
        execute(board_dir, id, &mission_transitions::RESUME)?;
        println!("Resumed mission: {}", id);
        Ok(())
    }

    pub fn achieve(ctx: &spoke_auth::ExecutionContext, board_dir: &Path, id: &str) -> Result<()> {
        let board = load_board(board_dir)?;
        let mission = board.require_mission(id)?;

        let problems = evaluate_mission_transition(&board, mission, MissionTransition::Achieve);
        if !problems.is_empty() {
            return Err(anyhow!(format_gate_error("mission", "achieve", &problems)));
        }

        // Apply mutation
        execute(board_dir, id, &mission_transitions::ACHIEVE)?;

        // Audit log the actor
        match &ctx.actor {
            spoke_auth::Actor::LocalSystem { os_user } => {
                Self::log(
                    board_dir,
                    id,
                    &format!("Mission achieved by local system user '{}'", os_user),
                )?;
            }
            spoke_auth::Actor::Authenticated { identity, role } => {
                Self::log(
                    board_dir,
                    id,
                    &format!(
                        "Mission achieved by authenticated actor '{}' ({})",
                        identity, role
                    ),
                )?;
            }
        }

        println!("Achieved mission: {}", id);
        Ok(())
    }

    pub fn verify(board_dir: &Path, id: &str, artifact: Option<&str>) -> Result<()> {
        let board = load_board(board_dir)?;
        let mission = board.require_mission(id)?;

        let problems = evaluate_mission_transition(&board, mission, MissionTransition::Verify);
        if !problems.is_empty() {
            return Err(anyhow!(format_gate_error("mission", "verify", &problems)));
        }

        // Apply state transition
        execute(board_dir, id, &mission_transitions::VERIFY)?;

        // Store artifact if provided
        if let Some(art) = artifact {
            if !art.to_lowercase().ends_with(".gif") {
                return Err(anyhow!("Mission verification artifact must be a .gif"));
            }

            let board = load_board(board_dir)?;
            let mission = board.require_mission(id)?;
            let readme_path = mission.path.clone();
            let content = std::fs::read_to_string(&readme_path)?;

            use crate::infrastructure::frontmatter_mutation::{Mutation, apply};
            let mutations = vec![Mutation::set("verification_artifact", art)];
            let updated_content = apply(&content, &mutations);

            std::fs::write(&readme_path, updated_content)?;
        }

        println!("Verified mission: {}", id);
        Ok(())
    }

    pub fn abandon(board_dir: &Path, id: &str) -> Result<()> {
        execute(board_dir, id, &mission_transitions::ABANDON)?;
        println!("Abandoned mission: {}", id);
        Ok(())
    }

    pub fn activate(board_dir: &Path, id: &str) -> Result<()> {
        let board = load_board(board_dir)?;
        let mission = board.require_mission(id)?;

        let problems = evaluate_mission_transition(&board, mission, MissionTransition::Activate);
        if !problems.is_empty() {
            return Err(anyhow!(format_gate_error("mission", "activate", &problems)));
        }

        execute(board_dir, id, &mission_transitions::ACTIVATE)?;
        println!("Activated mission: {}", id);
        Ok(())
    }

    pub fn log(board_dir: &Path, id: &str, entry: &str) -> Result<()> {
        let board = load_board(board_dir)?;
        let mission = board.require_mission(id)?;
        let mission_dir = mission.path.parent().unwrap();
        let log_path = mission_dir.join("LOG.md");

        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let formatted_entry = format!("\n## {}\n\n{}\n", now, entry);

        let mut file = std::fs::OpenOptions::new().append(true).open(&log_path)?;
        use std::io::Write;
        file.write_all(formatted_entry.as_bytes())?;

        println!("Added log entry to mission: {}", id);
        Ok(())
    }

    pub fn digest(board_dir: &Path, id: &str) -> Result<()> {
        let board = load_board(board_dir)?;
        let mission = board.require_mission(id)?;
        let mission_dir = mission.path.parent().unwrap();
        let log_path = mission_dir.join("LOG.md");
        let content = std::fs::read_to_string(&log_path)?;

        let (header, entries) = parse_log_entries(&content);
        if entries.len() <= 50 {
            println!(
                "Mission {} log has only {} entries. No digest needed (threshold is 50).",
                id,
                entries.len()
            );
            return Ok(());
        }

        let (to_digest, to_keep) = entries.split_at(entries.len() - 50);

        let mut new_content = header;
        if !new_content.ends_with('\n') {
            new_content.push('\n');
        }
        new_content.push_str("\n<details>\n<summary>Archived Log Entries</summary>\n\n");
        for entry in to_digest {
            new_content.push_str(&entry.raw);
            new_content.push_str("\n\n");
        }
        new_content.push_str("</details>\n\n");

        for entry in to_keep {
            new_content.push_str(&entry.raw);
            new_content.push_str("\n\n");
        }

        std::fs::write(&log_path, new_content)?;
        println!(
            "Digested {} archived entries in mission: {}",
            to_digest.len(),
            id
        );

        Ok(())
    }
}

struct LogEntry {
    pub raw: String,
}

fn parse_log_entries(content: &str) -> (String, Vec<LogEntry>) {
    let mut header = String::new();
    let mut entries = Vec::new();
    let mut current_entry = String::new();
    let mut in_header = true;

    for line in content.lines() {
        if line.starts_with("## ") {
            in_header = false;
            if !current_entry.is_empty() {
                entries.push(LogEntry {
                    raw: current_entry.trim().to_string(),
                });
            }
            current_entry = line.to_string();
        } else if in_header {
            header.push_str(line);
            header.push('\n');
        } else {
            current_entry.push('\n');
            current_entry.push_str(line);
        }
    }
    if !current_entry.is_empty() {
        entries.push(LogEntry {
            raw: current_entry.trim().to_string(),
        });
    }

    (header, entries)
}

fn get_next_question(content: &str) -> Option<String> {
    let goals = charter::parse_mission_goals(content);

    // 1. Check Goals
    if goals.is_empty() {
        return Some(
            "What is the primary objective of this mission? (This will become MG-01)".to_string(),
        );
    }

    if goals.iter().any(charter::goal_needs_description) {
        return Some("Please provide a description for the primary objective (MG-01).".to_string());
    }

    if goals.iter().any(charter::goal_needs_verification) {
        return Some("Please provide a concrete verification for MG-01 (for example `board: EPIC-1`, `metric: p95 < 1s`, or `manual: stakeholder review`).".to_string());
    }

    // 2. Check for board: goal
    if !goals
        .iter()
        .any(|g| matches!(&g.verification, GoalVerification::Board(target) if !target.trim().is_empty() && target.trim() != "..."))
    {
        return Some("Please add at least one goal with 'board:' verification (e.g. 'board: EPIC-1') so the system can track progress automatically.".to_string());
    }

    // 3. Check Constraints
    if extract_section(content, "## Constraints").is_none() {
        return Some("Please add a ## Constraints section to the CHARTER.md".to_string());
    }
    if !charter::has_authored_constraints(content) {
        return Some("Are there any operational constraints or boundaries for this mission? (e.g. budget, timeframe, technology)".to_string());
    }

    // 4. Check Halting Rules
    if extract_section(content, "## Halting Rules").is_none() {
        return Some("Please add a ## Halting Rules section to the CHARTER.md".to_string());
    }
    if !charter::has_authored_halting_rules(content) {
        return Some(
            "Please replace the default halting rules with mission-specific rules.".to_string(),
        );
    }

    None
}

fn process_answer(content: &str, answer: &str) -> Result<String> {
    if let Some(goals_section) = extract_section(content, "## Goals")
        && goals_section.contains("{{goal}}")
    {
        let updated_goals = goals_section.replace("{{goal}}", answer);
        return Ok(replace_section(content, "## Goals", &updated_goals));
    }

    if let Some(goals_section) = extract_section(content, "## Goals")
        && goals_section.contains("board: ...")
    {
        let updated_goals = goals_section.replacen("board: ...", answer.trim(), 1);
        return Ok(replace_section(content, "## Goals", &updated_goals));
    }

    if let Some(_constraints_section) = extract_section(content, "## Constraints")
        && !charter::has_authored_constraints(content)
    {
        let updated_constraints = format!("- {}\n", answer);
        return Ok(replace_section(
            content,
            "## Constraints",
            &updated_constraints,
        ));
    }

    if extract_section(content, "## Halting Rules").is_some()
        && !charter::has_authored_halting_rules(content)
    {
        let updated_rules = answer
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| {
                if line.starts_with("- ") {
                    line.to_string()
                } else {
                    format!("- {}", line)
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        let updated_rules = format!("{updated_rules}\n");
        return Ok(replace_section(content, "## Halting Rules", &updated_rules));
    }

    Err(anyhow!(
        "No active question to answer. Mission charter might already be complete."
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBearing, TestBoardBuilder, TestEpic, TestMission, TestVoyage};
    use std::fs;

    fn authored_charter(board_target: &str) -> String {
        format!(
            r#"
## Goals
| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Test goal | board: {board_target} |

## Constraints
- Keep the scope focused on one epic.

## Halting Rules
- Halt after the first epic is decomposed into ready voyages and stories.
- Yield to human review before changing external workflow contracts.
"#
        )
    }

    #[test]
    fn test_mission_activate() {
        let temp = TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Mission One")
                    .status("defining"),
            )
            .epic(TestEpic::new("E1").mission("M1"))
            .voyage(TestVoyage::new("V1", "E1").status("planned"))
            .build();

        // Add a goal to CHARTER.md manually
        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(charter_path, authored_charter("E1")).unwrap();

        MissionLifecycleService::activate(temp.path(), "M1").unwrap();

        let readme = fs::read_to_string(temp.path().join("missions/M1/README.md")).unwrap();
        assert!(readme.contains("status: active"));
        assert!(readme.contains("activated_at:"));
    }

    #[test]
    fn test_mission_activate_fails_without_children() {
        let temp = TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Mission One")
                    .status("defining"),
            )
            .build();

        // Add a goal to CHARTER.md manually
        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(charter_path, authored_charter("E1")).unwrap();

        let res = MissionLifecycleService::activate(temp.path(), "M1");
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("no child entities"));
    }

    #[test]
    fn test_mission_activate_fails_without_goals() {
        let temp = TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Mission One")
                    .status("defining"),
            )
            .epic(TestEpic::new("E1").mission("M1"))
            .voyage(TestVoyage::new("V1", "E1").status("planned"))
            .build();

        // Empty CHARTER.md (no goals)
        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(charter_path, "# Charter\n").unwrap();

        let res = MissionLifecycleService::activate(temp.path(), "M1");
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("at least one goal"));
    }

    #[test]
    fn test_mission_activate_fails_with_default_halting_rules() {
        let temp = TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Mission One")
                    .status("defining"),
            )
            .epic(TestEpic::new("E1").mission("M1"))
            .voyage(TestVoyage::new("V1", "E1").status("planned"))
            .build();

        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(
            charter_path,
            r#"
## Goals
| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Test goal | board: E1 |

## Constraints
- Keep the scope focused on one epic.

## Halting Rules
- DO NOT halt while any MG-* goal has unfinished board work.
- HALT when all MG-* goals with `board:` verification are satisfied.
- YIELD to human when only `metric:` or `manual:` goals remain.
"#,
        )
        .unwrap();

        let res = MissionLifecycleService::activate(temp.path(), "M1");
        assert!(res.is_err());
        assert!(
            res.unwrap_err()
                .to_string()
                .contains("mission-specific rules")
        );
    }

    #[test]
    fn test_mission_activate_fails_with_only_draft_epics_and_no_bearings() {
        let temp = TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Mission One")
                    .status("defining"),
            )
            .epic(TestEpic::new("E1").mission("M1"))
            .build();

        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(charter_path, authored_charter("E1")).unwrap();

        let res = MissionLifecycleService::activate(temp.path(), "M1");
        assert!(res.is_err());
        assert!(
            res.unwrap_err()
                .to_string()
                .contains("planned epic or bearing")
        );
    }

    #[test]
    fn test_mission_activate_allows_bearing_without_planned_epic() {
        let temp = TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Mission One")
                    .status("defining"),
            )
            .epic(TestEpic::new("E1").mission("M1"))
            .bearing(TestBearing::new("B1").mission("M1"))
            .build();

        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(charter_path, authored_charter("B1")).unwrap();

        MissionLifecycleService::activate(temp.path(), "M1").unwrap();

        let readme = fs::read_to_string(temp.path().join("missions/M1/README.md")).unwrap();
        assert!(readme.contains("status: active"));
    }

    #[test]
    fn test_mission_achieve_checks_goals() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .voyage(TestVoyage::new("V1", "E1").status("done"))
            .build();

        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        let charter = r#"
## Goals
| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Test goal | board: MISSING |
"#;
        fs::write(charter_path, charter).unwrap();

        // Should fail because the board target is not satisfied even though the
        // mission's child entities are already terminal.
        let res = MissionLifecycleService::achieve(
            &spoke_auth::ExecutionContext::new_local("test".into()),
            temp.path(),
            "M1",
        );
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("unmet board goal"));
    }

    #[test]
    fn test_mission_achieve_requires_log_and_children() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .build();

        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(charter_path, "## Goals\n| ID | Description | Verification |\n|----|-------------|--------------|\n| MG-01 | G1 | manual: test |\n").unwrap();

        // Should fail because no children
        let res = MissionLifecycleService::achieve(
            &spoke_auth::ExecutionContext::new_local("test".into()),
            temp.path(),
            "M1",
        );
        assert!(res.is_err());
        assert!(
            res.unwrap_err()
                .to_string()
                .contains("no child entities found")
        );

        // Add a child (epic)
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .voyage(TestVoyage::new("V1", "E1").status("done"))
            .build();
        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(charter_path, "## Goals\n| ID | Description | Verification |\n|----|-------------|--------------|\n| MG-01 | G1 | manual: test |\n").unwrap();

        // Should fail because no log entries
        let res = MissionLifecycleService::achieve(
            &spoke_auth::ExecutionContext::new_local("test".into()),
            temp.path(),
            "M1",
        );
        assert!(res.is_err());
        assert!(
            res.unwrap_err()
                .to_string()
                .contains("requires at least one entry in LOG.md")
        );

        // Add a log entry
        MissionLifecycleService::log(temp.path(), "M1", "Did some work").unwrap();

        // Should now succeed (no board goals, manual goal doesn't block)
        let res = MissionLifecycleService::achieve(
            &spoke_auth::ExecutionContext::new_local("test".into()),
            temp.path(),
            "M1",
        );
        assert!(res.is_ok());
    }

    #[test]
    fn test_mission_achieve_requires_terminal_children() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .voyage(TestVoyage::new("V1", "E1").status("done"))
            .bearing(
                TestBearing::new("B1")
                    .mission("M1")
                    .status("ready")
                    .has_evidence(true)
                    .has_assessment(true),
            )
            .build();

        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(
            charter_path,
            "## Goals\n| ID | Description | Verification |\n|----|-------------|--------------|\n| MG-01 | G1 | manual: test |\n",
        )
        .unwrap();
        MissionLifecycleService::log(temp.path(), "M1", "Did some work").unwrap();

        let res = MissionLifecycleService::achieve(
            &spoke_auth::ExecutionContext::new_local("test".into()),
            temp.path(),
            "M1",
        );
        assert!(res.is_err());
        let err = res.unwrap_err().to_string();
        assert!(err.contains("non-terminal child entities"));
        assert!(err.contains("bearing B1 (ready)"));
    }

    #[test]
    fn test_mission_verify_with_artifact() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("achieved"))
            .epic(TestEpic::new("E1").mission("M1"))
            .voyage(TestVoyage::new("V1", "E1").status("done"))
            .build();

        let artifact = "verification.gif";
        MissionLifecycleService::verify(temp.path(), "M1", Some(artifact)).unwrap();

        let readme = fs::read_to_string(temp.path().join("missions/M1/README.md")).unwrap();
        assert!(readme.contains("status: verified"));
        assert!(readme.contains("verification_artifact: verification.gif"));
    }

    #[test]
    fn test_mission_verify_requires_terminal_children() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("achieved"))
            .epic(TestEpic::new("E1").mission("M1"))
            .voyage(TestVoyage::new("V1", "E1").status("done"))
            .bearing(
                TestBearing::new("B1")
                    .mission("M1")
                    .status("ready")
                    .has_evidence(true)
                    .has_assessment(true),
            )
            .build();

        MissionLifecycleService::log(temp.path(), "M1", "Did some work").unwrap();

        let res = MissionLifecycleService::verify(temp.path(), "M1", None);
        assert!(res.is_err());
        let err = res.unwrap_err().to_string();
        assert!(err.contains("non-terminal child entities"));
        assert!(err.contains("bearing B1 (ready)"));
    }

    #[test]
    fn test_mission_refine_flow() {
        let temp = TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Mission One")
                    .status("defining"),
            )
            .build();

        let charter_path = temp.path().join("missions/M1/CHARTER.md");

        // Initial question
        let content = fs::read_to_string(&charter_path).unwrap();
        let q1 = get_next_question(&content).unwrap();
        assert!(q1.contains("primary objective"));

        // Answer q1
        let content = process_answer(&content, "My objective").unwrap();
        assert!(content.contains("MG-01 | My objective |"));

        // Second question
        let q2 = get_next_question(&content).unwrap();
        assert!(q2.contains("concrete verification"));

        // Answer q2
        let content = process_answer(&content, "board: E1").unwrap();
        assert!(content.contains("board: E1"));

        // Third question
        let q3 = get_next_question(&content).unwrap();
        assert!(q3.contains("operational constraints"));

        // Answer q3
        let content = process_answer(&content, "My constraint").unwrap();
        assert!(content.contains("- My constraint"));

        // Fourth question
        let q4 = get_next_question(&content).unwrap();
        assert!(q4.contains("halting rules"));

        // Answer q4
        let content = process_answer(
            &content,
            "- Halt after the first epic is decomposed into ready voyages.\n- Yield to human if the mission needs a new ADR.",
        )
        .unwrap();
        assert!(content.contains("new ADR"));

        // No more questions
        let q5 = get_next_question(&content);
        assert!(q5.is_none());
    }

    #[test]
    fn test_parse_log_entries() {
        let log = r#"# Log
Header stuff

## 2026-03-09T10:00:00
Entry 1

## 2026-03-09T11:00:00
Entry 2
"#;
        let (header, entries) = parse_log_entries(log);
        assert!(header.contains("# Log"));
        assert_eq!(entries.len(), 2);
        assert!(entries[0].raw.contains("Entry 1"));
        assert!(entries[1].raw.contains("Entry 2"));
    }
}
