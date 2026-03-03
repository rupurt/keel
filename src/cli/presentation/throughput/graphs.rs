//! Weekly sparkline rendering for throughput diagnostics.

use std::fmt::Write;

use crate::cli::presentation::theme::Theme;
use crate::cli::style;
use crate::read_model::throughput_history::{ThroughputHistory, WeeklyThroughputBucket};

/// Render weekly throughput/timing sparkline graphs.
pub fn render_throughput_graphs(
    history: &ThroughputHistory,
    width: usize,
    theme: &Theme,
) -> String {
    let mut output = String::new();
    let weekly: Vec<&WeeklyThroughputBucket> = history.weekly.iter().rev().collect(); // oldest -> newest
    let sparkline_width = width.saturating_sub(26).clamp(12, 48);
    let throughput_lines = throughput_lines(&weekly, theme, sparkline_width);
    let timing_lines = timing_lines(&weekly, theme, sparkline_width);

    writeln!(output, "{}", style::heavy_rule(width, Some(theme))).unwrap();
    writeln!(output, "  Weekly Flow Sparklines").unwrap();
    writeln!(output, "{}", style::rule(width, Some(theme))).unwrap();
    render_section(&mut output, &throughput_lines);
    writeln!(output, "{}", style::rule(width, Some(theme))).unwrap();
    render_section(&mut output, &timing_lines);
    writeln!(output, "{}", style::heavy_rule(width, Some(theme))).unwrap();
    writeln!(output).unwrap();

    output
}

fn render_section(output: &mut String, lines: &[String]) {
    for line in lines {
        writeln!(output, "{}", line).unwrap();
    }
}

fn throughput_lines(
    weekly: &[&WeeklyThroughputBucket],
    theme: &Theme,
    sparkline_width: usize,
) -> Vec<String> {
    let story_counts: Vec<f64> = weekly.iter().map(|w| w.stories_done as f64).collect();
    let voyage_counts: Vec<f64> = weekly.iter().map(|w| w.voyages_done as f64).collect();

    let story_start = first_nonzero_index(&story_counts);
    let voyage_start = first_nonzero_index(&voyage_counts);
    let story_elapsed_weeks = elapsed_weeks(story_start, story_counts.len());
    let voyage_elapsed_weeks = elapsed_weeks(voyage_start, voyage_counts.len());
    let show_3m = story_elapsed_weeks > 4 || voyage_elapsed_weeks > 4;

    let story_4w = rolling_average_relative(&story_counts, story_start, 4);
    let voyage_4w = rolling_average_relative(&voyage_counts, voyage_start, 4);
    let story_12w = rolling_average_relative(&story_counts, story_start, 12);
    let voyage_12w = rolling_average_relative(&voyage_counts, voyage_start, 12);

    let story_line = sparkline_from_values(
        &story_counts.iter().map(|v| Some(*v)).collect::<Vec<_>>(),
        sparkline_width,
    );
    let voyage_line = sparkline_from_values(
        &voyage_counts.iter().map(|v| Some(*v)).collect::<Vec<_>>(),
        sparkline_width,
    );
    let mut lines = vec![
        "  Throughput (weekly)".to_string(),
        format!(
            "  {:<15} {}",
            "Stories done",
            colorize(&story_line, theme.agent, theme)
        ),
        format!(
            "  {:<15} {}",
            "Voyages done",
            colorize(&voyage_line, theme.human, theme)
        ),
        format!(
            "  {:<15} {}",
            "Stories 1m",
            colorize(
                &sparkline_from_values(&story_4w, sparkline_width),
                theme.warning,
                theme
            )
        ),
        format!(
            "  {:<15} {}",
            "Voyages 1m",
            colorize(
                &sparkline_from_values(&voyage_4w, sparkline_width),
                theme.warning,
                theme
            )
        ),
    ];

    if show_3m {
        lines.push(format!(
            "  {:<15} {}",
            "Stories 3m",
            colorize(
                &sparkline_from_values(&story_12w, sparkline_width),
                theme.muted,
                theme
            )
        ));
        lines.push(format!(
            "  {:<15} {}",
            "Voyages 3m",
            colorize(
                &sparkline_from_values(&voyage_12w, sparkline_width),
                theme.muted,
                theme
            )
        ));
    }

    lines.push(format!(
        "  Avg now S:{}  V:{}",
        format_avg(last_some(&story_4w)),
        format_avg(last_some(&voyage_4w))
    ));
    if show_3m {
        lines.push(format!(
            "  Avg 3m S:{}  V:{}",
            format_avg(last_some(&story_12w)),
            format_avg(last_some(&voyage_12w))
        ));
    }
    lines
}

fn timing_lines(
    weekly: &[&WeeklyThroughputBucket],
    theme: &Theme,
    sparkline_width: usize,
) -> Vec<String> {
    let cycle_min_hours: Vec<Option<f64>> = weekly.iter().map(|w| w.cycle_min_hours).collect();
    let cycle_median_hours: Vec<Option<f64>> =
        weekly.iter().map(|w| w.cycle_median_hours).collect();
    let cycle_max_hours: Vec<Option<f64>> = weekly.iter().map(|w| w.cycle_max_hours).collect();
    let acceptance_wait_hours: Vec<Option<f64>> = weekly
        .iter()
        .map(|w| w.acceptance_wait_median_hours)
        .collect();

    vec![
        "  Story timing (hours)".to_string(),
        "  Cycle: start -> submit/done".to_string(),
        "  Wait : submit -> done".to_string(),
        format!(
            "  {:<15} {}",
            "Cycle min",
            colorize(
                &sparkline_from_values(&cycle_min_hours, sparkline_width),
                theme.muted,
                theme
            )
        ),
        format!(
            "  {:<15} {}",
            "Cycle med",
            colorize(
                &sparkline_from_values(&cycle_median_hours, sparkline_width),
                theme.human,
                theme
            )
        ),
        format!(
            "  {:<15} {}",
            "Cycle max",
            colorize(
                &sparkline_from_values(&cycle_max_hours, sparkline_width),
                theme.warning,
                theme
            )
        ),
        format!(
            "  {:<15} {}",
            "Wait med",
            colorize(
                &sparkline_from_values(&acceptance_wait_hours, sparkline_width),
                theme.agent,
                theme
            )
        ),
        format!(
            "  Latest cyc  {}/{}/{}",
            format_hours(last_some(&cycle_min_hours)),
            format_hours(last_some(&cycle_median_hours)),
            format_hours(last_some(&cycle_max_hours))
        ),
        format!(
            "  Latest wait {}",
            format_hours(last_some(&acceptance_wait_hours))
        ),
    ]
}

