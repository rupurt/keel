#![allow(dead_code)]
//! Next command - selective action surfacing

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Result, bail};
use owo_colors::OwoColorize;
use serde::Serialize;

pub use super::next_support::{
    AcceptDecision, AdrDecision, BlockedDecision, DecomposeDecision, EmptyDecision, ItemFilter,
    NextDecision, ResearchDecision, StoryDecision, calculate_next, format_decision,
};
use crate::cli::commands::management::guidance::{
    CanonicalGuidance, CommandGuidance, RoleContextGuidance, render_command_guidance,
};
use crate::cli::commands::management::story::guidance::{
    accept_command_for_role as story_accept_command_for_role,
    creation_command as story_creation_command,
};
use crate::domain::model::Story;
use crate::infrastructure::loader::load_board;
use crate::read_model::workflow_topology::{self, ResolvedWorkflowTopology};

#[derive(Serialize)]
struct JsonResult {
    decision: String,
    details: JsonDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance: Option<CanonicalGuidance>,
}

#[derive(Serialize, Clone)]
struct JsonPairwiseBlocker {
    story_id: String,
    blocked_by: String,
    reasons: Vec<String>,
    confidence: f64,
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
    Mission {
        id: String,
        title: String,
        unmet_goals: Vec<crate::infrastructure::validation::charter::ParsedMissionGoal>,
        suggestion: String,
    },
    Missions {
        missions: Vec<JsonMission>,
    },
    Empty {
        suggestions: Vec<String>,
    },
    ParallelWork {
        next: Option<JsonStory>,
        ready: Vec<JsonStory>,
        sequential_chains: BTreeMap<String, Vec<JsonStory>>,
        blocked_pairs: Vec<JsonPairwiseBlocker>,
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

#[derive(Serialize)]
struct JsonMission {
    id: String,
    title: String,
}

struct ParallelProjection<'a> {
    ready: Vec<&'a Story>,
    sequential_chains: BTreeMap<String, Vec<&'a Story>>,
    blocked_pairs:
        Vec<crate::cli::commands::management::next_support::parallel_threshold::PairwiseBlocker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolvedActorContext {
    lane_name: String,
    queue_lane: crate::read_model::queue_policy::ActorQueueLane,
    supports_parallel: bool,
    role_context: Option<RoleContextGuidance>,
}

/// Parse optional actor role taxonomy string for `next` filtering.
pub fn parse_actor_role(
    role: Option<&str>,
) -> Result<Option<crate::domain::model::taxonomy::RoleTaxonomy>> {
    Ok(role
        .map(crate::domain::model::taxonomy::parse)
        .transpose()?)
}

pub(crate) fn legacy_next_flag_guidance(args: &[String]) -> Option<String> {
    let next_pos = args.iter().position(|arg| arg == "next")?;
    let next_args = &args[next_pos + 1..];
    let has_agent = next_args.iter().any(|arg| arg == "--agent");
    let has_human = next_args.iter().any(|arg| arg == "--human");

    if !has_agent && !has_human {
        return None;
    }

    let (management_role, delivery_role) = workflow_topology::current_default_role_examples()
        .unwrap_or_else(|| ("manager".to_string(), "operator".to_string()));
    let mut message = format!(
        "`keel next` no longer accepts `--agent` or `--human`. Use `--role {delivery_role}` for delivery work or `--role {management_role}` for management decisions."
    );
    if next_args.iter().any(|arg| arg == "--role") {
        message.push_str(" Do not combine legacy queue flags with `--role`.");
    }
    Some(message)
}

fn default_actor_context(
    topology: &ResolvedWorkflowTopology,
    parallel: bool,
) -> ResolvedActorContext {
    let lane = if parallel {
        topology.default_delivery_lane()
    } else {
        topology.default_management_lane()
    };
    let queue_lane = if parallel {
        topology.default_delivery_queue_lane()
    } else {
        topology.default_management_queue_lane()
    };

    ResolvedActorContext {
        lane_name: lane.name.clone(),
        queue_lane,
        supports_parallel: lane.parallel,
        role_context: None,
    }
}

fn parallel_lane_error(topology: &ResolvedWorkflowTopology, lane_name: &str) -> anyhow::Error {
    let delivery_role = topology.delivery_role_example();
    let delivery_lane = topology.default_delivery_lane();

    if delivery_lane.parallel {
        anyhow::anyhow!(
            "`keel next --parallel` is not enabled for lane `{lane_name}`. Use `keel next --role {delivery_role} --parallel` or omit `--parallel`."
        )
    } else {
        anyhow::anyhow!(
            "`keel next --parallel` is not enabled for lane `{lane_name}`. The configured delivery role example `{delivery_role}` also resolves to a non-parallel lane, so omit `--parallel` or update the workflow topology."
        )
    }
}

fn parallel_queue_surface_error(
    topology: &ResolvedWorkflowTopology,
    lane_name: &str,
) -> anyhow::Error {
    anyhow::anyhow!(
        "`keel next --parallel` requires a lane that sources execution work. Lane `{lane_name}` does not expose backlog or in-progress stories. Reconfigure the delivery role example `{}` for delivery work or omit `--parallel`.",
        topology.delivery_role_example(),
    )
}

fn unsupported_role_error(
    error: workflow_topology::UnknownRoleFamily,
    topology: &ResolvedWorkflowTopology,
) -> anyhow::Error {
    anyhow::anyhow!(
        "Unsupported `keel next --role` family `{}`. Try `keel next --role {}` for management work or `keel next --role {}` for delivery work.",
        error.base_role,
        topology.management_role_example(),
        topology.delivery_role_example(),
    )
}

pub(crate) fn calculate_next_for_role(
    board: &crate::domain::model::Board,
    board_dir: &Path,
    parallel: bool,
    actor_role: Option<&crate::domain::model::taxonomy::RoleTaxonomy>,
) -> Result<NextDecision> {
    let topology = workflow_topology::load_for_board(board_dir)?;
    
    let effective_role = match actor_role {
        Some(r) => r.clone(),
        None => crate::domain::model::taxonomy::parse(&topology.defaults.management_role)?,
    };
    
    let actor_topology = topology.resolve_actor_context(&effective_role)
        .map_err(|error| unsupported_role_error(error, &topology))?;
    
    let execution_mode = matches!(
        actor_topology.queue_lane,
        crate::read_model::queue_policy::ActorQueueLane::Execution
    );

    if parallel && !actor_topology.parallel {
        bail!(
            "`keel next --parallel` is not enabled for lane `{}`. Use `keel next --role {} --parallel` or omit `--parallel`.",
            actor_topology.lane_name,
            topology.delivery_role_example(),
        );
    }
    
    let filter = ItemFilter {
        mission_id: None,
        actor_role: Some(&effective_role),
    };
    
    calculate_next(board, board_dir, execution_mode, &filter)
}

/// Run the next command
pub fn run(
    board_dir: &Path,
    json: bool,
    parallel: bool,
    actor_role: Option<&crate::domain::model::taxonomy::RoleTaxonomy>,
) -> Result<()> {
    let board = load_board(board_dir)?;
    let topology = workflow_topology::load_for_board(board_dir)?;
    
    let effective_role = match actor_role {
        Some(r) => r.clone(),
        None => crate::domain::model::taxonomy::parse(&topology.defaults.management_role)?,
    };
    
    let actor_topology = topology.resolve_actor_context(&effective_role)
        .map_err(|error| unsupported_role_error(error, &topology))?;
    
    let management_role_example = topology.management_role_example().to_string();

    if parallel {
        if !actor_topology.parallel {
            bail!(
                "`keel next --parallel` is not enabled for lane `{}`. Use `keel next --role {} --parallel` or omit `--parallel`.",
                actor_topology.lane_name,
                topology.delivery_role_example(),
            );
        }
        
        let role_context =
            crate::read_model::role_context::resolve_role_context(&topology, &effective_role)
                .ok()
                .map(|contract| RoleContextGuidance::from_contract(&effective_role, contract));

        let resolved_context = ResolvedActorContext {
            lane_name: actor_topology.lane_name.clone(),
            queue_lane: actor_topology.queue_lane,
            supports_parallel: actor_topology.parallel,
            role_context,
        };

        return run_parallel(
            &board,
            board_dir,
            json,
            Some(&effective_role),
            Some(&resolved_context),
            &management_role_example,
        );
    }

    let decision = calculate_next_for_role(&board, board_dir, false, Some(&effective_role))?;
    
    let role_context =
        crate::read_model::role_context::resolve_role_context(&topology, &effective_role)
            .ok()
            .map(|contract| RoleContextGuidance::from_contract(&effective_role, contract));

    if json {
        let result = decision_to_json(&decision, role_context.as_ref(), &management_role_example);
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", format_decision(&decision));
        print_human_guidance(
            guidance_for_decision(&decision, role_context.as_ref(), &management_role_example).as_ref(),
        );

        match &decision {
            NextDecision::Work(d) => {
                surface_ranked_knowledge(
                    board_dir,
                    "Relevant knowledge for this task:",
                    d.story.epic(),
                    d.story.frontmatter.scope.as_deref(),
                    5,
                );
            }
            NextDecision::NeedsPlanning(d) => {
                if let Some(voyage) = d.voyages.first() {
                    let scope = voyage.scope_path();
                    surface_ranked_knowledge(
                        board_dir,
                        "Relevant knowledge for planning:",
                        Some(&voyage.epic_id),
                        Some(&scope),
                        5,
                    );
                }
            }
            NextDecision::NeedsStories(d) => {
                if let Some(voyage) = d.voyages.first() {
                    let scope = voyage.scope_path();
                    surface_ranked_knowledge(
                        board_dir,
                        "Relevant knowledge for planning:",
                        Some(&voyage.epic_id),
                        Some(&scope),
                        5,
                    );
                }
            }
            NextDecision::Research(_) => {
                surface_ranked_knowledge(
                    board_dir,
                    "Relevant knowledge for research:",
                    None,
                    None,
                    5,
                );
            }
            _ => {}
        }
    }

    Ok(())
}

fn surface_ranked_knowledge(
    board_dir: &Path,
    heading: &str,
    epic: Option<&str>,
    scope: Option<&str>,
    limit: usize,
) {
    let _ = crate::application::knowledge_context::surface_ranked_knowledge(
        board_dir, heading, epic, scope, limit, None,
    );
}

fn decision_to_json(
    decision: &NextDecision,
    role_context: Option<&RoleContextGuidance>,
    management_role_example: &str,
) -> JsonResult {
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
        NextDecision::Mission(d) => JsonDetails::Mission {
            id: d.mission.id().to_string(),
            title: d.mission.title().to_string(),
            unmet_goals: d.unmet_goals.clone(),
            suggestion: d.suggestion.clone(),
        },
        NextDecision::Missions(d) => JsonDetails::Missions {
            missions: d
                .missions
                .iter()
                .map(|m| JsonMission {
                    id: m.mission.id().to_string(),
                    title: m.mission.title().to_string(),
                })
                .collect(),
        },
        NextDecision::Empty(d) => JsonDetails::Empty {
            suggestions: d.suggestions.clone(),
        },
    };

    JsonResult {
        decision: decision_kind(decision).to_string(),
        details,
        guidance: guidance_for_decision(decision, role_context, management_role_example),
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
        NextDecision::Mission(_) => "mission",
        NextDecision::Missions(_) => "missions",
        NextDecision::Empty(_) => "empty",
    }
}

fn guidance_for_decision(
    decision: &NextDecision,
    role_context: Option<&RoleContextGuidance>,
    management_role_example: &str,
) -> Option<CanonicalGuidance> {
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
        NextDecision::Accept(d) => d.stories.first().map(|story| {
            CommandGuidance::next(story_accept_command_for_role(
                story.id(),
                management_role_example,
            ))
        }),
        NextDecision::Research(d) => d
            .bearings
            .first()
            .map(|bearing| CommandGuidance::next(format!("keel play {}", bearing.id()))),
        NextDecision::Blocked(d) => Some(CommandGuidance::recovery(story_accept_command_for_role(
            d.story.id(),
            management_role_example,
        ))),
        NextDecision::NeedsStories(d) => d.voyages.first().map(|voyage| {
            CommandGuidance::next(story_creation_command(
                "<title>",
                Some(voyage.epic_id.as_str()),
                Some(voyage.id()),
            ))
        }),
        NextDecision::NeedsPlanning(d) => d
            .voyages
            .first()
            .map(|voyage| CommandGuidance::next(format!("keel voyage plan {}", voyage.id()))),
        NextDecision::Mission(d) => Some(CommandGuidance::next(format!(
            "keel mission show {}",
            d.mission.id()
        ))),
        NextDecision::Missions(_) => Some(CommandGuidance::next("keel mission list".to_string())),
        NextDecision::Empty(_) => None,
    };

