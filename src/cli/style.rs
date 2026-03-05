//! Terminal styling for keel output
//!
//! Provides consistent color theming across commands.
//! Palette: subdued with strategic pops of color on what matters.

use owo_colors::OwoColorize;
use regex::Regex;
use std::sync::LazyLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

use crate::domain::model::EpicState;
use crate::domain::model::StoryState;
use crate::domain::model::StoryType;
use crate::domain::state_machine::voyage::VoyageState;

/// Regex for SRS requirement references like [SRS-01/AC-01]
pub static AC_REQ_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[(SRS-\d+)/AC-\d+\]").unwrap());

/// Color a stage label by its workflow meaning
pub fn styled_stage(stage: &StoryState) -> String {
    match stage {
        StoryState::Backlog => format!("{}", stage.dimmed()),
        StoryState::InProgress => format!("{}", stage.blue()),
        StoryState::NeedsHumanVerification => format!("{}", stage.yellow()),
        StoryState::Done => format!("{}", stage.green()),
        StoryState::Rejected => format!("{}", stage.red()),
        StoryState::Icebox => format!("{}", stage.dimmed()),
    }
}

/// Color an epic stage label
pub fn styled_epic_stage(stage: &EpicState) -> String {
    match stage {
        EpicState::Draft => format!("{}", stage.dimmed()),
        EpicState::Active => format!("{}", stage.yellow()),
        EpicState::Done => format!("{}", stage.green()),
    }
}

/// Color a voyage/epic stage label
pub fn styled_voyage_stage(stage: &VoyageState) -> String {
    match stage {
        VoyageState::Draft => format!("{}", stage.dimmed()),
        VoyageState::Planned => format!("{}", stage.blue()),
        VoyageState::InProgress => format!("{}", stage.yellow()),
        VoyageState::Done => format!("{}", stage.green()),
    }
}

/// Color a story type badge
pub fn styled_type(story_type: &StoryType) -> String {
    match story_type {
        StoryType::Feat => format!("{}", story_type.magenta()),
        StoryType::Bug | StoryType::Fix => format!("{}", story_type.red()),
        StoryType::Refactor => format!("{}", story_type.cyan()),
        StoryType::Chore => format!("{}", story_type.dimmed()),
        StoryType::Docs => format!("{}", story_type.blue()),
    }
}

/// Render an acceptance criteria line with color
pub fn styled_ac(line: &str) -> String {
    let trimmed = line.trim_start_matches("- ");

    // Parse checkbox
    let (checked, rest) = if trimmed.starts_with("[x] ") || trimmed.starts_with("[X] ") {
        (true, &trimmed[4..])
    } else if let Some(rest) = trimmed.strip_prefix("[ ] ") {
        (false, rest)
    } else {
        // Not an AC line, return as-is
        return line.to_string();
    };

    let checkbox = if checked {
        format!("{}", "✓".green())
    } else {
        format!("{}", "○".red())
    };

    // Extract requirement ref like [SRS-01/AC-01]
    let (ref_part, after_ref) = extract_requirement_ref(rest);

    // Extract verify annotation like <!-- verify: manual -->
    let (description, verify_part) = extract_verify_annotation(after_ref);

    let mut result = format!("  {} ", checkbox);

    if let Some(req_ref) = ref_part {
        result.push_str(&format!("{} ", req_ref.cyan()));
    }

    result.push_str(description.trim());

    if let Some(verify) = verify_part {
        result.push_str(&format!(" {}", verify.dimmed()));
    }

    result
}

/// Extract [SRS-XX/AC-XX] style references from the start of text
fn extract_requirement_ref(text: &str) -> (Option<&str>, &str) {
    if text.starts_with('[')
        && let Some(end) = text.find("] ")
    {
        let ref_text = &text[..=end];
        let rest = &text[end + 2..];
        return (Some(ref_text), rest);
    }
    (None, text)
}

/// Extract <!-- verify: ... --> from the end of text
fn extract_verify_annotation(text: &str) -> (&str, Option<&str>) {
    if let Some(start) = text.find("<!-- verify:") {
        let description = text[..start].trim_end();
        // Extract just the verify type from <!-- verify: TYPE -->
        let annotation = &text[start..];
        let clean = annotation
            .trim_start_matches("<!-- verify:")
            .trim_end_matches("-->")
            .trim();
        return (description, Some(clean));
    }
    (text, None)
}

