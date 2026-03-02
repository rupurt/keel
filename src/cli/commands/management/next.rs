#![allow(dead_code)]
//! Next command - selective action surfacing

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use owo_colors::OwoColorize;
use serde::Serialize;

pub use super::next_support::{
    AcceptDecision, AdrDecision, BlockedDecision, DecomposeDecision, EmptyDecision, NextDecision,
    ResearchDecision, StoryDecision, calculate_next, format_decision,
};
use crate::domain::model::Story;
use crate::infrastructure::loader::load_board;

#[derive(Serialize)]
struct JsonResult {
    decision: String,
    details: JsonDetails,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum JsonDetails {
    Work {
        id: String,
        title: String,
        is_continuation: bool,
    },
    Decision {
        adrs: Vec<String>,
        blocked_stories: Vec<String>,
    },
    Accept {
        stories: Vec<String>,
    },
    Research {
        bearings: Vec<String>,
    },
    Blocked {
        story_id: String,
        total_blocked: usize,
    },
    NeedsStories {
        voyages: Vec<String>,
    },
    NeedsPlanning {
        voyages: Vec<String>,
    },
    Empty {
        suggestions: Vec<String>,
    },
    ParallelWork {
        next: Option<JsonStory>,
        ready: Vec<JsonStory>,
        sequential_chains: HashMap<String, Vec<JsonStory>>,
    },
}

#[derive(Serialize, Clone)]
struct JsonStory {
    id: String,
    title: String,
    scope: Option<String>,
    index: Option<u32>,
}

#[derive(Serialize)]
struct JsonBlockedByAdr {
    adr_id: String,
    stories: Vec<String>,
}

#[derive(Serialize)]
struct JsonBearing {
    id: String,
    title: String,
}

/// Parse optional actor role taxonomy string for `next` filtering.
pub fn parse_actor_role(
    role: Option<&str>,
) -> Option<crate::domain::model::taxonomy::RoleTaxonomy> {
    role.and_then(|s| crate::domain::model::taxonomy::parse(s).ok())
}

/// Run the next command
pub fn run(
    board_dir: &Path,
    agent_mode: bool,
    json: bool,
    parallel: bool,
    actor_role: Option<&crate::domain::model::taxonomy::RoleTaxonomy>,
) -> Result<()> {
    let board = load_board(board_dir)?;

    if parallel {
        return run_parallel(&board, board_dir, json, actor_role);
    }

    let decision = calculate_next(&board, board_dir, agent_mode, actor_role)?;

    if json {
        let result = match &decision {
            NextDecision::Work(d) => JsonResult {
                decision: "work".to_string(),
                details: JsonDetails::Work {
                    id: d.story.id().to_string(),
                    title: d.story.title().to_string(),
                    is_continuation: d.is_continuation,
                },
            },
            NextDecision::Decision(d) => JsonResult {
                decision: "decision".to_string(),
                details: JsonDetails::Decision {
                    adrs: d.adrs.iter().map(|a| a.id().to_string()).collect(),
                    blocked_stories: d
                        .blocked_stories
                        .iter()
                        .map(|s| s.id().to_string())
                        .collect(),
                },
            },
            NextDecision::Accept(d) => JsonResult {
                decision: "accept".to_string(),
                details: JsonDetails::Accept {
                    stories: d.stories.iter().map(|s| s.id().to_string()).collect(),
                },
            },
            NextDecision::Research(d) => JsonResult {
                decision: "research".to_string(),
                details: JsonDetails::Research {
                    bearings: d.bearings.iter().map(|b| b.id().to_string()).collect(),
                },
            },
            NextDecision::Blocked(d) => JsonResult {
                decision: "blocked".to_string(),
                details: JsonDetails::Blocked {
                    story_id: d.story.id().to_string(),
                    total_blocked: d.count,
                },
            },
            NextDecision::NeedsStories(d) => JsonResult {
                decision: "needs_stories".to_string(),
                details: JsonDetails::NeedsStories {
                    voyages: d.voyages.iter().map(|v| v.id().to_string()).collect(),
                },
            },
            NextDecision::NeedsPlanning(d) => JsonResult {
                decision: "needs_planning".to_string(),
                details: JsonDetails::NeedsPlanning {
                    voyages: d.voyages.iter().map(|v| v.id().to_string()).collect(),
                },
            },
            NextDecision::Empty(d) => JsonResult {
                decision: "empty".to_string(),
                details: JsonDetails::Empty {
                    suggestions: d.suggestions.clone(),
                },
            },
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", format_decision(&decision));

        // Surface relevant knowledge for the work decision
        if let NextDecision::Work(d) = &decision {
            let epic = d.story.epic();
            let scope = d.story.frontmatter.scope.as_deref();

            if let Ok(all_knowledge) = crate::read_model::knowledge::scan_all_knowledge(board_dir) {
                let relevant: Vec<_> = all_knowledge
                    .into_iter()
                    .filter(|k| {
                        // is_relevant logic duplicated here or we can export it from story::start
                        if let Some(s) = scope
                            && k.scope.as_deref() == Some(s)
                        {
                            return true;
                        }
                        if let Some(e) = epic
                            && let Some(k_scope) = &k.scope
                            && k_scope.starts_with(e)
                        {
                            return true;
                        }
                        false
                    })
                    .filter(|k| k.applied.is_empty())
                    .collect();

                if !relevant.is_empty() {
                    println!("\n{}", "Relevant knowledge for this task:".yellow().bold());
                    for k in relevant {
                        println!("  - [{}] {}", k.id.cyan(), k.title);
                        println!("    Insight: {}", k.insight);
                    }
                }
            }
        }
    }

    Ok(())
}

fn run_parallel(
    board: &crate::domain::model::Board,
    board_dir: &Path,
    json: bool,
    actor_role: Option<&crate::domain::model::taxonomy::RoleTaxonomy>,
) -> Result<()> {
    use crate::domain::state_machine::invariants;
    use crate::read_model::traceability::derive_implementation_dependencies;

    // Get all workable stories, optionally filtered by role
    let mut candidates: Vec<&Story> = board
        .stories
        .values()
        .filter(|s| invariants::story_workable(s, board, board_dir))
        .filter(|s| {
            actor_role
                .map(|actor| crate::domain::model::taxonomy::actor_matches_story(actor, s))
                .unwrap_or(true)
        })
        .collect();

    candidates.sort_by_key(|s| s.id());

    let deps = derive_implementation_dependencies(board);

    // Filter into parallel-safe (ready) and sequential chains
    let mut ready = Vec::new();
    let mut sequential: HashMap<String, Vec<&Story>> = HashMap::new();

    for story in candidates {
        let is_unblocked = deps.get(story.id()).is_none_or(|dep_ids| {
            dep_ids.iter().all(|id| {
                board
                    .stories
                    .get(id)
                    .map(|dep| dep.stage == crate::domain::model::StoryState::Done)
                    .unwrap_or(false)
            })
        });

        if is_unblocked {
            ready.push(story);
        } else if let Some(scope) = story.scope() {
            sequential.entry(scope.to_string()).or_default().push(story);
        }
    }

    // Sort sequential chains by index
    for chain in sequential.values_mut() {
        chain.sort_by_key(|s| s.index());
    }

    if json {
        let mut ready_json: Vec<JsonStory> = ready
            .iter()
            .map(|s| JsonStory {
                id: s.id().to_string(),
                title: s.title().to_string(),
                scope: s.scope().map(|sc| sc.to_string()),
                index: s.index(),
            })
            .collect();

        let mut sequential_json = HashMap::new();
        for (scope, stories) in sequential {
            let chain: Vec<JsonStory> = stories
                .iter()
                .map(|s| JsonStory {
                    id: s.id().to_string(),
                    title: s.title().to_string(),
                    scope: s.scope().map(|sc| sc.to_string()),
                    index: s.index(),
                })
                .collect();
            sequential_json.insert(scope, chain);
        }

        let next = ready_json.first().cloned();
        if !ready_json.is_empty() {
            ready_json.remove(0);
        }

        let result = JsonResult {
            decision: "parallel_work".to_string(),
            details: JsonDetails::ParallelWork {
                next,
                ready: ready_json,
                sequential_chains: sequential_json,
            },
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("Ready for Work (Parallel Safe):");
        if ready.is_empty() {
            println!("  (none)");
        } else {
            for story in &ready {
                println!("  - {}", parallel_story_with_scope(story));
            }

            // Surface relevant knowledge for the first ready story
            if let Some(story) = ready.first() {
                let epic = story.epic();
                let scope = story.frontmatter.scope.as_deref();

                if let Ok(all_knowledge) =
                    crate::read_model::knowledge::scan_all_knowledge(board_dir)
                {
                    let relevant: Vec<_> = all_knowledge
                        .into_iter()
                        .filter(|k| {
                            if let Some(s) = scope
                                && k.scope.as_deref() == Some(s)
                            {
                                return true;
                            }
                            if let Some(e) = epic
                                && let Some(k_scope) = &k.scope
                                && k_scope.starts_with(e)
                            {
                                return true;
                            }
                            false
                        })
                        .filter(|k| k.applied.is_empty())
                        .collect();

                    if !relevant.is_empty() {
                        println!(
                            "\n{}",
                            format!(
                                "Relevant knowledge for [{}]:",
                                crate::cli::style::styled_story_id(story.id())
                            )
                            .yellow()
                            .bold()
                        );
                        for k in relevant {
                            println!("  - [{}] {}", k.id.cyan(), k.title);
                            println!("    Insight: {}", k.insight);
                        }
                    }
                }
            }
        }

        if !sequential.is_empty() {
            println!("\nSequential Chains (by Scope):");
            for (scope, stories) in sequential {
                println!("  {}:", crate::cli::style::styled_scope(Some(&scope)));
                for story in stories {
                    println!("    - {}", parallel_story(story));
                }
            }
        }
    }

    Ok(())
}

fn parallel_story(story: &Story) -> String {
    format!(
        "{} {}",
        crate::cli::style::styled_story_id(story.id()),
        story.title()
    )
}

fn parallel_story_with_scope(story: &Story) -> String {
    format!(
        "{} [{}]",
        parallel_story(story),
        crate::cli::style::styled_scope(story.scope())
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestStory};

    #[test]
    fn exit_code_work_is_0() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").stage(StoryState::Backlog))
            .build();
        let result = run(temp.path(), true, false, false, None);
        assert!(result.is_ok());
    }

    #[test]
    fn parallel_story_with_scope_uses_shared_id_colors() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .title("Story 1")
                    .scope("EPIC-1/VOY-1")
                    .stage(StoryState::Backlog),
            )
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let story = board.stories.get("S1").unwrap();

        let line = parallel_story_with_scope(story);
        assert!(line.contains(&crate::cli::style::styled_story_id("S1")));
        assert!(line.contains(&crate::cli::style::styled_scope(Some("EPIC-1/VOY-1"))));
    }

    #[test]
    fn parallel_story_uses_shared_story_id_color() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S2")
                    .title("Story 2")
                    .stage(StoryState::Backlog),
            )
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let story = board.stories.get("S2").unwrap();

        let line = parallel_story(story);
        assert!(line.contains(&crate::cli::style::styled_story_id("S2")));
    }
}
