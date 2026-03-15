//! Flow state machine for system health
#![allow(dead_code)] // Types defined for future integration (SRS-10/11)
//!
//! Captures overall system health based on queue depths:
//!
//! ```text
//! ┌────────────────┐
//! │ HEALTHY_FLOW   │ ← Normal operation, work flowing
//! └────────────────┘
//!
//! ┌────────────────┐
//! │ AGENT_STARVED  │ ← No work available for agent
//! └────────────────┘
//!
//! ┌────────────────┐
//! │ VERIFY_BLOCKED │ ← Too many items awaiting verification
//! └────────────────┘
//!
//! ┌────────────────┐
//! │ PIPELINE_EMPTY │ ← No work in the entire pipeline
//! └────────────────┘
//! ```

/// Flow health states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowState {
    /// Normal operation - work is flowing through the pipeline
    HealthyFlow,
    /// Agent has no work available (ready queue empty)
    AgentStarved,
    /// Too many items awaiting human verification
    VerifyBlocked,
    /// Pipeline is completely empty
    PipelineEmpty,
}

impl FlowState {
    /// Get a human-readable description of this state
    pub fn description(&self) -> &'static str {
        match self {
            FlowState::HealthyFlow => "Work is flowing normally through the pipeline",
            FlowState::AgentStarved => "Agent has no work available",
            FlowState::VerifyBlocked => "Verification backlog is blocking progress",
            FlowState::PipelineEmpty => "No work in the pipeline",
        }
    }

    /// Get the recommended action for this state
    pub fn recommended_action(&self) -> &'static str {
        match self {
            FlowState::HealthyFlow => "Continue current work",
            FlowState::AgentStarved => "Start a voyage to generate work for agent",
            FlowState::VerifyBlocked => "Review and accept completed stories",
            FlowState::PipelineEmpty => "Create new bearings or start new voyages",
        }
    }
}

/// Queue depths for flow state computation
#[derive(Debug, Clone, Copy, Default)]
pub struct QueueDepths {
    /// Stories awaiting human verification
    pub verify_count: usize,
    /// Voyages ready to start
    pub start_count: usize,
    /// Stories ready for agent (backlog + in-progress)
    pub ready_count: usize,
    /// Draft voyages needing decomposition
    pub decompose_count: usize,
    /// Bearings in research pipeline
    pub research_count: usize,
}

impl QueueDepths {
    /// Create depths from flow metrics
    pub fn from_metrics(
        verify: usize,
        start: usize,
        ready: usize,
        decompose: usize,
        research: usize,
    ) -> Self {
        Self {
            verify_count: verify,
            start_count: start,
            ready_count: ready,
            decompose_count: decompose,
            research_count: research,
        }
    }
}

impl QueueDepths {
    /// Check if the entire pipeline is empty
    pub fn is_empty(&self) -> bool {
        self.verify_count == 0
            && self.start_count == 0
            && self.ready_count == 0
            && self.decompose_count == 0
            && self.research_count == 0
    }

    /// Get total items in all queues
    pub fn total(&self) -> usize {
        self.verify_count
            + self.start_count
            + self.ready_count
            + self.decompose_count
            + self.research_count
    }
}

/// Flow state machine instance
#[derive(Debug, Clone)]
pub struct FlowStateMachine {
    state: FlowState,
    depths: QueueDepths,
}

impl FlowStateMachine {
    /// Compute the flow state from queue depths
    pub fn from_depths(depths: QueueDepths) -> Self {
        let state = Self::compute_state(&depths);
        Self { state, depths }
    }

    /// Get the current state
    pub fn state(&self) -> FlowState {
        self.state
    }

    /// Get the current queue depths
    pub fn depths(&self) -> &QueueDepths {
        &self.depths
    }

