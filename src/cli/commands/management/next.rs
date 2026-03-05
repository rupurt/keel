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
use crate::cli::commands::management::guidance::{
    CanonicalGuidance, CommandGuidance, render_command_guidance,
};
use crate::domain::model::Story;
use crate::infrastructure::loader::load_board;

#[derive(Serialize)]
struct JsonResult {
    decision: String,
    details: JsonDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance: Option<CanonicalGuidance>,
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
        let result = decision_to_json(&decision);
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", format_decision(&decision));
        print_human_guidance(guidance_for_decision(&decision).as_ref());

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

fn decision_to_json(decision: &NextDecision) -> JsonResult {
    let details = match decision {
        NextDecision::Work(d) => JsonDetails::Work {
            id: d.story.id().to_string(),
            title: d.story.title().to_string(),
            is_continuation: d.is_continuation,
        },
        NextDecision::Decision(d) => JsonDetails::Decision {
            adrs: d.adrs.iter().map(|a| a.id().to_string()).collect(),
            blocked_stories: d
                .blocked_stories
                .iter()
                .map(|s| s.id().to_string())
                .collect(),
        },
        NextDecision::Accept(d) => JsonDetails::Accept {
            stories: d.stories.iter().map(|s| s.id().to_string()).collect(),
        },
        NextDecision::Research(d) => JsonDetails::Research {
            bearings: d.bearings.iter().map(|b| b.id().to_string()).collect(),
        },
        NextDecision::Blocked(d) => JsonDetails::Blocked {
            story_id: d.story.id().to_string(),
            total_blocked: d.count,
        },
        NextDecision::NeedsStories(d) => JsonDetails::NeedsStories {
            voyages: d.voyages.iter().map(|v| v.id().to_string()).collect(),
        },
        NextDecision::NeedsPlanning(d) => JsonDetails::NeedsPlanning {
            voyages: d.voyages.iter().map(|v| v.id().to_string()).collect(),
        },
        NextDecision::Empty(d) => JsonDetails::Empty {
            suggestions: d.suggestions.clone(),
        },
    };

    JsonResult {
        decision: decision_kind(decision).to_string(),
        details,
        guidance: guidance_for_decision(decision),
    }
}

fn decision_kind(decision: &NextDecision) -> &'static str {
    match decision {
        NextDecision::Work(_) => "work",
        NextDecision::Decision(_) => "decision",
        NextDecision::Accept(_) => "accept",
        NextDecision::Research(_) => "research",
        NextDecision::Blocked(_) => "blocked",
        NextDecision::NeedsStories(_) => "needs_stories",
        NextDecision::NeedsPlanning(_) => "needs_planning",
        NextDecision::Empty(_) => "empty",
    }
}

fn guidance_for_decision(decision: &NextDecision) -> Option<CanonicalGuidance> {
    let command_guidance = match decision {
        NextDecision::Work(d) => Some(if d.is_continuation {
            CommandGuidance::next(format!("keel story submit {}", d.story.id()))
        } else {
            CommandGuidance::next(format!("keel story start {}", d.story.id()))
        }),
        NextDecision::Decision(d) => d
            .adrs
            .first()
            .map(|adr| CommandGuidance::next(format!("keel adr accept {}", adr.id()))),
        NextDecision::Accept(d) => d
            .stories
            .first()
            .map(|story| CommandGuidance::next(format!("keel story accept {}", story.id()))),
        NextDecision::Research(d) => d
            .bearings
            .first()
            .map(|bearing| CommandGuidance::next(format!("keel play {}", bearing.id()))),
        NextDecision::Blocked(d) => Some(CommandGuidance::recovery(format!(
            "keel story accept {}",
            d.story.id()
        ))),
        NextDecision::NeedsStories(d) => d.voyages.first().map(|voyage| {
            CommandGuidance::next(format!(
                "keel story new \"<title>\" --epic {} --voyage {}",
                voyage.epic_id,
                voyage.id()
            ))
        }),
        NextDecision::NeedsPlanning(d) => d
            .voyages
            .first()
            .map(|voyage| CommandGuidance::next(format!("keel voyage plan {}", voyage.id()))),
        NextDecision::Empty(_) => None,
    };

    render_command_guidance(command_guidance)
}

