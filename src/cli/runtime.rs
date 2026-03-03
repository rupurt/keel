//! CLI runtime dispatch entrypoint.
//!
//! Keeps the binary `main.rs` as a thin adapter by hosting argument
//! parsing and command routing inside the CLI layer.

use super::{build_cli, resolve_board_dir};
use anyhow::Result;
use clap::ArgMatches;

pub fn run() -> Result<()> {
    let matches = build_cli().get_matches();

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
        Some(("status", _)) => super::commands::diagnostics::status::run(&resolve_board_dir()?),
        Some(("flow", m)) => {
            let no_color = *m.get_one::<bool>("no_color").unwrap_or(&false);
            super::commands::diagnostics::flow::run(&resolve_board_dir()?, no_color)
        }
        Some(("next", m)) => {
            let agent = *m.get_one::<bool>("agent").unwrap_or(&false);
            let json = *m.get_one::<bool>("json").unwrap_or(&false);
            let parallel = *m.get_one::<bool>("parallel").unwrap_or(&false);
            let role_str = m.get_one::<String>("role");
            let role =
                super::commands::management::next::parse_actor_role(role_str.map(String::as_str));

            super::commands::management::next::run(
                &resolve_board_dir()?,
                agent,
                json,
                parallel,
                role.as_ref(),
            )
        }
        Some(("capacity", _)) => super::commands::diagnostics::capacity::run(&resolve_board_dir()?),
        Some(("gaps", _)) => super::commands::diagnostics::gaps::run(&resolve_board_dir()?),
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
        Some(("verify", m)) => {
            let id = m.get_one::<String>("id").map(|s| s.as_str());
            let all = *m.get_one::<bool>("all").unwrap_or(&false);
            super::commands::management::verify::run(&resolve_board_dir()?, id, all)
        }
        Some(("knowledge", m)) => handle_knowledge_command(m),
        Some(("generate", _)) => super::commands::setup::generate::run(&resolve_board_dir()?),
        Some(("init", _)) => Ok(super::commands::setup::init::run()?),
        Some(("epic", m)) => handle_epic_command(m),
        Some(("voyage", m)) => handle_voyage_command(m),
        Some(("story", m)) => handle_story_command(m),
        Some(("bearing", m)) => handle_bearing_command(m),
        Some(("adr", m)) => handle_adr_command(m),
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
    let command = matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("Missing epic subcommand"))?;
    let action = match command {
        ("new", m) => super::commands::management::epic::EpicAction::New {
            name: m.get_one::<String>("name").expect("required").clone(),
            description: m.get_one::<String>("description").cloned(),
            goal: m.get_one::<String>("goal").cloned(),
        },
        ("show", m) => super::commands::management::epic::EpicAction::Show {
            id: m.get_one::<String>("id").expect("required").clone(),
        },
        ("list", m) => super::commands::management::epic::EpicAction::List {
            status: m.get_one::<String>("status").cloned(),
        },
        (name, _) => return Err(anyhow::anyhow!("Unsupported epic subcommand: {name}")),
    };

    super::commands::management::epic::run(action)
}

fn handle_voyage_command(matches: &ArgMatches) -> Result<()> {
    let command = matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("Missing voyage subcommand"))?;
    let action = match command {
        ("new", m) => super::commands::management::voyage::VoyageAction::New {
            name: m.get_one::<String>("name").expect("required").clone(),
            epic: m.get_one::<String>("epic").expect("required").clone(),
            goal: m.get_one::<String>("goal").cloned(),
        },
        ("start", m) => super::commands::management::voyage::VoyageAction::Start {
            id: m.get_one::<String>("id").expect("required").clone(),
            force: *m.get_one::<bool>("force").unwrap_or(&false),
            expect_version: m.get_one::<u64>("expect_version").copied(),
        },
        ("plan", m) => super::commands::management::voyage::VoyageAction::Plan {
            id: m.get_one::<String>("id").expect("required").clone(),
            no_review: *m.get_one::<bool>("no_review").unwrap_or(&false),
        },
        ("done", m) => super::commands::management::voyage::VoyageAction::Done {
            id: m.get_one::<String>("id").expect("required").clone(),
            well: m.get_one::<String>("well").cloned(),
            hard: m.get_one::<String>("hard").cloned(),
            different: m.get_one::<String>("different").cloned(),
        },
        ("show", m) => super::commands::management::voyage::VoyageAction::Show {
            id: m.get_one::<String>("id").expect("required").clone(),
        },
        ("list", m) => super::commands::management::voyage::VoyageAction::List {
            epic: m.get_one::<String>("epic").cloned(),
            status: m.get_one::<String>("status").cloned(),
        },
        (name, _) => return Err(anyhow::anyhow!("Unsupported voyage subcommand: {name}")),
    };

    super::commands::management::voyage::run(action)
}