    /// Get the recommended action based on current state
    pub fn recommended_action(&self) -> &'static str {
        self.state.recommended_action()
    }

    /// Update with new queue depths
    pub fn update(&mut self, depths: QueueDepths) {
        self.depths = depths;
        self.state = Self::compute_state(&depths);
    }

    /// Compute state from queue depths
    ///
    /// Priority order:
    /// 1. PipelineEmpty - if no work anywhere
    /// 2. VerifyBlocked - if verification backlog exceeds threshold
    /// 3. AgentStarved - if no ready work for agent
    /// 4. HealthyFlow - otherwise
    fn compute_state(depths: &QueueDepths) -> FlowState {
        let queue_pressure = crate::read_model::queue_policy::classify_queue_pressure(
            depths.verify_count,
            depths.ready_count,
        );

        if depths.is_empty() {
            FlowState::PipelineEmpty
        } else if queue_pressure.verification.blocks_flow() {
            FlowState::VerifyBlocked
        } else if queue_pressure.agent.is_starved() {
            FlowState::AgentStarved
        } else {
            FlowState::HealthyFlow
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::policy::queue::FLOW_VERIFY_BLOCK_THRESHOLD;

    fn make_depths(
        verify: usize,
        start: usize,
        ready: usize,
        decompose: usize,
        research: usize,
    ) -> QueueDepths {
        QueueDepths {
            verify_count: verify,
            start_count: start,
            ready_count: ready,
            decompose_count: decompose,
            research_count: research,
        }
    }

    #[test]
    fn healthy_flow_when_work_available() {
        let depths = make_depths(5, 2, 10, 3, 2);
        let sm = FlowStateMachine::from_depths(depths);

        assert_eq!(sm.state(), FlowState::HealthyFlow);
    }

    #[test]
    fn agent_starved_when_no_ready_work() {
        let depths = make_depths(5, 2, 0, 3, 2);
        let sm = FlowStateMachine::from_depths(depths);

        assert_eq!(sm.state(), FlowState::AgentStarved);
    }

    #[test]
    fn verify_blocked_when_threshold_exceeded() {
        let depths = make_depths(FLOW_VERIFY_BLOCK_THRESHOLD + 1, 2, 10, 3, 2);
        let sm = FlowStateMachine::from_depths(depths);

        assert_eq!(sm.state(), FlowState::VerifyBlocked);
    }

    #[test]
    fn verify_not_blocked_at_policy_threshold_boundary() {
        let depths = make_depths(FLOW_VERIFY_BLOCK_THRESHOLD, 2, 10, 3, 2);
        let sm = FlowStateMachine::from_depths(depths);

        assert_eq!(sm.state(), FlowState::HealthyFlow);
    }

    #[test]
    fn pipeline_empty_when_no_work() {
        let depths = make_depths(0, 0, 0, 0, 0);
        let sm = FlowStateMachine::from_depths(depths);

        assert_eq!(sm.state(), FlowState::PipelineEmpty);
    }

    #[test]
    fn verify_blocked_takes_priority_over_agent_starved() {
        // Both conditions met, but verify blocked should win
        let depths = make_depths(FLOW_VERIFY_BLOCK_THRESHOLD + 1, 2, 0, 3, 2);
        let sm = FlowStateMachine::from_depths(depths);

        assert_eq!(sm.state(), FlowState::VerifyBlocked);
    }

    #[test]
    fn pipeline_empty_takes_highest_priority() {
        let depths = make_depths(0, 0, 0, 0, 0);
        let sm = FlowStateMachine::from_depths(depths);

        assert_eq!(sm.state(), FlowState::PipelineEmpty);
    }

    #[test]
    fn update_changes_state() {
        let mut sm = FlowStateMachine::from_depths(make_depths(5, 2, 10, 3, 2));
        assert_eq!(sm.state(), FlowState::HealthyFlow);

        // Update to agent starved condition
        sm.update(make_depths(5, 2, 0, 3, 2));
        assert_eq!(sm.state(), FlowState::AgentStarved);
    }

    #[test]
    fn recommended_actions_are_meaningful() {
        assert!(!FlowState::HealthyFlow.recommended_action().is_empty());
        assert!(!FlowState::AgentStarved.recommended_action().is_empty());
        assert!(!FlowState::VerifyBlocked.recommended_action().is_empty());
        assert!(!FlowState::PipelineEmpty.recommended_action().is_empty());
    }

    #[test]
    fn queue_depths_total() {
        let depths = make_depths(1, 2, 3, 4, 5);
        assert_eq!(depths.total(), 15);
    }

    #[test]
    fn queue_depths_is_empty() {
        assert!(make_depths(0, 0, 0, 0, 0).is_empty());
        assert!(!make_depths(1, 0, 0, 0, 0).is_empty());
    }
}
