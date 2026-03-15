//! `keel topology` command adapter.

use std::io::{self, IsTerminal, Write};
use std::path::Path;

use anyhow::Result;
use crossterm::cursor;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

use crate::cli::presentation::terminal::{get_terminal_size, get_terminal_width};
use crate::cli::presentation::topology::{render_topology, render_topology_interactive};
use keel::infrastructure::loader::load_board;
use keel::read_model::world_map::{
    TopologyZoom, WorldMapBuildOptions, WorldMapNodeKind, WorldMapProjection,
    build_world_map_projection,
};

#[derive(Debug, Clone, PartialEq)]
struct TopologySnapshot {
    projection: WorldMapProjection,
    output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TopologyViewState {
    zoom: TopologyZoom,
    focus_id: Option<String>,
    include_done: bool,
}

impl TopologyViewState {
    fn new(zoom: TopologyZoom, focus_id: Option<&str>, include_done: bool) -> Self {
        Self {
            zoom,
            focus_id: focus_id.map(ToOwned::to_owned),
            include_done,
        }
    }

    fn zoom_in(&mut self) {
        self.zoom = self.zoom.zoom_in();
    }

    fn zoom_out(&mut self) {
        self.zoom = self.zoom.zoom_out();
    }

    fn toggle_done(&mut self) {
        self.include_done = !self.include_done;
    }

    fn clear_focus(&mut self) {
        self.focus_id = None;
    }

    fn cycle_focus(&mut self, projection: &WorldMapProjection, direction: FocusDirection) {
        let candidates = projection
            .nodes
            .iter()
            .filter(|node| node.kind != WorldMapNodeKind::World)
            .map(|node| node.id.as_str())
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            self.focus_id = None;
            return;
        }

        let next_index = match self.focus_id.as_deref() {
            Some(current) => {
                let current_index = candidates
                    .iter()
                    .position(|candidate| *candidate == current)
                    .unwrap_or(0);
                match direction {
                    FocusDirection::Next => (current_index + 1) % candidates.len(),
                    FocusDirection::Previous => {
                        (current_index + candidates.len().saturating_sub(1)) % candidates.len()
                    }
                }
            }
            None => match direction {
                FocusDirection::Next => 0,
                FocusDirection::Previous => candidates.len().saturating_sub(1),
            },
        };
        self.focus_id = Some(candidates[next_index].to_string());
    }

    fn focus_parent(&mut self, projection: &WorldMapProjection) {
        let Some(current_focus) = self.focus_id.as_deref() else {
            return;
        };
        let Some(node) = projection
            .nodes
            .iter()
            .find(|node| node.id == current_focus)
        else {
            self.focus_id = None;
            return;
        };
        let Some(parent_id) = node.parent_id.as_deref() else {
            self.focus_id = None;
            return;
        };
        let parent_is_world = projection
            .nodes
            .iter()
            .find(|candidate| candidate.id == parent_id)
            .is_some_and(|candidate| candidate.kind == WorldMapNodeKind::World);
        if parent_is_world {
            self.focus_id = None;
        } else {
            self.focus_id = Some(parent_id.to_string());
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FocusDirection {
    Previous,
    Next,
}

struct TopologyTerminalSession;

impl TopologyTerminalSession {
    fn enter(stdout: &mut io::Stdout) -> Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
        Ok(Self)
    }
}

impl Drop for TopologyTerminalSession {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen, cursor::Show);
    }
}

/// Run the topology command.
pub fn run(
    zoom: TopologyZoom,
    focus_id: Option<&str>,
    include_done: bool,
    static_output: bool,
) -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    run_with_dir(&board_dir, zoom, focus_id, include_done, static_output)
}

/// Run the topology command with an explicit board directory.
pub fn run_with_dir(
    board_dir: &Path,
    zoom: TopologyZoom,
    focus_id: Option<&str>,
    include_done: bool,
    static_output: bool,
) -> Result<()> {
    if should_run_interactive(static_output) {
        return run_interactive_with_dir(
            board_dir,
            TopologyViewState::new(zoom, focus_id, include_done),
        );
    }

    let output = build_topology_output(board_dir, zoom, focus_id, include_done)?;
    print!("{output}");
    Ok(())
}

fn should_run_interactive(static_output: bool) -> bool {
    !static_output && io::stdin().is_terminal() && io::stdout().is_terminal()
}

