//! Topology world-map presentation helpers.

use std::collections::{BTreeMap, HashMap};
use std::f64::consts::{FRAC_PI_2, TAU};
use std::fmt::Write as _;

use colored::Color;
use txtplot::{ChartContext, TextStyle};

use crate::cli::presentation::drift_surface::{
    render_drift_context, render_drift_coverage, render_drift_overview,
};
use keel::read_model::world_map::{
    WorldMapLinkKind, WorldMapNode, WorldMapNodeKind, WorldMapProjection,
};

const LABEL_NODE_LIMIT: usize = 18;
const MIN_INTERACTIVE_CHART_HEIGHT: usize = 6;
const LABEL_SIZE_MIN_RATIO: f64 = 0.45;
const LABEL_SIZE_MIN_CHARS: usize = 6;
struct PositionedNode<'a> {
    node: &'a WorldMapNode,
    x: isize,
    y: isize,
    angle: f64,
}

/// Render a world-map topology projection for terminal output.
pub fn render_topology(projection: &WorldMapProjection, width: usize) -> String {
    render_topology_with_mode(projection, width, RenderMode::Static)
}

/// Render a compact topology frame for interactive terminal output.
pub fn render_topology_interactive(
    projection: &WorldMapProjection,
    width: usize,
    height: usize,
) -> String {
    render_topology_with_mode(projection, width, RenderMode::Interactive { height })
        .replace('\u{2800}', " ")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderMode {
    Static,
    Interactive { height: usize },
}

fn render_topology_with_mode(
    projection: &WorldMapProjection,
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
        RenderMode::Static => available,
        RenderMode::Interactive { .. } => available,
    }
}

fn chart_height_for_mode(
    projection: &WorldMapProjection,
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

fn default_chart_height(projection: &WorldMapProjection, width: usize) -> usize {
    if projection.zoom.to_string() == "story" {
        26
    } else if width >= 120 {
        24
    } else if width >= 96 {
        22
    } else {
        18
    }
}

fn interactive_summary_budget(height: usize) -> usize {
    height
        .saturating_sub(MIN_INTERACTIVE_CHART_HEIGHT + 4)
        .max(1)
}

fn chart_title(projection: &WorldMapProjection, width: usize) -> String {
    let mut title = format!("Topology World Map · zoom {}", projection.zoom);
    if let Some(focus) = &projection.focus {
        title.push_str(&format!(" · focus {}", focus.id));
    }
    truncate_text(&title, width.saturating_sub(2).max(1))
}

fn render_summary(projection: &WorldMapProjection) -> String {
    let mut out = String::new();

    let visible = projection
        .kind_counts
        .iter()
        .map(|count| {
            format!(
                "{} {}",
                count.count,
                title_case_label(if count.count == 1 {
                    count.kind.singular_label()
                } else {
                    count.kind.plural_label()
                })
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    writeln!(
        out,
        "Zoom: {} | Focus: {}",
        projection.zoom,
        projection
            .focus
            .as_ref()
            .map(|focus| match &focus.timer {
                Some(timer) => format!("{} ({}, {timer})", focus.title, focus.state),
                None => format!("{} ({})", focus.title, focus.state),
            })
            .unwrap_or_else(|| "board-wide".to_string())
    )
    .unwrap();
    writeln!(
        out,
        "Visible: {}",
        if visible.is_empty() {
            "world only".to_string()
        } else {
            visible
        }
    )
    .unwrap();
    writeln!(
        out,
        "{} | {}",
        render_drift_overview("Drift Radar", &projection.drift),
        render_drift_coverage(&projection.drift)
    )
    .unwrap();
    if let Some(context) = render_drift_context(&projection.drift, 2) {
        writeln!(out, "Context: {context}").unwrap();
    }

    if !projection.layers.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "Layers:").unwrap();
        for layer in &projection.layers {
            writeln!(
                out,
                "  {}. {} ({})",
                layer.depth,
                title_case_compound_label(&layer.label),
                layer.count
            )
            .unwrap();
        }
    }

    writeln!(out).unwrap();
    writeln!(
        out,
        "Legend: missions, epics, bearings, ADRs, voyages, and stories occupy progressively deeper rings; routines float outside their target scope as pulsing buoys; dim nodes are terminal."
    )
    .unwrap();

    if !projection.highlights.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "Highlights:").unwrap();
        for highlight in &projection.highlights {
            writeln!(out, "  - {highlight}").unwrap();
        }
    }

    out.trim_end().to_string()
}

