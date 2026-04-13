use crate::read_model::diagnostics::types::*;
use crate::read_model::mission_stack::{self, MissionStackForeignExecutionState};
use std::path::{Path, PathBuf};

/// Report active Mission Stack protocol violations that should block execution.
pub fn check_mission_stack_protocol(board_dir: &Path) -> Vec<Problem> {
    let scan = match mission_stack::scan(board_dir) {
        Ok(scan) => scan,
        Err(error) => {
            return vec![
                Problem::error(
                    board_dir.to_path_buf(),
                    format!("Mission Stack scan failed: {error}"),
                )
                .with_check_id(CheckId::MissionStackProtocol),
            ];
        }
    };

    let mut problems: Vec<_> = scan
        .load_problems
        .into_iter()
        .map(|problem| {
            Problem::error(
                problem.path,
                format!("Mission Stack load error: {}", problem.message),
            )
            .with_check_id(CheckId::MissionStackProtocol)
        })
        .collect();

    for stack in scan.stacks {
        if stack.lifecycle != crate::read_model::mission_stack::MissionStackLifecycle::Active {
            continue;
        }

        if !stack.branch_matches {
            let current = stack.current_branch.as_deref().unwrap_or("<detached>");
            problems.push(
                Problem::error(
                    stack.manifest_path.clone(),
                    format!(
                        "Mission Stack `{}` expects branch `{}` but current checkout is `{}`",
                        stack.id, stack.branch, current
                    ),
                )
                .with_check_id(CheckId::MissionStackProtocol),
            );
        }

        if let Some(checkpoint) = &stack.checkpoint
            && checkpoint
                .required_members
                .iter()
                .any(|repo| repo == &stack.local_repo)
            && !stack.local_member.checkpoint_acknowledged
        {
            let waiting_on = stack.waiting_on_checkpoint_members();
            let suffix = if waiting_on.is_empty() {
                String::new()
            } else {
                format!("; waiting on {}", waiting_on.join(", "))
            };
            problems.push(
                Problem::error(
                    stack.manifest_path.clone(),
                    format!(
                        "Mission Stack `{}` checkpoint `{}` is missing local acknowledgment for `{}`{}",
                        stack.id, checkpoint.name, stack.local_repo, suffix
                    ),
                )
                .with_check_id(CheckId::MissionStackProtocol),
            );
        }

        if stack.checkout.foreign_execution_required
            && stack.checkout.foreign_execution_state != MissionStackForeignExecutionState::Ready
        {
            problems.push(
                Problem::error(
                    foreign_execution_problem_path(&stack),
                    format!(
                        "Mission Stack `{}` foreign execution requires a managed worktree; current state is `{}`",
                        stack.id,
                        foreign_execution_state_label(stack.checkout.foreign_execution_state)
                    ),
                )
                .with_check_id(CheckId::MissionStackProtocol),
            );
        }
    }

    problems
}

/// Warn when closed Mission Stacks still retain managed worktrees.
pub fn check_closed_stack_worktree_leftovers(board_dir: &Path) -> Vec<Problem> {
    let scan = match mission_stack::scan(board_dir) {
        Ok(scan) => scan,
        Err(error) => {
            return vec![
                Problem::warning(
                    board_dir.to_path_buf(),
                    format!("Mission Stack closeout scan failed: {error}"),
                )
                .with_check_id(CheckId::MissionStackCloseout),
            ];
        }
    };

    scan.closed_stacks()
        .filter(|stack| !stack.checkout.leftover_managed_paths.is_empty())
        .map(|stack| {
            Problem::warning(
                stack.manifest_path.clone(),
                format!(
                    "Mission Stack `{}` is closed but managed worktrees remain: {}",
                    stack.id,
                    stack
                        .checkout
                        .leftover_managed_paths
                        .iter()
                        .map(|path| path.display().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )
            .with_check_id(CheckId::MissionStackCloseout)
        })
        .collect()
}

fn foreign_execution_problem_path(
    stack: &crate::read_model::mission_stack::MissionStackProjection,
) -> PathBuf {
    stack
        .checkout
        .managed_path
        .clone()
        .or_else(|| stack.checkout.current_checkout.clone())
        .unwrap_or_else(|| stack.manifest_path.clone())
}

fn foreign_execution_state_label(state: MissionStackForeignExecutionState) -> &'static str {
    match state {
        MissionStackForeignExecutionState::NotRequired => "not_required",
        MissionStackForeignExecutionState::Ready => "ready",
        MissionStackForeignExecutionState::MissingManagedPath => "missing_managed_path",
        MissionStackForeignExecutionState::MissingManagedCheckout => "missing_managed_checkout",
        MissionStackForeignExecutionState::WrongCheckout => "wrong_checkout",
        MissionStackForeignExecutionState::PrimaryCheckoutDisallowed => {
            "primary_checkout_disallowed"
        }
    }
}
