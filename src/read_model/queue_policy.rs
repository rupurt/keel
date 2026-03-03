//! Shared queue-policy read model for decision and rendering consumers.
//!
//! This facade provides one API for queue classification so `next`, flow
//! bottleneck analysis, and flow state decisions consume identical policy data.

pub use crate::domain::policy::queue::{
    AgentQueueCategory, DraftVoyageQueueCategory, VerificationQueueCategory,
};
use crate::read_model::flow_metrics::FlowMetrics;

/// Canonical queue-policy snapshot derived from flow metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueuePolicySnapshot {
    pub verification: VerificationQueueCategory,
    pub agent: AgentQueueCategory,
    pub has_research_work: bool,
    pub has_planning_work: bool,
}

/// Minimal queue pressure classification for state-machine style callers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueuePressure {
    pub verification: VerificationQueueCategory,
    pub agent: AgentQueueCategory,
}

/// Classify queue pressure from verification and ready queue depths.
pub fn classify_queue_pressure(verify_count: usize, ready_count: usize) -> QueuePressure {
    QueuePressure {
        verification: crate::domain::policy::queue::classify_verification_queue(verify_count),
        agent: crate::domain::policy::queue::classify_agent_queue(ready_count),
    }
}

/// Build queue-policy snapshot from canonical flow metrics.
pub fn project(metrics: &FlowMetrics) -> QueuePolicySnapshot {
    let ready_count = metrics.execution.backlog_count + metrics.execution.in_progress_count;
    let queue_pressure = classify_queue_pressure(metrics.verification.count, ready_count);

    QueuePolicySnapshot {
        verification: queue_pressure.verification,
        agent: queue_pressure.agent,
        has_research_work: crate::domain::policy::queue::has_research_work(
            metrics.research.exploring_count,
            metrics.research.surveying_count,
            metrics.research.assessing_count,
        ),
        has_planning_work: crate::domain::policy::queue::has_planning_work(
            metrics.planning.draft_count,
        ),
    }
}

/// Classify draft-voyage queue semantics using canonical queue policy rules.
pub fn classify_draft_voyage(story_count: usize) -> DraftVoyageQueueCategory {
    crate::domain::policy::queue::classify_draft_voyage(story_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::policy::queue::{
        FLOW_VERIFY_BLOCK_THRESHOLD, HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD,
    };
    use crate::read_model::flow_metrics::{
        ExecutionMetrics, FlowMetrics, PlanningMetrics, ResearchMetrics, VerificationMetrics,
    };

    #[test]
    fn classify_queue_pressure_uses_canonical_boundaries() {
        let empty = classify_queue_pressure(0, 0);
        assert_eq!(empty.verification, VerificationQueueCategory::Empty);
        assert_eq!(empty.agent, AgentQueueCategory::Starved);

        let human_blocked =
            classify_queue_pressure(HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD, 1).verification;
        assert_eq!(human_blocked, VerificationQueueCategory::HumanBlocked);

        let flow_blocked = classify_queue_pressure(FLOW_VERIFY_BLOCK_THRESHOLD + 1, 1).verification;
        assert_eq!(flow_blocked, VerificationQueueCategory::FlowBlocked);
    }

    #[test]
    fn project_classifies_render_and_decision_inputs() {
        let metrics = FlowMetrics {
            execution: ExecutionMetrics {
                backlog_count: 2,
                in_progress_count: 1,
                ..ExecutionMetrics::default()
            },
            planning: PlanningMetrics {
                draft_count: 1,
                ..PlanningMetrics::default()
            },
            research: ResearchMetrics {
                exploring_count: 1,
                ..ResearchMetrics::default()
            },
            verification: VerificationMetrics {
                count: HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD - 1,
                ..VerificationMetrics::default()
            },
            ..FlowMetrics::default()
        };

        let snapshot = project(&metrics);
        assert_eq!(snapshot.verification, VerificationQueueCategory::Attention);
        assert_eq!(snapshot.agent, AgentQueueCategory::Ready);
        assert!(snapshot.has_research_work);
        assert!(snapshot.has_planning_work);
    }

    #[test]
    fn classify_draft_voyage_matches_story_count_semantics() {
        assert_eq!(
            classify_draft_voyage(0),
            DraftVoyageQueueCategory::NeedsStories
        );
        assert_eq!(
            classify_draft_voyage(1),
            DraftVoyageQueueCategory::NeedsPlanning
        );
    }
}
