//! Finance command - work capital and system solvency

use crate::cli::presentation::terminal::get_terminal_width;
use crate::cli::style::VisualPadding;
use anyhow::Result;
use keel::infrastructure::loader::load_board;
use keel::read_model::{diagnostics, flow_status};
use owo_colors::OwoColorize;

/// Run the finance command
pub fn run(board_dir: &std::path::Path, scene: bool) -> Result<()> {
    let board = load_board(board_dir)?;
    let metrics = flow_status::project(&board, chrono::Utc::now());
    let health = diagnostics::validate_report(board_dir)?;

    let width = get_terminal_width();

    if scene {
        render_vault_scene(&metrics, &health, width);
        return Ok(());
    }

    println!(
        "\n    {} WORK CAPITAL REPORT",
        " FINANCIAL ".on_yellow().black().bold()
    );

    let liquidity = metrics.execution.backlog_ready_count;
    let liabilities = metrics.execution.backlog_blocked_count;
    let debt = health.drift_coefficient;

    println!(
        "\n    {: <18} {}",
        "Current Assets:",
        format!("{} Ready Stories (Liquidity)", liquidity).green()
    );
    println!(
        "    {: <18} {}",
        "Liabilities:",
        format!("{} Blocked Stories (Debt)", liabilities).red()
    );
    println!(
        "    {: <18} {}",
        "Maintenance Cost:",
        format!("{:.2} Structural Drift (Sawdust)", debt).yellow()
    );

    let solvency = if liquidity > liabilities && debt < 0.3 {
        "SOLVENT".green().bold().to_string()
    } else if liquidity > 0 {
        "FIXED ASSETS ONLY".yellow().bold().to_string()
    } else {
        "INSOLVENT".red().bold().to_string()
    };

    println!("\n    Current Standing: {}", solvency);
    println!("    \"Keep the capital flowing to keep the lights on.\"");

    Ok(())
}

fn render_vault_scene(
    metrics: &keel::read_model::flow_metrics::FlowMetrics,
    health: &keel::read_model::diagnostics::types::DoctorReport,
    width: usize,
) {
    println!("\n    ┌{}┐", "─".repeat(width.saturating_sub(6)));
    println!(
        "    │ {: <width$} │",
        "THE VAULT (WORK CAPITAL)".bold(),
        width = width.saturating_sub(8)
    );
    println!("    └{}┘", "─".repeat(width.saturating_sub(6)));

    let liquidity = metrics.execution.backlog_ready_count;
    let blocked = metrics.execution.backlog_blocked_count;
    let drift = health.drift_coefficient;

    let mut vault = String::new();
    vault.push_str("             ._________________________.\n");
    vault.push_str("             | /                     \\ |\n");
    vault.push_str("             |/   [ WORK CAPITAL ]    \\|\n");
    vault.push_str("             |_________________________|\n");
    vault.push_str("             |            _            |\n");
    vault.push_str("             |        .-------.        |\n");
    vault.push_str("             |       /   _|_   \\       |\n");
    vault.push_str("             |      |  /\\ | /\\  |      |\n");
    vault.push_str("             |      | <──(O)──> |      |\n");
    vault.push_str("             |      |  \\/ | \\/  |      |\n");
    vault.push_str("             |       \\_______/       |\n");
    vault.push_str("             |         '---'           |\n");

    let status_label = if liquidity > blocked && drift < 0.3 {
        "SOLVENT".green().bold().to_string()
    } else {
        "DEBT WARNING".red().bold().to_string()
    };

    vault.push_str(&format!(
        "             |   {}   |\n",
        status_label.pad_to_width(17)
    ));
    vault.push_str("             |_________________________|\n");
    vault.push_str("            /                           \\\n");

    // Liquidity bars
    let mut assets = String::from(" [ ");
    for i in 0..10 {
        if i < liquidity {
            assets.push_str(&"█".green().to_string());
        } else {
            assets.push(' ');
        }
    }
    assets.push_str(" ] ASSETS (READY)");
    vault.push_str(&format!("    {}\n", assets.pad_to_width(40)));

    // Debt bars
    let mut debt = String::from(" [ ");
    for i in 0..10 {
        if i < blocked {
            debt.push_str(&"█".red().to_string());
        } else {
            debt.push(' ');
        }
    }
    debt.push_str(" ] DEBT (BLOCKED)");
    vault.push_str(&format!("    {}\n", debt.pad_to_width(40)));

    // Maintenance cost
    let remediation = health.estimated_remediation_hours;
    let oil_needed = format!(" [ {:.1}h ] MAINTENANCE INTEREST", remediation);
    vault.push_str(&format!(
        "    {}\n",
        oil_needed.yellow().to_string().pad_to_width(40)
    ));

    println!("{}", vault);

    if liquidity == 0 {
        println!(
            "\n    {} No liquidity! The lights will dim without ready stories.",
            " WARNING ".on_red().black()
        );
    } else if liquidity < 3 {
        println!(
            "\n    {} Low capital. Replenish the backlog to stay energized.",
            " CAUTION ".on_yellow().black()
        );
    } else {
        println!("\n    The vault is healthy. Work capital is sufficient for the 9-5 window.");
    }
}