fn run_interactive_with_dir(board_dir: &Path, mut state: TopologyViewState) -> Result<()> {
    let mut stdout = io::stdout();
    let _session = TopologyTerminalSession::enter(&mut stdout)?;

    loop {
        let (width, height) = get_terminal_size();
        let snapshot = match build_interactive_topology_snapshot_with_size(
            board_dir,
            state.zoom,
            state.focus_id.as_deref(),
            state.include_done,
            width,
            height,
        ) {
            Ok(snapshot) => snapshot,
            Err(error) if state.focus_id.is_some() => {
                state.clear_focus();
                build_interactive_topology_snapshot_with_size(
                    board_dir,
                    state.zoom,
                    state.focus_id.as_deref(),
                    state.include_done,
                    width,
                    height,
                )
                .or(Err(error))?
            }
            Err(error) => return Err(error),
        };

        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All)
        )?;
        write!(stdout, "{}", render_interactive_frame(&snapshot))?;
        stdout.flush()?;

        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        match key.code {
            KeyCode::Char('q') => break,
            KeyCode::Esc => {
                if state.focus_id.is_some() {
                    state.clear_focus();
                } else {
                    break;
                }
            }
            KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Up => state.zoom_in(),
            KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::Down => state.zoom_out(),
            KeyCode::Char('d') => state.toggle_done(),
            KeyCode::Char('c') => state.clear_focus(),
            KeyCode::Char('u') | KeyCode::Backspace => state.focus_parent(&snapshot.projection),
            KeyCode::Left | KeyCode::Char('[') => {
                state.cycle_focus(&snapshot.projection, FocusDirection::Previous);
            }
            KeyCode::Right | KeyCode::Char(']') => {
                state.cycle_focus(&snapshot.projection, FocusDirection::Next);
            }
            _ => {}
        }
    }

    Ok(())
}

fn render_interactive_frame(snapshot: &TopologySnapshot) -> String {
    let mut lines = snapshot
        .output
        .lines()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    lines
        .push("Controls: +/- zoom  [/] cycle focus  u parent  c clear  d done  q quit".to_string());
    lines.join("\r\n")
}

fn build_topology_output(
    board_dir: &Path,
    zoom: TopologyZoom,
    focus_id: Option<&str>,
    include_done: bool,
) -> Result<String> {
    build_topology_output_with_width(
        board_dir,
        zoom,
        focus_id,
        include_done,
        get_terminal_width(),
    )
}

fn build_topology_output_with_width(
    board_dir: &Path,
    zoom: TopologyZoom,
    focus_id: Option<&str>,
    include_done: bool,
    width: usize,
) -> Result<String> {
    Ok(build_topology_snapshot_with_width(board_dir, zoom, focus_id, include_done, width)?.output)
}

fn build_topology_snapshot_with_width(
    board_dir: &Path,
    zoom: TopologyZoom,
    focus_id: Option<&str>,
    include_done: bool,
    width: usize,
) -> Result<TopologySnapshot> {
    let board = load_board(board_dir)?;
    let projection = build_world_map_projection(
        &board,
        WorldMapBuildOptions {
            zoom,
            focus_id,
            include_done,
            reference_time: None,
        },
    )?;

    Ok(TopologySnapshot {
        output: render_topology(&projection, width),
        projection,
    })
}

