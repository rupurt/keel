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

    let auth_file = matches
        .get_one::<std::path::PathBuf>("auth-file")
        .map(|p| p.as_path());
    let ctx = spoke_auth::load_auth_context(auth_file)?;

    let result = match matches.subcommand() {
        Some(("doctor", m)) => {
            let fix = *m.get_one::<bool>("fix").unwrap_or(&false);
            let evidence = *m.get_one::<bool>("evidence").unwrap_or(&false);
            let watch = *m.get_one::<bool>("watch").unwrap_or(&false);
            let quick = *m.get_one::<bool>("quick").unwrap_or(&false);
            let scene = *m.get_one::<bool>("scene").unwrap_or(&false);
            let status = *m.get_one::<bool>("status").unwrap_or(&false);
            match super::commands::diagnostics::doctor::run(
                &resolve_board_dir()?,
                fix,
                evidence,
                watch,
                quick,
                scene,
                status,
            ) {
                Ok(_) => Ok(()),
                Err(e) => {
                    let msg = e.to_string();
                    if msg.contains("errors") {
                        super::presentation::audio::play(
                            super::presentation::audio::SoundEvent::DoctorError,
                        );
                        std::process::exit(2);
                    } else if msg.contains("warnings") {
                        super::presentation::audio::play(
                            super::presentation::audio::SoundEvent::DoctorWarning,
                        );
                        std::process::exit(1);
                    } else {
                        Err(e)
                    }
                }
            }
        }
        Some(("heartbeat", m)) => {
            let json = *m.get_one::<bool>("json").unwrap_or(&false);
            super::commands::diagnostics::heartbeat::run(&resolve_board_dir()?, json)
        }
        Some(("health", m)) => {
            let scene = *m.get_one::<bool>("scene").unwrap_or(&false);
            super::commands::diagnostics::health::run(&resolve_board_dir()?, scene)
        }
        Some(("flow", m)) => {
            let no_color = *m.get_one::<bool>("no_color").unwrap_or(&false);
            let hide_routines = *m.get_one::<bool>("hide-routines").unwrap_or(&false);
            let scene = *m.get_one::<bool>("scene").unwrap_or(&false);
            super::commands::diagnostics::flow::run(
                &resolve_board_dir()?,
                no_color,
                !hide_routines,
                scene,
            )
        }
        Some(("workshop", m)) => {
            let scene = *m.get_one::<bool>("scene").unwrap_or(&false);
            super::commands::diagnostics::workshop::run(&resolve_board_dir()?, scene)
        }
        Some(("turn", m)) => {
            let json = *m.get_one::<bool>("json").unwrap_or(&false);
            super::commands::diagnostics::turn::run(json)
        }
        Some(("roles", m)) => {
            let role = m.get_one::<String>("role").map(|value| value.as_str());
            let json = *m.get_one::<bool>("json").unwrap_or(&false);
            super::commands::management::roles::run(&resolve_board_dir()?, role, json)
        }
        Some(("throughput", m)) => {
            let no_color = *m.get_one::<bool>("no_color").unwrap_or(&false);
            super::commands::diagnostics::throughput::run(&resolve_board_dir()?, no_color)
        }
        Some(("next", m)) => {
            let json = *m.get_one::<bool>("json").unwrap_or(&false);
            let parallel = *m.get_one::<bool>("parallel").unwrap_or(&false);
            let explain = *m.get_one::<bool>("explain").unwrap_or(&false);
            let role_str = m.get_one::<String>("role").expect("required");
            let role = super::commands::management::next::parse_actor_role(role_str)?;

            super::commands::management::next::run(
                &resolve_board_dir()?,
                json,
                parallel,
                explain,
                &role,
            )
        }
        Some(("pulse", m)) => {
            let json = *m.get_one::<bool>("json").unwrap_or(&false);
            let scene = *m.get_one::<bool>("scene").unwrap_or(&false);
            super::commands::management::pulse::run(json, scene)
        }
        Some(("topology", m)) => {
            let zoom = m
                .get_one::<String>("zoom")
                .expect("defaulted")
                .parse()
                .expect("validated by clap");
            let focus_id = m.get_one::<String>("focus").map(|value| value.as_str());
            let include_done = *m.get_one::<bool>("include_done").unwrap_or(&false);
            let static_output = *m.get_one::<bool>("static").unwrap_or(&false);
            super::commands::management::topology::run(zoom, focus_id, include_done, static_output)
        }
        Some(("screen", m)) => {
            let zoom = m
                .get_one::<String>("zoom")
                .expect("defaulted")
                .parse()
                .expect("validated by clap");
            let focus_id = m.get_one::<String>("focus").map(|value| value.as_str());
            let static_output = *m.get_one::<bool>("static").unwrap_or(&false);
            super::commands::management::topology::run(zoom, focus_id, false, static_output)
        }
        Some(("play", m)) => {
            let bearing = m.get_one::<String>("bearing").cloned();
            let prop = m.get_one::<String>("prop").cloned();
            let theater = *m.get_one::<bool>("theater").unwrap_or(&false);
            let theme = m.get_one::<String>("theme").cloned();
            let persona = m.get_one::<String>("persona").cloned();
            let mood = m.get_one::<String>("mood").cloned();
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
                theater,
                theme,
                persona,
                mood,
            )
        }
        Some(("audit", m)) => {
            let id = m.get_one::<String>("id").cloned();
            super::commands::management::story::audit::run(
                &ctx,
                &resolve_board_dir()?,
                id.as_deref(),
            )
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
        Some(("knowledge", m)) => handle_knowledge_command(&ctx, m),
        Some(("ping", m)) => {
            let message = m
                .get_many::<String>("message")
                .expect("required")
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            let json = *m.get_one::<bool>("json").unwrap_or(&false);
            super::commands::comms::ping::run(&ctx, &resolve_board_dir()?, &message, json)
        }
        Some(("poke", m)) => {
            let id = m.get_one::<String>("id");
            let message = m
                .get_many::<String>("message")
                .map(|vals| vals.map(|s| s.as_str()).collect::<Vec<_>>().join(" "));
            let json = *m.get_one::<bool>("json").unwrap_or(&false);
            let is_self = m.get_flag("self");
            super::commands::comms::poke::run(
                &ctx,
                &resolve_board_dir()?,
                id.map(|s| s.as_str()),
                message.as_deref(),
                is_self,
                json,
            )
        }
        Some(("inbox", m)) => {
            let json = *m.get_one::<bool>("json").unwrap_or(&false);
            super::commands::comms::inbox::run(&ctx, &resolve_board_dir()?, json)
        }
        Some(("outbox", m)) => {
            let json = *m.get_one::<bool>("json").unwrap_or(&false);
            super::commands::comms::outbox::run(&ctx, &resolve_board_dir()?, json)
        }
        Some(("generate", _)) => super::commands::setup::generate::run(&resolve_board_dir()?),
        Some(("new", m)) => {
            let path = m
                .get_one::<std::path::PathBuf>("path")
                .map(|value| value.as_path());
            Ok(super::commands::setup::new::run(path)?)
        }
        Some(("hooks", m)) => match m.subcommand() {
            Some(("install", _)) => super::commands::setup::hooks::run(),
            _ => unreachable!("subcommand_required"),
        },
        Some(("roadmap", _)) => super::commands::management::roadmap::run(),
        Some(("epic", m)) => handle_epic_command(&ctx, m),
        Some(("routine", m)) => handle_routine_command(&ctx, m),
        Some(("voyage", m)) => handle_voyage_command(&ctx, m),
        Some(("story", m)) => handle_story_command(&ctx, m),
        Some(("bearing", m)) => handle_bearing_command(&ctx, m),
        Some(("adr", m)) => handle_adr_command(&ctx, m),
        Some(("mission", m)) => handle_mission_command(&ctx, m),
        Some(("watch", m)) => handle_watch_command(m),
        Some(("finance", m)) => {
            let scene = *m.get_one::<bool>("scene").unwrap_or(&false);
            super::commands::diagnostics::finance::run(&resolve_board_dir()?, scene)
        }
        Some(("config", m)) => handle_config_command(&ctx, m),
        None => {
            let mut cli = build_cli();
            cli.print_long_help()?;
            println!();
            Ok(())
        }
        _ => unreachable!("Unhandled command"),
    };

    if result.is_ok() {
        auto_sync_artifacts();
        auto_stage_board();
    }

    result
}

