//! Board-wide topology projection for the zoomable world map.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

use anyhow::{Result, anyhow};
use chrono::{NaiveDateTime, Utc};

use crate::domain::model::{Adr, Bearing, Board, Epic, Mission, MissionStatus, Story, Voyage};
use crate::infrastructure::utils::{cmp_optional_index_then_id, pluralize};
use crate::read_model::planning_show;
use crate::read_model::traceability;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldMapProjection {
    pub zoom: TopologyZoom,
    pub focus: Option<WorldMapFocus>,
    pub nodes: Vec<WorldMapNode>,
    pub links: Vec<WorldMapLink>,
    pub kind_counts: Vec<WorldMapKindCount>,
    pub layers: Vec<WorldMapLayer>,
    pub highlights: Vec<String>,
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
    let reference_time = options
        .reference_time
        .unwrap_or_else(|| Utc::now().naive_utc());
    let story_dependencies = story_dependency_map(board);
    let mut nodes = BTreeMap::new();

    nodes.insert(
        WORLD_NODE_ID.to_string(),
        WorldMapNode {
            id: WORLD_NODE_ID.to_string(),
            title: "Keel World".to_string(),
            kind: WorldMapNodeKind::World,
            state: "live".to_string(),
            parent_id: None,
            depth: 0,
            terminal: false,
            order_index: Some(0),
            summary: Some(format!(
                "{} {}, {} {}, {} {}, {} {}, {} {}, {} {}",
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
            )),
            timer: None,
            signals: Vec::new(),
        },
    );

    for mission in sorted_missions(board) {
        nodes.insert(
            mission.id().to_string(),
            WorldMapNode {
                id: mission.id().to_string(),
                title: mission.title().to_string(),
                kind: WorldMapNodeKind::Mission,
                state: mission.status().to_string(),
                parent_id: Some(WORLD_NODE_ID.to_string()),
                depth: 1,
                terminal: mission_terminal(mission.status()),
                order_index: None,
                summary: Some(mission_summary(board, mission)),
                timer: mission_timer(mission, reference_time),
                signals: Vec::new(),
            },
        );
    }

    for epic in sorted_epics(board) {
        let parent_id = epic
            .frontmatter
            .mission
            .clone()
            .unwrap_or_else(|| WORLD_NODE_ID.to_string());
        let parent_depth = nodes.get(&parent_id).map(|node| node.depth).unwrap_or(0);
        let (open_voyages, total_voyages) = epic_voyage_counts(board, epic);
        nodes.insert(
            epic.id().to_string(),
            WorldMapNode {
                id: epic.id().to_string(),
                title: epic.title().to_string(),
                kind: WorldMapNodeKind::Epic,
                state: epic.status().to_string(),
                parent_id: Some(parent_id),
                depth: parent_depth + 1,
                terminal: epic.status().to_string() == "done",
                order_index: epic.index(),
                summary: Some(format!(
                    "{open_voyages}/{total_voyages} open {}",
                    pluralize(total_voyages, "voyage", "voyages")
                )),
                timer: epic_timer(board, epic, reference_time),
                signals: Vec::new(),
            },
        );
    }

    for bearing in sorted_bearings(board) {
        let parent_id = bearing
            .frontmatter
            .mission
            .clone()
            .unwrap_or_else(|| WORLD_NODE_ID.to_string());
        let parent_depth = nodes.get(&parent_id).map(|node| node.depth).unwrap_or(0);
        nodes.insert(
            bearing.id().to_string(),
            WorldMapNode {
                id: bearing.id().to_string(),
                title: bearing.title().to_string(),
                kind: WorldMapNodeKind::Bearing,
                state: bearing.status().to_string(),
                parent_id: Some(parent_id),
                depth: parent_depth + 1,
                terminal: bearing.is_complete(),
                order_index: bearing.frontmatter.index,
                summary: None,
                timer: None,
                signals: Vec::new(),
            },
        );
    }

    for adr in sorted_adrs(board) {
        let parent_id = adr
            .frontmatter
            .mission
            .clone()
            .unwrap_or_else(|| WORLD_NODE_ID.to_string());
        let parent_depth = nodes.get(&parent_id).map(|node| node.depth).unwrap_or(0);
        nodes.insert(
            adr.id().to_string(),
            WorldMapNode {
                id: adr.id().to_string(),
                title: adr.title().to_string(),
                kind: WorldMapNodeKind::Adr,
                state: adr.status().to_string(),
                parent_id: Some(parent_id),
                depth: parent_depth + 1,
                terminal: adr.status().is_terminal(),
                order_index: adr.frontmatter.index,
                summary: None,
                timer: None,
                signals: Vec::new(),
            },
        );
    }

    for voyage in sorted_voyages(board) {
        let parent_id = if nodes.contains_key(&voyage.epic_id) {
            voyage.epic_id.clone()
        } else {
            WORLD_NODE_ID.to_string()
        };
        let parent_depth = nodes.get(&parent_id).map(|node| node.depth).unwrap_or(0);
        let (open_stories, total_stories) = voyage_story_counts(board, voyage);
        nodes.insert(
            voyage.id().to_string(),
            WorldMapNode {
                id: voyage.id().to_string(),
                title: voyage.title().to_string(),
                kind: WorldMapNodeKind::Voyage,
                state: voyage.status().to_string(),
                parent_id: Some(parent_id),
                depth: parent_depth + 1,
                terminal: voyage.status().to_string() == "done",
                order_index: voyage.index(),
                summary: Some(format!(
                    "{open_stories}/{total_stories} open {}",
                    pluralize(total_stories, "story", "stories")
                )),
                timer: voyage_timer(voyage, reference_time),
                signals: Vec::new(),
            },
        );
    }

    for story in sorted_stories(board) {
        let parent_id = story
            .voyage()
            .filter(|voyage_id| nodes.contains_key(*voyage_id))
            .map(ToOwned::to_owned)
            .or_else(|| {
                story
                    .epic()
                    .filter(|epic_id| nodes.contains_key(*epic_id))
                    .map(ToOwned::to_owned)
            })
            .unwrap_or_else(|| WORLD_NODE_ID.to_string());
        let parent_depth = nodes.get(&parent_id).map(|node| node.depth).unwrap_or(0);
        let signals = story_signals(board, story, &story_dependencies)?;
        nodes.insert(
            story.id().to_string(),
            WorldMapNode {
                id: story.id().to_string(),
                title: story.title().to_string(),
                kind: WorldMapNodeKind::Story,
                state: story.status().to_string(),
                parent_id: Some(parent_id),
                depth: parent_depth + 1,
                terminal: story.status().is_terminal(),
                order_index: story.index(),
                summary: Some(story_scope_summary(story)),
                timer: None,
                signals,
            },
        );
    }

    let child_map = build_child_map(&nodes);
    let focus = resolve_focus(&nodes, options.focus_id)?;
    let focus_ids = focus
        .as_ref()
        .map(|focus| focus_related_ids(focus, &nodes, &child_map, &story_dependencies))
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
    let links = visible_links(&visible_nodes, &story_dependencies, options.zoom);
    let highlights = highlight_lines(&visible_nodes, &focus);

    Ok(WorldMapProjection {
        zoom: options.zoom,
        focus,
        nodes: visible_nodes,
        links,
        kind_counts,
        layers,
        highlights,
    })
}

