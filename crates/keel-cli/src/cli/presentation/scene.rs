//! Reusable ASCII/ANSI scene generation primitives for the CLI.
//!
//! These functions provide visual metaphors for command states, as defined in `STAGE.md`.

use owo_colors::OwoColorize;

/// Renders a radar sweep visual for 'next' style commands.
/// `signal_count` determines how many "blips" appear on the radar.
pub fn render_radar(signal_count: usize) -> String {
    let mut blips = [" ", " ", " "];
    for blip in blips.iter_mut().take(signal_count.min(3)) {
        *blip = "●";
    }

    let b1 = blips[0].green();
    let b2 = blips[1].green();
    let b3 = blips[2].green();

    format!(
        r#"
      .-------.    
    .'  \ | /  '.  
   /  ---(+)---  \ 
  |       |       |
  | {}     |   {}   |
   \      |      / 
    '.  {} |    .'  
      `-------`    
"#,
        b1, b2, b3
    )
}

/// Renders an engine gear visual for automation commands like 'pulse'.
/// If `active` is true, the gears are rendered brightly (cyan) to show work was done.
/// If `active` is false, they are dimmed to show the engine idled.
pub fn render_gears(active: bool) -> String {
    let base_gears = r#"
      _   _ 
    /   V   \ 
   |  ( O )  | -- _ 
    \   ^   /   /   
      -   -    | (O) |
                \ _ /
"#;

    if active {
        base_gears.cyan().to_string()
    } else {
        base_gears.dimmed().to_string()
    }
}

/// Renders a 12hr analog watch face metaphor for time-constrained missions.
/// `hour` determines the hand position (0-11).
pub fn render_watch(hour: u32) -> String {
    let hour = hour % 12;

    let center = match hour {
        0 | 6 => "|",
        3 | 9 => "-",
        1 | 2 | 7 | 8 => "/",
        4 | 5 | 10 | 11 => "\\",
        _ => "+",
    }
    .yellow()
    .to_string();

    format!(
        r#"
      .-------.
    .'    12   '.
   /  9   {}   3  \
  |       |       |
  |       |       |
   \      6      /
    '.         .'
      `-------`
"#,
        center
    )
}

/// High-fidelity watch visualization for `--scene` mode.
///
/// Shows a 12-slot analog clock with:
/// - Hour hand pointing to current elapsed position
/// - Swept arc showing consumed time
/// - Color coding: green (plenty), yellow (halfway), red (critical)
pub fn render_watch_hifi(current_hour: u32, limit_hours: u32, title: &str) -> String {
    let hour = current_hour.min(limit_hours) % 12;
    let fraction = if limit_hours > 0 {
        current_hour as f64 / limit_hours as f64
    } else {
        0.0
    };

    // Choose palette based on how much time remains
    let (arc_color, status_word) = if fraction >= 1.0 {
        ("red", "EXPIRED")
    } else if fraction >= 0.75 {
        ("red", "CRITICAL")
    } else if fraction >= 0.5 {
        ("yellow", "HALFWAY")
    } else {
        ("green", "NOMINAL")
    };

    // Clock positions: 12 slots, mark consumed ones
    //  Positions:  12=0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
    let slots_consumed = if limit_hours > 0 {
        ((current_hour as f64 / limit_hours as f64) * 12.0).round() as u32
    } else {
        0
    };

    // Render each hour marker — filled if consumed, empty if remaining
    let marker = |pos: u32| -> String {
        let filled = pos <= slots_consumed && pos > 0;
        let at_hand = pos == hour;
        let ch = if at_hand {
            "◆"
        } else if filled {
            "●"
        } else {
            "○"
        };
        match arc_color {
            "red" if filled || at_hand => ch.red().bold().to_string(),
            "yellow" if filled || at_hand => ch.yellow().bold().to_string(),
            _ if filled || at_hand => ch.green().bold().to_string(),
            _ => ch.dimmed().to_string(),
        }
    };

    let remaining = limit_hours.saturating_sub(current_hour);

    // Progress bar
    let bar_width = 24;
    let filled_width = if limit_hours > 0 {
        ((current_hour as f64 / limit_hours as f64) * bar_width as f64).round() as usize
    } else {
        0
    }
    .min(bar_width);
    let empty_width = bar_width - filled_width;

    let bar_filled = "█".repeat(filled_width);
    let bar_empty = "░".repeat(empty_width);
    let bar = match arc_color {
        "red" => format!("{}{}", bar_filled.red(), bar_empty.dimmed()),
        "yellow" => format!("{}{}", bar_filled.yellow(), bar_empty.dimmed()),
        _ => format!("{}{}", bar_filled.green(), bar_empty.dimmed()),
    };

    let status_styled = match arc_color {
        "red" => status_word.red().bold().to_string(),
        "yellow" => status_word.yellow().bold().to_string(),
        _ => status_word.green().bold().to_string(),
    };

    let m12 = marker(0);
    let m1 = marker(1);
    let m2 = marker(2);
    let m3 = marker(3);
    let m4 = marker(4);
    let m5 = marker(5);
    let m6 = marker(6);
    let m7 = marker(7);
    let m8 = marker(8);
    let m9 = marker(9);
    let m10 = marker(10);
    let m11 = marker(11);

    format!(
        r#"
          ╭─────────────────╮
        ╭─┤       {m12}       ├─╮
       ╱  ╰─{m11}───────────{m1}─╯  ╲
      │                         │
     {m10}                           {m2}
      │                         │
     {m9}            ⊙            {m3}
      │                         │
     {m8}                           {m4}
      │                         │
       ╲    {m7}───────────{m5}    ╱
        ╰──┤       {m6}       ├──╯
           ╰─────────────────╯

    {bar}

    {}  {}h / {}h  ({}h remaining)  {}
"#,
        title.bold(),
        current_hour,
        limit_hours,
        remaining,
        status_styled,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use txt_scene::{SceneFrame, SceneLine, ScenePalette, visible_width};

    #[test]
    fn scene_line_pads_to_visible_width_with_ansi_content() {
        let mut line = SceneLine::new(8);
        line.push("AB".green().to_string());
        line.push("CD");

        let rendered = line.finish();
        assert_eq!(visible_width(&rendered), 8);
        assert!(rendered.ends_with("    "));
    }

    #[test]
    fn scene_frame_wraps_rows_to_exact_visible_width() {
        let frame = SceneFrame::new("    ", "│", "│", 10);
        let rendered = frame.row(|line| {
            line.pad_to(2).push("X");
        });

        assert_eq!(visible_width(&rendered), 16);
        assert!(rendered.starts_with("    │"));
        assert!(rendered.ends_with('│'));
    }

    #[test]
    fn scene_palette_disables_color_when_requested() {
        let palette = ScenePalette::new(false);
        assert_eq!(palette.green("█"), "█");
        assert_eq!(palette.yellow_bold("line"), "line");
    }
}
