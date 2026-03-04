//! Record proof for an acceptance criterion

use anyhow::{Context, Result, bail};
use owo_colors::OwoColorize;
use std::fs;
use std::path::Path;
use std::time::Duration;

use crate::infrastructure::loader::load_board;
use crate::infrastructure::utils::get_manual_input;
use crate::infrastructure::verification::executor::execute;
use crate::infrastructure::verification::parser::{Comparison, parse_verify_annotations};

use super::guidance::{
    StoryLifecycleAction, error_with_recovery, guidance_for_action, print_human,
};

/// Run the record command
pub fn run(
    board_dir: &Path,
    id: String,
    ac_index: Option<usize>,
    cmd_override: Option<String>,
    msg: Option<String>,
    judge: bool,
    files: Vec<String>,
) -> Result<()> {
    let story_id = id.clone();
    run_impl(board_dir, id, ac_index, cmd_override, msg, judge, files)
        .map_err(|err| error_with_recovery(StoryLifecycleAction::Record, &story_id, err))
}

fn run_impl(
    board_dir: &Path,
    id: String,
    ac_index: Option<usize>,
    cmd_override: Option<String>,
    msg: Option<String>,
    judge: bool,
    files: Vec<String>,
) -> Result<()> {
    let board = load_board(board_dir)?;
    let story = board
        .stories
        .get(&id)
        .or_else(|| board.stories.values().find(|s| s.matches(&id)))
        .context(format!("Story not found: {}", id))?;

    let content = fs::read_to_string(&story.path)?;
    let annotations = parse_verify_annotations(&content);

    if annotations.is_empty() {
        bail!(
            "No acceptance criteria with verify annotations found in story {}",
            story.id()
        );
    }

    let ac_idx = match ac_index {
        Some(idx) if idx > 0 && idx <= annotations.len() => idx - 1,
        Some(idx) => bail!("AC index {} out of range (1-{})", idx, annotations.len()),
        None => {
            // Interactive selection (simple for now)
            println!(
                "Acceptance Criteria for {}:",
                story.id().bright_blue().bold()
            );
            for (i, ann) in annotations.iter().enumerate() {
                let status = if ann.proof.is_some() {
                    format!("{}", "✓".green())
                } else {
                    format!("{}", "○".red())
                };
                println!(
                    "  {}. [{}] {} {}",
                    i + 1,
                    status,
                    ann.criterion,
                    if let Some(proof) = &ann.proof {
                        format!("(Proof: {})", proof).dimmed().to_string()
                    } else {
                        "".to_string()
                    }
                );
            }

            if annotations.len() == 1 {
                0
            } else {
                println!("\nPlease specify --ac <number>");
                return Ok(());
            }
        }
    };

    let ann = &annotations[ac_idx];
    println!("Recording proof for: {}", ann.criterion.cyan());

    let proof_filename = format!("ac-{}.log", ac_idx + 1);
    let proof_content;

    if judge {
        println!("Triggering LLM-Judge verification...");
        let res = crate::infrastructure::verification::executor::execute_llm_judge(
            board_dir,
            story.id(),
            &ann.criterion,
        )?;
        if res.exit_code != 0 {
            bail!("LLM-Judge failed: {}", res.stderr);
        }
        println!("✅ LLM-Judge passed!");
        // The execute_llm_judge already writes the transcript file,
        // but the record command expects to write the proof file itself.
        // Actually, execute_llm_judge returns ExecuteResult.
        // Let's re-read the transcript or just use the result.
        // Looking at execute_llm_judge implementation, it writes to a file.
        // I'll change execute_llm_judge to NOT write the file if I want record to handle it,
        // or just have record read it back.
        // Alternatively, record can just use the transcript logic.

        let mut combined = format!(
            "---\nrecorded_at: {}\nmode: llm-judge\n---\n",
            chrono::Local::now().to_rfc3339()
        );
        combined.push_str(&res.stdout);
        proof_content = combined;
    } else if !files.is_empty() {
        let mut combined = format!(
            "---\nrecorded_at: {}\nmode: manual\nfiles: {:?}\n---\n",
            chrono::Local::now().to_rfc3339(),
            files
        );

        for file_path_str in &files {
            let path = Path::new(file_path_str);
            if !path.exists() {
                bail!("File not found: {}", file_path_str);
            }
            let content = fs::read_to_string(path)?;
            let name = path.file_name().unwrap().to_string_lossy();

            combined.push_str(&format!("\n--- File: {} ---\n", name));
            combined.push_str(&content);
            combined.push('\n');
        }
        proof_content = combined;
    } else if let Some(manual_msg) = msg {
        let combined = format!(
            "---\nrecorded_at: {}\nmode: manual\n---\n{}",
            chrono::Local::now().to_rfc3339(),
            manual_msg
        );
        proof_content = combined;
    } else if ann.comparison == Comparison::Manual {
        let manual_msg = get_manual_input(&format!(
            "# Manual Evidence for: {}\n\nPlease describe the proof below.\n",
            ann.criterion
        ))?;
        let combined = format!(
            "---\nrecorded_at: {}\nmode: manual\n---\n{}",
            chrono::Local::now().to_rfc3339(),
            manual_msg
        );
        proof_content = combined;
    } else {
        let cmd = cmd_override
            .as_ref()
            .or(ann.command.as_ref())
            .context("No command specified for this AC and no --cmd override provided")?;

        println!("Running: {}", cmd.bright_white());
        let result = execute(cmd, board_dir, Duration::from_secs(60))?;

        if result.exit_code != 0 {
            println!("{}", result.stdout);
            eprintln!("{}", result.stderr);
            bail!("Command failed with exit code {}", result.exit_code);
        }

        println!("✅ Command passed!");
        let mut combined = format!(
            "---\nrecorded_at: {}\ncommand: {}\n---\n",
            chrono::Local::now().to_rfc3339(),
            cmd
        );
        combined.push_str(&result.stdout);
        if !result.stderr.is_empty() {
            combined.push_str("\n--- stderr ---\n");
            combined.push_str(&result.stderr);
        }
        proof_content = combined;
    }

    // Save the proof
    let story_dir = story.path.parent().unwrap();
    let evidence_dir = story_dir.join("EVIDENCE");
    if !evidence_dir.exists() {
        fs::create_dir_all(&evidence_dir)?;
    }

    let proof_path = evidence_dir.join(&proof_filename);
    fs::write(&proof_path, proof_content)?;
    println!(
        "Proof recorded to {}",
        proof_path.display().to_string().dimmed()
    );

    // Update the story file
    update_story_with_proof(&story.path, ac_idx, &proof_filename)?;
    println!(
        "Story {} updated with link to proof.",
        story.id().bright_blue()
    );
    let guidance = guidance_for_action(StoryLifecycleAction::Record, story.stage, story.id());
    print_human(guidance.as_ref());

    Ok(())
}