/// Regenerate persisted board artifacts (READMEs and reports) so they stay in
/// sync with the board state after every command. Idempotent when generated
/// content already matches the current board snapshot.
fn auto_sync_artifacts() {
    let Ok(board_dir) = resolve_board_dir() else {
        return;
    };
    let Ok(board) = keel::infrastructure::loader::load_board(&board_dir) else {
        return;
    };
    let _ = keel::infrastructure::generate::sync_board_artifacts(
        &board,
        &keel::infrastructure::generate::BoardArtifactSyncOptions::default(),
    );
}

/// If `workflow.auto_stage` is enabled, stage all `.keel/` changes.
fn auto_stage_board() {
    let (config, _) = keel::infrastructure::config::load_config();
    if !config.workflow.auto_stage {
        return;
    }
    let Ok(board_dir) = resolve_board_dir() else {
        return;
    };
    let _ = std::process::Command::new("git")
        .args(["add", &board_dir.to_string_lossy()])
        .output();
}

fn handle_epic_command(ctx: &spoke_auth::ExecutionContext, matches: &ArgMatches) -> Result<()> {
    super::commands::management::epic::run(ctx, parse_subcommand_action(matches)?)
}

fn handle_voyage_command(ctx: &spoke_auth::ExecutionContext, matches: &ArgMatches) -> Result<()> {
    super::commands::management::voyage::run(
        ctx,
        &resolve_board_dir()?,
        parse_subcommand_action(matches)?,
    )
}

