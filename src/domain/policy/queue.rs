//! Canonical queue policy for `next` and `flow`.
//!
//! This module centralizes queue thresholds and queue-state categories so
//! decision logic does not drift across pull-system call sites.

use std::cmp::Ordering;

/// Verification items threshold where human-mode `next` stops new work and
/// requires clearing acceptance backlog first.
pub const HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD: usize = 5;

/// Verification items threshold where system flow is considered verify-blocked.
///
/// Note: Flow blocking begins when backlog is strictly greater than this value.
pub const FLOW_VERIFY_BLOCK_THRESHOLD: usize = 20;

/// Classification of verification queue pressure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationQueueCategory {
    /// No stories awaiting human verification.
    Empty,
    /// Some stories need acceptance, but queue does not block new work yet.
    Attention,
    /// Queue is large enough to block human-mode `next`.
    HumanBlocked,
    /// Queue is overloaded enough to mark flow as verify-blocked.
    FlowBlocked,
}

impl VerificationQueueCategory {
    /// Whether there are stories awaiting acceptance.
    pub fn has_items(self) -> bool {
        !matches!(self, Self::Empty)
    }

    /// Whether this queue should block human-mode pull decisions.
    pub fn blocks_human_next(self) -> bool {
        matches!(self, Self::HumanBlocked | Self::FlowBlocked)
    }

    /// Whether this queue should classify system flow as verify-blocked.
    pub fn blocks_flow(self) -> bool {
        matches!(self, Self::FlowBlocked)
    }
}

/// Derive verification queue category from queue depth.
pub fn classify_verification_queue(count: usize) -> VerificationQueueCategory {
    if count == 0 {
        VerificationQueueCategory::Empty
    } else if count >= HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD {
        if count > FLOW_VERIFY_BLOCK_THRESHOLD {
            VerificationQueueCategory::FlowBlocked
        } else {
            VerificationQueueCategory::HumanBlocked
        }
    } else {
        VerificationQueueCategory::Attention
    }
}

/// Classification of agent queue readiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentQueueCategory {
    /// No ready work for agent.
    Starved,
    /// Backlog or in-progress work exists.
    Ready,
}

impl AgentQueueCategory {
    /// Whether agent queue has no work.
    pub fn is_starved(self) -> bool {
        matches!(self, Self::Starved)
    }
}

/// Derive agent queue category from ready queue depth.
pub fn classify_agent_queue(ready_count: usize) -> AgentQueueCategory {
    if ready_count == 0 {
        AgentQueueCategory::Starved
    } else {
        AgentQueueCategory::Ready
    }
}

/// Whether research queue has actionable work.
pub fn has_research_work(
    exploring_count: usize,
    surveying_count: usize,
    assessing_count: usize,
) -> bool {
    exploring_count + surveying_count + assessing_count > 0
}

/// Whether planning queue has draft voyages that need human action.
pub fn has_planning_work(draft_count: usize) -> bool {
    draft_count > 0
}

/// Classification for draft voyage handling in human queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftVoyageQueueCategory {
    /// Draft voyage has no stories and needs decomposition.
    NeedsStories,
    /// Draft voyage has stories and needs planning transition.
    NeedsPlanning,
}

/// Classify a draft voyage based on attached story count.
pub fn classify_draft_voyage(story_count: usize) -> DraftVoyageQueueCategory {
    if story_count == 0 {
        DraftVoyageQueueCategory::NeedsStories
    } else {
        DraftVoyageQueueCategory::NeedsPlanning
    }
}

/// Canonical comparison rule for queue work ordering.
pub fn compare_work_item_ids(a: &str, b: &str) -> Ordering {
    a.cmp(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_threshold_defaults_are_stable() {
        assert_eq!(HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD, 5);
        assert_eq!(FLOW_VERIFY_BLOCK_THRESHOLD, 20);
    }

    #[test]
    fn classify_verification_queue_respects_boundaries() {
        assert_eq!(
            classify_verification_queue(0),
            VerificationQueueCategory::Empty
        );
        assert_eq!(
            classify_verification_queue(1),
            VerificationQueueCategory::Attention
        );
        assert_eq!(
            classify_verification_queue(HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD - 1),
            VerificationQueueCategory::Attention
        );
        assert_eq!(
            classify_verification_queue(HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD),
            VerificationQueueCategory::HumanBlocked
        );
        assert_eq!(
            classify_verification_queue(FLOW_VERIFY_BLOCK_THRESHOLD),
            VerificationQueueCategory::HumanBlocked
        );
        assert_eq!(
            classify_verification_queue(FLOW_VERIFY_BLOCK_THRESHOLD + 1),
            VerificationQueueCategory::FlowBlocked
        );
    }

    #[test]
    fn verification_queue_category_helpers_match_policy() {
        assert!(!VerificationQueueCategory::Empty.has_items());
        assert!(VerificationQueueCategory::Attention.has_items());

        assert!(!VerificationQueueCategory::Attention.blocks_human_next());
        assert!(VerificationQueueCategory::HumanBlocked.blocks_human_next());
        assert!(VerificationQueueCategory::FlowBlocked.blocks_human_next());

        assert!(!VerificationQueueCategory::HumanBlocked.blocks_flow());
        assert!(VerificationQueueCategory::FlowBlocked.blocks_flow());
    }

    #[test]
    fn classify_agent_queue_respects_starvation_boundary() {
        assert_eq!(classify_agent_queue(0), AgentQueueCategory::Starved);
        assert_eq!(classify_agent_queue(1), AgentQueueCategory::Ready);
        assert!(classify_agent_queue(0).is_starved());
        assert!(!classify_agent_queue(2).is_starved());
    }

    #[test]
    fn research_and_planning_helpers_match_queue_presence() {
        assert!(!has_research_work(0, 0, 0));
        assert!(has_research_work(1, 0, 0));
        assert!(has_research_work(0, 1, 0));
        assert!(has_research_work(0, 0, 1));

        assert!(!has_planning_work(0));
        assert!(has_planning_work(1));
    }

    #[test]
    fn draft_voyage_classification_follows_story_count() {
        assert_eq!(
            classify_draft_voyage(0),
            DraftVoyageQueueCategory::NeedsStories
        );
        assert_eq!(
            classify_draft_voyage(1),
            DraftVoyageQueueCategory::NeedsPlanning
        );
    }

    #[test]
    fn work_item_ordering_is_deterministic() {
        assert_eq!(compare_work_item_ids("A", "B"), Ordering::Less);
        assert_eq!(compare_work_item_ids("B", "A"), Ordering::Greater);
        assert_eq!(compare_work_item_ids("A", "A"), Ordering::Equal);
    }
}
