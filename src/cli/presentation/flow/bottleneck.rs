//! Logic for identifying execution bottlenecks

use super::metrics::FlowMetrics;
use super::throughput::Throughput;
use crate::read_model::queue_policy::{self, VerificationQueueCategory};

/// Pipeline stages for constraint identification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PipelineStage {
    /// Research phase (bearings)
    Research,
    /// Planning phase (voyages)
    Planning,
    /// Execution phase (stories in progress)
    Execution,
    /// Verification phase (ready for acceptance)
    Verification,
}

impl std::fmt::Display for PipelineStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineStage::Research => write!(f, "Research"),
            PipelineStage::Planning => write!(f, "Planning"),
            PipelineStage::Execution => write!(f, "Execution"),
            PipelineStage::Verification => write!(f, "Verification"),
        }
    }
}

/// Risk of a stage running out of work
#[derive(Debug)]
pub struct StarvationRisk {
    /// Which stage is at risk
    pub stage: PipelineStage,
    /// Estimated weeks until empty
    pub weeks_until_empty: f64,
    /// Description of the risk
    pub description: String,
}

/// Overall flow health assessment
#[derive(Debug)]
pub struct BottleneckAnalysis {
    /// Current constraint (where work is piling up)
    pub constraint: PipelineStage,
    /// Reason for constraint identification
    pub constraint_reason: String,
    /// Starvation risks (stages that may run dry)
    pub risks: Vec<StarvationRisk>,
    /// Suggested actions to improve flow
    pub suggested_actions: Vec<String>,
}

/// Constraint identification metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BottleneckConstraint {
    /// Work is piling up in research
    Research,
    /// Work is piling up in planning
    Planning,
    /// Work is piling up in execution
    Execution,
    /// Work is piling up in verification
    Verification,
}

/// Thresholds for starvation risk (weeks until empty)
const RESEARCH_STARVATION_WEEKS: f64 = 4.0;
const PLANNING_STARVATION_WEEKS: f64 = 3.0;
const EXECUTION_STARVATION_WEEKS: f64 = 2.0;

/// Fallback heuristic thresholds (item counts)
const RESEARCH_MIN_ITEMS: usize = 3;
const PLANNING_MIN_ITEMS: usize = 5;
const EXECUTION_MIN_ITEMS: usize = 3;

/// Analyze pipeline health and identify bottlenecks
pub fn analyze_health(metrics: &FlowMetrics, throughput: &Throughput) -> BottleneckAnalysis {
    let constraint = identify_constraint(metrics, throughput);
    let constraint_reason = explain_constraint(&constraint, metrics, throughput);
    let risks = identify_starvation_risks(metrics, throughput);
    let suggested_actions = generate_suggestions(&constraint, &risks, metrics);

    BottleneckAnalysis {
        constraint,
        constraint_reason,
        risks,
        suggested_actions,
    }
}

///// Calculate flow ratio for a stage: downstream_count / upstream_throughput_per_week
fn calculate_flow_ratio(downstream_count: usize, upstream_throughput: f64) -> f64 {
    if upstream_throughput <= 0.0 {
        // No throughput = infinite ratio (work piling up)
        if downstream_count > 0 {
            f64::INFINITY
        } else {
            0.0
        }
    } else {
        downstream_count as f64 / upstream_throughput
    }
}