fn rolling_average_relative(
    series: &[f64],
    observed_start_idx: Option<usize>,
    window: usize,
) -> Vec<Option<f64>> {
    let mut out = vec![None; series.len()];
    let Some(start_idx) = observed_start_idx else {
        return out;
    };

    for i in start_idx..series.len() {
        let window_start = (i + 1).saturating_sub(window).max(start_idx);
        let slice = &series[window_start..=i];
        let sum: f64 = slice.iter().sum();
        out[i] = Some(sum / slice.len() as f64);
    }

    out
}

fn first_nonzero_index(series: &[f64]) -> Option<usize> {
    series.iter().position(|v| *v > 0.0)
}

fn elapsed_weeks(start_idx: Option<usize>, len: usize) -> usize {
    start_idx.map(|idx| len.saturating_sub(idx)).unwrap_or(0)
}

fn sparkline_from_values(values: &[Option<f64>], width: usize) -> String {
    let values = resample(values, width);
    const BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    let max = values.iter().flatten().copied().fold(0.0_f64, f64::max);

    values
        .iter()
        .map(|value| match value {
            Some(v) => {
                if max <= 0.0 {
                    BLOCKS[0]
                } else {
                    let idx = ((v / max).clamp(0.0, 1.0) * 7.0).round() as usize;
                    BLOCKS[idx.min(7)]
                }
            }
            None => '·',
        })
        .collect()
}

fn resample(values: &[Option<f64>], width: usize) -> Vec<Option<f64>> {
    if values.len() <= width || width == 0 {
        return values.to_vec();
    }

    let mut out = Vec::with_capacity(width);
    let step = values.len() as f64 / width as f64;
    for idx in 0..width {
        let start = (idx as f64 * step).floor() as usize;
        let mut end = ((idx as f64 + 1.0) * step).floor() as usize;
        if idx == width - 1 {
            end = values.len();
        } else {
            end = end.max(start + 1);
        }
        let slice = &values[start..end.min(values.len())];
        let mut sum = 0.0;
        let mut count = 0usize;
        for value in slice.iter().flatten() {
            sum += *value;
            count += 1;
        }
        out.push(if count == 0 {
            None
        } else {
            Some(sum / count as f64)
        });
    }

    out
}

fn colorize(text: &str, color: &str, theme: &Theme) -> String {
    format!("{}{}{}", color, text, theme.reset)
}

fn format_hours(value: Option<f64>) -> String {
    value
        .map(|v| format!("{v:.1}h"))
        .unwrap_or_else(|| "-".to_string())
}

fn format_avg(value: Option<f64>) -> String {
    value
        .map(|v| format!("{v:.2}/wk"))
        .unwrap_or_else(|| "-".to_string())
}

fn last_some(values: &[Option<f64>]) -> Option<f64> {
    values.iter().rev().find_map(|v| *v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn render_throughput_graphs_contains_expected_sections() {
        let history = ThroughputHistory {
            schema_version: 1,
            weekly: vec![
                WeeklyThroughputBucket {
                    week_start: NaiveDate::from_ymd_opt(2026, 2, 23).unwrap(),
                    stories_done: 3,
                    voyages_done: 1,
                    cycle_min_hours: Some(24.0),
                    cycle_median_hours: Some(48.0),
                    cycle_max_hours: Some(72.0),
                    acceptance_wait_median_hours: Some(12.0),
                },
                WeeklyThroughputBucket {
                    week_start: NaiveDate::from_ymd_opt(2026, 2, 16).unwrap(),
                    stories_done: 1,
                    voyages_done: 0,
                    cycle_min_hours: Some(12.0),
                    cycle_median_hours: Some(18.0),
                    cycle_max_hours: Some(30.0),
                    acceptance_wait_median_hours: None,
                },
            ],
        };

        let rendered = render_throughput_graphs(&history, 100, &Theme::no_color());
        assert!(rendered.contains("Weekly Flow Sparklines"));
        assert!(rendered.contains("Stories done"));
        assert!(rendered.contains("Voyages done"));
        assert!(rendered.contains("Story timing (hours)"));
        assert!(rendered.contains("Wait med"));
    }

    #[test]
    fn rolling_average_relative_uses_elapsed_weeks_not_prehistory() {
        let series = vec![0.0, 0.0, 10.0, 0.0];
        let start = first_nonzero_index(&series);
        let avg = rolling_average_relative(&series, start, 12);

        assert_eq!(avg, vec![None, None, Some(10.0), Some(5.0)]);
    }

    #[test]
    fn sparkline_resamples_to_requested_width() {
        let values = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            Some(4.0),
            Some(5.0),
            Some(6.0),
        ];

        let rendered = sparkline_from_values(&values, 3);
        assert_eq!(rendered.chars().count(), 3);
    }
}
