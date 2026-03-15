//! Shared progress rendering helpers for show surfaces.

use crate::cli::style;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}

pub fn render_capacity_bar(
    done: usize,
    in_flight: usize,
    ready: usize,
    width: usize,
) -> String {
    let total = done + in_flight + ready;
    if total == 0 {
        return format!("[{}]", " ".repeat(width));
    }

    let done_width = (done as f64 / total as f64 * width as f64).round() as usize;
    let in_flight_width = (in_flight as f64 / total as f64 * width as f64).round() as usize;
    let ready_width = (ready as f64 / total as f64 * width as f64).round() as usize;

    let mut bar = String::new();
    bar.push_str(&"▓".repeat(done_width));
    bar.push_str(&"▒".repeat(in_flight_width));
    bar.push_str(&"░".repeat(ready_width));

    let remaining = width.saturating_sub(bar.len());
    bar.push_str(&" ".repeat(remaining));

    format!("[{}]", bar)
}