/// Identify the current constraint (stage with highest flow ratio)
fn identify_constraint(metrics: &FlowMetrics, throughput: &Throughput) -> PipelineStage {
    // Calculate flow ratios for each stage
    // Flow ratio = downstream_count / upstream_throughput_per_week

    // Research: bearings waiting / (assumed constant input rate, use 1.0)
    let research_count = metrics.research.exploring_count
        + metrics.research.surveying_count
        + metrics.research.assessing_count;
    let research_ratio =
        calculate_flow_ratio(research_count, throughput.avg_bearings_per_month / 4.0);

    // Planning: draft voyages + epics needing voyages / bearings laid per month (converted to weeks)
    let planning_count = metrics.planning.draft_count + metrics.planning.epics_needing_voyages;
    let planning_ratio =
        calculate_flow_ratio(planning_count, throughput.avg_bearings_per_month / 4.0);

    // Execution: stories in progress or backlog / planned voyages throughput
    // Note: backlog stories are "ready" for agent, but they originate from planning
    let execution_count = metrics.execution.backlog_count + metrics.execution.in_progress_count;
    let execution_ratio = calculate_flow_ratio(execution_count, throughput.avg_stories_per_week);

    // Verification: stories awaiting acceptance / stories completed per week
    let verification_ratio =
        calculate_flow_ratio(metrics.verification.count, throughput.avg_stories_per_week);

    // Find highest ratio (the constraint)
    let ratios = [
        (PipelineStage::Research, research_ratio),
        (PipelineStage::Planning, planning_ratio),
        (PipelineStage::Execution, execution_ratio),
        (PipelineStage::Verification, verification_ratio),
    ];

    // Default to Research if everything is zero, so the user is encouraged to add work
    ratios
        .into_iter()
        .filter(|(_, ratio)| *ratio > 0.0)
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(stage, _)| stage)
        .unwrap_or(PipelineStage::Research)
}

/// Generate explanation for why a stage is the constraint
fn explain_constraint(
    constraint: &PipelineStage,
    metrics: &FlowMetrics,
    throughput: &Throughput,
) -> String {
    let research_count = metrics.research.exploring_count
        + metrics.research.surveying_count
        + metrics.research.assessing_count;
    let planning_count = metrics.planning.draft_count
        + metrics.planning.epics_needing_voyages
        + metrics.planning.planned_count;
    let execution_count = metrics.execution.backlog_count + metrics.execution.in_progress_count;

    // Special case for total system starvation
    if research_count == 0
        && planning_count == 0
        && execution_count == 0
        && metrics.verification.count == 0
    {
        return "System is starved. Add new bearings or epics to begin.".to_string();
    }

    match constraint {
        PipelineStage::Research => {
            format!(
                "{} bearings in research with {:.1} bearings laid/month",
                research_count, throughput.avg_bearings_per_month
            )
        }
        PipelineStage::Planning => {
            format!(
                "{} voyages in planning with {:.1} bearings laid/month",
                metrics.planning.draft_count + metrics.planning.epics_needing_voyages,
                throughput.avg_bearings_per_month
            )
        }
        PipelineStage::Execution => {
            format!(
                "{} stories in execution with {:.1} stories/week throughput",
                execution_count, throughput.avg_stories_per_week
            )
        }
        PipelineStage::Verification => {
            format!(
                "{} stories awaiting acceptance (avg {:.1} days old)",
                metrics.verification.count, metrics.verification.avg_age_days
            )
        }
    }
}

/// Identify stages at risk of starvation
fn identify_starvation_risks(
    metrics: &FlowMetrics,
    throughput: &Throughput,
) -> Vec<StarvationRisk> {
    let mut risks = Vec::new();

    // Research starvation risk
    let research_count = metrics.research.exploring_count
        + metrics.research.surveying_count
        + metrics.research.assessing_count;
    let research_weeks = if throughput.avg_bearings_per_month > 0.0 {
        research_count as f64 / (throughput.avg_bearings_per_month / 4.0)
    } else if research_count < RESEARCH_MIN_ITEMS {
        // Fallback heuristic
        0.0
    } else {
        f64::INFINITY
    };

    if research_weeks < RESEARCH_STARVATION_WEEKS || research_count < RESEARCH_MIN_ITEMS {
        risks.push(StarvationRisk {
            stage: PipelineStage::Research,
            weeks_until_empty: research_weeks,
            description: format!(
                "Research has {} items (~{:.1} weeks of work)",
                research_count, research_weeks
            ),
        });
    }

    // Planning starvation risk
    let planning_weeks = if throughput.avg_stories_per_week > 0.0 {
        metrics.planning.planned_count as f64 / throughput.avg_stories_per_week
    } else if metrics.planning.planned_count < PLANNING_MIN_ITEMS {
        0.0
    } else {
        f64::INFINITY
    };

    if planning_weeks < PLANNING_STARVATION_WEEKS
        || metrics.planning.planned_count < PLANNING_MIN_ITEMS
    {
        risks.push(StarvationRisk {
            stage: PipelineStage::Planning,
            weeks_until_empty: planning_weeks,
            description: format!(
                "Planning has {} voyages (~{:.1} weeks of work)",
                metrics.planning.planned_count, planning_weeks
            ),
        });
    }

    // Execution starvation risk
    let execution_count = metrics.execution.backlog_count + metrics.execution.in_progress_count;
    let execution_weeks = if throughput.avg_stories_per_week > 0.0 {
        execution_count as f64 / throughput.avg_stories_per_week
    } else if execution_count < EXECUTION_MIN_ITEMS {
        0.0
    } else {
        f64::INFINITY
    };

    if execution_weeks < EXECUTION_STARVATION_WEEKS || execution_count < EXECUTION_MIN_ITEMS {
        risks.push(StarvationRisk {
            stage: PipelineStage::Execution,
            weeks_until_empty: execution_weeks,
            description: format!(
                "Execution has {} stories (~{:.1} weeks of work)",
                execution_count, execution_weeks
            ),
        });
    }

    risks
}