fn render_interactive_summary(
    projection: &WorldMapProjection,
    width: usize,
    max_lines: usize,
) -> String {
    let mut lines = Vec::new();

    let focus = projection
        .focus
        .as_ref()
        .map(|focus| match &focus.timer {
            Some(timer) => format!("{} ({}, {timer})", focus.title, focus.state),
            None => format!("{} ({})", focus.title, focus.state),
        })
        .unwrap_or_else(|| "board-wide".to_string());
    lines.push(truncate_text(
        &format!("Zoom: {} | Focus: {focus}", projection.zoom),
        width,
    ));

    let visible = projection
        .kind_counts
        .iter()
        .map(|count| {
            format!(
                "{} {}",
                count.count,
                title_case_label(if count.count == 1 {
                    count.kind.singular_label()
                } else {
                    count.kind.plural_label()
                })
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    lines.push(truncate_text(
        &format!(
            "Visible: {}",
            if visible.is_empty() {
                "world only".to_string()
            } else {
                visible
            }
        ),
        width,
    ));
    lines.push(truncate_text(
        &match render_drift_context(&projection.drift, 1) {
            Some(context) => format!(
                "{} | {context}",
                render_drift_overview("Drift Radar", &projection.drift)
            ),
            None => render_drift_overview("Drift Radar", &projection.drift),
        },
        width,
    ));

    if !projection.layers.is_empty() && lines.len() < max_lines {
        lines.push(truncate_text("Layers:", width));
        for layer in &projection.layers {
            if lines.len() >= max_lines {
                break;
            }
            let layer_text = format!(
                "  {}. {} ({})",
                layer.depth,
                title_case_compound_label(&layer.label),
                layer.count
            );
            let remaining = max_lines - lines.len();
            lines.extend(
                wrap_text_lines(&layer_text, width, "     ")
                    .into_iter()
                    .take(remaining),
            );
        }
    }

    if lines.len() < max_lines
        && let Some(highlight) = projection.highlights.first()
    {
        lines.push(truncate_text(&format!("Highlight: {highlight}"), width));
    }

    lines
        .into_iter()
        .take(max_lines)
        .collect::<Vec<_>>()
        .join("\n")
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

fn wrap_text_lines(text: &str, width: usize, continuation_indent: &str) -> Vec<String> {
    if width == 0 {
        return Vec::new();
    }

    let mut remaining = text.trim();
    let mut lines = Vec::new();
    let mut indent = "";

    while !remaining.is_empty() {
        let available = width.saturating_sub(indent.chars().count());
        if available == 0 {
            break;
        }

        let mut segment = remaining.to_string();
        if segment.chars().count() > available {
            let mut split_idx = 0usize;
            let mut candidate = 0usize;
            for (idx, ch) in remaining.char_indices() {
                if remaining[..idx].chars().count() >= available {
                    break;
                }
                if ch.is_whitespace() {
                    candidate = idx;
                }
                split_idx = idx + ch.len_utf8();
            }

            let take_idx = if candidate > 0 { candidate } else { split_idx };
            segment = remaining[..take_idx].trim_end().to_string();
            remaining = remaining[take_idx..].trim_start();
        } else {
            remaining = "";
        }

        lines.push(format!("{indent}{segment}"));
        indent = continuation_indent;
    }

    if lines.is_empty() {
        lines.push(text.chars().take(width).collect());
    }

    lines
}

fn layout_positions<'a>(
    projection: &'a WorldMapProjection,
    chart: &ChartContext,
) -> HashMap<String, PositionedNode<'a>> {
    let child_map = build_child_map(&projection.nodes);
    let Some(root) = projection
        .nodes
        .iter()
        .find(|node| node.kind == WorldMapNodeKind::World)
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
        &projection.nodes,
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
    child_map: &HashMap<String, Vec<&'a WorldMapNode>>,
    nodes: &'a [WorldMapNode],
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
        let radius = radius_step * child.depth as f64 + buoy_radius_offset(child, radius_step);
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
            nodes,
            weights,
            positions,
        );

        cursor += child_span;
    }

    let _ = nodes;
}

fn buoy_radius_offset(node: &WorldMapNode, radius_step: f64) -> f64 {
    if node.kind == WorldMapNodeKind::Routine {
        radius_step * 0.38
    } else {
        0.0
    }
}

