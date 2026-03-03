//! Flow and pull-system modeling

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod bottleneck;
pub mod box_component;
pub mod capacity;
pub mod display;
pub mod format;
pub mod layout;
pub mod next_up;
pub mod throughput;

pub use crate::read_model::flow_metrics::{
    ExecutionMetrics, FlowMetrics, GovernanceMetrics, PlanningMetrics, ResearchMetrics,
    VerificationMetrics, calculate_metrics,
};
pub use bottleneck::{
    ActorQueue, BottleneckAnalysis, BottleneckConstraint, PipelineStage, TwoActorHealth,
    analyze_health, analyze_two_actor_health,
};
pub use box_component::BoxComponent;
pub use display::{render_annotated_flow, render_queue_boxes};
pub use format::{
    QueueItemDisplay, VoyageDepSummary, classify_stories, render_dependency_chains,
    render_epic_capacities,
};
pub use layout::LayoutConfig;
pub use next_up::{NextUpItem, NextUpSection, calculate_next_up};
pub use throughput::calculate_throughput;

/// Calculate board-wide flow state summary.
pub fn calculate_flow_state(
    board: &crate::domain::model::Board,
) -> crate::read_model::flow_metrics::FlowMetrics {
    crate::read_model::flow_metrics::calculate_metrics(board)
}
