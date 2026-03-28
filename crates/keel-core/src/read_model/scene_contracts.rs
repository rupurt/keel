//! Canonical scene-surface registry.

use super::command_catalog::CommandSurfaceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SceneSignal {
    DoctorReport,
    DerivedHeartbeat,
    WorkingHoursGate,
    FlowMetrics,
    WorkflowTopology,
    HumanAttentionQueue,
    PlanningClutter,
    RemediationEffort,
    StructuralDrift,
    PulseCycle,
    MissionConstraintWatch,
    MissionNextDecision,
    WorkCapital,
}

impl SceneSignal {
    pub const fn label(self) -> &'static str {
        match self {
            Self::DoctorReport => "doctor_report",
            Self::DerivedHeartbeat => "derived_heartbeat",
            Self::WorkingHoursGate => "working_hours_gate",
            Self::FlowMetrics => "flow_metrics",
            Self::WorkflowTopology => "workflow_topology",
            Self::HumanAttentionQueue => "human_attention_queue",
            Self::PlanningClutter => "planning_clutter",
            Self::RemediationEffort => "remediation_effort",
            Self::StructuralDrift => "structural_drift",
            Self::PulseCycle => "pulse_cycle",
            Self::MissionConstraintWatch => "mission_constraint_watch",
            Self::MissionNextDecision => "mission_next_decision",
            Self::WorkCapital => "work_capital",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SceneContract {
    pub surface_id: &'static str,
    pub name: &'static str,
    pub command_id: CommandSurfaceId,
    pub dependencies: &'static [SceneSignal],
    pub purpose: &'static str,
}

const MED_BAY_SIGNALS: &[SceneSignal] = &[SceneSignal::DoctorReport, SceneSignal::FlowMetrics];
const POWER_RACK_SIGNALS: &[SceneSignal] = &[
    SceneSignal::WorkingHoursGate,
    SceneSignal::DoctorReport,
    SceneSignal::DerivedHeartbeat,
    SceneSignal::FlowMetrics,
    SceneSignal::WorkflowTopology,
];
const WORKBENCH_SIGNALS: &[SceneSignal] = &[
    SceneSignal::DerivedHeartbeat,
    SceneSignal::HumanAttentionQueue,
    SceneSignal::PlanningClutter,
    SceneSignal::RemediationEffort,
    SceneSignal::StructuralDrift,
    SceneSignal::WorkflowTopology,
];
const DOCTOR_SCENE_SIGNALS: &[SceneSignal] = &[SceneSignal::DoctorReport];
const CLOCKTOWER_SIGNALS: &[SceneSignal] = &[SceneSignal::PulseCycle];
const CONSTRAINT_WATCH_SIGNALS: &[SceneSignal] = &[SceneSignal::MissionConstraintWatch];
const MISSION_RADAR_SIGNALS: &[SceneSignal] = &[
    SceneSignal::MissionNextDecision,
    SceneSignal::WorkflowTopology,
];
const VAULT_SIGNALS: &[SceneSignal] = &[SceneSignal::FlowMetrics, SceneSignal::WorkCapital];

pub const SCENE_CONTRACTS: &[SceneContract] = &[
    SceneContract {
        surface_id: "med-bay",
        name: "Med-Bay",
        command_id: CommandSurfaceId::Health,
        dependencies: MED_BAY_SIGNALS,
        purpose: "Fast structural triage before more work is introduced.",
    },
    SceneContract {
        surface_id: "power-rack",
        name: "Power Circuit",
        command_id: CommandSurfaceId::Flow,
        dependencies: POWER_RACK_SIGNALS,
        purpose: "Board-wide readiness view for power, flow, and watch pressure.",
    },
    SceneContract {
        surface_id: "workbench",
        name: "Workbench",
        command_id: CommandSurfaceId::Workshop,
        dependencies: WORKBENCH_SIGNALS,
        purpose: "Human-attention queue and drift surface.",
    },
    SceneContract {
        surface_id: "doctor-scene",
        name: "Repair Bay",
        command_id: CommandSurfaceId::Doctor,
        dependencies: DOCTOR_SCENE_SIGNALS,
        purpose: "Strict structural confidence surface.",
    },
    SceneContract {
        surface_id: "clocktower",
        name: "Automation Gear Train",
        command_id: CommandSurfaceId::Pulse,
        dependencies: CLOCKTOWER_SIGNALS,
        purpose: "Automation cycle state rendered as a visual mechanism.",
    },
    SceneContract {
        surface_id: "constraint-watch",
        name: "Constraint Watch",
        command_id: CommandSurfaceId::MissionShow,
        dependencies: CONSTRAINT_WATCH_SIGNALS,
        purpose: "Focused mission watch surface for a single objective.",
    },
    SceneContract {
        surface_id: "mission-radar",
        name: "Mission Radar",
        command_id: CommandSurfaceId::MissionNext,
        dependencies: MISSION_RADAR_SIGNALS,
        purpose: "Compact tactical scan of the next mission move.",
    },
    SceneContract {
        surface_id: "vault",
        name: "Vault",
        command_id: CommandSurfaceId::Finance,
        dependencies: VAULT_SIGNALS,
        purpose: "Work-capital and remediation-cost surface.",
    },
];

pub fn all_scene_contracts() -> &'static [SceneContract] {
    SCENE_CONTRACTS
}

pub fn scene_contract_for_command_id(id: CommandSurfaceId) -> Option<&'static SceneContract> {
    SCENE_CONTRACTS
        .iter()
        .find(|contract| contract.command_id == id)
}

pub fn scene_contract_for_surface_id(surface_id: &str) -> Option<&'static SceneContract> {
    SCENE_CONTRACTS
        .iter()
        .find(|contract| contract.surface_id == surface_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_model::command_catalog::scene_command_descriptors;

    #[test]
    fn every_scene_capable_command_has_a_contract() {
        for descriptor in scene_command_descriptors() {
            let surface_id = descriptor
                .scene_support
                .expect("scene command descriptor should have scene support")
                .surface_id;
            let contract = scene_contract_for_surface_id(surface_id)
                .expect("scene-capable command should have a scene contract");
            assert_eq!(contract.command_id, descriptor.id);
        }
    }

    #[test]
    fn scene_registry_covers_documented_scene_surface_count() {
        assert_eq!(all_scene_contracts().len(), 8);
        assert!(scene_contract_for_surface_id("mission-radar").is_some());
        assert!(scene_contract_for_surface_id("constraint-watch").is_some());
    }

    #[test]
    fn heartbeat_and_routing_scenes_are_expressed_through_dependencies() {
        let flow = scene_contract_for_surface_id("power-rack").expect("flow scene contract");
        assert!(flow.dependencies.contains(&SceneSignal::DerivedHeartbeat));
        assert!(flow.dependencies.contains(&SceneSignal::WorkflowTopology));

        let workbench =
            scene_contract_for_surface_id("workbench").expect("workbench scene contract");
        assert!(
            workbench
                .dependencies
                .contains(&SceneSignal::DerivedHeartbeat)
        );

        let mission =
            scene_contract_for_surface_id("mission-radar").expect("mission radar scene contract");
        assert!(
            mission
                .dependencies
                .contains(&SceneSignal::MissionNextDecision)
        );
        assert!(
            mission
                .dependencies
                .contains(&SceneSignal::WorkflowTopology)
        );
    }
}