fn update_story_with_proof(story_path: &Path, ac_idx: usize, proof_filename: &str) -> Result<()> {
    let content = fs::read_to_string(story_path)?;
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let mut current_ac_idx = 0;
    let mut updated = false;

    let proof_re = regex::Regex::new(r"proof:\s*[^, \n]+").unwrap();

    for line in lines.iter_mut() {
        if line.contains("<!--") && line.contains("verify:") {
            if current_ac_idx == ac_idx {
                // Find where the verify comment starts and ends
                if let Some(start_comment) = line.find("<!--")
                    && let Some(end_comment) = line.find("-->")
                {
                    let comment_content = &line[start_comment + 4..end_comment];

                    let new_comment_content =
                        if let Some(proof_match) = proof_re.find(comment_content) {
                            // Replace existing proof: filename
                            let mut new_content = comment_content.to_string();
                            new_content.replace_range(
                                proof_match.range(),
                                &format!("proof: {}", proof_filename),
                            );
                            new_content
                        } else {
                            // Append with comma
                            format!("{}, proof: {}", comment_content.trim_end(), proof_filename)
                        };

                    *line = format!(
                        "{}<!--{}-->{}",
                        &line[..start_comment],
                        new_comment_content,
                        &line[end_comment + 3..]
                    );
                    updated = true;
                    break;
                }
            }
            current_ac_idx += 1;
        }
    }

    if updated {
        fs::write(story_path, lines.join("\n") + "\n")?;
    } else {
        bail!("Failed to find AC index {} in file", ac_idx + 1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestStory};
    use tempfile::tempdir;

    #[test]
    fn test_update_story_with_proof_no_req() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.md");
        fs::write(&path, "- [ ] AC 1 <!-- verify: manual -->\n").unwrap();

        update_story_with_proof(&path, 0, "proof.log").unwrap();
        let result = fs::read_to_string(&path).unwrap();
        assert!(result.contains("manual, proof: proof.log"));
    }

    #[test]
    fn test_update_story_with_proof_with_req() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.md");
        fs::write(
            &path,
            "- [ ] AC 1 <!-- verify: cargo test, SRS-01:start -->\n",
        )
        .unwrap();

        update_story_with_proof(&path, 0, "proof.log").unwrap();
        let result = fs::read_to_string(&path).unwrap();
        assert!(result.contains("cargo test, SRS-01:start, proof: proof.log"));
    }

    #[test]
    fn test_update_story_with_proof_replace() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.md");
        fs::write(
            &path,
            "- [ ] AC 1 <!-- verify: cargo test, proof: old.log, SRS-01:start -->\n",
        )
        .unwrap();

        update_story_with_proof(&path, 0, "new.log").unwrap();
        let result = fs::read_to_string(&path).unwrap();
        assert!(result.contains("cargo test, proof: new.log, SRS-01:start"));
        assert!(!result.contains("old.log"));
    }

    #[test]
    fn test_record_manual_provenance() {
        let temp =
            TestBoardBuilder::new()
                .story(TestStory::new("S1").body(
                    "## Acceptance Criteria\n\n- [ ] AC 1 <!-- verify: manual, SRS-01:start -->",
                ))
                .build();

        run(
            temp.path(),
            "S1".to_string(),
            Some(1),
            None,
            Some("Manual note".to_string()),
            false,
            vec![],
        )
        .unwrap();

        let proof_path = temp.path().join("stories/S1/EVIDENCE/ac-1.log");
        assert!(proof_path.exists());

        let content = fs::read_to_string(proof_path).unwrap();
        assert!(content.contains("recorded_at:"));
        assert!(content.contains("mode: manual"));
        assert!(content.contains("Manual note"));

        let story_content = fs::read_to_string(temp.path().join("stories/S1/README.md")).unwrap();
        assert!(story_content.contains("proof: ac-1.log"));
    }

    #[test]
    fn test_record_command_provenance() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .body("## Acceptance Criteria\n\n- [ ] AC 1 <!-- verify: echo 'it works', SRS-01:start -->")
            )
            .build();

        run(
            temp.path(),
            "S1".to_string(),
            Some(1),
            None,
            None,
            false,
            vec![],
        )
        .unwrap();

        let proof_path = temp.path().join("stories/S1/EVIDENCE/ac-1.log");
        assert!(proof_path.exists());

        let content = fs::read_to_string(proof_path).unwrap();
        assert!(content.contains("recorded_at:"));
        assert!(content.contains("command: echo 'it works'"));
        assert!(content.contains("it works"));
    }

    #[test]
    fn test_record_manual_editor_integration() {
        let temp =
            TestBoardBuilder::new()
                .story(TestStory::new("S1").body(
                    "## Acceptance Criteria\n\n- [ ] AC 1 <!-- verify: manual, SRS-01:start -->",
                ))
                .build();

        // Mock EDITOR to just write a message and exit
        unsafe {
            std::env::set_var("EDITOR", "echo 'Editor content' >");
        }

        // This will fail because our mock editor doesn't actually work with Command::new("echo '...' >").arg(file)
        // because it expects a single executable.
        // Let's create a real script.
        let script_path = temp.path().join("mock_editor.sh");
        fs::write(
            &script_path,
            "#!/bin/sh\nprintf 'Editor content\\n' > \"$1\"",
        )
        .unwrap();
        // make it executable
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755)).unwrap();

        unsafe {
            std::env::set_var("EDITOR", script_path.to_str().unwrap());
        }

        run(
            temp.path(),
            "S1".to_string(),
            Some(1),
            None,
            None, // msg is None, should trigger editor
            false,
            vec![],
        )
        .unwrap();

        let proof_path = temp.path().join("stories/S1/EVIDENCE/ac-1.log");
        assert!(proof_path.exists());

        let content = fs::read_to_string(proof_path).unwrap();
        assert!(content.contains("recorded_at:"));
        assert!(content.contains("mode: manual"));
        assert!(content.contains("Editor content"));

        unsafe {
            std::env::remove_var("EDITOR");
        }
    }

    #[test]
    fn test_record_multiple_files() {
        let temp =
            TestBoardBuilder::new()
                .story(TestStory::new("S1").body(
                    "## Acceptance Criteria\n\n- [ ] AC 1 <!-- verify: manual, SRS-01:start -->",
                ))
                .build();

        let f1 = temp.path().join("file1.txt");
        let f2 = temp.path().join("file2.txt");
        fs::write(&f1, "content 1").unwrap();
        fs::write(&f2, "content 2").unwrap();

        run(
            temp.path(),
            "S1".to_string(),
            Some(1),
            None,
            None,
            false,
            vec![
                f1.to_str().unwrap().to_string(),
                f2.to_str().unwrap().to_string(),
            ],
        )
        .unwrap();

        let proof_path = temp.path().join("stories/S1/EVIDENCE/ac-1.log");
        assert!(proof_path.exists());

        let content = fs::read_to_string(proof_path).unwrap();
        assert!(content.contains("content 1"));
        assert!(content.contains("content 2"));
        assert!(content.contains("File: file1.txt"));
        assert!(content.contains("File: file2.txt"));
    }
}
