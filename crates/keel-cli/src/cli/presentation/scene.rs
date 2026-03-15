//! Reusable ASCII/ANSI scene generation primitives for the CLI.
//!
//! These functions provide visual metaphors for command states, as defined in `STAGE.md`.

use owo_colors::OwoColorize;

/// Renders a radar sweep visual for 'next' style commands.
/// `signal_count` determines how many "blips" appear on the radar.
pub fn render_radar(signal_count: usize) -> String {
    let mut blips = vec![" ", " ", " "];
    for i in 0..signal_count.min(3) {
        blips[i] = "●";
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