fn handle_story_command(matches: &ArgMatches) -> Result<()> {
    let command = matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("Missing story subcommand"))?;
    let action = match command {
        ("new", m) => super::commands::management::story::StoryAction::New {
            title: m.get_one::<String>("title").expect("required").clone(),
            r#type: m
                .get_one::<String>("type")
                .expect("defaulted in clap")
                .clone(),
            epic: m.get_one::<String>("epic").cloned(),
            voyage: m.get_one::<String>("voyage").cloned(),
            scope: m.get_one::<String>("scope").cloned(),
        },
        ("start", m) => super::commands::management::story::StoryAction::Start {
            id: m.get_one::<String>("id").expect("required").clone(),
            expect_version: m.get_one::<u64>("expect_version").copied(),
        },
        ("submit", m) => super::commands::management::story::StoryAction::Submit {
            id: m.get_one::<String>("id").expect("required").clone(),
        },
        ("accept", m) => super::commands::management::story::StoryAction::Accept {
            id: m.get_one::<String>("id").expect("required").clone(),
            human: *m.get_one::<bool>("human").unwrap_or(&false),
            reflect: m.get_one::<String>("reflect").cloned(),
        },
        ("reflect", m) => super::commands::management::story::StoryAction::Reflect {
            id: m.get_one::<String>("id").expect("required").clone(),
        },
        ("reject", m) => super::commands::management::story::StoryAction::Reject {
            id: m.get_one::<String>("id").expect("required").clone(),
            reason: m.get_one::<String>("reason").expect("required").clone(),
        },
        ("ice", m) => super::commands::management::story::StoryAction::Ice {
            id: m.get_one::<String>("id").expect("required").clone(),
        },
        ("thaw", m) => super::commands::management::story::StoryAction::Thaw {
            id: m.get_one::<String>("id").expect("required").clone(),
        },
        ("show", m) => super::commands::management::story::StoryAction::Show {
            id: m.get_one::<String>("id").expect("required").clone(),
        },
        ("list", m) => super::commands::management::story::StoryAction::List {
            stage: m.get_one::<String>("stage").cloned(),
            epic: m.get_one::<String>("epic").cloned(),
            reflections: *m.get_one::<bool>("reflections").unwrap_or(&false),
        },
        ("link", m) => super::commands::management::story::StoryAction::Link {
            id: m.get_one::<String>("id").expect("required").clone(),
            voyage: m.get_one::<String>("voyage").expect("required").clone(),
        },
        ("unlink", m) => super::commands::management::story::StoryAction::Unlink {
            id: m.get_one::<String>("id").expect("required").clone(),
            voyage: m.get_one::<String>("voyage").expect("required").clone(),
        },
        ("record", m) => super::commands::management::story::StoryAction::Record {
            id: m.get_one::<String>("id").expect("required").clone(),
            ac: m.get_one::<usize>("ac").cloned(),
            cmd: m.get_one::<String>("cmd").cloned(),
            msg: m.get_one::<String>("msg").cloned(),
            judge: *m.get_one::<bool>("judge").unwrap_or(&false),
            files: m
                .get_many::<String>("files")
                .map(|v| v.cloned().collect())
                .unwrap_or_default(),
        },
        (name, _) => return Err(anyhow::anyhow!("Unsupported story subcommand: {name}")),
    };

    super::commands::management::story::run(action)
}