    render_command_guidance(command_guidance)
        .map(|guidance| attach_role_context(guidance, role_context))
}

fn guidance_for_parallel_ready(
    ready: &[&Story],
    role_context: Option<&RoleContextGuidance>,
) -> Option<CanonicalGuidance> {
    render_command_guidance(
        ready
            .first()
            .map(|story| CommandGuidance::next(format!("keel story start {}", story.id()))),
    )
    .map(|guidance| attach_role_context(guidance, role_context))
}

fn attach_role_context(
    guidance: CanonicalGuidance,
    role_context: Option<&RoleContextGuidance>,
) -> CanonicalGuidance {
    match role_context {
        Some(role_context) => guidance.with_role_context(role_context.clone()),
        None => guidance,
    }
}

fn render_human_guidance(guidance: Option<&CanonicalGuidance>) -> String {
    let Some(guidance) = guidance else {
        return String::new();
    };

    let mut rendered = String::new();

    if let Some(role_context) = guidance.role_context.as_ref() {
        rendered.push_str("\nRole context:\n");
        rendered.push_str(&format!("  Role: {}\n", role_context.role));
        rendered.push_str(&format!(
            "  Operational Contract: {}\n",
            role_context.contract_id
        ));
        rendered.push_str(&format!("  Lane: {}\n", role_context.lane));
        rendered.push_str(&format!("  Persona: {}\n", role_context.persona));
        rendered.push_str("  Priorities:\n");
        for priority in &role_context.priorities {
            rendered.push_str(&format!("    - {priority}\n"));
        }
        rendered.push_str("  Workflow:\n");
        for hint in &role_context.workflow {
            rendered.push_str(&format!("    - {hint}\n"));
        }
    }

    if let Some(step) = guidance.next_step.as_ref() {
        rendered.push_str(&format!("\nNext step:\n  {}\n", step.command.bold()));
        return rendered;
    }

    if let Some(step) = guidance.recovery_step.as_ref() {
        rendered.push_str(&format!("\nRecovery step:\n  {}\n", step.command.bold()));
        return rendered;
    }

    rendered
}