fn build_interactive_topology_snapshot_with_size(
    board_dir: &Path,
    zoom: TopologyZoom,
    focus_id: Option<&str>,
    include_done: bool,
    width: usize,
    height: usize,
) -> Result<TopologySnapshot> {
    let board = load_board(board_dir)?;
    let projection = build_world_map_projection(
        &board,
        WorldMapBuildOptions {
            zoom,
            focus_id,
            include_done,
            reference_time: None,
        },
    )?;

    Ok(TopologySnapshot {
        output: render_topology_interactive(&projection, width, height),
        projection,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::domain::model::StoryState;
    use keel::read_model::knowledge_graph::{
        DriftFacetKind, DriftFacetSummary, DriftSurfaceSummary,
    };
    use keel::test_helpers::{
        TestBearing, TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage,
    };

    fn sample_drift() -> DriftSurfaceSummary {
        DriftSurfaceSummary {
            coefficient: 0.0,
            facets: vec![
                DriftFacetSummary {
                    kind: DriftFacetKind::EntityArtifacts,
                    covered: 0,
                    total: 0,
                },
                DriftFacetSummary {
                    kind: DriftFacetKind::KnowledgeProvenance,
                    covered: 0,
                    total: 0,
                },
                DriftFacetSummary {
                    kind: DriftFacetKind::SourceAttachments,
                    covered: 0,
                    total: 0,
                },
                DriftFacetSummary {
                    kind: DriftFacetKind::ProjectDocs,
                    covered: 0,
                    total: 0,
                },
            ],
        }
    }

    fn topology_fixture() -> tempfile::TempDir {
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
    fn topology_command_defaults_to_world_map() {
        let temp = topology_fixture();

        let output =
            build_topology_output_with_width(temp.path(), TopologyZoom::World, None, false, 100)
                .unwrap();

        assert!(output.contains("Topology World Map"));
        assert!(output.contains("Zoom: world"));
        assert!(output.contains("Mission One"));
        assert!(output.contains("Payments Research"));
        assert!(!output.contains("Voyage One"));
        assert!(!output.contains("Story One"));
    }

    #[test]
    fn topology_command_zoom_story_surfaces_story_signals() {
        let temp = topology_fixture();

        let output =
            build_topology_output_with_width(temp.path(), TopologyZoom::Story, None, false, 100)
                .unwrap();

        assert!(output.contains("Zoom: story"));
        assert!(output.contains("Blocked Story"));
        assert!(output.contains("blocked by S1"));
        assert!(output.contains("Stories"));
    }

    #[test]
    fn topology_command_focus_filters_to_selected_branch() {
        let temp = topology_fixture();

        let output = build_topology_output_with_width(
            temp.path(),
            TopologyZoom::Mission,
            Some("M1"),
            false,
            100,
        )
        .unwrap();

        assert!(output.contains("focus M1"));
        assert!(output.contains("Mission One"));
        assert!(output.contains("Epic One"));
        assert!(!output.contains("Mission Two"));
        assert!(!output.contains("Payments Research"));
    }

    #[test]
    fn topology_command_include_done_reveals_terminal_descendants() {
        let temp = topology_fixture();

        let hidden =
            build_topology_output_with_width(temp.path(), TopologyZoom::Mission, None, false, 100)
                .unwrap();
        let shown =
            build_topology_output_with_width(temp.path(), TopologyZoom::Mission, None, true, 100)
                .unwrap();

        assert!(!hidden.contains("Completed Epic"));
        assert!(shown.contains("Completed Epic"));
    }

    #[test]
    fn topology_view_state_cycles_visible_focus_nodes() {
        let temp = topology_fixture();
        let snapshot = build_topology_snapshot_with_width(
            temp.path(),
            TopologyZoom::Mission,
            None,
            false,
            100,
        )
        .unwrap();
        let mut state = TopologyViewState::new(TopologyZoom::Mission, None, false);

        state.cycle_focus(&snapshot.projection, FocusDirection::Next);
        assert_eq!(state.focus_id.as_deref(), Some("M1"));

        state.cycle_focus(&snapshot.projection, FocusDirection::Previous);
        assert_eq!(state.focus_id.as_deref(), Some("E1"));
    }

    #[test]
    fn topology_view_state_focus_parent_climbs_and_clears_world_parent() {
        let temp = topology_fixture();
        let mut state = TopologyViewState::new(TopologyZoom::Mission, Some("E1"), false);

        let epic_snapshot = build_topology_snapshot_with_width(
            temp.path(),
            TopologyZoom::Mission,
            state.focus_id.as_deref(),
            false,
            100,
        )
        .unwrap();
        state.focus_parent(&epic_snapshot.projection);
        assert_eq!(state.focus_id.as_deref(), Some("M1"));

        let mission_snapshot = build_topology_snapshot_with_width(
            temp.path(),
            TopologyZoom::Mission,
            state.focus_id.as_deref(),
            false,
            100,
        )
        .unwrap();
        state.focus_parent(&mission_snapshot.projection);
        assert!(state.focus_id.is_none());
    }

    #[test]
    fn interactive_frame_uses_crlf_line_endings() {
        let snapshot = TopologySnapshot {
            projection: WorldMapProjection {
                zoom: TopologyZoom::World,
                focus: None,
                nodes: Vec::new(),
                links: Vec::new(),
                kind_counts: Vec::new(),
                layers: Vec::new(),
                highlights: Vec::new(),
                drift: sample_drift(),
            },
            output: "line one\nline two".to_string(),
        };

        let frame = render_interactive_frame(&snapshot);

        assert!(frame.contains("line one\r\nline two\r\nControls:"));
        assert!(!frame.contains("line two\nControls:"));
    }
}
