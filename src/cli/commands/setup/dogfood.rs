use anyhow::Result;
use clap::ArgMatches;

use crate::infrastructure::dogfood_runner;
use crate::infrastructure::dogfood_workspace;

pub fn run(matches: &ArgMatches) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let repo_root = dogfood_runner::find_repo_root(&cwd)?;

    match matches.subcommand() {
        Some(("run", m)) => {
            let scenario = m.get_one::<String>("scenario").expect("required");
            let report = dogfood_runner::run_named_scenario(&repo_root, scenario)?;
            println!("Scenario:  {}", report.scenario);
            println!("Workspace: {}", report.workspace_root.display());
            println!("Board:     {}", report.artifact_board_root.display());
            println!("Story:     {}", report.owner_story_id);
            println!("Tape:      {}", report.tape_path.display());
            println!("GIF:       {}", report.gif_path.display());
            println!("Transcript: {}", report.transcript_path.display());
            println!("Log:       {}", report.log_path.display());
            println!("Manifest:  {}", report.manifest_path.display());
            Ok(())
        }
        Some(("reset", _)) => {
            dogfood_workspace::reset_workspace(&repo_root)?;
            println!(
                "Reset secondary workspace at {}",
                dogfood_workspace::workspace_root(&repo_root).display()
            );
            Ok(())
        }
        _ => unreachable!("dogfood subcommand required"),
    }
}