/// Generate suggested actions based on constraints and risks
fn generate_suggestions(
    constraint: &PipelineStage,
    risks: &[StarvationRisk],
    metrics: &FlowMetrics,
) -> Vec<String> {
    let mut suggestions = Vec::new();

    let research_count = metrics.research.exploring_count
        + metrics.research.surveying_count
        + metrics.research.assessing_count;
    let execution_count = metrics.execution.backlog_count + metrics.execution.in_progress_count;
    let queue_policy_snapshot = queue_policy::project(metrics);
    let verification_queue = queue_policy_snapshot.verification;

    // Suggestion based on constraint (only if there's actually work in that stage)
    match constraint {
        PipelineStage::Research if queue_policy_snapshot.has_research_work => {
            suggestions.push("Focus on completing bearing assessments".to_string());
        }
        PipelineStage::Planning => {
            if metrics.planning.epics_needing_voyages > 0 {
                suggestions.push("Decompose active epics into new voyages".to_string());
            } else if queue_policy_snapshot.has_planning_work {
                suggestions.push("Review and approve draft voyages".to_string());
            }
        }
        PipelineStage::Execution if execution_count > 0 => {
            suggestions.push("Focus on completing in-flight implementation".to_string());
        }
        PipelineStage::Verification if verification_queue.has_items() => {
            suggestions.push("Review and accept completed stories".to_string());
        }
        _ => {}
    }

    // If everything is empty, suggest adding work
    if suggestions.is_empty()
        && research_count == 0
        && metrics.planning.draft_count == 0
        && metrics.planning.epics_needing_voyages == 0
        && execution_count == 0
        && !verification_queue.has_items()
    {
        suggestions.push("Add new bearings to populate the research pipeline".to_string());
    }

    // Suggestions based on starvation risks
    for risk in risks {
        match risk.stage {
            PipelineStage::Research => {
                if !suggestions.iter().any(|s| s.contains("bearing")) {
                    suggestions.push("Add new bearings to replenish research pipeline".to_string());
                }
            }
            PipelineStage::Planning => {
                if !suggestions
                    .iter()
                    .any(|s| s.contains("voyage") || s.contains("decompose"))
                {
                    suggestions.push("Create voyages from laid bearings or epics".to_string());
                }
            }
            PipelineStage::Execution => {
                if !suggestions.iter().any(|s| s.contains("stories")) {
                    suggestions.push("Decompose planned voyages into stories".to_string());
                }
            }
            PipelineStage::Verification => {
                // Verification starvation is fine
            }
        }
    }

    suggestions
}

// =============================================================================
// Two-Actor Health Model
// =============================================================================

/// Queue items for a specific actor (human or agent)
#[derive(Debug, Clone)]
pub struct ActorQueue {
    /// Items in this actor's queue
    pub items: Vec<QueueItem>,
    /// Whether this actor is starved (no work available)
    pub is_starved: bool,
    /// Message explaining starvation (if starved)
    pub starvation_message: Option<String>,
}