/// Render AC progress bar
pub fn progress_bar(
    checked: usize,
    total: usize,
    width: usize,
    theme: Option<&crate::cli::presentation::theme::Theme>,
) -> String {
    if total == 0 {
        return String::new();
    }

    let filled = (checked * width) / total;
    let empty = width - filled;

    let bar_filled = "█".repeat(filled);
    let bar_empty = "░".repeat(empty);
    let ratio = format!("{}/{}", checked, total);

    if let Some(t) = theme {
        if checked == total {
            format!(
                "{}{}{} {}{}{}",
                t.agent, bar_filled, t.reset, t.agent, ratio, t.reset
            )
        } else {
            format!(
                "{}{}{}{}{}{} {}",
                t.agent, bar_filled, t.reset, t.muted, bar_empty, t.reset, ratio
            )
        }
    } else if checked == total {
        format!(
            "{} {}",
            format!("{}{}", bar_filled, bar_empty).green(),
            ratio.green()
        )
    } else {
        format!("{}{} {}", bar_filled.green(), bar_empty.dimmed(), ratio)
    }
}

/// Render Epic capacity progress bar with Done (█) and In-Flight (▒) states
pub fn capacity_progress_bar(
    done: usize,
    in_flight: usize,
    total: usize,
    width: usize,
    theme: Option<&crate::cli::presentation::theme::Theme>,
) -> String {
    if total == 0 {
        let empty = "░".repeat(width);
        if let Some(t) = theme {
            return format!("{}[{}{}{}]{}", t.reset, t.muted, empty, t.reset, t.reset);
        } else {
            return format!("[{}]", empty.dimmed());
        }
    }

    let done_filled = (done * width) / total;
    let in_flight_filled = ((done + in_flight) * width) / total - done_filled;
    let empty = width.saturating_sub(done_filled + in_flight_filled);

    let bar_done = "█".repeat(done_filled);
    let bar_in_flight = "▒".repeat(in_flight_filled);
    let bar_empty = "░".repeat(empty);

    if let Some(t) = theme {
        format!(
            "{}[{}{}{}{}{}{}]",
            t.reset, t.agent, bar_done, bar_in_flight, t.muted, bar_empty, t.reset
        )
    } else {
        format!(
            "[{} {} {}]",
            bar_done.green(),
            bar_in_flight.blue(),
            bar_empty.dimmed()
        )
    }
}

/// Color an evidence chain entry by phase.
/// Bookend phases (`:start`, `:end`, `:start:end`) get cyan.
/// `:continues` phases get dimmed to visually nest under bookends.
pub fn styled_evidence_entry(entry: &crate::read_model::evidence::EvidenceEntry) -> String {
    let line = format!(
        ":{} [{}] - \"{}\"",
        entry.phase,
        styled_story_id(&entry.story_id),
        entry.criterion
    );
    if entry.phase == "continues" {
        format!("{}", line.dimmed())
    } else {
        format!("{}", line.cyan())
    }
}

/// Map common fenced code block language tags to syntect file extensions.
/// Syntect uses Sublime Text syntax definitions which key off file extensions.
fn lang_to_extension(lang: &str) -> &str {
    match lang {
        "rust" | "rs" => "rs",
        "python" | "py" => "py",
        "javascript" | "js" => "js",
        "typescript" | "ts" => "ts",
        "ruby" | "rb" => "rb",
        "shell" | "bash" | "sh" | "zsh" => "sh",
        "toml" => "toml",
        "yaml" | "yml" => "yaml",
        "json" => "json",
        "html" => "html",
        "css" => "css",
        "sql" => "sql",
        "c" => "c",
        "cpp" | "c++" | "cxx" => "cpp",
        "go" => "go",
        "java" => "java",
        "markdown" | "md" => "md",
        "xml" => "xml",
        "diff" | "patch" => "diff",
        other => other,
    }
}

const THEME: &str = "base16-ocean.dark";