fn mission_terminal(status: MissionStatus) -> bool {
    matches!(status, MissionStatus::Verified | MissionStatus::Abandoned)
}

fn story_dependency_map(board: &Board) -> HashMap<String, Vec<String>> {
    let mut dependencies = traceability::derive_implementation_dependencies(board);
    for story in board.stories.values() {
        if story.frontmatter.blocked_by.is_empty() {
            continue;
        }

        let entry = dependencies.entry(story.id().to_string()).or_default();
        entry.extend(story.frontmatter.blocked_by.iter().cloned());
        entry.sort();
        entry.dedup();
    }

    dependencies
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

fn story_signals(
    board: &Board,
    story: &Story,
    dependencies: &HashMap<String, Vec<String>>,
) -> Result<Vec<String>> {
    let mut signals = Vec::new();
    let show = planning_show::build_story_show_projection(story)?;

    if let Some(required) = dependencies.get(story.id()) {
        let unmet: Vec<_> = required
            .iter()
            .filter_map(|dependency_id| {
                let dependency = board.require_story(dependency_id).ok()?;
                (!dependency.status().is_terminal()).then(|| dependency_id.clone())
            })
            .collect();
        if !unmet.is_empty() {
            signals.push(format!("blocked by {}", unmet.join(", ")));
        }
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
    nodes: &BTreeMap<String, WorldMapNode>,
    child_map: &HashMap<String, Vec<String>>,
    story_dependencies: &HashMap<String, Vec<String>>,
) -> HashSet<String> {
    let mut ids = HashSet::new();
    ids.insert(WORLD_NODE_ID.to_string());
    ids.insert(focus.id.clone());

    let mut cursor = focus.id.as_str();
    while let Some(parent_id) = nodes.get(cursor).and_then(|node| node.parent_id.as_deref()) {
        ids.insert(parent_id.to_string());
        if parent_id == WORLD_NODE_ID {
            break;
        }
        cursor = parent_id;
    }

    collect_descendants(&focus.id, child_map, &mut ids);

    if focus.kind == WorldMapNodeKind::Story {
        if let Some(required) = story_dependencies.get(&focus.id) {
            ids.extend(required.iter().cloned());
        }
        for (story_id, dependencies) in story_dependencies {
            if dependencies
                .iter()
                .any(|dependency| dependency == &focus.id)
            {
                ids.insert(story_id.clone());
            }
        }
    }

    ids
}

fn collect_descendants(
    id: &str,
    child_map: &HashMap<String, Vec<String>>,
    ids: &mut HashSet<String>,
) {
    if let Some(children) = child_map.get(id) {
        for child in children {
            if ids.insert(child.clone()) {
                collect_descendants(child, child_map, ids);
            }
        }
    }
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
    nodes: &[WorldMapNode],
    story_dependencies: &HashMap<String, Vec<String>>,
    zoom: TopologyZoom,
) -> Vec<WorldMapLink> {
    let visible_ids: HashSet<_> = nodes.iter().map(|node| node.id.as_str()).collect();
    let mut links = Vec::new();

    for node in nodes {
        if let Some(parent_id) = &node.parent_id
            && visible_ids.contains(parent_id.as_str())
        {
            links.push(WorldMapLink {
                from_id: parent_id.clone(),
                to_id: node.id.clone(),
                kind: WorldMapLinkKind::Hierarchy,
            });
        }
    }

    if zoom == TopologyZoom::Story {
        let mut dependency_links = Vec::new();
        for node in nodes
            .iter()
            .filter(|node| node.kind == WorldMapNodeKind::Story)
        {
            if let Some(required) = story_dependencies.get(&node.id) {
                for dependency_id in required {
                    if visible_ids.contains(dependency_id.as_str()) {
                        dependency_links.push(WorldMapLink {
                            from_id: dependency_id.clone(),
                            to_id: node.id.clone(),
                            kind: WorldMapLinkKind::Dependency,
                        });
                    }
                }
            }
        }
        dependency_links.sort_by(|left, right| {
            left.to_id
                .cmp(&right.to_id)
                .then_with(|| left.from_id.cmp(&right.from_id))
        });
        links.extend(dependency_links);
    }

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

fn sorted_missions(board: &Board) -> Vec<&Mission> {
    let mut missions: Vec<_> = board.missions.values().collect();
    missions.sort_by(|left, right| left.id().cmp(right.id()));
    missions
}

fn sorted_epics(board: &Board) -> Vec<&Epic> {
    let mut epics: Vec<_> = board.epics.values().collect();
    epics.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    epics
}

fn sorted_bearings(board: &Board) -> Vec<&Bearing> {
    let mut bearings: Vec<_> = board.bearings.values().collect();
    bearings.sort_by(|left, right| {
        cmp_optional_index_then_id(
            left.frontmatter.index,
            left.id(),
            right.frontmatter.index,
            right.id(),
        )
    });
    bearings
}

fn sorted_adrs(board: &Board) -> Vec<&Adr> {
    let mut adrs: Vec<_> = board.adrs.values().collect();
    adrs.sort_by(|left, right| {
        cmp_optional_index_then_id(
            left.frontmatter.index,
            left.id(),
            right.frontmatter.index,
            right.id(),
        )
    });
    adrs
}

fn sorted_voyages(board: &Board) -> Vec<&Voyage> {
    let mut voyages: Vec<_> = board.voyages.values().collect();
    voyages.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    voyages
}

fn sorted_stories(board: &Board) -> Vec<&Story> {
    let mut stories: Vec<_> = board.stories.values().collect();
    stories.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    stories
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{
        TestBearing, TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage,
    };

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
    fn world_zoom_shows_missions_and_orphan_strategic_entities() {
        let temp = world_fixture();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let projection = build_world_map_projection(
            &board,
            WorldMapBuildOptions {
                zoom: TopologyZoom::World,
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
        assert!(visible_ids.contains(&WORLD_NODE_ID));
        assert!(visible_ids.contains(&"M1"));
        assert!(visible_ids.contains(&"M2"));
        assert!(visible_ids.contains(&"B1"));
        assert!(!visible_ids.contains(&"E1"));
        assert!(!visible_ids.contains(&"V1"));
        assert!(!visible_ids.contains(&"S1"));
    }

    #[test]
    fn mission_zoom_reveals_mission_children() {
        let temp = world_fixture();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let projection = build_world_map_projection(
            &board,
            WorldMapBuildOptions {
                zoom: TopologyZoom::Mission,
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
        assert!(visible_ids.contains(&"E1"));
        assert!(!visible_ids.contains(&"V1"));
    }

    #[test]
    fn story_zoom_surfaces_dependency_links_and_story_signals() {
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
    fn focus_filters_to_selected_branch() {
        let temp = world_fixture();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let projection = build_world_map_projection(
            &board,
            WorldMapBuildOptions {
                zoom: TopologyZoom::Mission,
                focus_id: Some("M1"),
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
        assert!(visible_ids.contains(&"M1"));
        assert!(visible_ids.contains(&"E1"));
        assert!(!visible_ids.contains(&"M2"));
        assert!(!visible_ids.contains(&"B1"));
        assert_eq!(
            projection.focus.as_ref().map(|focus| focus.id.as_str()),
            Some("M1")
        );
    }

    #[test]
    fn format_elapsed_duration_compacts_days_and_hours() {
        assert_eq!(format_elapsed_duration(45), "<1m");
        assert_eq!(format_elapsed_duration(90 * 60), "1h 30m");
        assert_eq!(format_elapsed_duration(26 * 60 * 60), "1d 2h");
    }
}