fn print_human_guidance(guidance: Option<&CanonicalGuidance>) {
    let rendered = render_human_guidance(guidance);
    if !rendered.is_empty() {
        print!("{rendered}");
    }
}

fn render_parallel_blockers_human(
    blocked_pairs: &[crate::cli::commands::management::next_support::parallel_threshold::PairwiseBlocker],
) -> String {
    if blocked_pairs.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    out.push_str("\nPairwise Blockers:\n");
    for blocker in blocked_pairs {
        out.push_str(&format!(
            "  - {} -> {}: {}\n",
            crate::cli::style::styled_story_id(&blocker.story_id),
            crate::cli::style::styled_story_id(&blocker.blocked_by_story_id),
            blocker.reasons.join("; ")
        ));
    }

    out
}

fn project_parallel_work<'a>(
    board: &'a crate::domain::model::Board,
    board_dir: &Path,
    actor_role: Option<&crate::domain::model::taxonomy::RoleTaxonomy>,
) -> ParallelProjection<'a> {
    use crate::domain::state_machine::invariants;
    use crate::read_model::traceability::derive_implementation_dependencies;

    // Get all workable stories, optionally filtered by role.
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

    // Filter into parallel-safe (ready) and sequential chains.
    let mut ready = Vec::new();
    let mut sequential: BTreeMap<String, Vec<&Story>> = BTreeMap::new();

    for story in candidates {
        let is_unblocked = deps.get(story.id()).is_none_or(|dep_ids| {
            dep_ids.iter().all(|id| {
                board
                    .stories
                    .get(id)
                    .map(|dep| dep.status == crate::domain::model::StoryState::Done)
                    .unwrap_or(false)
            })
        });

        if is_unblocked {
            ready.push(story);
        } else if let Some(scope) = story.scope() {
            sequential.entry(scope.to_string()).or_default().push(story);
        }
    }

    // Sort sequential chains by index.
    for chain in sequential.values_mut() {
        chain.sort_by_key(|s| s.index());
    }

    // Compute deterministic semantic signals and conservative pairwise scores.
    let pairwise_feature_vectors =
        crate::cli::commands::management::next_support::parallel_features::extract_parallel_feature_vectors(
            board, &ready,
        );
    let pairwise_scores =
        crate::cli::commands::management::next_support::parallel_scoring::score_parallel_pairwise_conflicts(
            &pairwise_feature_vectors,
        );
    let threshold_selection =
        crate::cli::commands::management::next_support::parallel_threshold::select_parallel_candidates_with_confidence_threshold(
            &ready,
            &pairwise_scores,
        );

    ParallelProjection {
        ready: threshold_selection.selected,
        sequential_chains: sequential,
        blocked_pairs: threshold_selection.blocked_pairs,
    }
}

