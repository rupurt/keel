//! Board-wide topology projection for the zoomable world map.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

use anyhow::{Result, anyhow};
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::domain::model::{Board, Epic, Mission, Story, Voyage};
use crate::infrastructure::utils::{cmp_optional_index_then_id, pluralize};
use crate::read_model::board_graph::{
    BoardEdgeKind, BoardGraph, BoardGraphNode, BoardNodeId, BoardNodeKind, build_board_graph,
};
use crate::read_model::knowledge_graph::{DriftSurfaceSummary, project_structural_drift_summary};
use crate::read_model::planning_show;
use crate::read_model::scheduled_routines::{
    RoutineScheduleFilter, ScheduledRoutineProjection, ScheduledRoutineState,
    project_scheduled_routines,
};

const WORLD_NODE_ID: &str = "__world__";
const HIGHLIGHT_LIMIT: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TopologyZoom {
    #[default]
    World,
    Mission,
    Epic,
    Voyage,
    Story,
}

impl TopologyZoom {
    pub fn zoom_in(self) -> Self {
        match self {
            Self::World => Self::Mission,
            Self::Mission => Self::Epic,
            Self::Epic => Self::Voyage,
            Self::Voyage => Self::Story,
            Self::Story => Self::Story,
        }
    }

    pub fn zoom_out(self) -> Self {
        match self {
            Self::World => Self::World,
            Self::Mission => Self::World,
            Self::Epic => Self::Mission,
            Self::Voyage => Self::Epic,
            Self::Story => Self::Voyage,
        }
    }

    pub fn max_depth(self) -> usize {
        match self {
            Self::World => 1,
            Self::Mission => 2,
            Self::Epic => 3,
            Self::Voyage => 4,
            Self::Story => 5,
        }
    }

    pub fn layer_name(self) -> &'static str {
        match self {
            Self::World => "world",
            Self::Mission => "mission",
            Self::Epic => "epic",
            Self::Voyage => "voyage",
            Self::Story => "story",
        }
    }
}

impl fmt::Display for TopologyZoom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.layer_name())
    }
}

impl FromStr for TopologyZoom {
    type Err = String;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "world" => Ok(Self::World),
            "mission" => Ok(Self::Mission),
            "epic" => Ok(Self::Epic),
            "voyage" => Ok(Self::Voyage),
            "story" => Ok(Self::Story),
            other => Err(format!("unsupported topology zoom: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldMapNodeKind {
    World,
    Mission,
    Epic,
    Bearing,
    Adr,
    Voyage,
    Story,
    Routine,
    Watch,
    Heartbeat,
}

impl WorldMapNodeKind {
    pub fn singular_label(self) -> &'static str {
        match self {
            Self::World => "world",
            Self::Mission => "mission",
            Self::Epic => "epic",
            Self::Bearing => "bearing",
            Self::Adr => "ADR",
            Self::Voyage => "voyage",
            Self::Story => "story",
            Self::Routine => "routine",
            Self::Watch => "watch",
            Self::Heartbeat => "pacemaker",
        }
    }

    pub fn plural_label(self) -> &'static str {
        match self {
            Self::World => "worlds",
            Self::Mission => "missions",
            Self::Epic => "epics",
            Self::Bearing => "bearings",
            Self::Adr => "ADRs",
            Self::Voyage => "voyages",
            Self::Story => "stories",
            Self::Routine => "routines",
            Self::Watch => "watches",
            Self::Heartbeat => "pacemakers",
        }
    }

