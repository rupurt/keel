//! Effective workflow-topology projection derived from config.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::domain::model::taxonomy::RoleTaxonomy;
use crate::infrastructure::config::{Config, LaneConfig, RoleFamilyConfig, WorkflowDefaultsConfig};
use crate::read_model::queue_policy::ActorQueueLane;

const SEEDED_MANAGEMENT_CONTRACT: &str = "manager-core";
const SEEDED_DELIVERY_CONTRACT: &str = "operator-core";
const SEEDED_MANAGEMENT_DESCRIPTION: &str = "Planning, triage, calibration, acceptance";
const SEEDED_DELIVERY_DESCRIPTION: &str = "Work ready for execution";

const QUEUE_SOURCE_CATALOG: &[&str] = &[
    "bearing.declined",
    "bearing.evaluating",
    "bearing.exploring",
    "bearing.laid",
    "bearing.parked",
    "bearing.ready",
    "mission.abandoned",
    "mission.active",
    "mission.achieved",
    "mission.defining",
    "mission.paused",
    "mission.verified",
    "story.backlog",
    "story.done",
    "story.icebox",
    "story.in-progress",
    "story.needs-human-verification",
    "story.rejected",
    "voyage.done",
    "voyage.draft",
    "voyage.in-progress",
    "voyage.planned",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedWorkflowTopology {
    pub defaults: WorkflowDefaultsConfig,
    pub roles: BTreeMap<String, ResolvedRoleFamily>,
    pub lanes: BTreeMap<String, ResolvedLane>,
    pub role_overrides: BTreeMap<String, ResolvedRoleOverride>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRoleFamily {
    pub name: String,
    pub default_lane: String,
    pub operational_contract: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedLane {
    pub name: String,
    pub description: String,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub sources: Vec<String>,
    pub parallel: bool,
    pub manual_accept: bool,
    pub priority: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRoleOverride {
    pub taxonomy: String,
    pub operational_contract: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorTopologyContext {
    pub role: RoleTaxonomy,
    pub lane_name: String,
    pub queue_lane: ActorQueueLane,
    pub parallel: bool,
    pub manual_accept: bool,
    pub operational_contract: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownRoleFamily {
    pub base_role: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowTopologyError {
    MissingDefaultRole { role: String },
    MissingDefaultLane { lane: String },
    UnknownDefaultLaneForRole { role: String, lane: String },
    UnknownSelectorPattern { pattern: String },
    CrossLaneOverlap { lanes: Vec<String>, source: String },
}

impl std::fmt::Display for WorkflowTopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDefaultRole { role } => {
                write!(f, "workflow default role `{role}` is not defined")
            }
            Self::MissingDefaultLane { lane } => {
                write!(f, "workflow default lane `{lane}` is not defined")
            }
            Self::UnknownDefaultLaneForRole { role, lane } => {
                write!(f, "role `{role}` references unknown lane `{lane}`")
            }
            Self::UnknownSelectorPattern { pattern } => {
                write!(f, "unknown workflow selector pattern `{pattern}`")
            }
            Self::CrossLaneOverlap { lanes, source } => {
                write!(
                    f,
                    "source `{source}` is included in multiple lanes: {}",
                    lanes.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for WorkflowTopologyError {}

impl std::fmt::Display for UnknownRoleFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown workflow role family `{}`", self.base_role)
    }
}

impl std::error::Error for UnknownRoleFamily {}

impl ResolvedWorkflowTopology {
    pub fn ordered_lanes(&self) -> Vec<&ResolvedLane> {
        let mut lanes: Vec<_> = self.lanes.values().collect();
        lanes.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.name.cmp(&b.name))
        });
        lanes
    }

    pub fn management_role_example(&self) -> &str {
        &self.defaults.management_role
    }

    pub fn delivery_role_example(&self) -> &str {
        &self.defaults.delivery_role
    }

    pub fn resolve_actor_lane(
        &self,
        role: &RoleTaxonomy,
    ) -> Result<&ResolvedLane, UnknownRoleFamily> {
        let role_family = self
            .roles
            .get(role.role.as_str())
            .ok_or_else(|| UnknownRoleFamily {
                base_role: role.role.clone(),
            })?;

        Ok(self
            .lanes
            .get(role_family.default_lane.as_str())
            .expect("validated workflow topology must resolve role lanes"))
    }

    pub fn default_management_lane(&self) -> &ResolvedLane {
        self.lanes
            .get(self.defaults.management_lane.as_str())
            .expect("validated workflow topology must contain the management lane")
    }

    pub fn default_delivery_lane(&self) -> &ResolvedLane {
        self.lanes
            .get(self.defaults.delivery_lane.as_str())
            .expect("validated workflow topology must contain the delivery lane")
    }

    pub fn default_management_queue_lane(&self) -> ActorQueueLane {
        classify_next_queue_lane(self.default_management_lane())
    }

    pub fn default_delivery_queue_lane(&self) -> ActorQueueLane {
        classify_next_queue_lane(self.default_delivery_lane())
    }

    pub fn queue_lane_for_actor(
        &self,
        role: &RoleTaxonomy,
    ) -> Result<ActorQueueLane, UnknownRoleFamily> {
        Ok(classify_next_queue_lane(self.resolve_actor_lane(role)?))
    }

    pub fn allows_manual_accept(&self, role: &RoleTaxonomy) -> Result<bool, UnknownRoleFamily> {
        Ok(self.resolve_actor_lane(role)?.manual_accept)
    }

    pub fn supports_parallel(&self, role: &RoleTaxonomy) -> Result<bool, UnknownRoleFamily> {
        Ok(self.resolve_actor_lane(role)?.parallel)
    }

    pub fn resolve_operational_contract<'a>(
        &'a self,
        role: &RoleTaxonomy,
    ) -> Result<&'a str, UnknownRoleFamily> {
        let role_family = self
            .roles
            .get(role.role.as_str())
            .ok_or_else(|| UnknownRoleFamily {
                base_role: role.role.clone(),
            })?;

        Ok(self
            .role_overrides
            .get(role.to_string().as_str())
            .map(|override_| override_.operational_contract.as_str())
            .unwrap_or(role_family.operational_contract.as_str()))
    }

    pub fn resolve_actor_context(
        &self,
        role: &RoleTaxonomy,
    ) -> Result<ActorTopologyContext, UnknownRoleFamily> {
        let lane = self.resolve_actor_lane(role)?;
        let operational_contract = self.resolve_operational_contract(role)?;

        Ok(ActorTopologyContext {
            role: role.clone(),
            lane_name: lane.name.clone(),
            queue_lane: classify_next_queue_lane(lane),
            parallel: lane.parallel,
            manual_accept: lane.manual_accept,
            operational_contract: operational_contract.to_string(),
        })
    }
}

pub fn project_root_for_board(board_dir: &Path) -> &Path {
    board_dir
        .parent()
        .filter(|parent| parent.join("keel.toml").exists())
        .unwrap_or(board_dir)
}

pub fn load_for_board(board_dir: &Path) -> Result<ResolvedWorkflowTopology, WorkflowTopologyError> {
    let project_root = project_root_for_board(board_dir);
    let (config, _) = crate::infrastructure::config::load_config_from(project_root);
    resolve(&config)
}

pub fn current_default_role_examples() -> Option<(String, String)> {
    let (config, _) = crate::infrastructure::config::load_config();
    resolve(&config).ok().map(|topology| {
        (
            topology.management_role_example().to_string(),
            topology.delivery_role_example().to_string(),
        )
    })
}

pub fn resolve(config: &Config) -> Result<ResolvedWorkflowTopology, WorkflowTopologyError> {
    let defaults = config.workflow.defaults.clone();
    let mut roles = config.roles.clone();
    let mut lanes = config.lanes.clone();

    if defaults.management_lane == "management" {
        lanes
            .entry(defaults.management_lane.clone())
            .or_insert_with(seeded_management_lane);
    }
    if defaults.delivery_lane == "delivery" {
        lanes
            .entry(defaults.delivery_lane.clone())
            .or_insert_with(seeded_delivery_lane);
    }

    if defaults.management_role == "manager" {
        roles
            .entry(defaults.management_role.clone())
            .or_insert_with(|| seeded_management_role(&defaults.management_lane));
    }
    if defaults.delivery_role == "operator" {
        roles
            .entry(defaults.delivery_role.clone())
            .or_insert_with(|| seeded_delivery_role(&defaults.delivery_lane));
    }

    if !roles.contains_key(&defaults.management_role) {
        return Err(WorkflowTopologyError::MissingDefaultRole {
            role: defaults.management_role,
        });
    }
    if !roles.contains_key(&defaults.delivery_role) {
        return Err(WorkflowTopologyError::MissingDefaultRole {
            role: defaults.delivery_role,
        });
    }
    if !lanes.contains_key(&defaults.management_lane) {
        return Err(WorkflowTopologyError::MissingDefaultLane {
            lane: defaults.management_lane,
        });
    }
    if !lanes.contains_key(&defaults.delivery_lane) {
        return Err(WorkflowTopologyError::MissingDefaultLane {
            lane: defaults.delivery_lane,
        });
    }

    let resolved_roles: BTreeMap<_, _> = roles
        .into_iter()
        .map(|(name, role)| {
            if !lanes.contains_key(&role.default_lane) {
                return Err(WorkflowTopologyError::UnknownDefaultLaneForRole {
                    role: name,
                    lane: role.default_lane,
                });
            }

            Ok((
                name.clone(),
                ResolvedRoleFamily {
                    name,
                    default_lane: role.default_lane,
                    operational_contract: role.operational_contract,
                },
            ))
        })
        .collect::<Result<_, _>>()?;

    let resolved_lanes: BTreeMap<_, _> = lanes
        .into_iter()
        .map(|(name, lane)| {
            let sources = compile_lane_sources(&lane.include, &lane.exclude)?;
            Ok((
                name.clone(),
                ResolvedLane {
                    name,
                    description: lane.description,
                    include: lane.include,
                    exclude: lane.exclude,
                    sources,
                    parallel: lane.parallel,
                    manual_accept: lane.manual_accept,
                    priority: lane.priority,
                },
            ))
        })
        .collect::<Result<_, _>>()?;

    let resolved_role_overrides = config
        .role_overrides
        .iter()
        .map(|(taxonomy, override_)| {
            (
                taxonomy.clone(),
                ResolvedRoleOverride {
                    taxonomy: taxonomy.clone(),
                    operational_contract: override_.operational_contract.clone(),
                },
            )
        })
        .collect();

    // Check for cross-lane overlap
    let mut source_to_lanes: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, lane) in &resolved_lanes {
        for source in &lane.sources {
            source_to_lanes
                .entry(source.clone())
                .or_default()
                .push(name.clone());
        }
    }

    for (source, lane_names) in source_to_lanes {
        if lane_names.len() > 1 {
            return Err(WorkflowTopologyError::CrossLaneOverlap {
                lanes: lane_names,
                source,
            });
        }
    }

    Ok(ResolvedWorkflowTopology {
        defaults: config.workflow.defaults.clone(),
        roles: resolved_roles,
        lanes: resolved_lanes,
        role_overrides: resolved_role_overrides,
    })
}

pub fn queue_source_catalog() -> &'static [&'static str] {
    QUEUE_SOURCE_CATALOG
}