fn json_pairwise_blockers(
    blocked_pairs: &[crate::cli::commands::management::next_support::parallel_threshold::PairwiseBlocker],
) -> Vec<JsonPairwiseBlocker> {
    blocked_pairs
        .iter()
        .map(|blocker| JsonPairwiseBlocker {
            story_id: blocker.story_id.clone(),
            blocked_by: blocker.blocked_by_story_id.clone(),
            reasons: blocker.reasons.clone(),
            confidence: blocker.confidence,
        })
        .collect()
}

fn build_parallel_json_result(
    projection: &ParallelProjection<'_>,
    role_context: Option<&RoleContextGuidance>,
    _management_role_example: &str,
) -> JsonResult {
    let mut ready_json: Vec<JsonStory> = projection
        .ready
        .iter()
        .map(|s| JsonStory {
            id: s.id().to_string(),
            title: s.title().to_string(),
            scope: s.scope().map(|sc| sc.to_string()),
            index: s.index(),
        })
        .collect();

    let sequential_json = projection
        .sequential_chains
        .iter()
        .map(|(scope, stories)| {
            let chain: Vec<JsonStory> = stories
                .iter()
                .map(|s| JsonStory {
                    id: s.id().to_string(),
                    title: s.title().to_string(),
                    scope: s.scope().map(|sc| sc.to_string()),
                    index: s.index(),
                })
                .collect();
            (scope.clone(), chain)
        })
        .collect();

    let next = ready_json.first().cloned();
    if !ready_json.is_empty() {
        ready_json.remove(0);
    }

    JsonResult {
        decision: "parallel_work".to_string(),
        details: JsonDetails::ParallelWork {
            next,
            ready: ready_json,
            sequential_chains: sequential_json,
            blocked_pairs: json_pairwise_blockers(&projection.blocked_pairs),
        },
        guidance: guidance_for_parallel_ready(&projection.ready, role_context),
    }
}