fn subtree_weight(
    id: &str,
    child_map: &HashMap<String, Vec<&WorldMapNode>>,
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
    projection: &WorldMapProjection,
    positions: &HashMap<String, PositionedNode<'_>>,
) {
    let center_x = (chart.canvas.pixel_width() / 2) as isize;
    let center_y = (chart.canvas.pixel_height() / 2) as isize;
    let mut depth_radii: BTreeMap<usize, Vec<isize>> = projection
        .nodes
        .iter()
        .filter(|node| {
            node.kind != WorldMapNodeKind::World
                && node.kind != WorldMapNodeKind::Routine
                && node.depth > 0
        })
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
    projection: &WorldMapProjection,
    positions: &HashMap<String, PositionedNode<'_>>,
) {
    for link in &projection.links {
        if link.kind == WorldMapLinkKind::Hierarchy && projection.focus.is_none() {
            continue;
        }

        let Some(from) = positions.get(&link.from_id) else {
            continue;
        };
        let Some(to) = positions.get(&link.to_id) else {
            continue;
        };

        let color = match link.kind {
            WorldMapLinkKind::Hierarchy => Some(Color::BrightBlack),
            WorldMapLinkKind::Dependency => Some(Color::BrightRed),
        };
        chart.canvas.line_screen(from.x, from.y, to.x, to.y, color);
    }
}

fn draw_nodes(
    chart: &mut ChartContext,
    projection: &WorldMapProjection,
    positions: &HashMap<String, PositionedNode<'_>>,
) {
    for positioned in positions.values() {
        let color = node_color(positioned.node);
        let radius = node_radius(positioned.node.kind);
        draw_circle_filled_screen(chart, positioned.x, positioned.y, radius, Some(color));
        draw_buoy_pulse(chart, positioned.x, positioned.y, positioned.node);

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
                Some(Color::BrightYellow),
            );
        }
    }
}

fn draw_buoy_pulse(
    chart: &mut ChartContext,
    center_x: isize,
    center_y: isize,
    node: &WorldMapNode,
) {
    let Some(color) = buoy_pulse_color(node) else {
        return;
    };

    for radius in buoy_pulse_radii(node) {
        draw_circle_screen(chart, center_x, center_y, *radius, Some(color));
    }
}

fn buoy_pulse_color(node: &WorldMapNode) -> Option<Color> {
    if node.kind != WorldMapNodeKind::Routine {
        return None;
    }

    match node.state.as_str() {
        "due" => Some(Color::BrightYellow),
        "invalid" => Some(Color::BrightRed),
        "upcoming" => Some(Color::BrightCyan),
        _ => Some(Color::Cyan),
    }
}

fn buoy_pulse_radii(node: &WorldMapNode) -> &'static [isize] {
    match node.state.as_str() {
        "due" | "invalid" => &[3, 5],
        "upcoming" => &[3],
        _ => &[2],
    }
}