    fn sort_rank(self) -> u8 {
        match self {
            Self::World => 0,
            Self::Mission => 1,
            Self::Epic => 2,
            Self::Bearing => 3,
            Self::Adr => 4,
            Self::Voyage => 5,
            Self::Story => 6,
            Self::Routine => 7,
            Self::Watch => 8,
            Self::Heartbeat => 9,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldMapLinkKind {
    Hierarchy,
    Dependency,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldMapLink {
    pub from_id: String,
    pub to_id: String,
    pub kind: WorldMapLinkKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldMapFocus {
    pub id: String,
    pub title: String,
    pub kind: WorldMapNodeKind,
    pub state: String,
    pub timer: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldMapNode {
    pub id: String,
    pub title: String,
    pub kind: WorldMapNodeKind,
    pub state: String,
    pub parent_id: Option<String>,
    pub depth: usize,
    pub terminal: bool,
    pub order_index: Option<u32>,
    pub summary: Option<String>,
    pub timer: Option<String>,
    pub signals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldMapKindCount {
    pub kind: WorldMapNodeKind,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldMapLayer {
    pub depth: usize,
    pub label: String,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorldMapProjection {
    pub zoom: TopologyZoom,
    pub focus: Option<WorldMapFocus>,
    pub nodes: Vec<WorldMapNode>,
    pub links: Vec<WorldMapLink>,
    pub kind_counts: Vec<WorldMapKindCount>,
    pub layers: Vec<WorldMapLayer>,
    pub highlights: Vec<String>,
    pub drift: DriftSurfaceSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorldMapBuildOptions<'a> {
    pub zoom: TopologyZoom,
    pub focus_id: Option<&'a str>,
    pub include_done: bool,
    pub reference_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VisibilityWindow {
    include_done: bool,
    frontier_depth: usize,
    frontier_has_nonterminal: bool,
}

pub fn build_world_map_projection(
    board: &Board,
    options: WorldMapBuildOptions<'_>,
) -> Result<WorldMapProjection> {
    build_world_map_projection_with_builder(board, options, build_board_graph)
}

fn build_world_map_projection_with_builder<F>(
    board: &Board,
    options: WorldMapBuildOptions<'_>,
    build: F,
) -> Result<WorldMapProjection>
where
    F: FnOnce(&Board) -> BoardGraph,
{
    let reference_time = options
        .reference_time
        .unwrap_or_else(|| Utc::now().naive_utc());
    let schedule_reference = DateTime::<Utc>::from_naive_utc_and_offset(reference_time, Utc);
    let scheduled_routines =
        project_scheduled_routines(board, schedule_reference, RoutineScheduleFilter::default())
            .into_iter()
            .map(|routine| (routine.id.clone(), routine))
            .collect::<HashMap<_, _>>();
    let graph = build(board);
    let mut nodes = BTreeMap::new();
    let id_index: HashMap<_, _> = graph
        .nodes()
        .iter()
        .map(|node| (world_node_id(&node.id), node.id.clone()))
        .collect();

    for node in graph.nodes() {
        let world_id = world_node_id(&node.id);
        nodes.insert(
            world_id,
            build_world_map_node(board, &graph, &scheduled_routines, node, reference_time)?,
        );
    }

    let child_map = build_child_map(&nodes);
    let focus = resolve_focus(&nodes, options.focus_id)?;
    let focus_ids = focus
        .as_ref()
        .map(|focus| focus_related_ids(focus, &graph, &id_index))
        .unwrap_or_default();

    let depth_limited_ids: HashSet<_> = nodes
        .values()
        .filter(|node| node.depth <= options.zoom.max_depth())
        .filter(|node| focus_ids.is_empty() || focus_ids.contains(&node.id))
        .map(|node| node.id.clone())
        .collect();
    let frontier_depth = depth_limited_ids
        .iter()
        .filter_map(|id| nodes.get(id).map(|node| node.depth))
        .max()
        .unwrap_or(0);
    let frontier_has_nonterminal = depth_limited_ids
        .iter()
        .filter_map(|id| nodes.get(id))
        .filter(|node| node.depth == frontier_depth)
        .any(|node| !node.terminal);
    let visibility = VisibilityWindow {
        include_done: options.include_done,
        frontier_depth,
        frontier_has_nonterminal,
    };

    let visible_ids: HashSet<_> = depth_limited_ids
        .iter()
        .filter(|id| {
            should_show_node(
                id,
                &nodes,
                &child_map,
                &depth_limited_ids,
                &focus_ids,
                visibility,
            )
        })
        .cloned()
        .collect();

    let mut visible_nodes: Vec<_> = nodes
        .into_values()
        .filter(|node| visible_ids.contains(&node.id))
        .collect();
    visible_nodes.sort_by(compare_nodes);

    let kind_counts = visible_kind_counts(&visible_nodes);
    let layers = visible_layers(&visible_nodes);
    let links = visible_links(&graph, &visible_ids, options.zoom);
    let highlights = highlight_lines(&visible_nodes, &focus);
    let drift = project_structural_drift_summary(board)?;

    Ok(WorldMapProjection {
        zoom: options.zoom,
        focus,
        nodes: visible_nodes,
        links,
        kind_counts,
        layers,
        highlights,
        drift,
    })
}

fn build_world_map_node(
    board: &Board,
    graph: &BoardGraph,
    scheduled_routines: &HashMap<String, ScheduledRoutineProjection>,
    node: &BoardGraphNode,
    reference_time: NaiveDateTime,
) -> Result<WorldMapNode> {
    let id = world_node_id(&node.id);
    let parent_id = graph.parent(&node.id).map(world_node_id);
    let depth = match node.id {
        BoardNodeId::Board => 0,
        _ if parent_id.is_some() => graph.ancestors(&node.id).len(),
        _ => 1,
    };

    let (title, kind, state, summary, timer, signals) = match &node.id {
        BoardNodeId::Board => (
            "Keel World".to_string(),
            WorldMapNodeKind::World,
            node.state.clone(),
            Some(format!(
                "{} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}",
                board.missions.len(),
                pluralize(board.missions.len(), "mission", "missions"),
                board.epics.len(),
                pluralize(board.epics.len(), "epic", "epics"),
                board.bearings.len(),
                pluralize(board.bearings.len(), "bearing", "bearings"),
                board.adrs.len(),
                pluralize(board.adrs.len(), "ADR", "ADRs"),
                board.voyages.len(),
                pluralize(board.voyages.len(), "voyage", "voyages"),
                board.stories.len(),
                pluralize(board.stories.len(), "story", "stories"),
                board.routines.len(),
                pluralize(board.routines.len(), "routine", "routines"),
            )),
            None,
            Vec::new(),
        ),
        BoardNodeId::Mission(id) => {
            let mission = board
                .missions
                .get(id)
                .expect("graph mission nodes must resolve against the board");
            (
                node.title.clone(),
                world_node_kind(node.kind),
                node.state.clone(),
                Some(mission_summary(board, mission)),
                mission_timer(mission, reference_time),
                Vec::new(),
            )
        }
        BoardNodeId::Epic(id) => {
            let epic = board
                .epics
                .get(id)
                .expect("graph epic nodes must resolve against the board");
            let (open_voyages, total_voyages) = epic_voyage_counts(board, epic);
            (
                node.title.clone(),
                world_node_kind(node.kind),
                node.state.clone(),
                Some(format!(
                    "{open_voyages}/{total_voyages} open {}",
                    pluralize(total_voyages, "voyage", "voyages")
                )),
                epic_timer(board, epic, reference_time),
                Vec::new(),
            )
        }
        BoardNodeId::Bearing(_) | BoardNodeId::Adr(_) => (
            node.title.clone(),
            world_node_kind(node.kind),
            node.state.clone(),
            None,
            None,
            Vec::new(),
        ),
        BoardNodeId::Voyage(id) => {
            let voyage = board
                .voyages
                .get(id)
                .expect("graph voyage nodes must resolve against the board");
            let (open_stories, total_stories) = voyage_story_counts(board, voyage);
            (
                node.title.clone(),
                world_node_kind(node.kind),
                node.state.clone(),
                Some(format!(
                    "{open_stories}/{total_stories} open {}",
                    pluralize(total_stories, "story", "stories")
                )),
                voyage_timer(voyage, reference_time),
                Vec::new(),
            )
        }
        BoardNodeId::Story(id) => {
            let story = board
                .stories
                .get(id)
                .expect("graph story nodes must resolve against the board");
            (
                node.title.clone(),
                world_node_kind(node.kind),
                node.state.clone(),
                Some(story_scope_summary(story)),
                None,
                story_signals(graph, story)?,
            )
        }
        BoardNodeId::Routine(id) => {
            let routine = board
                .routines
                .get(id)
                .expect("graph routine nodes must resolve against the board");
            let scheduled = scheduled_routines.get(id);
            (
                node.title.clone(),
                world_node_kind(node.kind),
                routine_state_label(scheduled),
                Some(format!("targets {}", routine.target_scope())),
                routine_timer(scheduled),
                routine_signals(scheduled),
            )
        }
        BoardNodeId::Watch(id) => {
            let watch = board
                .watches
                .get(id)
                .expect("graph watch nodes must resolve against the board");
            (
                node.title.clone(),
                world_node_kind(node.kind),
                node.state.clone(),
                Some(format!("{}h limit", watch.limit_hours())),
                None,
                Vec::new(),
            )
        }
        BoardNodeId::Heartbeat => (
            node.title.clone(),
            WorldMapNodeKind::Heartbeat,
            node.state.clone(),
            Some("System pacemaker".to_string()),
            None,
            Vec::new(),
        ),
    };

    Ok(WorldMapNode {
        id,
        title,
        kind,
        state,
        parent_id,
        depth,
        terminal: node.terminal,
        order_index: node.order_index,
        summary,
        timer,
        signals,
    })
}

fn world_node_id(id: &BoardNodeId) -> String {
    match id {
        BoardNodeId::Board => WORLD_NODE_ID.to_string(),
        BoardNodeId::Heartbeat => "pacemaker".to_string(),
        BoardNodeId::Mission(id)
        | BoardNodeId::Epic(id)
        | BoardNodeId::Bearing(id)
        | BoardNodeId::Adr(id)
        | BoardNodeId::Voyage(id)
        | BoardNodeId::Story(id)
        | BoardNodeId::Routine(id)
        | BoardNodeId::Watch(id) => id.clone(),
    }
}

fn world_node_kind(kind: BoardNodeKind) -> WorldMapNodeKind {
    match kind {
        BoardNodeKind::Board => WorldMapNodeKind::World,
        BoardNodeKind::Mission => WorldMapNodeKind::Mission,
        BoardNodeKind::Epic => WorldMapNodeKind::Epic,
        BoardNodeKind::Bearing => WorldMapNodeKind::Bearing,
        BoardNodeKind::Adr => WorldMapNodeKind::Adr,
        BoardNodeKind::Voyage => WorldMapNodeKind::Voyage,
        BoardNodeKind::Story => WorldMapNodeKind::Story,
        BoardNodeKind::Routine => WorldMapNodeKind::Routine,
        BoardNodeKind::Watch => WorldMapNodeKind::Watch,
        BoardNodeKind::Heartbeat => WorldMapNodeKind::Heartbeat,
    }
}

fn mission_timer(mission: &Mission, reference_time: NaiveDateTime) -> Option<String> {
    let start = mission
        .frontmatter
        .activated_at
        .or(mission.frontmatter.created_at)?;
    let end = mission
        .frontmatter
        .verified_at
        .or(mission.frontmatter.achieved_at)
        .unwrap_or(reference_time);
    compact_elapsed(start, end)
}

fn epic_timer(board: &Board, epic: &Epic, reference_time: NaiveDateTime) -> Option<String> {
    let start = board
        .voyages_for_epic_id(epic.id())
        .into_iter()
        .filter_map(|voyage| voyage.frontmatter.started_at)
        .min()
        .or(epic.frontmatter.created_at)?;
    let end = if epic.status().to_string() == "done" {
        board
            .voyages_for_epic_id(epic.id())
            .into_iter()
            .filter_map(|voyage| voyage.frontmatter.completed_at)
            .max()
            .unwrap_or(reference_time)
    } else {
        reference_time
    };
    compact_elapsed(start, end)
}

fn voyage_timer(voyage: &Voyage, reference_time: NaiveDateTime) -> Option<String> {
    let start = voyage
        .frontmatter
        .started_at
        .or(voyage.frontmatter.created_at)?;
    let end = voyage.frontmatter.completed_at.unwrap_or(reference_time);
    compact_elapsed(start, end)
}

fn routine_state_label(scheduled: Option<&ScheduledRoutineProjection>) -> String {
    match scheduled.map(|routine| routine.state) {
        Some(ScheduledRoutineState::Due) => "due".to_string(),
        Some(ScheduledRoutineState::Upcoming) => "upcoming".to_string(),
        Some(ScheduledRoutineState::Invalid) => "invalid".to_string(),
        None => "scheduled".to_string(),
    }
}

fn routine_timer(scheduled: Option<&ScheduledRoutineProjection>) -> Option<String> {
    match scheduled {
        Some(routine) => match routine.state {
            ScheduledRoutineState::Due => Some("due now".to_string()),
            ScheduledRoutineState::Upcoming => routine.countdown.clone(),
            ScheduledRoutineState::Invalid => Some("invalid cadence".to_string()),
        },
        None => None,
    }
}

fn routine_signals(scheduled: Option<&ScheduledRoutineProjection>) -> Vec<String> {
    match scheduled {
        Some(routine) => match routine.state {
            ScheduledRoutineState::Due => vec!["due now".to_string()],
            ScheduledRoutineState::Invalid => vec![
                routine
                    .error
                    .clone()
                    .unwrap_or_else(|| "invalid cadence".to_string()),
            ],
            ScheduledRoutineState::Upcoming => Vec::new(),
        },
        None => Vec::new(),
    }
}

fn compact_elapsed(start: NaiveDateTime, end: NaiveDateTime) -> Option<String> {
    (end >= start).then(|| format_elapsed_duration((end - start).num_seconds()))
}

fn format_elapsed_duration(total_seconds: i64) -> String {
    if total_seconds < 60 {
        return "<1m".to_string();
    }

    let mut seconds = total_seconds;
    let days = seconds / 86_400;
    seconds %= 86_400;
    let hours = seconds / 3_600;
    seconds %= 3_600;
    let minutes = seconds / 60;

    if days > 0 {
        format!("{days}d {hours}h")
    } else if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}

fn mission_summary(board: &Board, mission: &Mission) -> String {
    let epics = board.epics_for_mission(mission.id());
    let bearings = board.bearings_for_mission(mission.id());
    let adrs = board.adrs_for_mission(mission.id());
    let total = epics.len() + bearings.len() + adrs.len();
    let open = epics
        .iter()
        .filter(|epic| epic.status().to_string() != "done")
        .count()
        + bearings
            .iter()
            .filter(|bearing| !bearing.is_complete())
            .count()
        + adrs
            .iter()
            .filter(|adr| !adr.status().is_terminal())
            .count();

    format!(
        "{open}/{total} open strategic {}",
        pluralize(total, "entity", "entities")
    )
}

fn epic_voyage_counts(board: &Board, epic: &Epic) -> (usize, usize) {
    let voyages = board.voyages_for_epic(epic);
    let total = voyages.len();
    let open = voyages
        .iter()
        .filter(|voyage| voyage.status().to_string() != "done")
        .count();
    (open, total)
}

fn voyage_story_counts(board: &Board, voyage: &Voyage) -> (usize, usize) {
    let stories = board.stories_for_voyage(voyage);
    let total = stories.len();
    let open = stories
        .iter()
        .filter(|story| !story.status().is_terminal())
        .count();
    (open, total)
}

fn story_signals(graph: &BoardGraph, story: &Story) -> Result<Vec<String>> {
    let mut signals = Vec::new();
    let show = planning_show::build_story_show_projection(story)?;

    let story_id = BoardNodeId::Story(story.id().to_string());
    let unmet: Vec<_> = graph
        .dependencies(&story_id)
        .iter()
        .filter_map(|dependency_id| {
            let dependency = graph.node(dependency_id)?;
            (!dependency.terminal).then(|| world_node_id(dependency_id))
        })
        .collect();
    if !unmet.is_empty() {
        signals.push(format!("blocked by {}", unmet.join(", ")));
    }

    if !show.evidence.missing_proofs.is_empty() {
        signals.push(format!(
            "missing proof {}",
            show.evidence.missing_proofs.join(", ")
        ));
    }

    if show.evidence.items.is_empty() {
        signals.push("no verification coverage".to_string());
    }

    Ok(signals)
}

fn story_scope_summary(story: &Story) -> String {
    story
        .frontmatter
        .scope
        .as_deref()
        .map(|scope| format!("scope {scope}"))
        .unwrap_or_else(|| "unscoped".to_string())
}

fn resolve_focus(
    nodes: &BTreeMap<String, WorldMapNode>,
    focus_id: Option<&str>,
) -> Result<Option<WorldMapFocus>> {
    let Some(focus_id) = focus_id else {
        return Ok(None);
    };
    let Some(node) = nodes.get(focus_id) else {
        return Err(anyhow!(
            "Topology focus '{focus_id}' was not found on the board."
        ));
    };

    Ok(Some(WorldMapFocus {
        id: node.id.clone(),
        title: node.title.clone(),
        kind: node.kind,
        state: node.state.clone(),
        timer: node.timer.clone(),
    }))
}

fn focus_related_ids(
    focus: &WorldMapFocus,
    graph: &BoardGraph,
    id_index: &HashMap<String, BoardNodeId>,
) -> HashSet<String> {
    let mut ids = HashSet::new();
    ids.insert(WORLD_NODE_ID.to_string());
    ids.insert(focus.id.clone());

    let Some(focus_id) = id_index.get(&focus.id) else {
        return ids;
    };

    ids.extend(
        graph
            .ancestors(focus_id)
            .into_iter()
            .map(|id| world_node_id(&id)),
    );
    ids.extend(
        graph
            .descendants(focus_id)
            .into_iter()
            .map(|id| world_node_id(&id)),
    );

    if focus.kind == WorldMapNodeKind::Story {
        ids.extend(graph.dependencies(focus_id).iter().map(world_node_id));
        ids.extend(
            graph
                .incoming(focus_id, BoardEdgeKind::DependsOn)
                .iter()
                .map(world_node_id),
        );
    }

    ids
}

fn should_show_node(
    id: &str,
    nodes: &BTreeMap<String, WorldMapNode>,
    child_map: &HashMap<String, Vec<String>>,
    depth_limited_ids: &HashSet<String>,
    focus_ids: &HashSet<String>,
    visibility: VisibilityWindow,
) -> bool {
    let node = nodes
        .get(id)
        .expect("depth-limited ids must reference known nodes");
    if node.id == WORLD_NODE_ID || visibility.include_done || node.depth <= 1 || !node.terminal {
        return true;
    }
    if node.depth == visibility.frontier_depth && !visibility.frontier_has_nonterminal {
        return true;
    }
    if focus_ids.contains(&node.id) {
        return true;
    }

    child_map
        .get(id)
        .into_iter()
        .flatten()
        .filter(|child_id| depth_limited_ids.contains(*child_id))
        .any(|child_id| {
            should_show_node(
                child_id,
                nodes,
                child_map,
                depth_limited_ids,
                focus_ids,
                visibility,
            )
        })
}

fn build_child_map(nodes: &BTreeMap<String, WorldMapNode>) -> HashMap<String, Vec<String>> {
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    for node in nodes.values() {
        if let Some(parent_id) = &node.parent_id {
            children
                .entry(parent_id.clone())
                .or_default()
                .push(node.id.clone());
        }
    }

    for ids in children.values_mut() {
        ids.sort_by(|left, right| {
            let left_node = nodes.get(left).expect("child ids must resolve");
            let right_node = nodes.get(right).expect("child ids must resolve");
            compare_nodes(left_node, right_node)
        });
    }

    children
}

fn visible_kind_counts(nodes: &[WorldMapNode]) -> Vec<WorldMapKindCount> {
    let mut counts: BTreeMap<u8, (WorldMapNodeKind, usize)> = BTreeMap::new();
    for node in nodes
        .iter()
        .filter(|node| node.kind != WorldMapNodeKind::World)
    {
        let entry = counts
            .entry(node.kind.sort_rank())
            .or_insert((node.kind, 0));
        entry.1 += 1;
    }

    counts
        .into_values()
        .map(|(kind, count)| WorldMapKindCount { kind, count })
        .collect()
}

fn visible_layers(nodes: &[WorldMapNode]) -> Vec<WorldMapLayer> {
    let mut layers: BTreeMap<usize, BTreeSet<&'static str>> = BTreeMap::new();
    let mut counts: BTreeMap<usize, usize> = BTreeMap::new();

    for node in nodes
        .iter()
        .filter(|node| node.kind != WorldMapNodeKind::World)
    {
        layers
            .entry(node.depth)
            .or_default()
            .insert(node.kind.plural_label());
        *counts.entry(node.depth).or_default() += 1;
    }

    layers
        .into_iter()
        .map(|(depth, labels)| WorldMapLayer {
            depth,
            label: labels.into_iter().collect::<Vec<_>>().join(" + "),
            count: counts.get(&depth).copied().unwrap_or(0),
        })
        .collect()
}

fn visible_links(
    graph: &BoardGraph,
    visible_ids: &HashSet<String>,
    zoom: TopologyZoom,
) -> Vec<WorldMapLink> {
    let mut links = Vec::new();

    for edge in graph.edges() {
        match edge.kind {
            BoardEdgeKind::Contains => {
                let from_id = world_node_id(&edge.from);
                let to_id = world_node_id(&edge.to);
                if visible_ids.contains(&from_id) && visible_ids.contains(&to_id) {
                    links.push(WorldMapLink {
                        from_id,
                        to_id,
                        kind: WorldMapLinkKind::Hierarchy,
                    });
                }
            }
            BoardEdgeKind::DependsOn if zoom == TopologyZoom::Story => {
                let from_id = world_node_id(&edge.to);
                let to_id = world_node_id(&edge.from);
                if visible_ids.contains(&from_id) && visible_ids.contains(&to_id) {
                    links.push(WorldMapLink {
                        from_id,
                        to_id,
                        kind: WorldMapLinkKind::Dependency,
                    });
                }
            }
            _ => {}
        }
    }

    links.sort_by(|left, right| {
        let left_kind = match left.kind {
            WorldMapLinkKind::Hierarchy => 0,
            WorldMapLinkKind::Dependency => 1,
        };
        let right_kind = match right.kind {
            WorldMapLinkKind::Hierarchy => 0,
            WorldMapLinkKind::Dependency => 1,
        };
        left_kind
            .cmp(&right_kind)
            .then_with(|| left.to_id.cmp(&right.to_id))
            .then_with(|| left.from_id.cmp(&right.from_id))
    });
    links.dedup();

    links
}

fn highlight_lines(nodes: &[WorldMapNode], focus: &Option<WorldMapFocus>) -> Vec<String> {
    let mut highlights = Vec::new();

    if let Some(focus) = focus {
        let mut detail = format!(
            "focus {} {} ({})",
            focus.kind.singular_label(),
            focus.title,
            focus.state
        );
        if let Some(timer) = &focus.timer {
            detail.push_str(&format!(" · {timer}"));
        }
        highlights.push(detail);
    }

    let mut candidates: Vec<_> = nodes
        .iter()
        .filter(|node| node.kind != WorldMapNodeKind::World)
        .filter(|node| node.depth <= 2 || !node.signals.is_empty())
        .collect();
    candidates.sort_by(|left, right| {
        left.depth
            .cmp(&right.depth)
            .then_with(|| {
                (!left.signals.is_empty())
                    .cmp(&!right.signals.is_empty())
                    .reverse()
            })
            .then_with(|| compare_nodes(left, right))
    });

    for node in candidates {
        if highlights.len() >= HIGHLIGHT_LIMIT {
            break;
        }

        let mut detail = format!(
            "{} {} ({})",
            node.kind.singular_label(),
            node.title,
            node.state
        );
        if let Some(timer) = &node.timer {
            detail.push_str(&format!(" · {timer}"));
        }
        if let Some(summary) = &node.summary {
            detail.push_str(&format!(" · {summary}"));
        }
        if let Some(signal) = node.signals.first() {
            detail.push_str(&format!(" · {signal}"));
        }

        if !highlights.iter().any(|line| line == &detail) {
            highlights.push(detail);
        }
    }

    highlights
}

fn compare_nodes(left: &WorldMapNode, right: &WorldMapNode) -> std::cmp::Ordering {
    left.depth
        .cmp(&right.depth)
        .then_with(|| left.kind.sort_rank().cmp(&right.kind.sort_rank()))
        .then_with(|| {
            cmp_optional_index_then_id(left.order_index, &left.id, right.order_index, &right.id)
        })
        .then_with(|| left.title.cmp(&right.title))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use std::cell::Cell;
    use std::fs;
    use std::path::Path;

    use crate::domain::model::StoryState;
    use crate::test_helpers::{
        TestBearing, TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage,
    };

    fn write_routine(root: &Path, id: &str, target_scope: &str, cadence_block: &str) {
        let routine_dir = root.join("routines").join(id);
        fs::create_dir_all(&routine_dir).unwrap();
        fs::write(
            routine_dir.join("README.md"),
            format!(
                r#"---
id: {id}
title: {id}
cadence:
{cadence_block}
target-scope: {target_scope}
created_at: 2026-01-01T00:00:00
updated_at: 2026-01-01T00:00:00
---

# Blueprint
"#
            ),
        )
        .unwrap();
    }

    fn world_fixture() -> tempfile::TempDir {
        TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One").status("active"))
            .mission(
                TestMission::new("M2")
                    .title("Mission Two")
                    .status("verified"),
            )
            .epic(TestEpic::new("E1").title("Epic One").mission("M1").index(1))
            .epic(
                TestEpic::new("E2")
                    .title("Completed Epic")
                    .mission("M2")
                    .index(1),
            )
            .voyage(
                TestVoyage::new("V1", "E1")
                    .title("Voyage One")
                    .status("planned")
                    .index(1),
            )
            .voyage(
                TestVoyage::new("V2", "E2")
                    .title("Done Voyage")
                    .status("done")
                    .index(1),
            )
            .story(
                TestStory::new("S1")
                    .title("Story One")
                    .scope("E1/V1")
                    .index(1)
                    .status(StoryState::Backlog)
                    .body("## Acceptance Criteria\n\n- [ ] ship the first slice"),
            )
            .story(
                TestStory::new("S2")
                    .title("Blocked Story")
                    .scope("E1/V1")
                    .index(2)
                    .status(StoryState::Backlog)
                    .blocked_by(&["S1"])
                    .body("## Acceptance Criteria\n\n- [ ] ship the blocked slice"),
            )
            .bearing(
                TestBearing::new("B1")
                    .title("Payments Research")
                    .status("ready")
                    .has_evidence(true)
                    .has_assessment(true),
            )
            .build()
    }

    #[test]
    fn world_map_uses_board_graph_relationships() {
        let temp = world_fixture();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let projection = build_world_map_projection(
            &board,
            WorldMapBuildOptions {
                zoom: TopologyZoom::Story,
                focus_id: None,
                include_done: false,
                reference_time: None,
            },
        )
        .unwrap();

        assert!(projection.links.iter().any(|link| {
            link.kind == WorldMapLinkKind::Hierarchy && link.from_id == "M1" && link.to_id == "E1"
        }));
        assert!(projection.links.iter().any(|link| {
            link.kind == WorldMapLinkKind::Hierarchy && link.from_id == "V1" && link.to_id == "S2"
        }));
        assert!(
            projection
                .links
                .iter()
                .any(|link| link.kind == WorldMapLinkKind::Dependency
                    && link.from_id == "S1"
                    && link.to_id == "S2")
        );

        let blocked_story = projection
            .nodes
            .iter()
            .find(|node| node.id == "S2")
            .expect("blocked story should be visible");
        assert!(
            blocked_story
                .signals
                .iter()
                .any(|signal| signal.contains("blocked by S1"))
        );
    }

    #[test]
    fn story_zoom_surfaces_targeted_routines_with_due_state() {
        let temp = world_fixture();
        write_routine(
            temp.path(),
            "routine-weekly-review",
            "E1/V1",
            "  cron: 0 9 * * 1\n  timezone: America/Los_Angeles",
        );
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let projection = build_world_map_projection(
            &board,
            WorldMapBuildOptions {
                zoom: TopologyZoom::Story,
                focus_id: Some("V1"),
                include_done: false,
                reference_time: Some(
                    chrono::Utc
                        .with_ymd_and_hms(2026, 1, 5, 18, 0, 0)
                        .unwrap()
                        .naive_utc(),
                ),
            },
        )
        .unwrap();

        let routine = projection
            .nodes
            .iter()
            .find(|node| node.id == "routine-weekly-review")
            .expect("routine should be visible on the story zoom");

        assert_eq!(routine.kind, WorldMapNodeKind::Routine);
        assert_eq!(routine.parent_id.as_deref(), Some("V1"));
        assert_eq!(routine.state, "due");
        assert_eq!(routine.timer.as_deref(), Some("due now"));
        assert!(
            routine
                .signals
                .iter()
                .any(|signal| signal.contains("due now"))
        );
        assert!(projection.links.iter().any(|link| {
            link.kind == WorldMapLinkKind::Hierarchy
                && link.from_id == "V1"
                && link.to_id == "routine-weekly-review"
        }));
    }

    #[test]
    fn world_map_board_graph_preserves_behavior() {
        let temp = world_fixture();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let world = build_world_map_projection(
            &board,
            WorldMapBuildOptions {
                zoom: TopologyZoom::World,
                focus_id: None,
                include_done: false,
                reference_time: None,
            },
        )
        .unwrap();
        let world_ids: Vec<_> = world.nodes.iter().map(|node| node.id.as_str()).collect();
        assert!(world_ids.contains(&WORLD_NODE_ID));
        assert!(world_ids.contains(&"M1"));
        assert!(world_ids.contains(&"M2"));
        assert!(world_ids.contains(&"B1"));
        assert!(!world_ids.contains(&"E1"));
        assert!(!world_ids.contains(&"V1"));
        assert!(!world_ids.contains(&"S1"));

        let mission = build_world_map_projection(
            &board,
            WorldMapBuildOptions {
                zoom: TopologyZoom::Mission,
                focus_id: Some("M1"),
                include_done: false,
                reference_time: None,
            },
        )
        .unwrap();
        let mission_ids: Vec<_> = mission.nodes.iter().map(|node| node.id.as_str()).collect();
        assert!(mission_ids.contains(&"M1"));
        assert!(mission_ids.contains(&"E1"));
        assert!(!mission_ids.contains(&"V1"));
        assert!(!mission_ids.contains(&"M2"));
        assert!(!mission_ids.contains(&"B1"));
        assert_eq!(
            mission.focus.as_ref().map(|focus| focus.id.as_str()),
            Some("M1")
        );
    }

    #[test]
    fn world_map_board_graph_is_canonical_path() {
        let temp = world_fixture();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let build_count = Cell::new(0);

        let projection = build_world_map_projection_with_builder(
            &board,
            WorldMapBuildOptions {
                zoom: TopologyZoom::Story,
                focus_id: Some("S2"),
                include_done: false,
                reference_time: None,
            },
            |board| {
                build_count.set(build_count.get() + 1);
                build_board_graph(board)
            },
        )
        .unwrap();

        assert_eq!(build_count.get(), 1);
        assert!(
            projection
                .links
                .iter()
                .any(|link| link.kind == WorldMapLinkKind::Dependency
                    && link.from_id == "S1"
                    && link.to_id == "S2")
        );
    }

    #[test]
    fn story_zoom_reveals_terminal_frontier_when_no_open_lower_layer_exists() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One").status("active"))
            .epic(TestEpic::new("E1").title("Epic One").mission("M1").index(1))
            .voyage(
                TestVoyage::new("V1", "E1")
                    .title("Done Voyage")
                    .status("done")
                    .index(1),
            )
            .story(
                TestStory::new("S1")
                    .title("Done Story")
                    .scope("E1/V1")
                    .index(1)
                    .status(StoryState::Done)
                    .body("## Acceptance Criteria\n\n- [x] ship the finished slice"),
            )
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let projection = build_world_map_projection(
            &board,
            WorldMapBuildOptions {
                zoom: TopologyZoom::Story,
                focus_id: None,
                include_done: false,
                reference_time: None,
            },
        )
        .unwrap();

        let visible_ids: Vec<_> = projection
            .nodes
            .iter()
            .map(|node| node.id.as_str())
            .collect();
        assert!(visible_ids.contains(&"V1"));
        assert!(visible_ids.contains(&"S1"));
    }

    #[test]
    fn format_elapsed_duration_compacts_days_and_hours() {
        assert_eq!(format_elapsed_duration(45), "<1m");
        assert_eq!(format_elapsed_duration(90 * 60), "1h 30m");
        assert_eq!(format_elapsed_duration(26 * 60 * 60), "1d 2h");
    }
}