/// Single item in an actor's queue (domain data only)
#[derive(Debug, Clone)]
pub struct QueueItem {
    /// Display label (e.g., "to accept", "to start")
    pub label: String,
    /// Current count of items
    pub count: usize,
    /// Age in days of oldest item (raw domain data)
    pub age_days: Option<usize>,
    /// Secondary count for segmented gauge (e.g., WIP portion)
    pub secondary_count: Option<usize>,
}

/// Two-actor flow health (separates human and agent work)
#[derive(Debug)]
pub struct TwoActorHealth {
    /// Human's work queue
    pub human_queue: ActorQueue,
    /// Agent's work queue
    pub agent_queue: ActorQueue,
    /// Summary of recommended action
    pub action_summary: String,
}

/// Analyze flow health for two-actor model
pub fn analyze_two_actor_health(metrics: &FlowMetrics) -> TwoActorHealth {
    let human_queue = build_human_queue(metrics);
    let agent_queue = build_agent_queue(metrics);
    let queue_policy_snapshot = queue_policy::project(metrics);

    let action_summary = summarize_action(
        queue_policy_snapshot.verification,
        queue_policy_snapshot.agent.is_starved(),
    );

    TwoActorHealth {
        human_queue,
        agent_queue,
        action_summary,
    }
}

fn summarize_action(verification_queue: VerificationQueueCategory, agent_starved: bool) -> String {
    match verification_queue {
        VerificationQueueCategory::FlowBlocked => {
            "Verification queue is blocked; review and accept completed stories".to_string()
        }
        VerificationQueueCategory::HumanBlocked => {
            "Human queue is blocked; review and accept completed stories".to_string()
        }
        VerificationQueueCategory::Attention => "Stories await human acceptance".to_string(),
        VerificationQueueCategory::Empty if agent_starved => {
            "Start a voyage to unblock agent".to_string()
        }
        VerificationQueueCategory::Empty => "Flow is healthy".to_string(),
    }
}

fn build_human_queue(metrics: &FlowMetrics) -> ActorQueue {
    let mut items = Vec::new();

    // Proposed ADRs: governance decisions needed (highest priority)
    if metrics.governance.proposed_count > 0 {
        items.push(QueueItem {
            label: "proposed ADRs".to_string(),
            count: metrics.governance.proposed_count,
            age_days: None, // ADRs don't track age currently
            secondary_count: None,
        });
    }

    // To accept: stories in needs-human-verification
    let accept_count = metrics.verification.count;
    let oldest_age_days = if metrics.verification.max_age_days > 0 {
        Some(metrics.verification.max_age_days)
    } else {
        None
    };
    items.push(QueueItem {
        label: "to accept".to_string(),
        count: accept_count,
        age_days: oldest_age_days,
        secondary_count: None,
    });

    // To start: voyages in planned status (ready to start execution)
    items.push(QueueItem {
        label: "to start".to_string(),
        count: metrics.planning.planned_count,
        age_days: None,
        secondary_count: None,
    });

    // To decompose: epics needing voyages + draft voyages needing stories
    let decompose_count = metrics.planning.epics_needing_voyages + metrics.planning.draft_count;
    if decompose_count > 0 {
        items.push(QueueItem {
            label: "to decompose".to_string(),
            count: decompose_count,
            age_days: None,
            secondary_count: None,
        });
    }

    // Calibrate: bearings in pipeline needing calibration (exploring + surveying + assessing)
    let calibrate_count = metrics.research.exploring_count
        + metrics.research.surveying_count
        + metrics.research.assessing_count;
    items.push(QueueItem {
        label: "to calibrate".to_string(),
        count: calibrate_count,
        age_days: None,
        secondary_count: None,
    });

    ActorQueue {
        items,
        is_starved: false, // Human is never "starved" in the same way
        starvation_message: None,
    }
}