fn classify_next_queue_lane(lane: &ResolvedLane) -> ActorQueueLane {
    if lane
        .sources
        .iter()
        .any(|source| matches!(source.as_str(), "story.backlog" | "story.in-progress"))
    {
        ActorQueueLane::Execution
    } else {
        ActorQueueLane::Management
    }
}

fn compile_lane_sources(
    include: &[String],
    exclude: &[String],
) -> Result<Vec<String>, WorkflowTopologyError> {
    let mut selected = BTreeSet::new();

    for pattern in include {
        for source in expand_selector_pattern(pattern)? {
            selected.insert(source);
        }
    }

    for pattern in exclude {
        for source in expand_selector_pattern(pattern)? {
            selected.remove(&source);
        }
    }

    Ok(selected.into_iter().collect())
}

fn expand_selector_pattern(pattern: &str) -> Result<Vec<String>, WorkflowTopologyError> {
    if queue_source_catalog().contains(&pattern) {
        return Ok(vec![pattern.to_string()]);
    }

    if let Some(prefix) = pattern.strip_suffix(".*") {
        let prefix = format!("{prefix}.");
        let matches: Vec<_> = queue_source_catalog()
            .iter()
            .filter(|source| source.starts_with(&prefix))
            .map(|source| (*source).to_string())
            .collect();

        if !matches.is_empty() {
            return Ok(matches);
        }
    }

    Err(WorkflowTopologyError::UnknownSelectorPattern {
        pattern: pattern.to_string(),
    })
}

