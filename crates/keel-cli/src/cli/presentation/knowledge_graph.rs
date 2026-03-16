//! Knowledge-graph presentation helpers.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::f64::consts::{FRAC_PI_2, TAU};
use std::fmt::{Display, Write as _};
use std::str::FromStr;

use colored::Color;
use txtplot::{ChartContext, TextStyle};

use crate::cli::presentation::drift_surface::{
    render_drift_context, render_drift_coverage, render_drift_overview,
};
use keel::read_model::knowledge_graph::{
    DriftSurfaceSummary, KnowledgeGraphEdgeKind, KnowledgeGraphNodeKind, KnowledgeGraphProjection,
    build_structural_drift_summary,
};

const LABEL_NODE_LIMIT: usize = 20;
const MIN_INTERACTIVE_CHART_HEIGHT: usize = 8;
const MAX_INTERACTIVE_SUMMARY_LINES: usize = 4;
const LABEL_SIZE_MIN_RATIO: f64 = 0.45;
const LABEL_SIZE_MIN_CHARS: usize = 6;
const HELIX_STEPS: usize = 30;
const HELIX_TWISTS: f64 = 6.0;
const HELIX_AMPLITUDE: f64 = 1.8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KnowledgeGraphZoom {
    #[default]
    World,
    Delivery,
    Artifact,
    Source,
}

impl KnowledgeGraphZoom {
    pub fn zoom_in(self) -> Self {
        match self {
            Self::World => Self::Delivery,
            Self::Delivery => Self::Artifact,
            Self::Artifact => Self::Source,
            Self::Source => Self::Source,
        }
    }

    pub fn zoom_out(self) -> Self {
        match self {
            Self::World => Self::World,
            Self::Delivery => Self::World,
            Self::Artifact => Self::Delivery,
            Self::Source => Self::Artifact,
        }
    }

    fn max_depth(self) -> usize {
        match self {
            Self::World => 2,
            Self::Delivery => 3,
            Self::Artifact => 4,
            Self::Source => 5,
        }
    }
}

impl Display for KnowledgeGraphZoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::World => write!(f, "world"),
            Self::Delivery => write!(f, "delivery"),
            Self::Artifact => write!(f, "artifact"),
            Self::Source => write!(f, "source"),
        }
    }
}