/// Highlight a code block using syntect.
/// Returns highlighted lines with ANSI escape codes, or None if the
/// language isn't recognized (caller should fall back to plain rendering).
pub fn highlight_code_block(code: &str, lang: &str) -> Option<String> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let ext = lang_to_extension(lang);
    let syntax = ss.find_syntax_by_extension(ext)?;

    let theme = &ts.themes[THEME];
    let mut h = HighlightLines::new(syntax, theme);
    let mut output = String::new();

    for line in LinesWithEndings::from(code) {
        let ranges = h.highlight_line(line, &ss).ok()?;
        output.push_str(&as_24_bit_terminal_escaped(&ranges, false));
    }

    // Reset terminal colors after the block
    output.push_str("\x1b[0m");
    Some(output)
}

/// Styled header line
pub fn header(id: &str, title: &str, style_fn: fn(&str) -> String) -> String {
    format!("  {} {}", style_fn(id), title.bold(),)
}

/// Color an entity ID consistently (defaults to Story ID)
pub fn styled_id(id: &str) -> String {
    styled_story_id(id)
}

/// Color an Epic ID
pub fn styled_epic_id(id: &str) -> String {
    id.cyan().bold().to_string()
}

/// Color a Voyage ID
pub fn styled_voyage_id(id: &str) -> String {
    id.magenta().bold().to_string()
}

/// Color a Story ID
pub fn styled_story_id(id: &str) -> String {
    id.bright_blue().bold().to_string()
}

/// Color an SRS requirement ID
pub fn styled_requirement_id(id: &str) -> String {
    id.cyan().to_string()
}

/// Styled scope (Epic/Voyage) in standardized format
pub fn styled_scope(scope: Option<&str>) -> String {
    let s = scope.unwrap_or("-");
    if s.contains('/') {
        let parts: Vec<_> = s.split('/').collect();
        if parts.len() >= 2 {
            format!(
                "{}/{}",
                styled_epic_id(parts[0]),
                styled_voyage_id(parts[1])
            )
        } else {
            styled_id(s)
        }
    } else if s == "-" {
        s.dimmed().to_string()
    } else {
        styled_epic_id(s)
    }
}

/// Dim horizontal rule
pub fn rule(width: usize, theme: Option<&crate::cli::presentation::theme::Theme>) -> String {
    let s = "─".repeat(width);
    if let Some(t) = theme {
        format!("{}{}{}", t.muted, s, t.reset)
    } else {
        format!("{}", s.dimmed())
    }
}

