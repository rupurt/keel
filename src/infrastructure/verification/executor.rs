//! Execution of verification commands

use anyhow::Result;
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use crate::domain::model::Manifest;
use crate::infrastructure::utils::{get_git_sha, hash_file};

#[derive(Debug, Clone)]
pub struct ExecuteResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub error: Option<String>,
}

impl ExecuteResult {
    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn unwrap(self) -> Self {
        if self.is_err() {
            panic!(
                "called `ExecuteResult::unwrap()` on an `Err` value: {:?}",
                self.error
            );
        }
        self
    }

    pub fn unwrap_err(self) -> String {
        self.error
            .clone()
            .expect("called `ExecuteResult::unwrap_err()` on an `Ok` value")
    }
}

use wait_timeout::ChildExt;

pub fn execute(cmd: &str, cwd: &Path, timeout: Duration) -> Result<ExecuteResult> {
    // Basic implementation for tests/compilation
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .current_dir(cwd)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let (exit_status, stdout, stderr) = match child.wait_timeout(timeout)? {
        Some(status) => {
            let output = child.wait_with_output()?;
            (
                status,
                String::from_utf8_lossy(&output.stdout).to_string(),
                String::from_utf8_lossy(&output.stderr).to_string(),
            )
        }
        None => {
            child.kill()?;
            (
                std::process::ExitStatus::default(), // Dummy since we timed out
                String::new(),
                "Command timed out".to_string(),
            )
        }
    };

    if stderr.contains("Command timed out") {
        return Ok(ExecuteResult {
            exit_code: 1,
            stdout,
            stderr,
            error: Some("Timeout".to_string()),
        });
    }

    Ok(ExecuteResult {
        exit_code: exit_status.code().unwrap_or(1),
        stdout,
        stderr,
        error: None,
    })
}

fn execute_vhs(_board_dir: &Path, story_dir: &Path, cmd: &str) -> Result<ExecuteResult> {
    let tape_file = cmd.strip_prefix("vhs ").unwrap_or(cmd).trim();
    let tape_path = story_dir.join(tape_file);

    if !tape_path.exists() {
        return Ok(ExecuteResult {
            exit_code: 1,
            stdout: String::new(),
            stderr: format!("Tape file not found: {:?}", tape_path),
            error: Some("TapeNotFound".to_string()),
        });
    }

    // Ensure EVIDENCE/ directory exists
    let evidence_dir = story_dir.join("EVIDENCE");
    if !evidence_dir.exists() {
        std::fs::create_dir_all(&evidence_dir)?;
    }

    let output_gif = evidence_dir.join("record-cli.gif");
    crate::infrastructure::vhs::run_tape(story_dir, &tape_path, std::slice::from_ref(&output_gif))
}

pub fn execute_llm_judge(
    board_dir: &Path,
    story_id: &str,
    criterion: &str,
) -> Result<ExecuteResult> {
    let story_dir = board_dir.join("stories").join(story_id);
    let evidence_dir = story_dir.join("EVIDENCE");
    if !evidence_dir.exists() {
        std::fs::create_dir_all(&evidence_dir)?;
    }

    let bundle_path = crate::infrastructure::verification::judge_bundle::materialize_judge_bundle(
        board_dir, story_id, criterion,
    )?;

    let project_root = board_dir.parent().unwrap_or(board_dir);
    let execution = match Command::new("llm-judge")
        .arg(&bundle_path)
        .current_dir(project_root)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
    {
        Ok(mut child) => match child.wait_timeout(Duration::from_secs(60))? {
            Some(status) => {
                let output = child.wait_with_output()?;
                ExecuteResult {
                    exit_code: status.code().unwrap_or(1),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    error: None,
                }
            }
            None => {
                child.kill()?;
                ExecuteResult {
                    exit_code: 1,
                    stdout: String::new(),
                    stderr: "llm-judge timed out".to_string(),
                    error: Some("Timeout".to_string()),
                }
            }
        },
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => ExecuteResult {
            exit_code: 1,
            stdout: String::new(),
            stderr: "llm-judge executable not found on PATH. Install or configure a provider-agnostic llm-judge wrapper.".to_string(),
            error: Some("JudgeUnavailable".to_string()),
        },
        Err(err) => ExecuteResult {
            exit_code: 1,
            stdout: String::new(),
            stderr: format!("Failed to launch llm-judge: {err}"),
            error: Some("JudgeLaunchFailed".to_string()),
        },
    };

    persist_judge_outputs(
        &evidence_dir,
        criterion,
        &bundle_path,
        &execution.stdout,
        &execution.stderr,
        execution.exit_code,
    )?;

    Ok(execution)
}