fn build_agent_queue(metrics: &FlowMetrics) -> ActorQueue {
    let ready_count = metrics.execution.backlog_count + metrics.execution.in_progress_count;
    let queue_pressure =
        queue_policy::classify_queue_pressure(metrics.verification.count, ready_count);
    let is_starved = queue_pressure.agent.is_starved();

    // Agent queue shows ready items with secondary shading for WIP
    // Primary (█) = backlog, Secondary (▒) = in-progress
    let items = vec![QueueItem {
        label: "ready".to_string(),
        count: ready_count,
        age_days: None,
        secondary_count: Some(metrics.execution.in_progress_count),
    }];

    let starvation_message = if is_starved {
        Some("Start a voyage to unblock.".to_string())
    } else {
        None
    };

    ActorQueue {
        items,
        is_starved,
        starvation_message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::presentation::flow::metrics::{
        ExecutionMetrics, GovernanceMetrics, PlanningMetrics, ResearchMetrics, VerificationMetrics,
    };
    use crate::cli::presentation::flow::throughput::{MonthThroughput, WeekThroughput};
    use crate::domain::policy::queue::{
        FLOW_VERIFY_BLOCK_THRESHOLD, HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD,
    };

    fn make_metrics() -> FlowMetrics {
        FlowMetrics {
            governance: GovernanceMetrics {
                proposed_count: 0,
                accepted_count: 2,
            },
            research: ResearchMetrics {
                exploring_count: 2,
                surveying_count: 1,
                assessing_count: 1,
                laid_count: 5,
                parked_count: 0,
            },
            planning: PlanningMetrics {
                epics_needing_voyages: 0,
                draft_count: 0,
                planned_count: 3,
            },
            execution: ExecutionMetrics {
                backlog_count: 5,
                in_progress_count: 2,
                active_voyages_count: 1,
            },
            verification: VerificationMetrics {
                count: 1,
                avg_age_days: 2.0,
                max_age_days: 3,
                items: vec![("s1".to_string(), 3)],
            },
            done_count: 10,
        }
    }

    fn make_throughput() -> Throughput {
        Throughput {
            stories_per_week: vec![
                WeekThroughput {
                    week_start: chrono::NaiveDate::from_ymd_opt(2026, 1, 20).unwrap(),
                    count: 3,
                },
                WeekThroughput {
                    week_start: chrono::NaiveDate::from_ymd_opt(2026, 1, 13).unwrap(),
                    count: 2,
                },
            ],
            bearings_per_month: vec![MonthThroughput {
                year: 2026,
                month: 1,
                count: 4,
            }],
            avg_stories_per_week: 2.5,
            avg_bearings_per_month: 4.0,
        }
    }

    #[test]
    fn calculate_flow_ratio_with_throughput() {
        assert!((calculate_flow_ratio(10, 2.0) - 5.0).abs() < 0.01);
        assert!((calculate_flow_ratio(5, 2.5) - 2.0).abs() < 0.01);
    }

    #[test]
    fn calculate_flow_ratio_zero_throughput() {
        assert!(calculate_flow_ratio(5, 0.0).is_infinite());
        assert!((calculate_flow_ratio(0, 0.0) - 0.0).abs() < 0.01);
    }

    #[test]
    fn identify_constraint_finds_highest_ratio() {
        let mut metrics = make_metrics();
        let throughput = make_throughput();

        // Pile up verification - should become constraint
        metrics.verification.count = FLOW_VERIFY_BLOCK_THRESHOLD;
        let constraint = identify_constraint(&metrics, &throughput);
        assert_eq!(constraint, PipelineStage::Verification);

        // Clear verification, pile up execution
        metrics.verification.count = 0;
        metrics.execution.backlog_count = 50;
        metrics.execution.in_progress_count = 10;
        let constraint = identify_constraint(&metrics, &throughput);
        assert_eq!(constraint, PipelineStage::Execution);
    }

    #[test]
    fn analyze_health_returns_constraint_and_suggestions() {
        let metrics = make_metrics();
        let throughput = make_throughput();

        let health = analyze_health(&metrics, &throughput);

        assert!(!health.constraint_reason.is_empty());
        assert!(!health.suggested_actions.is_empty());
    }

    #[test]
    fn identify_starvation_risk_research_low() {
        let mut metrics = make_metrics();
        metrics.research.exploring_count = 1;
        metrics.research.surveying_count = 0;
        metrics.research.assessing_count = 0;

        let throughput = make_throughput();
        let risks = identify_starvation_risks(&metrics, &throughput);

        assert!(risks.iter().any(|r| r.stage == PipelineStage::Research));
    }

    #[test]
    fn identify_starvation_risk_planning_low() {
        let mut metrics = make_metrics();
        metrics.planning.planned_count = 1;

        let throughput = make_throughput();
        let risks = identify_starvation_risks(&metrics, &throughput);

        assert!(risks.iter().any(|r| r.stage == PipelineStage::Planning));
    }

    #[test]
    fn identify_starvation_risk_execution_low() {
        let mut metrics = make_metrics();
        metrics.execution.backlog_count = 1;
        metrics.execution.in_progress_count = 0;

        let throughput = make_throughput();
        let risks = identify_starvation_risks(&metrics, &throughput);

        assert!(risks.iter().any(|r| r.stage == PipelineStage::Execution));
    }

    #[test]
    fn no_starvation_risk_when_healthy() {
        let mut metrics = make_metrics();
        // Lots of items in each stage
        metrics.research.exploring_count = 5;
        metrics.research.surveying_count = 3;
        metrics.research.assessing_count = 2;
        metrics.planning.planned_count = 10;
        metrics.execution.backlog_count = 10;
        metrics.execution.in_progress_count = 3;

        let throughput = make_throughput();
        let risks = identify_starvation_risks(&metrics, &throughput);

        assert!(risks.is_empty());
    }

    #[test]
    fn suggestions_address_constraint() {
        let metrics = make_metrics();
        let throughput = make_throughput();

        let health = analyze_health(&metrics, &throughput);

        // Should have at least one suggestion
        assert!(!health.suggested_actions.is_empty());
    }

    #[test]
    fn fallback_heuristics_when_no_throughput() {
        let mut metrics = make_metrics();
        metrics.research.exploring_count = 1;
        metrics.research.surveying_count = 0;
        metrics.research.assessing_count = 0;

        let throughput = Throughput::default(); // No historical data

        let risks = identify_starvation_risks(&metrics, &throughput);

        // Should still identify risk based on item count heuristic
        assert!(risks.iter().any(|r| r.stage == PipelineStage::Research));
    }

    #[test]
    fn analyze_health_empty_system() {
        let metrics = FlowMetrics::default();
        let throughput = Throughput::default();

        let health = analyze_health(&metrics, &throughput);

        assert_eq!(health.constraint, PipelineStage::Research);
        assert!(health.constraint_reason.contains("System is starved"));
        assert!(
            health
                .suggested_actions
                .contains(&"Add new bearings to populate the research pipeline".to_string())
        );
    }

    #[test]
    fn pipeline_stage_display() {
        assert_eq!(format!("{}", PipelineStage::Research), "Research");
        assert_eq!(format!("{}", PipelineStage::Planning), "Planning");
        assert_eq!(format!("{}", PipelineStage::Execution), "Execution");
        assert_eq!(format!("{}", PipelineStage::Verification), "Verification");
    }

    // ==========================================================================
    // Two-Actor Health Model Tests
    // ==========================================================================

    #[test]
    fn two_actor_human_queue_contains_accept_items() {
        let metrics = make_metrics();
        let health = analyze_two_actor_health(&metrics);

        let accept_item = health
            .human_queue
            .items
            .iter()
            .find(|i| i.label == "to accept");
        assert!(accept_item.is_some());
        assert_eq!(accept_item.unwrap().count, metrics.verification.count);
    }

    #[test]
    fn two_actor_human_queue_contains_start_items() {
        let metrics = make_metrics();
        let health = analyze_two_actor_health(&metrics);

        let start_item = health
            .human_queue
            .items
            .iter()
            .find(|i| i.label == "to start");
        assert!(start_item.is_some());
        assert_eq!(start_item.unwrap().count, metrics.planning.planned_count);
    }

    #[test]
    fn two_actor_human_queue_contains_calibrate_items() {
        let metrics = make_metrics();
        let health = analyze_two_actor_health(&metrics);

        let calibrate_item = health
            .human_queue
            .items
            .iter()
            .find(|i| i.label == "to calibrate");
        assert!(calibrate_item.is_some());

        let expected = metrics.research.exploring_count
            + metrics.research.surveying_count
            + metrics.research.assessing_count;
        assert_eq!(calibrate_item.unwrap().count, expected);
    }

    #[test]
    fn two_actor_agent_queue_contains_ready_items() {
        let metrics = make_metrics();
        let health = analyze_two_actor_health(&metrics);

        let ready_item = health.agent_queue.items.iter().find(|i| i.label == "ready");
        assert!(ready_item.is_some());

        let expected = metrics.execution.backlog_count + metrics.execution.in_progress_count;
        assert_eq!(ready_item.unwrap().count, expected);
    }

    #[test]
    fn two_actor_agent_starved_when_no_work() {
        let mut metrics = make_metrics();
        metrics.execution.backlog_count = 0;
        metrics.execution.in_progress_count = 0;
        metrics.verification.count = 0;

        let health = analyze_two_actor_health(&metrics);

        assert!(health.agent_queue.is_starved);
        assert!(health.agent_queue.starvation_message.is_some());
    }

    #[test]
    fn two_actor_agent_not_starved_with_backlog() {
        let mut metrics = make_metrics();
        metrics.execution.backlog_count = 5;
        metrics.execution.in_progress_count = 0;

        let health = analyze_two_actor_health(&metrics);

        assert!(!health.agent_queue.is_starved);
        assert!(health.agent_queue.starvation_message.is_none());
    }

    #[test]
    fn two_actor_agent_not_starved_with_wip() {
        let mut metrics = make_metrics();
        metrics.execution.backlog_count = 0;
        metrics.execution.in_progress_count = 2;

        let health = analyze_two_actor_health(&metrics);

        assert!(!health.agent_queue.is_starved);
    }

    #[test]
    fn two_actor_action_summary_suggests_unblock_when_starved() {
        let mut metrics = make_metrics();
        metrics.execution.backlog_count = 0;
        metrics.execution.in_progress_count = 0;
        metrics.verification.count = 0;

        let health = analyze_two_actor_health(&metrics);

        assert!(health.action_summary.to_lowercase().contains("voyage"));
    }

    #[test]
    fn two_actor_action_summary_suggests_accept_when_verify_attention() {
        let mut metrics = make_metrics();
        metrics.execution.backlog_count = 3;
        metrics.execution.in_progress_count = 1;
        metrics.verification.count = HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD - 1;

        let health = analyze_two_actor_health(&metrics);

        assert!(health.action_summary.to_lowercase().contains("accept"));
    }

    #[test]
    fn two_actor_action_summary_marks_blocked_at_human_threshold() {
        let mut metrics = make_metrics();
        metrics.execution.backlog_count = 3;
        metrics.execution.in_progress_count = 1;
        metrics.verification.count = HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD;

        let health = analyze_two_actor_health(&metrics);
        let summary = health.action_summary.to_lowercase();

        assert!(summary.contains("blocked"));
        assert!(summary.contains("accept"));
    }

    #[test]
    fn two_actor_action_summary_suggests_accept_when_verify_flow_blocked() {
        let mut metrics = make_metrics();
        metrics.execution.backlog_count = 3;
        metrics.execution.in_progress_count = 1;
        metrics.verification.count = FLOW_VERIFY_BLOCK_THRESHOLD + 1;

        let health = analyze_two_actor_health(&metrics);

        assert!(health.action_summary.to_lowercase().contains("blocked"));
        assert!(health.action_summary.to_lowercase().contains("accept"));
    }

    #[test]
    fn two_actor_accept_item_shows_oldest_age() {
        let mut metrics = make_metrics();
        metrics.verification.max_age_days = 5;

        let health = analyze_two_actor_health(&metrics);

        let accept_item = health
            .human_queue
            .items
            .iter()
            .find(|i| i.label == "to accept")
            .unwrap();
        assert_eq!(accept_item.age_days, Some(5));
    }
}
