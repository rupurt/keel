//! CLI runtime dispatch entrypoint.
//!
//! Keeps the binary `main.rs` as a thin adapter by hosting argument
//! parsing and command routing inside the CLI layer.

use super::{build_cli, resolve_board_dir};
use anyhow::Result;
use clap::{ArgMatches, FromArgMatches};

pub fn run() -> Result<()> {
    let raw_args: Vec<String> = std::env::args().collect();
    if let Some(message) = super::commands::management::next::legacy_next_flag_guidance(&raw_args) {
        anyhow::bail!(message);
    }
    if let Some(message) =
        super::commands::management::story::accept::legacy_story_accept_flag_guidance(&raw_args)
    {
        anyhow::bail!(message);
    }
    let matches = build_cli().get_matches_from(raw_args);

    match matches.subcommand() {
        Some(("doctor", m)) => {
            let fix = *m.get_one::<bool>("fix").unwrap_or(&false);
            let evidence = *m.get_one::<bool>("evidence").unwrap_or(&false);
            let watch = *m.get_one::<bool>("watch").unwrap_or(&false);
            let quick = *m.get_one::<bool>("quick").unwrap_or(&false);
            match super::commands::diagnostics::doctor::run(
                &resolve_board_dir()?,
                fix,
                evidence,
                watch,
                quick,
            ) {
                Ok(_) => Ok(()),
                Err(e) => {
                    let msg = e.to_string();
                    if msg.contains("errors") {
                        std::process::exit(2);
                    } else if msg.contains("warnings") {
                        std::process::exit(1);
                    } else {
                        Err(e)
                    }
                }
            }
        }
        Some(("flow", m)) => {
            let no_color = *m.get_one::<bool>("no_color").unwrap_or(&false);
            super::commands::diagnostics::flow::run(&resolve_board_dir()?, no_color)
        }
        Some(("throughput", m)) => {
            let no_color = *m.get_one::<bool>("no_color").unwrap_or(&false);
            super::commands::diagnostics::throughput::run(&resolve_board_dir()?, no_color)
        }
        Some(("next", m)) => {
            let json = *m.get_one::<bool>("json").unwrap_or(&false);
            let parallel = *m.get_one::<bool>("parallel").unwrap_or(&false);
            let role_str = m.get_one::<String>("role");
            let role =
                super::commands::management::next::parse_actor_role(role_str.map(String::as_str))?;

            super::commands::management::next::run(
                &resolve_board_dir()?,
                json,
                parallel,
                role.as_ref(),
            )
        }
        Some(("topology", m)) => {
            let epic_id = m.get_one::<String>("epic").expect("required");
            let include_done = *m.get_one::<bool>("include_done").unwrap_or(&false);
            super::commands::management::topology::run(epic_id, include_done)
        }
        Some(("play", m)) => {
            let bearing = m.get_one::<String>("bearing").cloned();
            let prop = m.get_one::<String>("prop").cloned();
            let cross = m
                .get_many::<String>("cross")
                .map(|values| values.cloned().collect());
            let list_props = *m.get_one::<bool>("list_props").unwrap_or(&false);
            let suggest = m.get_one::<String>("suggest").cloned();
            super::commands::management::play::run(
                &resolve_board_dir()?,
                bearing,
                prop,
                cross,
                list_props,
                suggest,
            )
        }
        Some(("audit", m)) => {
            let id = m.get_one::<String>("id").cloned();
            super::commands::management::story::audit::run(&resolve_board_dir()?, id.as_deref())
        }
        Some(("verify", m)) => match m.subcommand() {
            Some(("run", run_m)) => {
                let id = run_m.get_one::<String>("id").map(|s| s.as_str());
                let all = *run_m.get_one::<bool>("all").unwrap_or(&false);
                let json = *run_m.get_one::<bool>("json").unwrap_or(&false);
                super::commands::management::verify::run(&resolve_board_dir()?, id, all, json)
            }
            Some(("detect", detect_m)) => {
                let json = *detect_m.get_one::<bool>("json").unwrap_or(&false);
                super::commands::management::verify::detect(&resolve_board_dir()?, json)
            }
            Some(("recommend", recommend_m)) => {
                let json = *recommend_m.get_one::<bool>("json").unwrap_or(&false);
                super::commands::management::verify::recommend(&resolve_board_dir()?, json)
            }
            _ => unreachable!("verify subcommand required"),
        },
        Some(("knowledge", m)) => handle_knowledge_command(m),
        Some(("generate", _)) => super::commands::setup::generate::run(&resolve_board_dir()?),
        Some(("init", _)) => Ok(super::commands::setup::init::run()?),
        Some(("epic", m)) => handle_epic_command(m),
        Some(("voyage", m)) => handle_voyage_command(m),
        Some(("story", m)) => handle_story_command(m),
        Some(("bearing", m)) => handle_bearing_command(m),
        Some(("adr", m)) => handle_adr_command(m),
        Some(("mission", m)) => handle_mission_command(m),
        Some(("config", m)) => handle_config_command(m),
        None => {
            let mut cli = build_cli();
            cli.print_long_help()?;
            println!();
            Ok(())
        }
        _ => unreachable!("Unhandled command"),
    }
}

fn handle_epic_command(matches: &ArgMatches) -> Result<()> {
    super::commands::management::epic::run(parse_subcommand_action(matches)?)
}

fn handle_voyage_command(matches: &ArgMatches) -> Result<()> {
    super::commands::management::voyage::run(parse_subcommand_action(matches)?)
}

fn handle_story_command(matches: &ArgMatches) -> Result<()> {
    super::commands::management::story::run(parse_subcommand_action(matches)?)
}

fn handle_bearing_command(matches: &ArgMatches) -> Result<()> {
    super::commands::management::bearing::run(parse_subcommand_action(matches)?)
}

fn handle_adr_command(matches: &ArgMatches) -> Result<()> {
    super::commands::management::adr::run(parse_subcommand_action(matches)?)
}

fn handle_mission_command(matches: &ArgMatches) -> Result<()> {
    super::commands::management::mission::run(parse_subcommand_action(matches)?)
}

fn handle_knowledge_command(matches: &ArgMatches) -> Result<()> {
    let command = matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("Missing knowledge subcommand"))?;
    let action = match command {
        ("list", m) => {
            let category = m.get_one::<String>("category").cloned();
            let pending = *m.get_one::<bool>("pending").unwrap_or(&false);
            let sort = m
                .get_one::<String>("sort")
                .cloned()
                .unwrap_or_else(|| "id".to_string());
            super::commands::management::knowledge::KnowledgeAction::List {
                category,
                pending,
                sort,
            }
        }
        ("show", m) => super::commands::management::knowledge::KnowledgeAction::Show {
            id: m.get_one::<String>("id").expect("required").clone(),
        },
        ("explore", _) => super::commands::management::knowledge::KnowledgeAction::Explore,
        ("graph", _) => super::commands::management::knowledge::KnowledgeAction::Graph,
        ("impact", _) => super::commands::management::knowledge::KnowledgeAction::Impact,
        ("prune", _) => super::commands::management::knowledge::KnowledgeAction::Prune,
        (name, _) => return Err(anyhow::anyhow!("Unsupported knowledge subcommand: {name}")),
    };

    super::commands::management::knowledge::run(&resolve_board_dir()?, action)
}

fn handle_config_command(matches: &ArgMatches) -> Result<()> {
    super::commands::setup::config::run(parse_subcommand_action(matches)?)
}

fn parse_subcommand_action<T>(matches: &ArgMatches) -> Result<T>
where
    T: FromArgMatches,
{
    Ok(T::from_arg_matches(matches)?)
}