fn persist_judge_outputs(
    evidence_dir: &Path,
    criterion: &str,
    bundle_path: &Path,
    stdout: &str,
    stderr: &str,
    exit_code: i32,
) -> Result<()> {
    let slug = crate::infrastructure::utils::slugify(criterion);
    let transcript_name = format!("llm-judge-{slug}.txt");
    let transcript_path = evidence_dir.join(&transcript_name);
    let bundle_rel = format!(
        "EVIDENCE/{}",
        bundle_path.file_name().unwrap().to_string_lossy()
    );

    let transcript_body = if stdout.trim().is_empty() {
        format!(
            "LLM judge produced no stdout transcript.\nCriterion: {criterion}\nBundle: {bundle_rel}\nExit code: {exit_code}\n"
        )
    } else {
        stdout.to_string()
    };
    std::fs::write(&transcript_path, transcript_body)?;

    let stderr_name = if stderr.trim().is_empty() {
        None
    } else {
        let name = format!("llm-judge-{slug}.stderr.txt");
        std::fs::write(evidence_dir.join(&name), stderr)?;
        Some(format!("EVIDENCE/{name}"))
    };

    let result_path = evidence_dir.join(format!("llm-judge-{slug}.result.json"));
    let result_json = serde_json::json!({
        "criterion": criterion,
        "passed": exit_code == 0,
        "exit_code": exit_code,
        "bundle_path": bundle_rel,
        "transcript_path": format!("EVIDENCE/{transcript_name}"),
        "stderr_path": stderr_name,
    });
    std::fs::write(result_path, serde_json::to_vec_pretty(&result_json)?)?;

    Ok(())
}

pub fn verify_story(
    board_dir: &Path,
    story_id: &str,
    content: &str,
) -> Result<super::reporter::VerificationReport> {
    let annotations = super::parser::parse_verify_annotations(content);
    let mut results = Vec::new();
    let story_dir = board_dir.join("stories").join(story_id);

    for ann in annotations {
        let cmd = ann.command.as_deref().unwrap_or("manual");
        if cmd == "manual" {
            results.push(super::reporter::VerificationResult {
                criterion: ann.criterion.clone(),
                passed: false,
                actual: "manual verification required".to_string(),
                expected: "success".to_string(),
                requires_human_review: true,
            });
            continue;
        }

        if cmd.starts_with("vhs ") {
            let res = execute_vhs(board_dir, &story_dir, cmd)?;
            results.push(super::reporter::VerificationResult {
                criterion: ann.criterion.clone(),
                passed: res.exit_code == 0,
                actual: if res.exit_code == 0 {
                    "vhs recording successful".to_string()
                } else {
                    format!("vhs failed: {}", res.stderr)
                },
                expected: "vhs recording".to_string(),
                requires_human_review: false,
            });
            continue;
        }

        if cmd == "llm-judge" {
            let res = execute_llm_judge(board_dir, story_id, &ann.criterion)?;
            results.push(super::reporter::VerificationResult {
                criterion: ann.criterion.clone(),
                passed: res.exit_code == 0,
                actual: if res.exit_code == 0 {
                    "llm-judge passed".to_string()
                } else {
                    format!("llm-judge failed: {}", res.stderr)
                },
                expected: "llm-judge signature".to_string(),
                requires_human_review: false,
            });
            continue;
        }

        let res = execute(cmd, board_dir, Duration::from_secs(30))?;
        results.push(super::reporter::VerificationResult {
            criterion: ann.criterion.clone(),
            passed: res.exit_code == 0,
            actual: format!("exit code {}", res.exit_code),
            expected: "exit code 0".to_string(),
            requires_human_review: false,
        });
    }

    // Generate manifest if verification succeeded (or even if not, to capture state?)
    // AC-01: `keel verify` generates a signed manifest linking artifacts to current Git SHA
    if let Err(e) = generate_manifest(board_dir, story_id) {
        eprintln!(
            "Warning: Failed to generate manifest for {}: {}",
            story_id, e
        );
    }

    Ok(super::reporter::VerificationReport {
        story_id: story_id.to_string(),
        results,
    })
}