fn handle_routine_command(ctx: &spoke_auth::ExecutionContext, matches: &ArgMatches) -> Result<()> {
    super::commands::management::routine::run(ctx, parse_subcommand_action(matches)?)
}

fn handle_story_command(ctx: &spoke_auth::ExecutionContext, matches: &ArgMatches) -> Result<()> {
    super::commands::management::story::run(ctx, parse_subcommand_action(matches)?)
}

fn handle_bearing_command(ctx: &spoke_auth::ExecutionContext, matches: &ArgMatches) -> Result<()> {
    super::commands::management::bearing::run(ctx, parse_subcommand_action(matches)?)
}

fn handle_adr_command(ctx: &spoke_auth::ExecutionContext, matches: &ArgMatches) -> Result<()> {
    super::commands::management::adr::run(ctx, parse_subcommand_action(matches)?)
}

fn handle_mission_command(ctx: &spoke_auth::ExecutionContext, matches: &ArgMatches) -> Result<()> {
    super::commands::management::mission::run(ctx, parse_subcommand_action(matches)?)
}

fn handle_watch_command(matches: &ArgMatches) -> Result<()> {
    super::commands::management::watch::run(parse_subcommand_action(matches)?)
}

fn handle_knowledge_command(
    ctx: &spoke_auth::ExecutionContext,
    matches: &ArgMatches,
) -> Result<()> {
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
        ("graph", m) => super::commands::management::knowledge::KnowledgeAction::Graph {
            zoom: m
                .get_one::<String>("zoom")
                .cloned()
                .unwrap_or_else(|| "world".to_string()),
            focus: m.get_one::<String>("focus").cloned(),
            static_output: *m.get_one::<bool>("static").unwrap_or(&false),
        },
        ("impact", _) => super::commands::management::knowledge::KnowledgeAction::Impact,
        ("prune", _) => super::commands::management::knowledge::KnowledgeAction::Prune,
        (name, _) => return Err(anyhow::anyhow!("Unsupported knowledge subcommand: {name}")),
    };

    super::commands::management::knowledge::run(ctx, &resolve_board_dir()?, action)
}

fn handle_config_command(ctx: &spoke_auth::ExecutionContext, matches: &ArgMatches) -> Result<()> {
    super::commands::setup::config::run(ctx, parse_subcommand_action(matches)?)
}

fn parse_subcommand_action<T>(matches: &ArgMatches) -> Result<T>
where
    T: FromArgMatches,
{
    Ok(T::from_arg_matches(matches)?)
}