impl FromStr for KnowledgeGraphZoom {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "world" => Ok(Self::World),
            "delivery" => Ok(Self::Delivery),
            "artifact" => Ok(Self::Artifact),
            "source" => Ok(Self::Source),
            other => Err(format!("unsupported knowledge graph zoom: {other}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KnowledgeGraphViewProjection {
    pub zoom: KnowledgeGraphZoom,
    pub focus: Option<KnowledgeGraphFocus>,
    pub nodes: Vec<KnowledgeGraphViewNode>,
    pub links: Vec<KnowledgeGraphViewLink>,
    pub drift: DriftSurfaceSummary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeGraphFocus {
    pub id: String,
    pub title: String,
    pub state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeGraphViewNode {
    pub id: String,
    pub title: String,
    pub kind: KnowledgeGraphNodeKind,
    pub state: Option<String>,
    pub parent_id: Option<String>,
    pub depth: usize,
    pub terminal: bool,
    pub signal_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KnowledgeGraphViewLink {
    pub from_id: String,
    pub to_id: String,
    pub kind: KnowledgeGraphViewLinkKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KnowledgeGraphViewLinkKind {
    Hierarchy,
    Traceability,
    Knowledge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeGraphKindCount {
    pub kind: KnowledgeGraphNodeKind,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LayerSummary {
    depth: usize,
    label: String,
    count: usize,
}

struct PositionedNode<'a> {
    node: &'a KnowledgeGraphViewNode,
    x: isize,
    y: isize,
    angle: f64,
}

pub fn build_knowledge_graph_view(
    projection: &KnowledgeGraphProjection,
    zoom: KnowledgeGraphZoom,
    focus_id: Option<&str>,
) -> KnowledgeGraphViewProjection {
    let visible_base = projection
        .nodes
        .iter()
        .filter(|node| depth_for_kind(node.kind) <= zoom.max_depth())
        .map(|node| node.id.clone())
        .collect::<BTreeSet<_>>();

    let focused_ids = focus_id
        .filter(|candidate| visible_base.contains(*candidate))
        .map(|candidate| build_focus_set(projection, candidate, &visible_base));
    let visible_ids = focused_ids.as_ref().unwrap_or(&visible_base);

    let signal_counts = build_signal_counts(projection);
    let mut nodes = projection
        .nodes
        .iter()
        .filter(|node| visible_ids.contains(&node.id))
        .map(|node| KnowledgeGraphViewNode {
            id: node.id.clone(),
            title: node.title.clone(),
            kind: node.kind,
            state: node.state.clone(),
            parent_id: node
                .parent_id
                .clone()
                .filter(|parent_id| visible_ids.contains(parent_id)),
            depth: depth_for_kind(node.kind),
            terminal: is_terminal_state(node.state.as_deref()),
            signal_count: *signal_counts.get(&node.id).unwrap_or(&0),
        })
        .collect::<Vec<_>>();
    nodes.sort_by(|left, right| {
        kind_sort_rank(left.kind)
            .cmp(&kind_sort_rank(right.kind))
            .then_with(|| left.id.cmp(&right.id))
    });

    let node_ids = nodes
        .iter()
        .map(|node| node.id.clone())
        .collect::<BTreeSet<_>>();

    let mut links = BTreeSet::new();
    for node in &nodes {
        if let Some(parent_id) = &node.parent_id {
            links.insert(KnowledgeGraphViewLink {
                from_id: parent_id.clone(),
                to_id: node.id.clone(),
                kind: KnowledgeGraphViewLinkKind::Hierarchy,
            });
        }
    }

    for edge in &projection.edges {
        if !node_ids.contains(&edge.from) || !node_ids.contains(&edge.to) {
            continue;
        }
        if matches!(
            edge.kind,
            KnowledgeGraphEdgeKind::Contains | KnowledgeGraphEdgeKind::Documents
        ) && nodes
            .iter()
            .find(|node| node.id == edge.to)
            .and_then(|node| node.parent_id.as_ref())
            .is_some_and(|parent_id| parent_id == &edge.from)
        {
            continue;
        }
        let kind = match edge.kind {
            KnowledgeGraphEdgeKind::KnowledgeLink
            | KnowledgeGraphEdgeKind::AppliesTo
            | KnowledgeGraphEdgeKind::Attachment
            | KnowledgeGraphEdgeKind::Provenance => KnowledgeGraphViewLinkKind::Knowledge,
            _ => KnowledgeGraphViewLinkKind::Traceability,
        };
        links.insert(KnowledgeGraphViewLink {
            from_id: edge.from.clone(),
            to_id: edge.to.clone(),
            kind,
        });
    }

    let focus = focused_ids
        .as_ref()
        .and_then(|_| {
            projection
                .nodes
                .iter()
                .find(|node| Some(node.id.as_str()) == focus_id)
        })
        .map(|node| KnowledgeGraphFocus {
            id: node.id.clone(),
            title: node.title.clone(),
            state: node.state.clone(),
        });

    KnowledgeGraphViewProjection {
        zoom,
        focus,
        nodes,
        links: links.into_iter().collect(),
        drift: build_structural_drift_summary(projection),
    }
}

pub fn render_knowledge_graph(projection: &KnowledgeGraphViewProjection, width: usize) -> String {
    render_knowledge_graph_with_mode(projection, width, RenderMode::Static).replace('\u{2800}', " ")
}

pub fn render_knowledge_graph_interactive(
    projection: &KnowledgeGraphViewProjection,
    width: usize,
    height: usize,
) -> String {
    render_knowledge_graph_with_mode(projection, width, RenderMode::Interactive { height })
        .replace('\u{2800}', " ")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderMode {
    Static,
    Interactive { height: usize },
}

fn render_knowledge_graph_with_mode(
    projection: &KnowledgeGraphViewProjection,
    width: usize,
    mode: RenderMode,
) -> String {
    let line_width = width.max(1);
    let chart_width = chart_width_for_mode(line_width, mode);
    let summary = match mode {
        RenderMode::Static => render_summary(projection),
        RenderMode::Interactive { height } => {
            render_interactive_summary(projection, line_width, interactive_summary_budget(height))
        }
    };
    let summary_lines = summary.lines().count();
    let chart_height = chart_height_for_mode(projection, line_width, summary_lines, mode);

    let mut chart = ChartContext::new(chart_width, chart_height);
    let positions = layout_positions(projection, &chart);

    draw_orbits(&mut chart, projection, &positions);
    draw_links(&mut chart, projection, &positions);
    draw_nodes(&mut chart, projection, &positions);
    draw_labels(&mut chart, projection, &positions);

    let mut rendered = String::new();
    rendered.push_str(
        &chart
            .canvas
            .render_with_options(true, Some(&chart_title(projection, line_width))),
    );
    rendered.push('\n');
    rendered.push_str(&summary);
    rendered
}

fn chart_width_for_mode(width: usize, mode: RenderMode) -> usize {
    let available = width.saturating_sub(2).max(1);
    match mode {
        RenderMode::Static | RenderMode::Interactive { .. } => available,
    }
}

fn chart_height_for_mode(
    projection: &KnowledgeGraphViewProjection,
    width: usize,
    summary_lines: usize,
    mode: RenderMode,
) -> usize {
    match mode {
        RenderMode::Static => default_chart_height(projection, width),
        RenderMode::Interactive { height } => height
            .saturating_sub(summary_lines + 4)
            .max(MIN_INTERACTIVE_CHART_HEIGHT),
    }
}

fn default_chart_height(projection: &KnowledgeGraphViewProjection, width: usize) -> usize {
    let scaled = width.max(1) / 4;
    let base = match projection.zoom {
        KnowledgeGraphZoom::Source => 10,
        KnowledgeGraphZoom::Artifact => 6,
        _ => 4,
    };
    (scaled + base).max(10)
}

fn interactive_summary_budget(height: usize) -> usize {
    height
        .saturating_sub(14)
        .clamp(1, MAX_INTERACTIVE_SUMMARY_LINES)
}

fn chart_title(projection: &KnowledgeGraphViewProjection, width: usize) -> String {
    let mut title = format!("Knowledge Graph · zoom {}", projection.zoom);
    if let Some(focus) = &projection.focus {
        title.push_str(&format!(" · focus {}", focus.id));
    }
    truncate_text(&title, width.saturating_sub(2).max(1))
}

fn render_summary(projection: &KnowledgeGraphViewProjection) -> String {
    let mut out = String::new();
    writeln!(
        out,
        "Zoom: {} | Focus: {}",
        projection.zoom,
        projection
            .focus
            .as_ref()
            .map(render_focus_summary)
            .unwrap_or_else(|| "board-wide".to_string())
    )
    .unwrap();
    writeln!(
        out,
        "Visible: {}",
        render_kind_counts(&kind_counts(projection))
    )
    .unwrap();
    writeln!(out, "Links: {}", render_link_counts(projection)).unwrap();
    writeln!(
        out,
        "{} | {}",
        render_drift_overview("Drift", &projection.drift),
        render_drift_coverage(&projection.drift)
    )
    .unwrap();
    if let Some(context) = render_drift_context(&projection.drift, 2) {
        writeln!(out, "Context: {context}").unwrap();
    }

    let layers = layer_summaries(projection);
    if !layers.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "Layers:").unwrap();
        for layer in layers {
            writeln!(out, "  {}. {} ({})", layer.depth, layer.label, layer.count).unwrap();
        }
    }

    if let Some(highlight) = highlight(projection) {
        writeln!(out).unwrap();
        writeln!(out, "Highlight: {highlight}").unwrap();
    }

    out.trim_end().to_string()
}

fn render_interactive_summary(
    projection: &KnowledgeGraphViewProjection,
    width: usize,
    max_lines: usize,
) -> String {
    let mut lines = vec![
        truncate_text(
            &format!(
                "Zoom: {} | Focus: {}",
                projection.zoom,
                projection
                    .focus
                    .as_ref()
                    .map(render_focus_summary)
                    .unwrap_or_else(|| "board-wide".to_string())
            ),
            width,
        ),
        truncate_text(
            &format!("Visible: {}", render_kind_counts(&kind_counts(projection))),
            width,
        ),
        truncate_text(
            &match render_drift_context(&projection.drift, 1) {
                Some(context) => format!(
                    "{} | {context}",
                    render_drift_overview("Drift", &projection.drift)
                ),
                None => render_drift_overview("Drift", &projection.drift),
            },
            width,
        ),
    ];

    if max_lines >= 4 {
        if let Some(highlight) = highlight(projection) {
            lines.push(truncate_text(&format!("Highlight: {highlight}"), width));
        } else {
            lines.push(truncate_text(
                &format!("Links: {}", render_link_counts(projection)),
                width,
            ));
        }
    }

    lines
        .into_iter()
        .take(max_lines)
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_focus_summary(focus: &KnowledgeGraphFocus) -> String {
    match focus.state.as_deref() {
        Some(state) => format!("{} ({state})", focus.title),
        None => focus.title.clone(),
    }
}

fn render_kind_counts(counts: &[KnowledgeGraphKindCount]) -> String {
    if counts.is_empty() {
        return "world only".to_string();
    }

    counts
        .iter()
        .map(|count| {
            format!(
                "{} {}",
                count.count,
                title_case_label(if count.count == 1 {
                    singular_label(count.kind)
                } else {
                    plural_label(count.kind)
                })
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn kind_counts(projection: &KnowledgeGraphViewProjection) -> Vec<KnowledgeGraphKindCount> {
    let mut counts = BTreeMap::<KnowledgeGraphNodeKind, usize>::new();
    for node in &projection.nodes {
        if node.kind == KnowledgeGraphNodeKind::World {
            continue;
        }
        *counts.entry(node.kind).or_default() += 1;
    }

    counts
        .into_iter()
        .map(|(kind, count)| KnowledgeGraphKindCount { kind, count })
        .collect()
}

fn render_link_counts(projection: &KnowledgeGraphViewProjection) -> String {
    let mut hierarchy = 0;
    let mut traceability = 0;
    let mut knowledge = 0;
    for link in &projection.links {
        match link.kind {
            KnowledgeGraphViewLinkKind::Hierarchy => hierarchy += 1,
            KnowledgeGraphViewLinkKind::Traceability => traceability += 1,
            KnowledgeGraphViewLinkKind::Knowledge => knowledge += 1,
        }
    }

    let mut parts = Vec::new();
    if hierarchy > 0 {
        parts.push(format!("{hierarchy} hierarchy"));
    }
    if traceability > 0 {
        parts.push(format!("{traceability} traceability"));
    }
    if knowledge > 0 {
        parts.push(format!("{knowledge} knowledge"));
    }
    if parts.is_empty() {
        "none".to_string()
    } else {
        parts.join(", ")
    }
}

fn layer_summaries(projection: &KnowledgeGraphViewProjection) -> Vec<LayerSummary> {
    let mut counts = BTreeMap::<usize, Vec<KnowledgeGraphNodeKind>>::new();
    for node in &projection.nodes {
        if node.kind == KnowledgeGraphNodeKind::World {
            continue;
        }
        counts.entry(node.depth).or_default().push(node.kind);
    }

    counts
        .into_iter()
        .map(|(depth, kinds)| {
            let count = kinds.len();
            let label = kinds
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .map(compound_label)
                .collect::<Vec<_>>()
                .join(" + ");
            LayerSummary {
                depth,
                label,
                count,
            }
        })
        .collect()
}

fn highlight(projection: &KnowledgeGraphViewProjection) -> Option<String> {
    projection
        .nodes
        .iter()
        .filter(|node| node.kind != KnowledgeGraphNodeKind::World)
        .max_by(|left, right| {
            left.signal_count
                .cmp(&right.signal_count)
                .then_with(|| left.id.cmp(&right.id))
        })
        .map(|node| {
            format!(
                "{} {} · {} graph links{}",
                singular_label(node.kind),
                node.title,
                node.signal_count,
                node.state
                    .as_deref()
                    .map(|state| format!(" · {state}"))
                    .unwrap_or_default()
            )
        })
}

fn layout_positions<'a>(
    projection: &'a KnowledgeGraphViewProjection,
    chart: &ChartContext,
) -> HashMap<String, PositionedNode<'a>> {
    let child_map = build_child_map(&projection.nodes);
    let Some(root) = projection
        .nodes
        .iter()
        .find(|node| node.kind == KnowledgeGraphNodeKind::World)
    else {
        return HashMap::new();
    };
    let max_depth = projection
        .nodes
        .iter()
        .map(|node| node.depth)
        .max()
        .unwrap_or(0)
        .max(1);
    let center_x = (chart.canvas.pixel_width() / 2) as isize;
    let center_y = (chart.canvas.pixel_height() / 2) as isize;
    let min_dimension = chart.canvas.pixel_width().min(chart.canvas.pixel_height()) as f64;
    let margin = (min_dimension * 0.12).clamp(4.0, 12.0);
    let max_radius = (chart.canvas.pixel_width().min(chart.canvas.pixel_height()) as f64 / 2.0
        - margin)
        .max(8.0);
    let radius_step = max_radius / (max_depth as f64 + 0.35);

    let mut weights = HashMap::new();
    let mut positions = HashMap::new();
    positions.insert(
        root.id.clone(),
        PositionedNode {
            node: root,
            x: center_x,
            y: center_y,
            angle: -FRAC_PI_2,
        },
    );

    assign_child_positions(
        root.id.as_str(),
        -FRAC_PI_2,
        TAU,
        center_x,
        center_y,
        radius_step,
        &child_map,
        &mut weights,
        &mut positions,
    );

    positions
}

#[allow(clippy::too_many_arguments)]
fn assign_child_positions<'a>(
    parent_id: &str,
    start_angle: f64,
    angle_span: f64,
    center_x: isize,
    center_y: isize,
    radius_step: f64,
    child_map: &HashMap<String, Vec<&'a KnowledgeGraphViewNode>>,
    weights: &mut HashMap<String, usize>,
    positions: &mut HashMap<String, PositionedNode<'a>>,
) {
    let Some(children) = child_map.get(parent_id) else {
        return;
    };
    if children.is_empty() {
        return;
    }

    let total_weight: usize = children
        .iter()
        .map(|child| subtree_weight(child.id.as_str(), child_map, weights))
        .sum();

    let mut cursor = start_angle;
    for child in children {
        let weight = subtree_weight(child.id.as_str(), child_map, weights);
        let child_span = if total_weight == 0 {
            angle_span / children.len().max(1) as f64
        } else {
            angle_span * weight as f64 / total_weight as f64
        };
        let angle = cursor + child_span / 2.0;
        let radius = radius_step * child.depth as f64;
        let x = center_x as f64 + radius * angle.cos();
        let y = center_y as f64 + radius * angle.sin();

        positions.insert(
            child.id.clone(),
            PositionedNode {
                node: child,
                x: x.round() as isize,
                y: y.round() as isize,
                angle,
            },
        );

        let child_sector = if children.len() == 1 {
            angle_span.min(TAU * 0.85)
        } else {
            child_span * 0.88
        };
        assign_child_positions(
            child.id.as_str(),
            angle - child_sector / 2.0,
            child_sector,
            center_x,
            center_y,
            radius_step,
            child_map,
            weights,
            positions,
        );

        cursor += child_span;
    }
}

fn subtree_weight(
    id: &str,
    child_map: &HashMap<String, Vec<&KnowledgeGraphViewNode>>,
    weights: &mut HashMap<String, usize>,
) -> usize {
    if let Some(weight) = weights.get(id) {
        return *weight;
    }

    let weight = 1 + child_map
        .get(id)
        .map(|children| {
            children
                .iter()
                .map(|child| subtree_weight(child.id.as_str(), child_map, weights))
                .sum::<usize>()
        })
        .unwrap_or(0);
    weights.insert(id.to_string(), weight);
    weight
}

fn draw_orbits(
    chart: &mut ChartContext,
    projection: &KnowledgeGraphViewProjection,
    positions: &HashMap<String, PositionedNode<'_>>,
) {
    let center_x = (chart.canvas.pixel_width() / 2) as isize;
    let center_y = (chart.canvas.pixel_height() / 2) as isize;
    let mut depth_radii: BTreeMap<usize, Vec<isize>> = projection
        .nodes
        .iter()
        .filter(|node| node.kind != KnowledgeGraphNodeKind::World && node.depth > 0)
        .filter_map(|node| positions.get(&node.id))
        .fold(BTreeMap::new(), |mut acc, position| {
            let dx = position.x - center_x;
            let dy = position.y - center_y;
            let radius = (((dx * dx + dy * dy) as f64).sqrt().round() as isize).max(1);
            acc.entry(position.node.depth).or_default().push(radius);
            acc
        });

    for radii in depth_radii.values_mut() {
        let total: isize = radii.iter().copied().sum();
        let radius = (total as f64 / radii.len() as f64).round() as isize;
        draw_circle_screen(chart, center_x, center_y, radius, Some(Color::BrightBlack));
    }
}

fn draw_links(
    chart: &mut ChartContext,
    projection: &KnowledgeGraphViewProjection,
    positions: &HashMap<String, PositionedNode<'_>>,
) {
    for link in &projection.links {
        if link.kind == KnowledgeGraphViewLinkKind::Hierarchy && projection.focus.is_none() {
            continue;
        }

        let Some(from) = positions.get(&link.from_id) else {
            continue;
        };
        let Some(to) = positions.get(&link.to_id) else {
            continue;
        };

        let color = match link.kind {
            KnowledgeGraphViewLinkKind::Hierarchy => Some(Color::BrightBlack),
            KnowledgeGraphViewLinkKind::Traceability => Some(Color::BrightBlue),
            KnowledgeGraphViewLinkKind::Knowledge => Some(Color::BrightYellow),
        };

        if let Some(color) = color {
            if should_render_source_zoom_double_helix(
                projection.zoom,
                from.node,
                to.node,
                link.kind,
            ) {
                draw_double_helix_link(chart, from, to, color);
            } else {
                chart
                    .canvas
                    .line_screen(from.x, from.y, to.x, to.y, Some(color));
            }
        }
    }
}

fn should_render_source_zoom_double_helix(
    zoom: KnowledgeGraphZoom,
    from: &KnowledgeGraphViewNode,
    to: &KnowledgeGraphViewNode,
    kind: KnowledgeGraphViewLinkKind,
) -> bool {
    if zoom != KnowledgeGraphZoom::Source {
        return false;
    }

    if kind == KnowledgeGraphViewLinkKind::Hierarchy {
        return false;
    }

    is_source_ring_transition(from.kind, to.kind)
}

fn is_source_ring_transition(left: KnowledgeGraphNodeKind, right: KnowledgeGraphNodeKind) -> bool {
    (left == KnowledgeGraphNodeKind::Knowledge && right == KnowledgeGraphNodeKind::SourceFile)
        || (left == KnowledgeGraphNodeKind::SourceFile
            && right == KnowledgeGraphNodeKind::Knowledge)
}

fn draw_double_helix_link(
    chart: &mut ChartContext,
    from: &PositionedNode<'_>,
    to: &PositionedNode<'_>,
    color: Color,
) {
    let center_x = (chart.canvas.pixel_width() / 2) as f64;
    let center_y = (chart.canvas.pixel_height() / 2) as f64;

    let from_dx = from.x as f64 - center_x;
    let from_dy = from.y as f64 - center_y;
    let to_dx = to.x as f64 - center_x;
    let to_dy = to.y as f64 - center_y;

    let r1 = (from_dx * from_dx + from_dy * from_dy).sqrt();
    let r2 = (to_dx * to_dx + to_dy * to_dy).sqrt();
    let a1 = from_dy.atan2(from_dx);
    let a2 = to_dy.atan2(to_dx);
    let span = angle_span(a1, a2);

    for phase in [0.0_f64, std::f64::consts::PI] {
        draw_helix_strand(
            chart, center_x, center_y, from.x, from.y, to.x, to.y, r1, r2, a1, span, phase, color,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_helix_strand(
    chart: &mut ChartContext,
    center_x: f64,
    center_y: f64,
    from_x: isize,
    from_y: isize,
    to_x: isize,
    to_y: isize,
    r1: f64,
    r2: f64,
    start_angle: f64,
    angle_span: f64,
    phase: f64,
    color: Color,
) {
    let mut prev = (from_x, from_y);

    for step in 1..=HELIX_STEPS {
        let ratio = step as f64 / HELIX_STEPS as f64;
        let angle = start_angle + angle_span * ratio;
        let radius = r1 + (r2 - r1) * ratio;
        let wave = (ratio * TAU * HELIX_TWISTS + phase).sin();
        let normal_x = -angle.sin();
        let normal_y = angle.cos();

        let x =
            (center_x + radius * angle.cos() + normal_x * wave * HELIX_AMPLITUDE).round() as isize;
        let y =
            (center_y + radius * angle.sin() + normal_y * wave * HELIX_AMPLITUDE).round() as isize;
        let mut point_x = x;
        let mut point_y = y;

        if step == HELIX_STEPS {
            point_x = to_x;
            point_y = to_y;
        }

        chart
            .canvas
            .line_screen(prev.0, prev.1, point_x, point_y, Some(color));
        prev = (point_x, point_y);
    }
}

fn angle_span(from_angle: f64, to_angle: f64) -> f64 {
    let mut span = (to_angle - from_angle).rem_euclid(TAU);
    if span > std::f64::consts::PI {
        span -= TAU;
    }
    span
}

fn draw_nodes(
    chart: &mut ChartContext,
    projection: &KnowledgeGraphViewProjection,
    positions: &HashMap<String, PositionedNode<'_>>,
) {
    for positioned in positions.values() {
        let color = node_color(positioned.node);
        let radius = node_radius(positioned.node.kind);
        draw_circle_filled_screen(chart, positioned.x, positioned.y, radius, Some(color));

        if projection
            .focus
            .as_ref()
            .is_some_and(|focus| focus.id == positioned.node.id)
        {
            draw_circle_screen(
                chart,
                positioned.x,
                positioned.y,
                radius + 3,
                Some(Color::BrightWhite),
            );
        }
    }
}

fn draw_labels(
    chart: &mut ChartContext,
    projection: &KnowledgeGraphViewProjection,
    positions: &HashMap<String, PositionedNode<'_>>,
) {
    let label_all = projection.nodes.len() <= LABEL_NODE_LIMIT;
    let deepest_visible_depth = projection
        .nodes
        .iter()
        .map(|node| node.depth)
        .max()
        .unwrap_or(0);

    for positioned in positions.values() {
        if positioned.node.kind == KnowledgeGraphNodeKind::World {
            draw_text_screen(
                chart,
                "Keel Knowledge",
                positioned.x - 16,
                positioned.y - 2,
                TextStyle::new().with_foreground(Color::White).bold(),
            );
            continue;
        }

        let should_label = projection
            .focus
            .as_ref()
            .is_some_and(|focus| focus.id == positioned.node.id)
            || positioned.node.depth <= 1
            || positioned.node.signal_count > 1
            || (label_all && positioned.node.depth == deepest_visible_depth);
        if !should_label {
            continue;
        }

        let label = depth_label_text(
            &positioned.node.title,
            positioned.node.depth,
            deepest_visible_depth,
        );
        let style = depth_label_style(
            positioned.node,
            deepest_visible_depth,
            node_color(positioned.node),
        );
        let offset_x = if positioned.angle.cos() >= 0.0 {
            8
        } else {
            -((label.chars().count() as isize) * 2 + 8)
        };
        let offset_y = if positioned.angle.sin() >= 0.0 { 6 } else { -6 };
        draw_text_screen(
            chart,
            &label,
            positioned.x + offset_x,
            positioned.y + offset_y,
            style,
        );
    }
}

fn depth_label_text(label: &str, depth: usize, max_depth: usize) -> String {
    if label.is_empty() || max_depth <= 1 || depth == 0 {
        return label.to_string();
    }

    let char_count = label.chars().count();
    let max_chars = depth_label_max_chars(char_count, depth, max_depth);
    if char_count <= max_chars {
        return label.to_string();
    }

    let visible_chars = max_chars.saturating_sub(1);
    let visible_chars = if visible_chars == 0 { 1 } else { visible_chars };

    let mut clipped = label.chars().take(visible_chars).collect::<String>();
    clipped.push('…');
    clipped
}

fn depth_label_max_chars(char_count: usize, depth: usize, max_depth: usize) -> usize {
    if char_count <= LABEL_SIZE_MIN_CHARS || max_depth <= 1 || depth == 0 {
        return char_count;
    }

    let span = (max_depth.saturating_sub(1)).max(1) as f64;
    let ratio = LABEL_SIZE_MIN_RATIO
        + ((depth.saturating_sub(1) as f64 / span) * (1.0 - LABEL_SIZE_MIN_RATIO));
    let ratio = ratio.clamp(LABEL_SIZE_MIN_RATIO, 1.0);
    let minimum_chars = LABEL_SIZE_MIN_CHARS.min(char_count);
    let scaled = (char_count as f64 * ratio).round() as usize;

    scaled.clamp(minimum_chars, char_count)
}

fn depth_label_style(
    node: &KnowledgeGraphViewNode,
    max_depth: usize,
    base_color: Color,
) -> TextStyle {
    if node.terminal {
        return TextStyle::new().with_foreground(base_color);
    }
    if max_depth <= 1 || node.depth == 0 {
        return TextStyle::new().with_foreground(base_color);
    }

    let span = (max_depth.saturating_sub(1)).max(1) as f64;
    let depth_ratio = (node.depth.saturating_sub(1) as f64 / span).clamp(0.0, 1.0);
    let mut style = TextStyle::new().with_foreground(base_color);

    if depth_ratio <= (1.0 / 3.0) {
        style = style.bold();
    } else if depth_ratio <= (2.0 / 3.0) {
        style = style.normal();
    } else {
        style = style.dim();
    }

    style
}

fn truncate_text(text: &str, limit: usize) -> String {
    if limit == 0 {
        return String::new();
    }
    if text.chars().count() <= limit {
        return text.to_string();
    }

    let mut truncated = text
        .chars()
        .take(limit.saturating_sub(1))
        .collect::<String>();
    truncated.push('…');
    truncated
}

fn build_child_map(
    nodes: &[KnowledgeGraphViewNode],
) -> HashMap<String, Vec<&KnowledgeGraphViewNode>> {
    let mut child_map: HashMap<String, Vec<&KnowledgeGraphViewNode>> = HashMap::new();
    for node in nodes {
        if let Some(parent_id) = &node.parent_id {
            child_map.entry(parent_id.clone()).or_default().push(node);
        }
    }

    for children in child_map.values_mut() {
        children.sort_by(|left, right| {
            kind_sort_rank(left.kind)
                .cmp(&kind_sort_rank(right.kind))
                .then_with(|| left.id.cmp(&right.id))
        });
    }

    child_map
}

fn draw_circle_screen(
    chart: &mut ChartContext,
    center_x: isize,
    center_y: isize,
    radius: isize,
    color: Option<Color>,
) {
    if radius <= 0 {
        chart
            .canvas
            .set_pixel_screen(center_x.max(0) as usize, center_y.max(0) as usize, color);
        return;
    }

    let segments = (radius * 8).clamp(24, 144) as usize;
    let mut previous = None;
    for step in 0..=segments {
        let angle = TAU * step as f64 / segments as f64;
        let x = center_x as f64 + radius as f64 * angle.cos();
        let y = center_y as f64 + radius as f64 * angle.sin();
        let current = (x.round() as isize, y.round() as isize);
        if let Some((prev_x, prev_y)) = previous {
            chart
                .canvas
                .line_screen(prev_x, prev_y, current.0, current.1, color);
        }
        previous = Some(current);
    }
}

fn draw_circle_filled_screen(
    chart: &mut ChartContext,
    center_x: isize,
    center_y: isize,
    radius: isize,
    color: Option<Color>,
) {
    if radius <= 0 {
        chart
            .canvas
            .set_pixel_screen(center_x.max(0) as usize, center_y.max(0) as usize, color);
        return;
    }

    for dy in -radius..=radius {
        let chord = ((radius * radius - dy * dy) as f64).sqrt().round() as isize;
        chart.canvas.line_screen(
            center_x - chord,
            center_y + dy,
            center_x + chord,
            center_y + dy,
            color,
        );
    }
}

fn draw_text_screen(
    chart: &mut ChartContext,
    text: &str,
    x_px: isize,
    y_px: isize,
    style: TextStyle,
) {
    let max_col = chart.canvas.width.saturating_sub(1) as isize;
    let max_row = chart.canvas.height.saturating_sub(1) as isize;
    let col = (x_px / 2).clamp(0, max_col) as usize;
    let row = (y_px / 4).clamp(0, max_row) as usize;
    let x_norm = if chart.canvas.width <= 1 {
        0.0
    } else {
        col as f64 / (chart.canvas.width - 1) as f64
    };
    let y_norm = if chart.canvas.height <= 1 {
        1.0
    } else {
        1.0 - row as f64 / (chart.canvas.height - 1) as f64
    };
    chart.text_styled(text, x_norm, y_norm, style);
}

fn build_focus_set(
    projection: &KnowledgeGraphProjection,
    focus_id: &str,
    visible_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut keep = BTreeSet::from(["world:board".to_string(), focus_id.to_string()]);
    keep.extend(ancestor_chain(projection, focus_id, visible_ids));
    keep.extend(descendants(projection, focus_id, visible_ids));

    let branch_ids = keep.clone().into_iter().collect::<Vec<_>>();
    for branch_id in branch_ids {
        for edge in &projection.edges {
            let neighbor = if edge.from == branch_id {
                Some(edge.to.as_str())
            } else if edge.to == branch_id {
                Some(edge.from.as_str())
            } else {
                None
            };
            let Some(neighbor) = neighbor else {
                continue;
            };
            if !visible_ids.contains(neighbor) {
                continue;
            }
            keep.insert(neighbor.to_string());
            keep.extend(ancestor_chain(projection, neighbor, visible_ids));
        }
    }

    keep
}

fn ancestor_chain(
    projection: &KnowledgeGraphProjection,
    start: &str,
    visible_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut ancestors = BTreeSet::new();
    let mut current = projection
        .nodes
        .iter()
        .find(|node| node.id == start)
        .and_then(|node| node.parent_id.clone());

    while let Some(parent_id) = current {
        if !visible_ids.contains(&parent_id) {
            break;
        }
        ancestors.insert(parent_id.clone());
        current = projection
            .nodes
            .iter()
            .find(|node| node.id == parent_id)
            .and_then(|node| node.parent_id.clone());
    }

    ancestors
}

fn descendants(
    projection: &KnowledgeGraphProjection,
    start: &str,
    visible_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut descendants = BTreeSet::new();
    let mut frontier = vec![start.to_string()];
    while let Some(parent_id) = frontier.pop() {
        for child in projection
            .nodes
            .iter()
            .filter(|node| node.parent_id.as_deref() == Some(parent_id.as_str()))
        {
            if !visible_ids.contains(&child.id) || !descendants.insert(child.id.clone()) {
                continue;
            }
            frontier.push(child.id.clone());
        }
    }
    descendants
}

fn build_signal_counts(projection: &KnowledgeGraphProjection) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for edge in &projection.edges {
        if matches!(
            edge.kind,
            KnowledgeGraphEdgeKind::Contains | KnowledgeGraphEdgeKind::Documents
        ) {
            continue;
        }
        *counts.entry(edge.from.clone()).or_default() += 1;
        *counts.entry(edge.to.clone()).or_default() += 1;
    }
    counts
}

fn depth_for_kind(kind: KnowledgeGraphNodeKind) -> usize {
    match kind {
        KnowledgeGraphNodeKind::World => 0,
        KnowledgeGraphNodeKind::Mission => 1,
        KnowledgeGraphNodeKind::Epic
        | KnowledgeGraphNodeKind::Bearing
        | KnowledgeGraphNodeKind::Adr => 2,
        KnowledgeGraphNodeKind::Voyage
        | KnowledgeGraphNodeKind::Story
        | KnowledgeGraphNodeKind::Routine => 3,
        KnowledgeGraphNodeKind::Artifact
        | KnowledgeGraphNodeKind::Knowledge
        | KnowledgeGraphNodeKind::ProjectDoc => 4,
        KnowledgeGraphNodeKind::SourceFile => 5,
        KnowledgeGraphNodeKind::Heartbeat => 1, // Pacemaker sits at mission/world boundary
    }
}

fn kind_sort_rank(kind: KnowledgeGraphNodeKind) -> u8 {
    match kind {
        KnowledgeGraphNodeKind::World => 0,
        KnowledgeGraphNodeKind::Mission => 1,
        KnowledgeGraphNodeKind::Epic => 2,
        KnowledgeGraphNodeKind::Bearing => 3,
        KnowledgeGraphNodeKind::Adr => 4,
        KnowledgeGraphNodeKind::Voyage => 5,
        KnowledgeGraphNodeKind::Story => 6,
        KnowledgeGraphNodeKind::Routine => 7,
        KnowledgeGraphNodeKind::Artifact => 8,
        KnowledgeGraphNodeKind::Knowledge => 9,
        KnowledgeGraphNodeKind::ProjectDoc => 10,
        KnowledgeGraphNodeKind::SourceFile => 11,
        KnowledgeGraphNodeKind::Heartbeat => 5,
    }
}

fn node_color(node: &KnowledgeGraphViewNode) -> Color {
    if node.terminal {
        return Color::BrightBlack;
    }

    match node.kind {
        KnowledgeGraphNodeKind::World => Color::White,
        KnowledgeGraphNodeKind::Mission => Color::BrightCyan,
        KnowledgeGraphNodeKind::Epic => Color::BrightBlue,
        KnowledgeGraphNodeKind::Bearing => Color::BrightYellow,
        KnowledgeGraphNodeKind::Adr => Color::BrightMagenta,
        KnowledgeGraphNodeKind::Voyage => Color::BrightGreen,
        KnowledgeGraphNodeKind::Story => Color::White,
        KnowledgeGraphNodeKind::Routine => Color::BrightCyan,
        KnowledgeGraphNodeKind::Artifact => Color::Cyan,
        KnowledgeGraphNodeKind::Knowledge => Color::Yellow,
        KnowledgeGraphNodeKind::ProjectDoc => Color::Magenta,
        KnowledgeGraphNodeKind::SourceFile => Color::Green,
        KnowledgeGraphNodeKind::Heartbeat => Color::BrightYellow,
    }
}

fn node_radius(kind: KnowledgeGraphNodeKind) -> isize {
    match kind {
        KnowledgeGraphNodeKind::World => 4,
        KnowledgeGraphNodeKind::Mission => 3,
        KnowledgeGraphNodeKind::Epic
        | KnowledgeGraphNodeKind::Bearing
        | KnowledgeGraphNodeKind::Adr
        | KnowledgeGraphNodeKind::Voyage
        | KnowledgeGraphNodeKind::Story
        | KnowledgeGraphNodeKind::Routine => 2,
        KnowledgeGraphNodeKind::Artifact
        | KnowledgeGraphNodeKind::Knowledge
        | KnowledgeGraphNodeKind::ProjectDoc
        | KnowledgeGraphNodeKind::SourceFile => 1,
        KnowledgeGraphNodeKind::Heartbeat => 3,
    }
}

fn is_terminal_state(state: Option<&str>) -> bool {
    matches!(
        state,
        Some(
            "done"
                | "verified"
                | "laid"
                | "accepted"
                | "rejected"
                | "iced"
                | "parked"
                | "declined"
                | "stable"
        )
    )
}

fn singular_label(kind: KnowledgeGraphNodeKind) -> &'static str {
    match kind {
        KnowledgeGraphNodeKind::World => "world",
        KnowledgeGraphNodeKind::Mission => "mission",
        KnowledgeGraphNodeKind::Epic => "epic",
        KnowledgeGraphNodeKind::Bearing => "bearing",
        KnowledgeGraphNodeKind::Adr => "ADR",
        KnowledgeGraphNodeKind::Voyage => "voyage",
        KnowledgeGraphNodeKind::Story => "story",
        KnowledgeGraphNodeKind::Routine => "routine",
        KnowledgeGraphNodeKind::Artifact => "artifact",
        KnowledgeGraphNodeKind::Knowledge => "knowledge unit",
        KnowledgeGraphNodeKind::ProjectDoc => "project doc",
        KnowledgeGraphNodeKind::SourceFile => "source file",
        KnowledgeGraphNodeKind::Heartbeat => "pacemaker",
    }
}

fn plural_label(kind: KnowledgeGraphNodeKind) -> &'static str {
    match kind {
        KnowledgeGraphNodeKind::World => "worlds",
        KnowledgeGraphNodeKind::Mission => "missions",
        KnowledgeGraphNodeKind::Epic => "epics",
        KnowledgeGraphNodeKind::Bearing => "bearings",
        KnowledgeGraphNodeKind::Adr => "ADRs",
        KnowledgeGraphNodeKind::Voyage => "voyages",
        KnowledgeGraphNodeKind::Story => "stories",
        KnowledgeGraphNodeKind::Routine => "routines",
        KnowledgeGraphNodeKind::Artifact => "artifacts",
        KnowledgeGraphNodeKind::Knowledge => "knowledge units",
        KnowledgeGraphNodeKind::ProjectDoc => "project docs",
        KnowledgeGraphNodeKind::SourceFile => "source files",
        KnowledgeGraphNodeKind::Heartbeat => "pacemakers",
    }
}

fn compound_label(kind: KnowledgeGraphNodeKind) -> String {
    title_case_label(plural_label(kind))
}

fn title_case_label(label: &str) -> String {
    let mut chars = label.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let mut out = first.to_uppercase().collect::<String>();
    out.push_str(chars.as_str());
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::read_model::knowledge_graph::{
        KnowledgeGraphEdge, KnowledgeGraphNode, StructuralDriftInputs,
    };

    fn sample_projection() -> KnowledgeGraphProjection {
        KnowledgeGraphProjection {
            schema_version: 1,
            nodes: vec![
                KnowledgeGraphNode {
                    id: "world:board".to_string(),
                    kind: KnowledgeGraphNodeKind::World,
                    title: "Board".to_string(),
                    state: Some("live".to_string()),
                    path: None,
                    parent_id: None,
                },
                KnowledgeGraphNode {
                    id: "mission:M1".to_string(),
                    kind: KnowledgeGraphNodeKind::Mission,
                    title: "Mission".to_string(),
                    state: Some("active".to_string()),
                    path: None,
                    parent_id: Some("world:board".to_string()),
                },
                KnowledgeGraphNode {
                    id: "story:S1".to_string(),
                    kind: KnowledgeGraphNodeKind::Story,
                    title: "Story".to_string(),
                    state: Some("backlog".to_string()),
                    path: None,
                    parent_id: Some("mission:M1".to_string()),
                },
                KnowledgeGraphNode {
                    id: "artifact:a".to_string(),
                    kind: KnowledgeGraphNodeKind::Artifact,
                    title: "Artifact".to_string(),
                    state: None,
                    path: None,
                    parent_id: Some("story:S1".to_string()),
                },
                KnowledgeGraphNode {
                    id: "source:src/lib.rs".to_string(),
                    kind: KnowledgeGraphNodeKind::SourceFile,
                    title: "src/lib.rs".to_string(),
                    state: None,
                    path: None,
                    parent_id: Some("world:board".to_string()),
                },
            ],
            edges: vec![
                KnowledgeGraphEdge {
                    from: "world:board".to_string(),
                    to: "mission:M1".to_string(),
                    kind: KnowledgeGraphEdgeKind::Contains,
                },
                KnowledgeGraphEdge {
                    from: "mission:M1".to_string(),
                    to: "story:S1".to_string(),
                    kind: KnowledgeGraphEdgeKind::Contains,
                },
                KnowledgeGraphEdge {
                    from: "story:S1".to_string(),
                    to: "artifact:a".to_string(),
                    kind: KnowledgeGraphEdgeKind::Documents,
                },
                KnowledgeGraphEdge {
                    from: "artifact:a".to_string(),
                    to: "source:src/lib.rs".to_string(),
                    kind: KnowledgeGraphEdgeKind::Traceability,
                },
            ],
            drift_inputs: StructuralDriftInputs::default(),
        }
    }

    #[test]
    fn world_zoom_hides_deeper_layers() {
        let view =
            build_knowledge_graph_view(&sample_projection(), KnowledgeGraphZoom::World, None);

        let ids = view
            .nodes
            .iter()
            .map(|node| node.id.as_str())
            .collect::<BTreeSet<_>>();

        assert!(ids.contains("world:board"));
        assert!(ids.contains("mission:M1"));
        assert!(!ids.contains("story:S1"));
        assert!(!ids.contains("artifact:a"));
    }

    #[test]
    fn focus_retains_branch_and_neighbors() {
        let view = build_knowledge_graph_view(
            &sample_projection(),
            KnowledgeGraphZoom::Source,
            Some("story:S1"),
        );

        let ids = view
            .nodes
            .iter()
            .map(|node| node.id.as_str())
            .collect::<BTreeSet<_>>();

        assert!(ids.contains("world:board"));
        assert!(ids.contains("mission:M1"));
        assert!(ids.contains("story:S1"));
        assert!(ids.contains("artifact:a"));
        assert!(ids.contains("source:src/lib.rs"));
    }

    #[test]
    fn interactive_render_respects_height_budget() {
        let projection =
            build_knowledge_graph_view(&sample_projection(), KnowledgeGraphZoom::Source, None);
        let rendered = render_knowledge_graph_interactive(&projection, 100, 18);

        assert!(rendered.lines().count() <= 18);
        assert!(rendered.contains("Knowledge Graph"));
    }

    #[test]
    fn depth_label_text_uses_more_characters_for_outer_rings() {
        let title = "Mission With A Very Long Name";
        let max_depth = 4;
        let inner = depth_label_text(title, 1, max_depth);
        let outer = depth_label_text(title, max_depth, max_depth);

        assert!(inner.chars().count() < title.chars().count());
        assert_eq!(outer, title);
        assert!(inner.len() <= outer.len());
    }

    #[test]
    fn depth_label_max_chars_increases_monotonic() {
        let title_len = 24;
        let max_depth = 5;
        let inner_chars = depth_label_max_chars(title_len, 1, max_depth);
        let mid_chars = depth_label_max_chars(title_len, 3, max_depth);
        let outer_chars = depth_label_max_chars(title_len, max_depth, max_depth);

        assert!(inner_chars <= mid_chars);
        assert!(mid_chars <= outer_chars);
        assert!(outer_chars <= title_len);
    }

    #[test]
    fn depth_label_style_maps_depth_to_text_weight() {
        let build_node = |depth: usize, terminal: bool| KnowledgeGraphViewNode {
            id: format!("n:{depth}"),
            title: format!("Node {depth}"),
            kind: KnowledgeGraphNodeKind::Mission,
            state: None,
            parent_id: None,
            depth,
            terminal,
            signal_count: 0,
        };

        let inner = depth_label_style(&build_node(1, false), 5, Color::BrightCyan);
        let mid = depth_label_style(&build_node(3, false), 5, Color::BrightCyan);
        let outer = depth_label_style(&build_node(5, false), 5, Color::BrightCyan);
        let terminal = depth_label_style(&build_node(5, true), 5, Color::BrightCyan);

        assert_eq!(inner.intensity, txtplot::TextIntensity::Bold);
        assert_eq!(mid.intensity, txtplot::TextIntensity::Normal);
        assert_eq!(outer.intensity, txtplot::TextIntensity::Dim);
        assert_eq!(terminal.intensity, txtplot::TextIntensity::Normal);
    }

    #[test]
    fn source_zoom_double_helix_only_renders_between_source_and_knowledge_links() {
        let build_node = |kind: KnowledgeGraphNodeKind| KnowledgeGraphViewNode {
            id: format!("n:{kind:?}"),
            title: format!("{kind:?}"),
            kind,
            state: None,
            parent_id: None,
            depth: 5,
            terminal: false,
            signal_count: 0,
        };

        let knowledge = build_node(KnowledgeGraphNodeKind::Knowledge);
        let source = build_node(KnowledgeGraphNodeKind::SourceFile);
        let artifact = build_node(KnowledgeGraphNodeKind::Artifact);

        assert!(should_render_source_zoom_double_helix(
            KnowledgeGraphZoom::Source,
            &knowledge,
            &source,
            KnowledgeGraphViewLinkKind::Knowledge,
        ));
        assert!(should_render_source_zoom_double_helix(
            KnowledgeGraphZoom::Source,
            &source,
            &knowledge,
            KnowledgeGraphViewLinkKind::Knowledge,
        ));
        assert!(!should_render_source_zoom_double_helix(
            KnowledgeGraphZoom::Artifact,
            &knowledge,
            &source,
            KnowledgeGraphViewLinkKind::Knowledge,
        ));
        assert!(!should_render_source_zoom_double_helix(
            KnowledgeGraphZoom::Source,
            &knowledge,
            &source,
            KnowledgeGraphViewLinkKind::Hierarchy,
        ));
        assert!(!should_render_source_zoom_double_helix(
            KnowledgeGraphZoom::Source,
            &knowledge,
            &artifact,
            KnowledgeGraphViewLinkKind::Knowledge,
        ));
    }

    #[test]
    fn chart_width_uses_available_space() {
        assert_eq!(chart_width_for_mode(160, RenderMode::Static), 158);
        assert_eq!(
            chart_width_for_mode(160, RenderMode::Interactive { height: 20 }),
            158
        );
    }

    #[test]
    fn static_chart_height_scales_with_terminal_width() {
        let projection =
            build_knowledge_graph_view(&sample_projection(), KnowledgeGraphZoom::Source, None);
        let small = default_chart_height(&projection, 80);
        let large = default_chart_height(&projection, 160);

        assert!(large > small);
    }

    #[test]
    fn static_render_surfaces_structural_drift_summary() {
        let mut projection = sample_projection();
        projection.drift_inputs = StructuralDriftInputs {
            total_entities: 5,
            entities_with_artifacts: 3,
            entities_without_artifacts: 2,
            total_knowledge_units: 2,
            knowledge_with_source_attachments: 1,
            knowledge_without_source_attachments: 1,
            total_source_files: 7,
            source_files_with_attachments: 4,
            source_files_without_attachments: 3,
            total_project_docs: 3,
            linked_project_docs: 2,
            unlinked_project_docs: 1,
        };
        let view = build_knowledge_graph_view(&projection, KnowledgeGraphZoom::Source, None);
        let rendered = render_knowledge_graph(&view, 120);

        assert!(rendered.contains("Drift: 0.42 (elevated)"));
        assert!(rendered.contains("entities 3/5 | knowledge 1/2 | source 4/7 | docs 2/3"));
        assert!(rendered.contains("Context: 3 source files lack graph attachments"));
    }
}