pub fn generate_manifest(board_dir: &Path, story_id: &str) -> Result<()> {
    let story_dir = board_dir.join("stories").join(story_id);
    if !story_dir.exists() {
        return Ok(());
    }

    let git_sha = match get_git_sha(board_dir) {
        Ok(sha) => sha,
        Err(e) => {
            // If not in a git repo, use a placeholder or fail?
            // Since we are an agent in a git repo, we expect this to work.
            return Err(e);
        }
    };
    let mut evidence = BTreeMap::new();

    let evidence_dir = story_dir.join("EVIDENCE");
    if evidence_dir.exists() {
        for entry in std::fs::read_dir(evidence_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                let hash = hash_file(&path)?;
                evidence.insert(format!("EVIDENCE/{}", filename), hash);
            }
        }
    }

    let manifest = Manifest {
        id: story_id.to_string(),
        git_sha,
        evidence,
    };

    let manifest_path = story_dir.join("manifest.yaml");
    let yaml = serde_yaml::to_string(&manifest)?;
    std::fs::write(manifest_path, yaml)?;

    Ok(())
}

pub fn verify_all(board_dir: &Path) -> Result<Vec<super::reporter::VerificationReport>> {
    let board = crate::infrastructure::loader::load_board(board_dir)?;
    let mut reports = Vec::new();

    let mut stories: Vec<_> = board.stories.values().collect();

    // Sort by: Epic index (asc), Voyage index (asc), Story index (asc)
    stories.sort_by(|a, b| {
        // 1. Epic index (asc)
        let epic_a = a.epic().and_then(|id| board.epics.get(id));
        let epic_b = b.epic().and_then(|id| board.epics.get(id));
        let epic_idx_a = epic_a.and_then(|e| e.frontmatter.index).unwrap_or(0);
        let epic_idx_b = epic_b.and_then(|e| e.frontmatter.index).unwrap_or(0);

        let epic_cmp = epic_idx_a.cmp(&epic_idx_b);
        if epic_cmp != std::cmp::Ordering::Equal {
            return epic_cmp;
        }

        // 2. Voyage index (asc)
        let voyage_a = a.voyage().and_then(|id| board.voyages.get(id));
        let voyage_b = b.voyage().and_then(|id| board.voyages.get(id));
        let voyage_idx_a = voyage_a.and_then(|v| v.frontmatter.index).unwrap_or(0);
        let voyage_idx_b = voyage_b.and_then(|v| v.frontmatter.index).unwrap_or(0);

        let voyage_cmp = voyage_idx_a.cmp(&voyage_idx_b);
        if voyage_cmp != std::cmp::Ordering::Equal {
            return voyage_cmp;
        }

        // 3. Story index (asc)
        let story_idx_a = a.index().unwrap_or(0);
        let story_idx_b = b.index().unwrap_or(0);

        let story_cmp = story_idx_a.cmp(&story_idx_b);
        if story_cmp != std::cmp::Ordering::Equal {
            return story_cmp;
        }

        // Fallback to ID (asc)
        a.id().cmp(b.id())
    });

    for story in stories {
        let content = std::fs::read_to_string(&story.path)?;
        let report = verify_story(board_dir, story.id(), &content)?;
        if !report.results.is_empty() {
            reports.push(report);
        }
    }

    Ok(reports)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::commands::management::story::record;
    use crate::domain::model::Manifest;
    use crate::test_helpers::{TestBoardBuilder, TestStory};
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::process::Command as ProcessCommand;
    use tempfile::tempdir;

    struct PathGuard(Option<String>);

    impl PathGuard {
        fn prepend(dir: &Path) -> Self {
            let original = std::env::var("PATH").ok();
            let mut entries = vec![dir.display().to_string()];
            if let Some(existing) = &original {
                entries.push(existing.clone());
            }
            unsafe {
                std::env::set_var("PATH", entries.join(":"));
            }
            Self(original)
        }
    }

    impl Drop for PathGuard {
        fn drop(&mut self) {
            match self.0.take() {
                Some(path) => unsafe { std::env::set_var("PATH", path) },
                None => unsafe { std::env::remove_var("PATH") },
            }
        }
    }

    fn write_mock_llm_judge(dir: &Path, body: &str) {
        let script_path = dir.join("llm-judge");
        fs::write(&script_path, body).unwrap();
        let mut perms = fs::metadata(&script_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).unwrap();
    }

    fn install_contract_asserting_llm_judge(dir: &Path) -> std::path::PathBuf {
        let invocation_log = dir.join("judge-invocation.log");
        write_mock_llm_judge(
            dir,
            &format!(
                r#"#!/usr/bin/env bash
set -euo pipefail
bundle="$1"
test -f "$bundle"
grep -q '"id": "S1"' "$bundle"
grep -q '"text": "AC 1"' "$bundle"
printf '%s' "$bundle" > "{}"
printf 'PASS: %s\n' "$bundle"
"#,
                invocation_log.display()
            ),
        );
        invocation_log
    }

    fn install_failing_llm_judge(dir: &Path) {
        write_mock_llm_judge(
            dir,
            r#"#!/usr/bin/env bash
set -euo pipefail
bundle="$1"
test -f "$bundle"
printf 'FAIL: %s\n' "$bundle"
printf 'judge rejected artifact bundle\n' >&2
exit 2
"#,
        );
    }

    fn init_git_repo(dir: &Path) {
        assert!(
            ProcessCommand::new("git")
                .args(["init", "-q"])
                .current_dir(dir)
                .status()
                .unwrap()
                .success()
        );
        assert!(
            ProcessCommand::new("git")
                .args(["config", "user.email", "test@example.com"])
                .current_dir(dir)
                .status()
                .unwrap()
                .success()
        );
        assert!(
            ProcessCommand::new("git")
                .args(["config", "user.name", "Test User"])
                .current_dir(dir)
                .status()
                .unwrap()
                .success()
        );
        assert!(
            ProcessCommand::new("git")
                .args(["add", "."])
                .current_dir(dir)
                .status()
                .unwrap()
                .success()
        );
        assert!(
            ProcessCommand::new("git")
                .args(["commit", "-qm", "init"])
                .current_dir(dir)
                .status()
                .unwrap()
                .success()
        );
    }

    #[test]
    fn test_verify_story_executes_grep_proof() {
        let dir = tempdir().unwrap();
        let story_path = dir.path().join("README.md");
        fs::write(&story_path, "## Acceptance Criteria\n\n- [ ] AC 1 <!-- verify: grep -q 'UNI''QUE' README.md, SRS-01:start -->").unwrap();

        // This should fail because 'UNIQUE' is not in the file (only 'UNI' 'QUE' in the comment)
        let report =
            verify_story(dir.path(), "S1", &fs::read_to_string(&story_path).unwrap()).unwrap();
        assert_eq!(report.results.len(), 1);
        assert!(!report.results[0].passed);

        // Now make it pass
        fs::write(&story_path, "UNIQUE\n## Acceptance Criteria\n\n- [ ] AC 1 <!-- verify: grep -q 'UNI''QUE' README.md, SRS-01:start -->").unwrap();
        let report =
            verify_story(dir.path(), "S1", &fs::read_to_string(&story_path).unwrap()).unwrap();
        assert!(report.results[0].passed);
    }

    #[test]
    fn llm_judge_uses_artifact_bundle_contract() {
        let dir = tempdir().unwrap();
        let invocation_log = install_contract_asserting_llm_judge(dir.path());
        let _path_guard = PathGuard::prepend(dir.path());
        let stories_dir = dir.path().join("stories").join("S1");
        fs::create_dir_all(stories_dir.join("EVIDENCE")).unwrap();
        let story_path = stories_dir.join("README.md");
        fs::write(
            &story_path,
            r#"---
id: S1
title: Judge Story
type: feat
status: in-progress
created_at: 2026-03-06T00:00:00
updated_at: 2026-03-06T00:00:00
---

## Acceptance Criteria

- [ ] [SRS-01/AC-01] AC 1 <!-- verify: llm-judge, SRS-01:start:end -->"#,
        )
        .unwrap();
        fs::write(stories_dir.join("EVIDENCE/ac-1.log"), "proof").unwrap();

        let report =
            verify_story(dir.path(), "S1", &fs::read_to_string(&story_path).unwrap()).unwrap();
        assert_eq!(report.results.len(), 1);
        assert!(report.results[0].passed);
        assert_eq!(report.results[0].actual, "llm-judge passed");
        let bundle_path = stories_dir.join("EVIDENCE/judge-bundle-ac-1.json");
        assert_eq!(
            fs::read_to_string(invocation_log).unwrap(),
            bundle_path.display().to_string()
        );
    }

    #[test]
    fn verification_executor_materializes_judge_bundle() {
        let dir = tempdir().unwrap();
        let _path_guard = PathGuard::prepend(dir.path());
        install_contract_asserting_llm_judge(dir.path());
        let stories_dir = dir.path().join("stories").join("S1");
        fs::create_dir_all(stories_dir.join("EVIDENCE")).unwrap();
        let story_path = stories_dir.join("README.md");
        fs::write(
            &story_path,
            r#"---
id: S1
title: Judge Story
type: feat
status: in-progress
created_at: 2026-03-06T00:00:00
updated_at: 2026-03-06T00:00:00
---

## Acceptance Criteria

- [ ] [SRS-01/AC-01] AC 1 <!-- verify: llm-judge, SRS-01:start:end -->"#,
        )
        .unwrap();
        fs::write(stories_dir.join("EVIDENCE/ac-1.log"), "proof").unwrap();

        let report =
            verify_story(dir.path(), "S1", &fs::read_to_string(&story_path).unwrap()).unwrap();
        assert!(report.results[0].passed);

        let bundle_path = stories_dir.join("EVIDENCE/judge-bundle-ac-1.json");
        assert!(bundle_path.exists());

        let bundle: crate::infrastructure::verification::JudgeBundle =
            serde_json::from_str(&fs::read_to_string(&bundle_path).unwrap()).unwrap();
        assert_eq!(bundle.story.id, "S1");
        assert_eq!(bundle.criterion.text, "AC 1");
        assert_eq!(bundle.criterion.srs_requirement.as_deref(), Some("SRS-01"));
    }

    #[test]
    fn judge_results_persist_as_story_evidence() {
        let verify_dir = tempdir().unwrap();
        let invocation_log = install_contract_asserting_llm_judge(verify_dir.path());
        let _path_guard = PathGuard::prepend(verify_dir.path());
        let stories_dir = verify_dir.path().join("stories").join("S1");
        fs::create_dir_all(stories_dir.join("EVIDENCE")).unwrap();
        let story_path = stories_dir.join("README.md");
        fs::write(
            &story_path,
            r#"---
id: S1
title: Judge Story
type: feat
status: in-progress
created_at: 2026-03-06T00:00:00
updated_at: 2026-03-06T00:00:00
---

## Acceptance Criteria

- [ ] [SRS-01/AC-01] AC 1 <!-- verify: llm-judge, SRS-01:start:end -->"#,
        )
        .unwrap();
        fs::write(stories_dir.join("EVIDENCE/ac-1.log"), "proof").unwrap();
        init_git_repo(verify_dir.path());

        let report = verify_story(
            verify_dir.path(),
            "S1",
            &fs::read_to_string(&story_path).unwrap(),
        )
        .unwrap();
        assert!(report.results[0].passed);

        let bundle_path = stories_dir.join("EVIDENCE/judge-bundle-ac-1.json");
        let transcript_path = stories_dir.join("EVIDENCE/llm-judge-ac-1.txt");
        let result_path = stories_dir.join("EVIDENCE/llm-judge-ac-1.result.json");
        assert_eq!(
            fs::read_to_string(invocation_log).unwrap(),
            bundle_path.display().to_string()
        );
        assert!(transcript_path.exists());
        assert!(result_path.exists());

        let manifest: Manifest =
            serde_yaml::from_str(&fs::read_to_string(stories_dir.join("manifest.yaml")).unwrap())
                .unwrap();
        assert!(
            manifest
                .evidence
                .contains_key("EVIDENCE/judge-bundle-ac-1.json")
        );
        assert!(
            manifest
                .evidence
                .contains_key("EVIDENCE/llm-judge-ac-1.txt")
        );
        assert!(
            manifest
                .evidence
                .contains_key("EVIDENCE/llm-judge-ac-1.result.json")
        );

        drop(_path_guard);

        let record_temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1").body(
                    "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] AC 1 <!-- verify: llm-judge, SRS-01:start:end -->",
                ),
            )
            .build();
        install_failing_llm_judge(record_temp.path());
        let _path_guard = PathGuard::prepend(record_temp.path());
        init_git_repo(record_temp.path());

        let err = record::run(
            record_temp.path(),
            "S1".to_string(),
            Some(1),
            None,
            None,
            true,
            vec![],
        )
        .unwrap_err()
        .to_string();
        assert!(err.contains("LLM-Judge failed"));

        let record_evidence = record_temp.path().join("stories/S1/EVIDENCE");
        assert!(record_evidence.join("judge-bundle-ac-1.json").exists());
        assert!(record_evidence.join("llm-judge-ac-1.txt").exists());
        assert!(record_evidence.join("llm-judge-ac-1.stderr.txt").exists());
        assert!(record_evidence.join("llm-judge-ac-1.result.json").exists());
    }

    #[test]
    fn test_verify_story_executes_vhs() {
        let dir = tempdir().unwrap();
        let stories_dir = dir.path().join("stories").join("S1");
        fs::create_dir_all(&stories_dir).unwrap();
        let story_path = stories_dir.join("README.md");
        fs::write(
            &story_path,
            "## Acceptance Criteria\n\n- [ ] AC 1 <!-- verify: vhs test.tape, SRS-01:start -->",
        )
        .unwrap();

        // Create tape file
        fs::write(stories_dir.join("test.tape"), "Sleep 1s").unwrap();

        // Use mock
        unsafe {
            std::env::set_var("KEEL_MOCK_VHS", "1");
        }

        let report =
            verify_story(dir.path(), "S1", &fs::read_to_string(&story_path).unwrap()).unwrap();
        assert_eq!(report.results.len(), 1);
        assert!(report.results[0].passed);
        assert_eq!(report.results[0].actual, "vhs recording successful");

        // Verify gif was created
        let evidence_dir = stories_dir.join("EVIDENCE");
        assert!(evidence_dir.exists());
        let gif_path = evidence_dir.join("record-cli.gif");
        assert!(gif_path.exists());

        unsafe {
            std::env::remove_var("KEEL_MOCK_VHS");
        }
    }
}