fn draw_labels(
    chart: &mut ChartContext,
    projection: &WorldMapProjection,
    positions: &HashMap<String, PositionedNode<'_>>,
) {
    let label_all = projection.nodes.len() <= LABEL_NODE_LIMIT;
    let dense_map = projection.nodes.len() > LABEL_NODE_LIMIT
        || projection
            .nodes
            .iter()
            .filter(|node| node.depth == 1)
            .count()
            > 10;
    let deepest_visible_depth = projection
        .nodes
        .iter()
        .map(|node| node.depth)
        .max()
        .unwrap_or(0);
    for positioned in positions.values() {
        if positioned.node.kind == WorldMapNodeKind::World {
            draw_text_screen(
                chart,
                "Keel World",
                positioned.x - 10,
                positioned.y - 2,
                TextStyle::new().with_foreground(Color::White).bold(),
            );
            continue;
        }

        let should_label = projection
            .focus
            .as_ref()
            .is_some_and(|focus| focus.id == positioned.node.id)
            || !positioned.node.signals.is_empty()
            || (positioned.node.kind == WorldMapNodeKind::Routine && !dense_map)
            || (!positioned.node.terminal && positioned.node.depth <= 2)
            || (!dense_map && positioned.node.depth == 1)
            || (label_all && positioned.node.depth == deepest_visible_depth);
        if !should_label {
            continue;
        }

        let label = depth_label_text(
            positioned.node.title.as_str(),
            positioned.node.depth,
            deepest_visible_depth,
        );
        let style = depth_label_style(
            positioned.node,
            deepest_visible_depth,
            node_color(positioned.node),
        );
        let (offset_x, offset_y) = label_offsets(positioned.node, &label, positioned.angle);
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

fn depth_label_style(node: &WorldMapNode, max_depth: usize, base_color: Color) -> TextStyle {
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

fn label_offsets(node: &WorldMapNode, label: &str, angle: f64) -> (isize, isize) {
    let horizontal_gap = node_radius(node.kind) * 2 + 4;
    let offset_x = if angle.cos() >= 0.0 {
        horizontal_gap
    } else {
        -((label.chars().count() as isize) * 2 + horizontal_gap)
    };
    (offset_x, 0)
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

fn title_case_compound_label(label: &str) -> String {
    label
        .split(" + ")
        .map(title_case_label)
        .collect::<Vec<_>>()
        .join(" + ")
}

fn build_child_map(nodes: &[WorldMapNode]) -> HashMap<String, Vec<&WorldMapNode>> {
    let mut child_map: HashMap<String, Vec<&WorldMapNode>> = HashMap::new();
    for node in nodes {
        if let Some(parent_id) = &node.parent_id {
            child_map.entry(parent_id.clone()).or_default().push(node);
        }
    }

    for children in child_map.values_mut() {
        children.sort_by(|left, right| {
            kind_sort_rank(left.kind)
                .cmp(&kind_sort_rank(right.kind))
                .then_with(|| left.order_index.cmp(&right.order_index))
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
    let text_width = text.chars().count() as isize;
    let max_start_col = (chart.canvas.width as isize - text_width).clamp(0, max_col);
    let col = (x_px / 2).clamp(0, max_start_col) as usize;
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

fn kind_sort_rank(kind: WorldMapNodeKind) -> u8 {
    match kind {
        WorldMapNodeKind::World => 0,
        WorldMapNodeKind::Mission => 1,
        WorldMapNodeKind::Epic => 2,
        WorldMapNodeKind::Bearing => 3,
        WorldMapNodeKind::Adr => 4,
        WorldMapNodeKind::Voyage => 5,
        WorldMapNodeKind::Story => 6,
        WorldMapNodeKind::Routine => 7,
        WorldMapNodeKind::Watch => 8,
        WorldMapNodeKind::Heartbeat => 9,
    }
}

fn node_color(node: &WorldMapNode) -> Color {
    if node.terminal {
        return Color::BrightBlack;
    }

    match node.kind {
        WorldMapNodeKind::World => Color::White,
        WorldMapNodeKind::Mission => Color::BrightCyan,
        WorldMapNodeKind::Epic => Color::BrightBlue,
        WorldMapNodeKind::Bearing => Color::BrightYellow,
        WorldMapNodeKind::Adr => Color::BrightMagenta,
        WorldMapNodeKind::Voyage => Color::BrightGreen,
        WorldMapNodeKind::Story => {
            if !node.signals.is_empty() {
                Color::BrightRed
            } else {
                Color::White
            }
        }
        WorldMapNodeKind::Routine => match node.state.as_str() {
            "due" => Color::BrightYellow,
            "invalid" => Color::BrightRed,
            "upcoming" => Color::BrightCyan,
            _ => Color::Cyan,
        },
        WorldMapNodeKind::Watch => Color::BrightYellow,
        WorldMapNodeKind::Heartbeat => Color::BrightYellow,
    }
}

fn node_radius(kind: WorldMapNodeKind) -> isize {
    match kind {
        WorldMapNodeKind::World => 4,
        WorldMapNodeKind::Mission => 3,
        WorldMapNodeKind::Epic | WorldMapNodeKind::Bearing | WorldMapNodeKind::Adr => 2,
        WorldMapNodeKind::Voyage => 2,
        WorldMapNodeKind::Story => 1,
        WorldMapNodeKind::Routine => 1,
        WorldMapNodeKind::Watch => 2,
        WorldMapNodeKind::Heartbeat => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::read_model::knowledge_graph::{
        DriftFacetKind, DriftFacetSummary, DriftSurfaceSummary,
    };
    use keel::read_model::world_map::{
        TopologyZoom, WorldMapKindCount, WorldMapLayer, WorldMapLink, WorldMapNode,
    };

    fn sample_drift() -> DriftSurfaceSummary {
        DriftSurfaceSummary {
            coefficient: 0.41,
            facets: vec![
                DriftFacetSummary {
                    kind: DriftFacetKind::EntityArtifacts,
                    covered: 3,
                    total: 5,
                },
                DriftFacetSummary {
                    kind: DriftFacetKind::KnowledgeProvenance,
                    covered: 1,
                    total: 2,
                },
                DriftFacetSummary {
                    kind: DriftFacetKind::SourceAttachments,
                    covered: 4,
                    total: 7,
                },
                DriftFacetSummary {
                    kind: DriftFacetKind::ProjectDocs,
                    covered: 2,
                    total: 3,
                },
            ],
        }
    }

    fn test_projection() -> WorldMapProjection {
        WorldMapProjection {
            zoom: TopologyZoom::Story,
            focus: None,
            nodes: vec![
                WorldMapNode {
                    id: "__world__".to_string(),
                    title: "Keel World".to_string(),
                    kind: WorldMapNodeKind::World,
                    state: "live".to_string(),
                    parent_id: None,
                    depth: 0,
                    terminal: false,
                    order_index: Some(0),
                    summary: None,
                    timer: None,
                    signals: Vec::new(),
                },
                WorldMapNode {
                    id: "M1".to_string(),
                    title: "Mission With A Very Long Name".to_string(),
                    kind: WorldMapNodeKind::Mission,
                    state: "active".to_string(),
                    parent_id: Some("__world__".to_string()),
                    depth: 1,
                    terminal: false,
                    order_index: Some(1),
                    summary: Some("1/2 open strategic entities".to_string()),
                    timer: Some("12h 30m".to_string()),
                    signals: Vec::new(),
                },
            ],
            links: vec![WorldMapLink {
                from_id: "__world__".to_string(),
                to_id: "M1".to_string(),
                kind: WorldMapLinkKind::Hierarchy,
            }],
            kind_counts: vec![WorldMapKindCount {
                kind: WorldMapNodeKind::Mission,
                count: 1,
            }],
            layers: vec![WorldMapLayer {
                depth: 1,
                label: "missions".to_string(),
                count: 1,
            }],
            highlights: vec![String::from(
                "mission Mission With A Very Long Name (active) · 12h 30m · 1/2 open strategic entities",
            )],
            drift: sample_drift(),
        }
    }

    #[test]
    fn interactive_render_respects_viewport_height() {
        let rendered = render_topology_interactive(&test_projection(), 80, 18);
        assert!(rendered.lines().count() <= 18);
    }

    #[test]
    fn interactive_chart_uses_full_available_height_budget() {
        let mut projection = test_projection();
        projection.zoom = TopologyZoom::World;

        let chart_height =
            chart_height_for_mode(&projection, 120, 4, RenderMode::Interactive { height: 24 });

        assert_eq!(chart_height, 16);
    }

    #[test]
    fn interactive_chart_uses_full_available_width_budget() {
        let chart_width = chart_width_for_mode(160, RenderMode::Interactive { height: 24 });

        assert_eq!(chart_width, 158);
    }

    #[test]
    fn interactive_summary_truncates_to_viewport_width() {
        let mut projection = test_projection();
        projection.layers = vec![WorldMapLayer {
            depth: 1,
            label: "missions + epics + bearings + voyages + stories".to_string(),
            count: 5,
        }];

        let summary = render_interactive_summary(&projection, 44, interactive_summary_budget(18));
        let summary_lines = summary.lines().collect::<Vec<_>>();
        let summary_text = summary_lines
            .join(" ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        assert!(!summary_lines.is_empty());
        assert!(summary_lines.iter().all(|line| line.chars().count() <= 44));
        assert!(summary_lines.iter().any(|line| line == &"Layers:"));
        assert!(summary_text.contains("Missions + Epics + Bearings"));
        assert!(summary_text.contains("Voyages + Stories"));
    }

    #[test]
    fn static_render_keeps_full_node_titles_without_ellipsis() {
        let mut projection = test_projection();
        projection.highlights.clear();

        let rendered = render_topology(&projection, 100);

        assert!(rendered.contains("Mission With A Very Long Name"));
        assert!(!rendered.contains("Mission With A…"));
    }

    #[test]
    fn label_offsets_keep_node_labels_inline_with_nodes() {
        let mission = WorldMapNode {
            id: "M1".to_string(),
            title: "Mission One".to_string(),
            kind: WorldMapNodeKind::Mission,
            state: "active".to_string(),
            parent_id: Some("__world__".to_string()),
            depth: 1,
            terminal: false,
            order_index: Some(1),
            summary: None,
            timer: None,
            signals: Vec::new(),
        };

        let (offset_x, offset_y) = label_offsets(&mission, &mission.title, 0.0);

        assert!(offset_x > 0);
        assert_eq!(offset_y, 0);
    }

    #[test]
    fn depth_label_text_uses_more_characters_for_outer_rings() {
        let title = "Mission With A Very Long Name";
        let max_depth = 4;
        let inner = depth_label_text(title, 1, max_depth);
        let outer = depth_label_text(title, max_depth, max_depth);
        let chars = title.chars().count();

        assert!(inner.chars().count() < chars);
        assert_eq!(outer, title);
        assert!(inner.len() <= outer.len());
    }

    #[test]
    fn depth_label_max_chars_increases_monotonic() {
        let title_len = 24;
        let max_depth = 4;

        let inner_chars = depth_label_max_chars(title_len, 1, max_depth);
        let mid_chars = depth_label_max_chars(title_len, 2, max_depth);
        let outer_chars = depth_label_max_chars(title_len, max_depth, max_depth);

        assert!(inner_chars <= mid_chars);
        assert!(mid_chars <= outer_chars);
        assert!(outer_chars <= title_len);
    }

    #[test]
    fn depth_label_style_maps_depth_to_text_weight() {
        let build_node = |depth: usize, terminal: bool| WorldMapNode {
            id: format!("n:{depth}"),
            title: format!("Node {depth}"),
            kind: WorldMapNodeKind::Mission,
            state: "active".to_string(),
            parent_id: Some("parent".to_string()),
            depth,
            terminal,
            order_index: Some(1),
            summary: None,
            timer: None,
            signals: Vec::new(),
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
    fn static_render_surfaces_drift_radar_summary() {
        let rendered = render_topology(&test_projection(), 120);

        assert!(rendered.contains("Drift Radar: 0.41 (elevated)"));
        assert!(rendered.contains("entities 3/5 | knowledge 1/2 | source 4/7 | docs 2/3"));
    }

    #[test]
    fn routine_buoys_render_outside_their_parent_ring() {
        let projection = WorldMapProjection {
            zoom: TopologyZoom::Story,
            focus: None,
            nodes: vec![
                WorldMapNode {
                    id: "__world__".to_string(),
                    title: "Keel World".to_string(),
                    kind: WorldMapNodeKind::World,
                    state: "live".to_string(),
                    parent_id: None,
                    depth: 0,
                    terminal: false,
                    order_index: Some(0),
                    summary: None,
                    timer: None,
                    signals: Vec::new(),
                },
                WorldMapNode {
                    id: "V1".to_string(),
                    title: "Voyage One".to_string(),
                    kind: WorldMapNodeKind::Voyage,
                    state: "in-progress".to_string(),
                    parent_id: Some("__world__".to_string()),
                    depth: 1,
                    terminal: false,
                    order_index: Some(1),
                    summary: None,
                    timer: None,
                    signals: Vec::new(),
                },
                WorldMapNode {
                    id: "S1".to_string(),
                    title: "Story One".to_string(),
                    kind: WorldMapNodeKind::Story,
                    state: "backlog".to_string(),
                    parent_id: Some("V1".to_string()),
                    depth: 2,
                    terminal: false,
                    order_index: Some(1),
                    summary: None,
                    timer: None,
                    signals: Vec::new(),
                },
                WorldMapNode {
                    id: "R1".to_string(),
                    title: "Weekly Review".to_string(),
                    kind: WorldMapNodeKind::Routine,
                    state: "due".to_string(),
                    parent_id: Some("V1".to_string()),
                    depth: 2,
                    terminal: false,
                    order_index: Some(2),
                    summary: Some("targets E1/V1".to_string()),
                    timer: Some("due now".to_string()),
                    signals: vec!["due now".to_string()],
                },
            ],
            links: vec![],
            kind_counts: vec![],
            layers: vec![],
            highlights: vec![],
            drift: sample_drift(),
        };

        let chart = ChartContext::new(120, 24);
        let positions = layout_positions(&projection, &chart);
        let center_x = (chart.canvas.pixel_width() / 2) as isize;
        let center_y = (chart.canvas.pixel_height() / 2) as isize;
        let story = positions.get("S1").unwrap();
        let routine = positions.get("R1").unwrap();
        let story_radius =
            (((story.x - center_x).pow(2) + (story.y - center_y).pow(2)) as f64).sqrt();
        let routine_radius =
            (((routine.x - center_x).pow(2) + (routine.y - center_y).pow(2)) as f64).sqrt();

        assert!(routine_radius > story_radius);
    }
}