fn handle_bearing_command(matches: &ArgMatches) -> Result<()> {
    let command = matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("Missing bearing subcommand"))?;
    let action = match command {
        ("new", m) => super::commands::management::bearing::BearingAction::New {
            name: m.get_one::<String>("name").expect("required").clone(),
        },
        ("survey", m) => super::commands::management::bearing::BearingAction::Survey {
            name: m.get_one::<String>("name").expect("required").clone(),
        },
        ("assess", m) => super::commands::management::bearing::BearingAction::Assess {
            name: m.get_one::<String>("name").expect("required").clone(),
        },
        ("list", m) => super::commands::management::bearing::BearingAction::List {
            status: m.get_one::<String>("status").cloned(),
        },
        ("show", m) => super::commands::management::bearing::BearingAction::Show {
            name: m.get_one::<String>("name").expect("required").clone(),
        },
        ("park", m) => super::commands::management::bearing::BearingAction::Park {
            name: m.get_one::<String>("name").expect("required").clone(),
        },
        ("decline", m) => super::commands::management::bearing::BearingAction::Decline {
            name: m.get_one::<String>("name").expect("required").clone(),
            reason: m.get_one::<String>("reason").expect("required").clone(),
        },
        ("lay", m) => super::commands::management::bearing::BearingAction::Lay {
            name: m.get_one::<String>("name").expect("required").clone(),
        },
        (name, _) => return Err(anyhow::anyhow!("Unsupported bearing subcommand: {name}")),
    };

    super::commands::management::bearing::run(action)
}

fn handle_adr_command(matches: &ArgMatches) -> Result<()> {
    let command = matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("Missing adr subcommand"))?;
    let action = match command {
        ("new", m) => super::commands::management::adr::AdrAction::New {
            title: m.get_one::<String>("title").expect("required").clone(),
        },
        ("list", m) => super::commands::management::adr::AdrAction::List {
            status: m.get_one::<String>("status").cloned(),
        },
        ("show", m) => super::commands::management::adr::AdrAction::Show {
            id: m.get_one::<String>("id").expect("required").clone(),
        },
        ("accept", m) => super::commands::management::adr::AdrAction::Accept {
            id: m.get_one::<String>("id").expect("required").clone(),
        },
        ("reject", m) => super::commands::management::adr::AdrAction::Reject {
            id: m.get_one::<String>("id").expect("required").clone(),
            reason: m.get_one::<String>("reason").expect("required").clone(),
        },
        ("deprecate", m) => super::commands::management::adr::AdrAction::Deprecate {
            id: m.get_one::<String>("id").expect("required").clone(),
            reason: m.get_one::<String>("reason").expect("required").clone(),
        },
        ("supersede", m) => super::commands::management::adr::AdrAction::Supersede {
            new_id: m.get_one::<String>("new_id").expect("required").clone(),
            old_id: m.get_one::<String>("old_id").expect("required").clone(),
        },
        (name, _) => return Err(anyhow::anyhow!("Unsupported adr subcommand: {name}")),
    };

    super::commands::management::adr::run(action)
}

fn handle_knowledge_command(matches: &ArgMatches) -> Result<()> {
    let command = matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("Missing knowledge subcommand"))?;
    match command {
        ("list", m) => {
            let category = m.get_one::<String>("category").cloned();
            let pending = *m.get_one::<bool>("pending").unwrap_or(&false);
            super::commands::management::knowledge::run(
                &resolve_board_dir()?,
                super::commands::management::knowledge::KnowledgeAction::List { category, pending },
            )
        }
        ("show", m) => {
            let id = m.get_one::<String>("id").expect("required");
            super::commands::management::knowledge::run(
                &resolve_board_dir()?,
                super::commands::management::knowledge::KnowledgeAction::Show { id: id.clone() },
            )
        }
        ("explore", _) => super::commands::management::knowledge::run(
            &resolve_board_dir()?,
            super::commands::management::knowledge::KnowledgeAction::Explore,
        ),
        ("graph", _) => super::commands::management::knowledge::run(
            &resolve_board_dir()?,
            super::commands::management::knowledge::KnowledgeAction::Graph,
        ),
        ("impact", _) => super::commands::management::knowledge::run(
            &resolve_board_dir()?,
            super::commands::management::knowledge::KnowledgeAction::Impact,
        ),
        (name, _) => Err(anyhow::anyhow!("Unsupported knowledge subcommand: {name}")),
    }
}

fn handle_config_command(matches: &ArgMatches) -> Result<()> {
    let command = matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("Missing config subcommand"))?;
    match command {
        ("show", _) => super::commands::setup::config::run_show(),
        ("mode", m) => {
            let name = m.get_one::<String>("name").cloned();
            super::commands::setup::config::run_mode(name)
        }
        (name, _) => Err(anyhow::anyhow!("Unsupported config subcommand: {name}")),
    }
}