fn guidance_for_parallel_ready(ready: &[&Story]) -> Option<CanonicalGuidance> {
    render_command_guidance(
        ready
            .first()
            .map(|story| CommandGuidance::next(format!("keel story start {}", story.id()))),
    )
}

fn render_human_guidance(guidance: Option<&CanonicalGuidance>) -> String {
    if let Some(step) = guidance.and_then(|g| g.next_step.as_ref()) {
        return format!("\nNext step:\n  {}\n", step.command.bold());
    }

    if let Some(step) = guidance.and_then(|g| g.recovery_step.as_ref()) {
        return format!("\nRecovery step:\n  {}\n", step.command.bold());
    }

    String::new()
}

fn print_human_guidance(guidance: Option<&CanonicalGuidance>) {
    let rendered = render_human_guidance(guidance);
    if !rendered.is_empty() {
        print!("{rendered}");
    }
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

    // Compute deterministic semantic signals and conservative pairwise scores.
    let pairwise_feature_vectors =
        crate::cli::commands::management::next_support::parallel_features::extract_parallel_feature_vectors(
            board, &ready,
        );
    let _pairwise_scores =
        crate::cli::commands::management::next_support::parallel_scoring::score_parallel_pairwise_conflicts(
            &pairwise_feature_vectors,
        );

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
            guidance: guidance_for_parallel_ready(&ready),
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

        print_human_guidance(guidance_for_parallel_ready(&ready).as_ref());
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
    use crate::domain::model::Story;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{
        AdrFactory, BearingFactory, StoryFactory, TestBoardBuilder, TestStory, VoyageFactory,
    };

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

    fn make_story(id: &str) -> Story {
        StoryFactory::new(id).title("Story").build()
    }

    fn assert_human_json_guidance_parity(decision: &NextDecision) {
        let guidance = guidance_for_decision(decision);
        let rendered = render_human_guidance(guidance.as_ref());
        let json = serde_json::to_value(decision_to_json(decision)).unwrap();

        match guidance.as_ref() {
            Some(g) if g.next_step.is_some() => {
                let command = &g.next_step.as_ref().unwrap().command;
                assert_eq!(json["guidance"]["next_step"]["command"], command.as_str());
                assert!(json["guidance"]["recovery_step"].is_null());
                assert!(rendered.contains("Next step:"));
                assert!(rendered.contains(command));
            }
            Some(g) if g.recovery_step.is_some() => {
                let command = &g.recovery_step.as_ref().unwrap().command;
                assert_eq!(
                    json["guidance"]["recovery_step"]["command"],
                    command.as_str()
                );
                assert!(json["guidance"]["next_step"].is_null());
                assert!(rendered.contains("Recovery step:"));
                assert!(rendered.contains(command));
            }
            None => {
                assert!(json.get("guidance").is_none());
                assert!(rendered.is_empty());
            }
            _ => panic!("Guidance must contain exactly one canonical command"),
        }
    }

    #[test]
    fn decision_to_json_work_includes_next_step_guidance() {
        let decision = NextDecision::Work(StoryDecision {
            story: make_story("S1"),
            is_continuation: false,
            warning: None,
        });

        let payload = decision_to_json(&decision);
        let json = serde_json::to_value(payload).unwrap();

        assert_eq!(json["decision"], "work");
        assert_eq!(
            json["guidance"]["next_step"]["command"],
            "keel story start S1"
        );
        assert!(json["guidance"]["recovery_step"].is_null());
    }

    #[test]
    fn decision_to_json_continuation_work_maps_to_submit_command() {
        let decision = NextDecision::Work(StoryDecision {
            story: make_story("S1"),
            is_continuation: true,
            warning: None,
        });

        let payload = decision_to_json(&decision);
        let json = serde_json::to_value(payload).unwrap();

        assert_eq!(json["decision"], "work");
        assert_eq!(
            json["guidance"]["next_step"]["command"],
            "keel story submit S1"
        );
        assert!(json["guidance"]["recovery_step"].is_null());
    }

    #[test]
    fn decision_to_json_accept_maps_to_accept_command() {
        let decision = NextDecision::Accept(AcceptDecision {
            stories: vec![make_story("S2")],
        });

        let payload = decision_to_json(&decision);
        let json = serde_json::to_value(payload).unwrap();

        assert_eq!(json["decision"], "accept");
        assert_eq!(
            json["guidance"]["next_step"]["command"],
            "keel story accept S2"
        );
        assert!(json["guidance"]["recovery_step"].is_null());
    }

    #[test]
    fn decision_to_json_blocked_includes_recovery_guidance() {
        let decision = NextDecision::Blocked(BlockedDecision {
            story: make_story("S9"),
            count: 9,
        });

        let payload = decision_to_json(&decision);
        let json = serde_json::to_value(payload).unwrap();

        assert_eq!(json["decision"], "blocked");
        assert_eq!(
            json["guidance"]["recovery_step"]["command"],
            "keel story accept S9"
        );
        assert!(json["guidance"]["next_step"].is_null());
    }

    #[test]
    fn decision_to_json_empty_omits_guidance() {
        let decision = NextDecision::Empty(EmptyDecision {
            suggestions: vec!["Refuel".to_string()],
        });

        let payload = decision_to_json(&decision);
        let json = serde_json::to_value(payload).unwrap();

        assert!(json.get("guidance").is_none());
    }

    #[test]
    fn actionable_decisions_keep_human_and_json_guidance_in_sync() {
        let work = NextDecision::Work(StoryDecision {
            story: make_story("S10"),
            is_continuation: false,
            warning: None,
        });
        assert_human_json_guidance_parity(&work);

        let continuation = NextDecision::Work(StoryDecision {
            story: make_story("S11"),
            is_continuation: true,
            warning: None,
        });
        assert_human_json_guidance_parity(&continuation);

        let decision = NextDecision::Decision(AdrDecision {
            adrs: vec![AdrFactory::new("ADR10").title("Decision 10").build()],
            blocked_stories: vec![make_story("S12")],
        });
        assert_human_json_guidance_parity(&decision);

        let accept = NextDecision::Accept(AcceptDecision {
            stories: vec![make_story("S13")],
        });
        assert_human_json_guidance_parity(&accept);

        let research = NextDecision::Research(ResearchDecision {
            bearings: vec![BearingFactory::new("B10").title("Research 10").build()],
        });
        assert_human_json_guidance_parity(&research);

        let needs_stories = NextDecision::NeedsStories(DecomposeDecision {
            voyages: vec![VoyageFactory::new("V10", "E10").title("Voyage 10").build()],
        });
        assert_human_json_guidance_parity(&needs_stories);

        let needs_planning = NextDecision::NeedsPlanning(DecomposeDecision {
            voyages: vec![VoyageFactory::new("V11", "E11").title("Voyage 11").build()],
        });
        assert_human_json_guidance_parity(&needs_planning);
    }

    #[test]
    fn blocked_and_empty_decisions_keep_human_and_json_guidance_in_sync() {
        let blocked = NextDecision::Blocked(BlockedDecision {
            story: make_story("SBLOCK"),
            count: 4,
        });
        assert_human_json_guidance_parity(&blocked);

        let empty = NextDecision::Empty(EmptyDecision {
            suggestions: vec!["Refuel".to_string()],
        });
        assert_human_json_guidance_parity(&empty);
    }

    #[test]
    fn parallel_ready_guidance_matches_json_and_human_rendering() {
        let ready_story = make_story("SREADY");
        let guidance = guidance_for_parallel_ready(&[&ready_story]).unwrap();
        let json = serde_json::to_value(&guidance).unwrap();
        let rendered = render_human_guidance(Some(&guidance));

        assert_eq!(json["next_step"]["command"], "keel story start SREADY");
        assert!(json["recovery_step"].is_null());
        assert!(rendered.contains("Next step:"));
        assert!(rendered.contains("keel story start SREADY"));
    }
}