fn run_parallel(
    board: &crate::domain::model::Board,
    board_dir: &Path,
    json: bool,
    actor_role: Option<&crate::domain::model::taxonomy::RoleTaxonomy>,
    actor_context: Option<&ResolvedActorContext>,
    management_role_example: &str,
) -> Result<()> {
    let projection = project_parallel_work(board, board_dir, actor_role);
    let role_context = actor_context.and_then(|context| context.role_context.as_ref());

    if json {
        let result = build_parallel_json_result(&projection, role_context, management_role_example);
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("Ready for Work (Parallel Safe):");
        if projection.ready.is_empty() {
            println!("  (none)");
        } else {
            for story in &projection.ready {
                println!("  - {}", parallel_story_with_scope(story));
            }

            // Surface relevant knowledge for the first ready story
            if let Some(story) = projection.ready.first() {
                surface_ranked_knowledge(
                    board_dir,
                    &format!(
                        "Relevant knowledge for [{}]:",
                        crate::cli::style::styled_story_id(story.id())
                    ),
                    story.epic(),
                    story.frontmatter.scope.as_deref(),
                    5,
                );
            }
        }

        if !projection.sequential_chains.is_empty() {
            println!("\nSequential Chains (by Scope):");
            for (scope, stories) in &projection.sequential_chains {
                println!("  {}:", crate::cli::style::styled_scope(Some(scope)));
                for story in stories {
                    println!("    - {}", parallel_story(story));
                }
            }
        }

        let blockers_human = render_parallel_blockers_human(&projection.blocked_pairs);
        if !blockers_human.is_empty() {
            print!("{blockers_human}");
        }

        print_human_guidance(guidance_for_parallel_ready(&projection.ready, role_context).as_ref());
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
        AdrFactory, BearingFactory, StoryFactory, TestBoardBuilder, TestEpic, TestStory,
        TestVoyage, VoyageFactory,
    };
    use std::fs;

    #[test]
    fn exit_code_work_is_0() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::Backlog))
            .build();
        let result = run(temp.path(), false, false, None);
        assert!(result.is_ok());
    }

    #[test]
    fn parallel_story_with_scope_uses_shared_id_colors() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .title("Story 1")
                    .scope("EPIC-1/VOY-1")
                    .status(StoryState::Backlog),
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
                    .status(StoryState::Backlog),
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

    fn default_topology() -> ResolvedWorkflowTopology {
        workflow_topology::resolve(&crate::infrastructure::config::Config::default()).unwrap()
    }

    fn make_role_context(role: &str) -> RoleContextGuidance {
        let taxonomy = crate::domain::model::taxonomy::parse(role).unwrap();
        let topology = default_topology();
        let template =
            crate::read_model::role_context::resolve_role_context(&topology, &taxonomy).unwrap();
        RoleContextGuidance::from_contract(&taxonomy, template)
    }

    fn write_custom_topology_config(path: &Path) {
        fs::write(
            path.join("keel.toml"),
            r#"[workflow.defaults]
management_role = "director"
delivery_role = "maker"
management_lane = "review"
delivery_lane = "delivery"

[roles.director]
default_lane = "review"
operational_contract = "director-core"

[roles.maker]
default_lane = "delivery"
operational_contract = "maker-core"

[lanes.review]
description = "Review and approvals"
include = ["bearing.*", "story.needs-human-verification", "voyage.draft"]
parallel = false
manual_accept = true
priority = 100

[lanes.delivery]
description = "Implementation work"
include = ["story.*"]
exclude = ["story.done", "story.icebox", "story.needs-human-verification", "story.rejected"]
parallel = true
manual_accept = false
priority = 50
"#,
        )
        .unwrap();
    }

    fn assert_human_json_guidance_parity(decision: &NextDecision) {
        let guidance = guidance_for_decision(decision, None, "manager");
        let rendered = render_human_guidance(guidance.as_ref());
        let json = serde_json::to_value(decision_to_json(decision, None, "manager")).unwrap();

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

        let payload = decision_to_json(&decision, None, "manager");
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

        let payload = decision_to_json(&decision, None, "manager");
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

        let payload = decision_to_json(&decision, None, "manager");
        let json = serde_json::to_value(payload).unwrap();

        assert_eq!(json["decision"], "accept");
        assert_eq!(
            json["guidance"]["next_step"]["command"],
            "keel story accept S2 --role manager"
        );
        assert!(json["guidance"]["recovery_step"].is_null());
    }

    #[test]
    fn decision_to_json_blocked_includes_recovery_guidance() {
        let decision = NextDecision::Blocked(BlockedDecision {
            story: make_story("S9"),
            count: 9,
        });

        let payload = decision_to_json(&decision, None, "manager");
        let json = serde_json::to_value(payload).unwrap();

        assert_eq!(json["decision"], "blocked");
        assert_eq!(
            json["guidance"]["recovery_step"]["command"],
            "keel story accept S9 --role manager"
        );
        assert!(json["guidance"]["next_step"].is_null());
    }

    #[test]
    fn decision_to_json_empty_omits_guidance() {
        let decision = NextDecision::Empty(EmptyDecision {
            suggestions: vec!["Refuel".to_string()],
        });

        let payload = decision_to_json(&decision, None, "manager");
        let json = serde_json::to_value(payload).unwrap();

        assert!(json.get("guidance").is_none());
    }

    #[test]
    fn decision_to_json_with_role_context_includes_resolved_contract_payload() {
        let decision = NextDecision::Work(StoryDecision {
            story: make_story("SCTX"),
            is_continuation: false,
            warning: None,
        });
        let role_context = make_role_context("operator/software");

        let payload = decision_to_json(&decision, Some(&role_context), "manager");
        let json = serde_json::to_value(payload).unwrap();

        assert_eq!(
            json["guidance"]["role_context"]["role"],
            "operator/software"
        );
        assert_eq!(
            json["guidance"]["role_context"]["contract_id"],
            "operator-core"
        );
        assert_eq!(json["guidance"]["role_context"]["lane"], "delivery");
        assert_eq!(
            json["guidance"]["next_step"]["command"],
            "keel story start SCTX"
        );
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
        let guidance = guidance_for_parallel_ready(&[&ready_story], None).unwrap();
        let json = serde_json::to_value(&guidance).unwrap();
        let rendered = render_human_guidance(Some(&guidance));

        assert_eq!(json["next_step"]["command"], "keel story start SREADY");
        assert!(json["recovery_step"].is_null());
        assert!(rendered.contains("Next step:"));
        assert!(rendered.contains("keel story start SREADY"));
    }

    #[test]
    fn next_parallel_pairwise_blockers_render_human() {
        let blocked_pairs = vec![
            crate::cli::commands::management::next_support::parallel_threshold::PairwiseBlocker {
                story_id: "S2".to_string(),
                blocked_by_story_id: "S1".to_string(),
                reasons: vec!["confidence 0.50 below threshold 0.70".to_string()],
                confidence: 0.5,
            },
        ];

        let rendered = render_parallel_blockers_human(&blocked_pairs);
        assert!(rendered.contains("Pairwise Blockers:"));
        assert!(rendered.contains("S2"));
        assert!(rendered.contains("S1"));
        assert!(rendered.contains("->"));
        assert!(rendered.contains("confidence 0.50 below threshold 0.70"));
    }

    #[test]
    fn next_parallel_pairwise_blockers_render_json() {
        let result = JsonResult {
            decision: "parallel_work".to_string(),
            details: JsonDetails::ParallelWork {
                next: None,
                ready: vec![],
                sequential_chains: BTreeMap::new(),
                blocked_pairs: vec![JsonPairwiseBlocker {
                    story_id: "S2".to_string(),
                    blocked_by: "S1".to_string(),
                    reasons: vec!["confidence 0.50 below threshold 0.70".to_string()],
                    confidence: 0.5,
                }],
            },
            guidance: None,
        };

        let json = serde_json::to_value(result).unwrap();
        assert_eq!(
            json["details"]["parallel_work"]["blocked_pairs"][0]["story_id"],
            "S2"
        );
        assert_eq!(
            json["details"]["parallel_work"]["blocked_pairs"][0]["blocked_by"],
            "S1"
        );
        assert_eq!(
            json["details"]["parallel_work"]["blocked_pairs"][0]["reasons"][0],
            "confidence 0.50 below threshold 0.70"
        );
        assert_eq!(
            json["details"]["parallel_work"]["blocked_pairs"][0]["confidence"],
            0.5
        );
    }

    #[test]
    fn next_parallel_output_is_deterministic() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-parallel", "keel").srs_content(srs))
            .story(
                TestStory::new("S2")
                    .title("Follow-on core work")
                    .scope("keel/01-parallel")
                    .body("- [ ] [SRS-02/AC-01] follow-on")
                    .status(StoryState::Backlog),
            )
            .story(
                TestStory::new("S1")
                    .title("Core foundation")
                    .scope("keel/01-parallel")
                    .body("- [ ] [SRS-01/AC-01] foundation")
                    .status(StoryState::Backlog),
            )
            .story(
                TestStory::new("S3")
                    .title("Ops lane")
                    .scope("ops/01-parallel")
                    .status(StoryState::Backlog),
            )
            .build();

        let board_first = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let board_second = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let first_projection = project_parallel_work(&board_first, temp.path(), None);
        let second_projection = project_parallel_work(&board_second, temp.path(), None);

        let first_output = serde_json::to_string_pretty(&build_parallel_json_result(
            &first_projection,
            None,
            "manager",
        ))
        .unwrap();
        let second_output = serde_json::to_string_pretty(&build_parallel_json_result(
            &second_projection,
            None,
            "manager",
        ))
        .unwrap();

        assert_eq!(first_output, second_output);

        let json = serde_json::from_str::<serde_json::Value>(&first_output).unwrap();
        assert_eq!(json["details"]["parallel_work"]["next"]["id"], "S1");
        assert_eq!(json["details"]["parallel_work"]["ready"][0]["id"], "S3");
        assert_eq!(
            json["details"]["parallel_work"]["sequential_chains"]["keel/01-parallel"][0]["id"],
            "S2"
        );
    }

    #[test]
    fn next_parallel_pairwise_blockers_render_consistently() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-parallel", "keel").srs_content(srs))
            .story(
                TestStory::new("S1")
                    .title("Core lane")
                    .scope("keel/01-parallel")
                    .status(StoryState::Backlog),
            )
            .story(
                TestStory::new("S2")
                    .title("Ops lane")
                    .scope("keel/01-parallel")
                    .blocked_by(&["S1"])
                    .status(StoryState::Backlog),
            )
            .story(
                TestStory::new("S3")
                    .title("Docs lane")
                    .scope("keel/01-parallel")
                    .status(StoryState::Backlog),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let projection = project_parallel_work(&board, temp.path(), None);

        let selected_ids: Vec<_> = projection.ready.iter().map(|story| story.id()).collect();
        assert_eq!(selected_ids, vec!["S1", "S3"]);
        assert_eq!(projection.blocked_pairs.len(), 1);

        let blocker = &projection.blocked_pairs[0];
        let human = render_parallel_blockers_human(&projection.blocked_pairs);
        let json = serde_json::to_value(build_parallel_json_result(&projection, None, "manager"))
            .expect("json payload");
        let json_blocker = &json["details"]["parallel_work"]["blocked_pairs"][0];

        assert_eq!(json_blocker["story_id"], blocker.story_id);
        assert_eq!(json_blocker["blocked_by"], blocker.blocked_by_story_id);
        assert_eq!(json_blocker["reasons"][0], blocker.reasons[0]);
        assert_eq!(
            json_blocker["confidence"].as_f64().unwrap(),
            blocker.confidence
        );
        assert!(human.contains(&blocker.story_id));
        assert!(human.contains(&blocker.blocked_by_story_id));
        assert!(human.contains(blocker.reasons[0].as_str()));
    }

    #[test]
    fn render_human_guidance_surfaces_role_context_and_lane() {
        let guidance = CanonicalGuidance::next("keel story start S1")
            .with_role_context(make_role_context("manager"));

        let rendered = render_human_guidance(Some(&guidance));

        assert!(rendered.contains("Role context:"));
        assert!(rendered.contains("Role: manager"));
        assert!(rendered.contains("Operational Contract: manager-core"));
        assert!(rendered.contains("Lane: management"));
        assert!(
            rendered.contains("Persona: Mission steward for scope, approvals, and coordination.")
        );
        assert!(rendered.contains("Priorities:"));
        assert!(rendered.contains("Workflow:"));
        assert!(rendered.contains("Next step:"));
    }

    #[test]
    fn next_role_topology_rejects_unknown_family_with_configured_default_examples() {
        let temp = TestBoardBuilder::new().build();
        write_custom_topology_config(temp.path());
        let topology = workflow_topology::load_for_board(temp.path()).unwrap();
        let analyst = crate::domain::model::taxonomy::parse("analyst/research").unwrap();

        let error = topology.resolve_actor_context(&analyst)
            .unwrap_err()
            .to_string();

        assert_eq!(
            error,
            "unknown workflow role family `analyst`"
        );
    }

    #[test]
    fn next_parallel_topology_rejects_non_parallel_lane_with_delivery_role_guidance() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::Backlog))
            .build();
        let topology = workflow_topology::load_for_board(temp.path()).unwrap();
        let manager = crate::domain::model::taxonomy::parse("manager").unwrap();
        let actor_context = topology.resolve_actor_context(&manager).unwrap();

        assert!(!actor_context.parallel);
    }

    #[test]
    fn next_parallel_topology_resolution_is_deterministic() {
        let temp = TestBoardBuilder::new().build();
        write_custom_topology_config(temp.path());
        let topology = workflow_topology::load_for_board(temp.path()).unwrap();
        let maker = crate::domain::model::taxonomy::parse("maker").unwrap();

        let first = topology.resolve_actor_context(&maker).unwrap();
        let second = topology.resolve_actor_context(&maker).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.queue_lane, second.queue_lane);
    }
}
