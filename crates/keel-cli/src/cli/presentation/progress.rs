//! Shared progress rendering helpers for show surfaces.

use crate::cli::style;
use owo_colors::OwoColorize;

pub fn render_count_bar(
    done: usize,
    total: usize,
    bar_width: usize,
    suffix: Option<&str>,
) -> String {
    let suffix = suffix.unwrap_or_default();
    let suffix = if suffix.is_empty() {
        String::new()
    } else {
        format!(" {suffix}")
    };

    if total == 0 {
        return format!("0/0{suffix}");
    }

    format!(
        "{done}/{total} {}{suffix}",
        style::progress_bar(done, total, bar_width, None)
    )
}

pub fn render_capacity_bar(
    done: usize,
    in_flight: usize,
    ready: usize,
    width: usize,
    color: Option<owo_colors::AnsiColors>,
) -> String {
    let total = done + in_flight + ready;
    if total == 0 {
        return format!("[{}]", " ".repeat(width));
    }

    // Round cumulative boundaries so the segment widths always sum to the
    // requested bar width, even for proportions like 50/50 on odd widths.
    let done_width = scaled_boundary(done, total, width);
    let in_flight_boundary = scaled_boundary(done + in_flight, total, width);
    let in_flight_width = in_flight_boundary.saturating_sub(done_width);
    let ready_width = width.saturating_sub(done_width + in_flight_width);

    let mut bar = String::new();
    if let Some(color) = color {
        bar.push_str(&"▓".color(color).to_string().repeat(done_width));
        bar.push_str(&"▒".color(color).to_string().repeat(in_flight_width));
        bar.push_str(&"░".color(color).to_string().repeat(ready_width));
    } else {
        bar.push_str(&"▓".repeat(done_width));
        bar.push_str(&"▒".repeat(in_flight_width));
        bar.push_str(&"░".repeat(ready_width));
    }

    let remaining = width.saturating_sub(done_width + in_flight_width + ready_width);
    bar.push_str(&" ".repeat(remaining));

    format!("[{}]", bar)
}

fn scaled_boundary(count: usize, total: usize, width: usize) -> usize {
    ((count as f64 / total as f64) * width as f64).round() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::infrastructure::utils::visible_width;

    #[test]
    fn render_count_bar_handles_zero_total() {
        assert_eq!(
            render_count_bar(0, 0, 15, Some("(functional)")),
            "0/0 (functional)"
        );
    }

    #[test]
    fn render_count_bar_renders_bar_and_suffix() {
        let rendered = render_count_bar(2, 4, 10, Some("(stories)"));
        assert!(rendered.contains("2/4"));
        assert!(rendered.contains("(stories)"));
    }

    #[test]
    fn render_capacity_bar_preserves_requested_width_for_split_work() {
        let rendered = render_capacity_bar(1, 1, 0, 15, None);
        assert_eq!(visible_width(&rendered), 17);
        assert_eq!(rendered, "[▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒]");
    }

    #[test]
    fn render_capacity_bar_preserves_requested_width_when_colored() {
        let rendered = render_capacity_bar(1, 1, 0, 15, Some(owo_colors::AnsiColors::Green));
        assert_eq!(visible_width(&rendered), 17);
    }
}