/// Bold horizontal rule
pub fn heavy_rule(width: usize, theme: Option<&crate::cli::presentation::theme::Theme>) -> String {
    let s = "═".repeat(width);
    if let Some(t) = theme {
        format!("{}{}{}", t.muted, s, t.reset)
    } else {
        format!("{}", s.dimmed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_ref_parses_srs_format() {
        let (ref_part, rest) = extract_requirement_ref("[SRS-01/AC-01] Do the thing");
        assert_eq!(ref_part, Some("[SRS-01/AC-01]"));
        assert_eq!(rest, "Do the thing");
    }

    #[test]
    fn extract_ref_returns_none_for_no_ref() {
        let (ref_part, rest) = extract_requirement_ref("Do the thing");
        assert!(ref_part.is_none());
        assert_eq!(rest, "Do the thing");
    }

    #[test]
    fn extract_verify_parses_annotation() {
        let (desc, verify) = extract_verify_annotation("Do the thing <!-- verify: manual -->");
        assert_eq!(desc, "Do the thing");
        assert_eq!(verify, Some("manual"));
    }

    #[test]
    fn extract_verify_returns_none_for_no_annotation() {
        let (desc, verify) = extract_verify_annotation("Do the thing");
        assert_eq!(desc, "Do the thing");
        assert!(verify.is_none());
    }

    #[test]
    fn progress_bar_renders_complete() {
        let bar = progress_bar(5, 5, 10, None);
        assert!(bar.contains("5/5"));
    }

    #[test]
    fn progress_bar_renders_partial() {
        let bar = progress_bar(3, 5, 10, None);
        assert!(bar.contains("3/5"));
    }

    #[test]
    fn progress_bar_handles_zero_total() {
        assert_eq!(progress_bar(0, 0, 10, None), "");
    }

    #[test]
    fn capacity_progress_bar_renders_segments() {
        // 10 total, 2 done (20%), 3 in_flight (30%), width 10
        // Expected: 2 done, 3 in_flight, 5 empty
        let bar = capacity_progress_bar(2, 3, 10, 10, None);
        assert!(bar.contains('█'));
        assert!(bar.contains('▒'));
        assert!(bar.contains('░'));
    }

    #[test]
    fn capacity_progress_bar_handles_zero_total() {
        let bar = capacity_progress_bar(0, 0, 0, 10, None);
        assert_eq!(bar, format!("[{}]", "░".repeat(10).dimmed()));
    }

    #[test]
    fn styled_ac_handles_checked() {
        let line = "- [x] [SRS-01/AC-01] Do the thing <!-- verify: manual -->";
        let result = styled_ac(line);
        // Should contain the check mark, not the raw [x]
        assert!(result.contains('✓'));
        assert!(!result.contains("[x]"));
    }

    #[test]
    fn styled_ac_handles_unchecked() {
        let line = "- [ ] Do the thing";
        let result = styled_ac(line);
        assert!(result.contains('○'));
    }

    #[test]
    fn styled_ac_passes_through_non_ac_lines() {
        let line = "Some regular text";
        assert_eq!(styled_ac(line), "Some regular text");
    }

    #[test]
    fn styled_evidence_entry_start_has_cyan() {
        let entry = crate::read_model::evidence::EvidenceEntry {
            requirement_id: "SRS-01".into(),
            story_id: "ABC".into(),
            story_title: "My Story".into(),
            criterion: "it works".into(),
            phase: "start".into(),
            proof: None,
        };
        let result = styled_evidence_entry(&entry);
        // Bookend phases should contain the text and use cyan ANSI codes
        assert!(result.contains(":start"));
        // ABC should be styled as story ID (bright blue)
        assert!(result.contains("ABC"));
        // Cyan ANSI escape = \x1b[36m
        assert!(result.starts_with("\x1b[36m:start"));
    }

    #[test]
    fn styled_evidence_entry_continues_is_dimmed() {
        let entry = crate::read_model::evidence::EvidenceEntry {
            requirement_id: "SRS-01".into(),
            story_id: "ABC".into(),
            story_title: "My Story".into(),
            criterion: "more work".into(),
            phase: "continues".into(),
            proof: None,
        };
        let result = styled_evidence_entry(&entry);
        assert!(result.contains(":continues"));
        // Dimmed ANSI escape = \x1b[2m
        assert!(result.starts_with("\x1b[2m:continues"));
        // Should NOT contain cyan
        assert!(
            !result.contains("\x1b[36m"),
            "Should not use cyan for :continues"
        );
    }

    #[test]
    fn styled_evidence_entry_end_has_cyan() {
        let entry = crate::read_model::evidence::EvidenceEntry {
            requirement_id: "SRS-01".into(),
            story_id: "XYZ".into(),
            story_title: "Final".into(),
            criterion: "done".into(),
            phase: "end".into(),
            proof: None,
        };
        let result = styled_evidence_entry(&entry);
        assert!(result.contains(":end"));
        assert!(result.starts_with("\x1b[36m:end"));
    }

    #[test]
    fn rule_renders_correct_length() {
        let r = rule(10, None);
        // Dimmed rule contains ANSI + 10 chars + reset
        assert_eq!(crate::infrastructure::utils::visible_width(&r), 10);
    }

    #[test]
    fn heavy_rule_renders_correct_chars() {
        let r = heavy_rule(5, None);
        assert!(r.contains('═'));
        assert_eq!(crate::infrastructure::utils::visible_width(&r), 5);
    }

    #[test]
    fn entity_id_styling_consistency() {
        let epic = styled_epic_id("EPIC-1");
        let voyage = styled_voyage_id("VOY-1");
        let story = styled_story_id("STORY-1");

        assert!(epic.contains("EPIC-1"));
        assert!(voyage.contains("VOY-1"));
        assert!(story.contains("STORY-1"));

        // All should be bold (ANSI [1m)
        assert!(epic.contains("\x1b[1m"));
        assert!(voyage.contains("\x1b[1m"));
        assert!(story.contains("\x1b[1m"));
    }

    #[test]
    fn styled_stage_colors() {
        assert!(styled_stage(&StoryState::InProgress).contains("\x1b[34m")); // blue
        assert!(styled_stage(&StoryState::Done).contains("\x1b[32m")); // green
        assert!(styled_stage(&StoryState::Rejected).contains("\x1b[31m")); // red
    }
}
