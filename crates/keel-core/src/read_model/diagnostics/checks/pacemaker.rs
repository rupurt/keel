//! Pacemaker integrity checks (heartbeat)

use crate::domain::model::Board;
use crate::infrastructure::validation::{CheckId, Problem, Severity};
use chrono::Utc;

/// Check whether repository activity has uncommitted energy.
pub fn check_pacemaker_stability(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();
    let heartbeat = crate::read_model::heartbeat::project(&board.root, Utc::now());

    if heartbeat.dirty {
        let project_root = board.root.parent().unwrap_or(&board.root);
        let path = heartbeat
            .latest_path
            .as_ref()
            .map(|relative| project_root.join(relative))
            .unwrap_or_else(|| project_root.to_path_buf());
        problems.push(Problem {
            severity: Severity::Warning,
            path,
            message:
                "Pacemaker has uncommitted energy. Commit worktree changes to stabilize the board."
                    .to_string(),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::check_pacemaker_stability;
    use crate::domain::model::Board;
    use crate::infrastructure::validation::Severity;
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use tempfile::TempDir;

    #[test]
    fn clean_repo_is_not_a_problem() {
        let temp = TempDir::new().unwrap();
        init_git_repo(temp.path());
        fs::create_dir_all(temp.path().join(".keel")).unwrap();
        fs::write(temp.path().join(".keel/README.md"), "# Board\n").unwrap();
        git(temp.path(), &["add", ".keel/README.md"]);
        git(temp.path(), &["commit", "-m", "seed"]);
        let board = Board::new(temp.path().join(".keel"));

        let problems = check_pacemaker_stability(&board);

        assert!(problems.is_empty());
    }

    #[test]
    fn dirty_worktree_remains_a_warning() {
        let temp = TempDir::new().unwrap();
        init_git_repo(temp.path());
        fs::create_dir_all(temp.path().join(".keel")).unwrap();
        fs::write(temp.path().join(".keel/README.md"), "# Board\n").unwrap();
        git(temp.path(), &["add", ".keel/README.md"]);
        git(temp.path(), &["commit", "-m", "seed"]);
        fs::write(temp.path().join("README.md"), "# Project change\n").unwrap();
        let board = Board::new(temp.path().join(".keel"));

        let problems = check_pacemaker_stability(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Warning);
        assert!(problems[0].message.contains("uncommitted energy"));
    }

    fn init_git_repo(dir: &Path) {
        git(dir, &["init", "--quiet"]);
        git(dir, &["config", "user.name", "Keel Test"]);
        git(dir, &["config", "user.email", "keel@example.com"]);
        git(dir, &["config", "commit.gpgsign", "false"]);
    }

    fn git(dir: &Path, args: &[&str]) {
        let output = Command::new("git")
            .args(args)
            .current_dir(dir)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {:?} failed: stdout=`{}` stderr=`{}`",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
