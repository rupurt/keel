//! Knowledge-graph visualization command.

use std::io::{self, IsTerminal, Write};
use std::path::Path;

use anyhow::Result;
use crossterm::cursor;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

use crate::cli::presentation::knowledge_graph::{
    KnowledgeGraphViewProjection, KnowledgeGraphZoom, build_knowledge_graph_view,
    render_knowledge_graph, render_knowledge_graph_interactive,
};
use crate::cli::presentation::terminal::{get_terminal_size, get_terminal_width};
use keel::infrastructure::knowledge_graph_store::save_knowledge_graph_cache;
use keel::infrastructure::loader::load_board;
use keel::read_model::knowledge_graph::{
    KnowledgeGraphProjection, build_knowledge_graph_projection, projection_input_hashes,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct KnowledgeGraphSnapshot {
    projection: KnowledgeGraphViewProjection,
    output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KnowledgeGraphViewState {
    zoom: KnowledgeGraphZoom,
    focus_id: Option<String>,
}

impl KnowledgeGraphViewState {
    fn new() -> Self {
        Self {
            zoom: KnowledgeGraphZoom::World,
            focus_id: None,
        }
    }

    fn zoom_in(&mut self) {
        self.zoom = self.zoom.zoom_in();
    }

    fn zoom_out(&mut self) {
        self.zoom = self.zoom.zoom_out();
    }

    fn clear_focus(&mut self) {
        self.focus_id = None;
    }

    fn cycle_focus(
        &mut self,
        projection: &KnowledgeGraphViewProjection,
        direction: FocusDirection,
    ) {
        let candidates = projection
            .nodes
            .iter()
            .filter(|node| {
                !matches!(
                    node.kind,
                    keel::read_model::knowledge_graph::KnowledgeGraphNodeKind::World
                )
            })
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

    fn focus_parent(&mut self, projection: &KnowledgeGraphViewProjection) {
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
        if parent_id == "world:board" {
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

struct KnowledgeGraphTerminalSession;

impl KnowledgeGraphTerminalSession {
    fn enter(stdout: &mut io::Stdout) -> Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
        Ok(Self)
    }
}

impl Drop for KnowledgeGraphTerminalSession {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen, cursor::Show);
    }
}

/// Run the graph command to visualize whole-project knowledge connections.
pub fn run(board_dir: &Path) -> Result<()> {
    run_with_dir(board_dir)
}

pub fn run_with_dir(board_dir: &Path) -> Result<()> {
    let projection = load_projection(board_dir)?;

    if should_run_interactive(io::stdin().is_terminal(), io::stdout().is_terminal()) {
        return run_interactive(projection);
    }

    let output = build_knowledge_graph_output_with_width(
        &projection,
        KnowledgeGraphZoom::World,
        None,
        get_terminal_width(),
    );
    print!("{output}");
    Ok(())
}

fn should_run_interactive(stdin_terminal: bool, stdout_terminal: bool) -> bool {
    stdin_terminal && stdout_terminal
}

fn run_interactive(projection: KnowledgeGraphProjection) -> Result<()> {
    let mut stdout = io::stdout();
    let _session = KnowledgeGraphTerminalSession::enter(&mut stdout)?;
    let mut state = KnowledgeGraphViewState::new();

    loop {
        let (width, height) = get_terminal_size();
        let snapshot = build_interactive_snapshot_with_size(
            &projection,
            state.zoom,
            state.focus_id.as_deref(),
            width,
            height,
        );

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

fn render_interactive_frame(snapshot: &KnowledgeGraphSnapshot) -> String {
    let mut lines = snapshot
        .output
        .lines()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    lines.push("Controls: +/- zoom  [/] cycle focus  u parent  c clear  q quit".to_string());
    lines.join("\r\n")
}

fn load_projection(board_dir: &Path) -> Result<KnowledgeGraphProjection> {
    let board = load_board(board_dir)?;
    let projection = build_knowledge_graph_projection(&board)?;
    let input_hashes = projection_input_hashes(&board, &projection)?;
    let _ = save_knowledge_graph_cache(
        board_dir,
        board.snapshot_version(),
        &projection,
        &input_hashes,
    )?;
    Ok(projection)
}

fn build_knowledge_graph_output_with_width(
    projection: &KnowledgeGraphProjection,
    zoom: KnowledgeGraphZoom,
    focus_id: Option<&str>,
    width: usize,
) -> String {
    let view = build_knowledge_graph_view(projection, zoom, focus_id);
    render_knowledge_graph(&view, width)
}

fn build_interactive_snapshot_with_size(
    projection: &KnowledgeGraphProjection,
    zoom: KnowledgeGraphZoom,
    focus_id: Option<&str>,
    width: usize,
    height: usize,
) -> KnowledgeGraphSnapshot {
    let view = build_knowledge_graph_view(projection, zoom, focus_id);
    KnowledgeGraphSnapshot {
        output: render_knowledge_graph_interactive(&view, width, height),
        projection: view,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::domain::model::StoryState;
    use keel::test_helpers::{TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage};
    use std::fs;

    fn graph_fixture() -> tempfile::TempDir {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One").status("active"))
            .epic(TestEpic::new("E1").title("Epic One").mission("M1").index(1))
            .voyage(
                TestVoyage::new("V1", "E1")
                    .title("Voyage One")
                    .status("in-progress")
                    .index(1),
            )
            .story(
                TestStory::new("S1")
                    .title("Story One")
                    .scope("E1/V1")
                    .index(1)
                    .status(StoryState::Backlog)
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] ship the graph slice\n"),
            )
            .build();

        fs::write(temp.path().join("README.md"), "# Root\n").unwrap();
        fs::create_dir_all(temp.path().join("knowledge")).unwrap();
        fs::write(
            temp.path().join("knowledge/graph.md"),
            r#"---
source_type: Adhoc
source: knowledge/graph.md
---

### 1AbCdE241: Graph Insight

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Graph |
| **Insight** | Surface the world model directly |
| **Suggested Action** | Render the graph |
| **Applies To** | src/lib.rs |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-12T00:00:00Z |
| **Score** | 0.90 |
| **Confidence** | 0.95 |
| **Applied** |  |
"#,
        )
        .unwrap();
        fs::create_dir_all(temp.path().join("src")).unwrap();
        fs::write(temp.path().join("src/lib.rs"), "pub fn world() {}\n").unwrap();

        temp
    }

    #[test]
    fn knowledge_graph_interactive_mode_uses_tty_controls_and_viewport() {
        let temp = graph_fixture();
        let projection = load_projection(temp.path()).unwrap();

        let interactive = build_interactive_snapshot_with_size(
            &projection,
            KnowledgeGraphZoom::World,
            None,
            100,
            18,
        );
        let frame = render_interactive_frame(&interactive);
        let fallback = build_knowledge_graph_output_with_width(
            &projection,
            KnowledgeGraphZoom::World,
            None,
            100,
        );

        assert!(should_run_interactive(true, true));
        assert!(!should_run_interactive(true, false));
        assert!(frame.contains("Knowledge Graph"));
        assert!(frame.contains("Controls: +/- zoom"));
        assert!(frame.lines().count() <= 18);
        assert!(fallback.contains("Visible:"));
        assert!(fallback.contains("Mission One"));
    }
}
