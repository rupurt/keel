//! Mission lifecycle application service

use anyhow::{Result, anyhow};
use std::path::Path;

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

    pub fn achieve(board_dir: &Path, id: &str) -> Result<()> {
        let board = load_board(board_dir)?;
        let mission = board.require_mission(id)?;

        // Verify goals before achievement
        let charter_path = mission.path.parent().unwrap().join("CHARTER.md");
        let charter_content = std::fs::read_to_string(&charter_path).unwrap_or_default();
        let goals = charter::parse_mission_goals(&charter_content);

        let unmet_goals: Vec<_> = goals
            .iter()
            .filter(|g| {
                matches!(g.verification, GoalVerification::Board(_))
                    && !is_goal_met(&board, g.verification.raw())
            })
            .collect();

        if !unmet_goals.is_empty() {
            println!(
                "Cannot achieve mission {}. The following board goals are unmet:",
                id
            );
            for goal in unmet_goals {
                println!("  - {}: {}", goal.id, goal.description);
            }
            return Err(anyhow!("Mission has unmet board goals"));
        }

        // Verify child entities
        if board.mission_child_count(id) == 0 {
            return Err(anyhow!(
                "Cannot achieve mission {}. At least one child entity (epic, bearing, or ADR) is required.",
                id
            ));
        }

        // Verify log entries
        let log_path = mission.path.parent().unwrap().join("LOG.md");
        let log_content = std::fs::read_to_string(&log_path).unwrap_or_default();
        let (_, entries) = parse_log_entries(&log_content);
        if entries.is_empty() {
            return Err(anyhow!(
                "Cannot achieve mission {}. At least one entry in LOG.md is required to document the session.",
                id
            ));
        }

        execute(board_dir, id, &mission_transitions::ACHIEVE)?;
        println!("Achieved mission: {}", id);
        Ok(())
    }

    pub fn verify(board_dir: &Path, id: &str) -> Result<()> {
        let board = load_board(board_dir)?;
        let _mission = board.require_mission(id)?;

        // Verify child entities
        if board.mission_child_count(id) == 0 {
            return Err(anyhow!(
                "Cannot verify mission {}. At least one child entity (epic, bearing, or ADR) is required.",
                id
            ));
        }

        // Verify log entries
        let mission = board.require_mission(id)?;
        let log_path = mission.path.parent().unwrap().join("LOG.md");
        let log_content = std::fs::read_to_string(&log_path).unwrap_or_default();
        let (_, entries) = parse_log_entries(&log_content);
        if entries.is_empty() {
            return Err(anyhow!(
                "Cannot verify mission {}. At least one entry in LOG.md is required to document the session.",
                id
            ));
        }

        execute(board_dir, id, &mission_transitions::VERIFY)?;
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

        let charter_path = mission.path.parent().unwrap().join("CHARTER.md");
        let charter_content = std::fs::read_to_string(&charter_path).unwrap_or_default();
        let goals = charter::parse_mission_goals(&charter_content);

        if goals.is_empty() || goals.iter().any(|g| g.description.contains("{{goal}}")) {
            return Err(anyhow!(
                "Cannot activate mission {}. It has no goals defined in CHARTER.md",
                id
            ));
        }

        // Verify child entities - missions need at least one child to be actionable
        if board.mission_child_count(id) == 0 {
            return Err(anyhow!(
                "Cannot activate mission {}. At least one child entity (epic, bearing, or ADR) is required before activation.",
                id
            ));
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

fn is_goal_met(board: &crate::domain::model::Board, verification: &str) -> bool {
    let target = verification.trim_start_matches("board:").trim();
    if target.is_empty() || target == "..." {
        return false;
    }

    // Check if it's an epic
    if let Some(epic) = board.epics.get(target) {
        return epic.status() == crate::domain::model::EpicState::Done;
    }

    // Check if it's a voyage
    if let Some(voyage) = board.voyages.get(target) {
        return voyage.status() == crate::domain::state_machine::voyage::VoyageState::Done;
    }

    // Check if it's a story
    if let Some(story) = board.stories.get(target) {
        return story.status == crate::domain::model::StoryState::Done;
    }

    false
}

fn get_next_question(content: &str) -> Option<String> {
    let goals = charter::parse_mission_goals(content);

    // 1. Check Goals
    if goals.is_empty() {
        return Some(
            "What is the primary objective of this mission? (This will become MG-01)".to_string(),
        );
    }

    if goals.iter().any(|g| g.description.contains("{{goal}}")) {
        return Some("Please provide a description for the primary objective (MG-01).".to_string());
    }

    // 2. Check for board: goal
    if !goals
        .iter()
        .any(|g| matches!(g.verification, GoalVerification::Board(_)))
    {
        return Some("Please add at least one goal with 'board:' verification (e.g. 'board: EPIC-1') so the system can track progress automatically.".to_string());
    }

    // 3. Check Constraints
    if let Some(constraints_section) = extract_section(content, "## Constraints") {
        if constraints_section.contains("(none yet)") || constraints_section.trim().is_empty() {
            return Some("Are there any operational constraints or boundaries for this mission? (e.g. budget, timeframe, technology)".to_string());
        }
    } else {
        return Some("Please add a ## Constraints section to the CHARTER.md".to_string());
    }

    // 4. Check Halting Rules
    if let Some(halting_section) = extract_section(content, "## Halting Rules") {
        if halting_section.trim().is_empty() {
            return Some("Please define Halting Rules for this mission.".to_string());
        }
    } else {
        return Some("Please add a ## Halting Rules section to the CHARTER.md".to_string());
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

    if let Some(constraints_section) = extract_section(content, "## Constraints")
        && constraints_section.contains("(none yet)")
    {
        let updated_constraints = format!("- {}\n", answer);
        return Ok(replace_section(
            content,
            "## Constraints",
            &updated_constraints,
        ));
    }

    Err(anyhow!(
        "No active question to answer. Mission charter might already be complete."
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestMission};
    use std::fs;

    #[test]
    fn test_mission_activate() {
        let temp = TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Mission One")
                    .status("defining"),
            )
            .epic(TestEpic::new("E1").mission("M1"))
            .build();

        // Add a goal to CHARTER.md manually
        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        let charter = r#"
## Goals
| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Test goal | board: E1 |
"#;
        fs::write(charter_path, charter).unwrap();

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
        let charter = r#"
## Goals
| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Test goal | board: E1 |
"#;
        fs::write(charter_path, charter).unwrap();

        let res = MissionLifecycleService::activate(temp.path(), "M1");
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("child entity"));
    }

    #[test]
    fn test_mission_activate_fails_without_goals() {
        let temp = TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Mission One")
                    .status("defining"),
            )
            .build();

        // Empty CHARTER.md (no goals)
        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(charter_path, "# Charter\n").unwrap();

        let res = MissionLifecycleService::activate(temp.path(), "M1");
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("no goals"));
    }

    #[test]
    fn test_mission_achieve_checks_goals() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One").status("active"))
            .epic(TestEpic::new("E1").mission("M1")) // Epic E1 is NOT done
            .build();

        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        let charter = r#"
## Goals
| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Test goal | board: E1 |
"#;
        fs::write(charter_path, charter).unwrap();

        // Should fail because E1 is not done
        let res = MissionLifecycleService::achieve(temp.path(), "M1");
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("unmet board goals"));
    }

    #[test]
    fn test_mission_achieve_requires_log_and_children() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .build();

        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(charter_path, "## Goals\n| ID | Description | Verification |\n|----|-------------|--------------|\n| MG-01 | G1 | manual: test |\n").unwrap();

        // Should fail because no children
        let res = MissionLifecycleService::achieve(temp.path(), "M1");
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("one child entity"));

        // Add a child (epic)
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .build();
        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(charter_path, "## Goals\n| ID | Description | Verification |\n|----|-------------|--------------|\n| MG-01 | G1 | manual: test |\n").unwrap();

        // Should fail because no log entries
        let res = MissionLifecycleService::achieve(temp.path(), "M1");
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("one entry in LOG.md"));

        // Add a log entry
        MissionLifecycleService::log(temp.path(), "M1", "Did some work").unwrap();

        // Should now succeed (no board goals, manual goal doesn't block)
        let res = MissionLifecycleService::achieve(temp.path(), "M1");
        assert!(res.is_ok());
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
        assert!(q2.contains("operational constraints"));

        // Answer q2
        let content = process_answer(&content, "My constraint").unwrap();
        assert!(content.contains("- My constraint"));

        // No more questions
        let q3 = get_next_question(&content);
        assert!(q3.is_none());
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