fn seeded_management_role(default_lane: &str) -> RoleFamilyConfig {
    RoleFamilyConfig {
        default_lane: default_lane.to_string(),
        operational_contract: SEEDED_MANAGEMENT_CONTRACT.to_string(),
    }
}

fn seeded_delivery_role(default_lane: &str) -> RoleFamilyConfig {
    RoleFamilyConfig {
        default_lane: default_lane.to_string(),
        operational_contract: SEEDED_DELIVERY_CONTRACT.to_string(),
    }
}

fn seeded_management_lane() -> LaneConfig {
    LaneConfig {
        description: SEEDED_MANAGEMENT_DESCRIPTION.to_string(),
        include: vec![
            "bearing.*".to_string(),
            "mission.achieved".to_string(),
            "story.needs-human-verification".to_string(),
            "voyage.draft".to_string(),
        ],
        exclude: Vec::new(),
        parallel: false,
        manual_accept: true,
        priority: 100,
    }
}

fn seeded_delivery_lane() -> LaneConfig {
    LaneConfig {
        description: SEEDED_DELIVERY_DESCRIPTION.to_string(),
        include: vec!["story.*".to_string()],
        exclude: vec![
            "story.done".to_string(),
            "story.icebox".to_string(),
            "story.needs-human-verification".to_string(),
            "story.rejected".to_string(),
        ],
        parallel: true,
        manual_accept: false,
        priority: 50,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::config::RoleOverrideConfig;

    #[test]
    fn workflow_topology_resolve_seeds_defaults_when_omitted() {
        let config = Config::default();

        let topology = resolve(&config).unwrap();

        assert_eq!(topology.defaults.management_role, "manager");
        assert_eq!(topology.defaults.delivery_role, "operator");
        assert_eq!(topology.roles["manager"].default_lane, "management");
        assert_eq!(topology.roles["operator"].default_lane, "delivery");
        assert_eq!(
            topology.roles["manager"].operational_contract,
            "manager-core"
        );
        assert_eq!(
            topology.roles["operator"].operational_contract,
            "operator-core"
        );
        assert_eq!(topology.lanes["management"].priority, 100);
        assert_eq!(topology.lanes["delivery"].priority, 50);
        assert!(topology.lanes["management"].manual_accept);
        assert!(topology.lanes["delivery"].parallel);
    }

    #[test]
    fn workflow_topology_lane_config_compiles_exact_and_glob_sources() {
        let mut config = Config::default();
        config.role_overrides.insert(
            "operator/software".to_string(),
            RoleOverrideConfig {
                operational_contract: "software-operator-core".to_string(),
            },
        );

        let topology = resolve(&config).unwrap();

        assert!(topology.role_overrides.contains_key("operator/software"));
        assert_eq!(
            topology.lanes["management"].sources,
            vec![
                "bearing.declined".to_string(),
                "bearing.evaluating".to_string(),
                "bearing.exploring".to_string(),
                "bearing.laid".to_string(),
                "bearing.parked".to_string(),
                "bearing.ready".to_string(),
                "mission.achieved".to_string(),
                "story.needs-human-verification".to_string(),
                "voyage.draft".to_string(),
            ]
        );
        assert_eq!(
            topology.lanes["delivery"].sources,
            vec!["story.backlog".to_string(), "story.in-progress".to_string()]
        );
    }

    #[test]
    fn workflow_topology_resolve_requires_custom_defaults_to_be_declared() {
        let mut config = Config::default();
        config.workflow.defaults.management_role = "director".to_string();

        let error = resolve(&config).unwrap_err().to_string();
        assert!(error.contains("director"));
    }

    #[test]
    fn doctor_topology_fails_on_overlap() {
        let mut config = Config::default();
        config.lanes.insert(
            "lane-a".to_string(),
            LaneConfig {
                description: "A".to_string(),
                include: vec!["story.backlog".to_string()],
                exclude: Vec::new(),
                parallel: true,
                manual_accept: false,
                priority: 50,
            },
        );
        config.lanes.insert(
            "lane-b".to_string(),
            LaneConfig {
                description: "B".to_string(),
                include: vec!["story.backlog".to_string()],
                exclude: Vec::new(),
                parallel: true,
                manual_accept: false,
                priority: 40,
            },
        );

        let error = resolve(&config).unwrap_err().to_string();
        assert!(error.contains("story.backlog"));
        assert!(error.contains("lane-a"));
        assert!(error.contains("lane-b"));
    }

    #[test]
    fn workflow_topology_selector_errors_fails_on_unknown() {
        let mut config = Config::default();
        config.lanes.insert(
            "delivery".to_string(),
            LaneConfig {
                description: "Broken".to_string(),
                include: vec!["story.not-real".to_string()],
                exclude: Vec::new(),
                parallel: true,
                manual_accept: false,
                priority: 50,
            },
        );

        let error = resolve(&config).unwrap_err().to_string();
        assert!(error.contains("story.not-real"));
    }
}
