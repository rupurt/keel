//! CLI parsing tests — extracted from main.rs
use crate::cli::commands::management::adr::AdrAction;
use crate::cli::commands::management::bearing::BearingAction;
use crate::cli::commands::management::epic::EpicAction;
use crate::cli::commands::management::mission::MissionAction;
use crate::cli::commands::management::routine::RoutineAction;
use crate::cli::commands::management::story::StoryAction;
use crate::cli::commands::management::voyage::VoyageAction;
use crate::cli::commands::setup::config::ConfigAction;
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "board")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(flatten)]
    Diagnostics(DiagnosticsCommands),
    #[command(flatten)]
    Management(ManagementCommands),
}

#[derive(Debug, Subcommand)]
enum DiagnosticsCommands {
    /// Validate board health and optionally fix issues
    Doctor {
        /// Auto-fix safe issues without prompting
        #[arg(long)]
        fix: bool,
        /// Show detailed evidence chains for requirement traceability
        #[arg(long)]
        evidence: bool,
        /// Watch mode: continuously monitor for changes and re-validate (SRS-06)
        #[arg(long)]
        watch: bool,
        /// Quick mode: run fast structural checks only (SRS-08, SRS-09)
        #[arg(long)]
        quick: bool,
    },
    /// Show workflow lane dashboard from configured topology
    Flow {
        /// Disable color output (also respects NO_COLOR env var)
        #[arg(long)]
        no_color: bool,
    },
    /// Show weekly throughput and timing sparklines
    Throughput {
        /// Disable color output (also respects NO_COLOR env var)
        #[arg(long)]
        no_color: bool,
    },
    /// Surface the single most important thing to work on
    Next {
        /// Role taxonomy controlling queue selection (e.g., "manager" or "operator/software:infrastructure")
        #[arg(long, required = true)]
        role: Option<String>,
        /// Output as JSON for scripting
        #[arg(long)]
        json: bool,
        /// Return all parallel-safe stories for batch dispatch
        #[arg(long)]
        parallel: bool,
    },
    /// Invite play-driven discovery
    Play {
        /// Bearing ID to generate a play scenario from
        bearing: Option<String>,
        /// Start with a specific prop equipped
        #[arg(long)]
        prop: Option<String>,
        /// Cross two bearings for a paired play session
        #[arg(long, value_names = ["id1", "id2"], num_args = 2)]
        cross: Option<Vec<String>>,
        /// List available props by category
        #[arg(long)]
        list_props: bool,
        /// Suggest a mask for a bearing based on its content
        #[arg(long)]
        suggest: Option<String>,
    },
    /// Execute verification proofs
    Verify {
        /// ID of story, voyage, or epic to verify (default: all)
        id: Option<String>,
        /// Verify all stories on the board
        #[arg(long)]
        all: bool,
    },
}

#[derive(Debug, Subcommand)]
enum ManagementCommands {
    /// Regenerate all README files
    Generate,
    /// Create a new Keel project scaffold
    New {
        /// Target directory for the new project scaffold (defaults to current directory)
        path: Option<std::path::PathBuf>,
    },
    /// Upgrade keel from the latest release or a git ref
    Upgrade {
        /// Optional git tag or commit to build and install from source
        #[arg(long = "ref")]
        ref_spec: Option<String>,
    },
    /// Epic commands
    Epic {
        #[command(subcommand)]
        action: EpicAction,
    },
    /// Voyage commands
    Voyage {
        #[command(subcommand)]
        action: VoyageAction,
    },
    /// Routine commands
    Routine {
        #[command(subcommand)]
        action: RoutineAction,
    },
    /// Story commands
    Story {
        #[command(subcommand)]
        action: StoryAction,
    },
    /// Bearing commands (research phase)
    Bearing {
        #[command(subcommand)]
        action: BearingAction,
    },
    /// ADR commands (architecture decisions)
    Adr {
        #[command(subcommand)]
        action: AdrAction,
    },
    /// Configuration commands
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Mission commands
    Mission {
        #[command(subcommand)]
        action: MissionAction,
    },
}

#[test]
fn cli_help_displays_top_level_commands() {
    let mut cmd = crate::cli::build_cli();
    let mut help = Vec::new();
    cmd.write_long_help(&mut help).unwrap();
    let help_str = String::from_utf8(help).unwrap();

    // Verify top-level commands
    assert!(help_str.contains("auth"), "Missing auth command");
    assert!(help_str.contains("doctor"), "Missing doctor command");
    assert!(help_str.contains("new"), "Missing new command");
    assert!(help_str.contains("upgrade"), "Missing upgrade command");
    assert!(help_str.contains("generate"), "Missing generate command");
    assert!(help_str.contains("story"), "Missing story subcommand");
    assert!(help_str.contains("epic"), "Missing epic subcommand");
    assert!(help_str.contains("routine"), "Missing routine subcommand");
    assert!(help_str.contains("pulse"), "Missing pulse command");
    assert!(help_str.contains("voyage"), "Missing voyage subcommand");
    assert!(help_str.contains("roadmap"), "Missing roadmap command");
    assert!(help_str.contains("verify"), "Missing verify command");

    // Verify groups (The Ramping Path section)
    let diag_section = help_str
        .find("The Ramping Path")
        .expect("Missing The Ramping Path section");
    let after_diag = &help_str[diag_section..];
    assert!(
        after_diag.contains("verify"),
        "verify command not in The Ramping Path section of help groups"
    );
}

#[test]
fn cli_parses_generate_command() {
    let cli = Cli::try_parse_from(["board", "generate"]).unwrap();
    assert!(matches!(
        cli.command,
        Commands::Management(ManagementCommands::Generate)
    ));
}

#[test]
fn cli_parses_new_command_with_optional_path() {
    let cli = Cli::try_parse_from(["board", "new", "harbor"]).unwrap();
    match cli.command {
        Commands::Management(ManagementCommands::New { path }) => {
            assert_eq!(path.unwrap(), std::path::PathBuf::from("harbor"));
        }
        other => panic!("unexpected command: {other:?}"),
    }
}

#[test]
fn cli_parses_upgrade_command_with_optional_ref() {
    let cli = Cli::try_parse_from(["board", "upgrade", "--ref", "v0.1.0"]).unwrap();
    match cli.command {
        Commands::Management(ManagementCommands::Upgrade { ref_spec }) => {
            assert_eq!(ref_spec.as_deref(), Some("v0.1.0"));
        }
        other => panic!("unexpected command: {other:?}"),
    }
}

#[test]
fn build_cli_parses_upgrade_without_ref() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from(["keel", "upgrade"])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("upgrade"));
    let upgrade = matches.subcommand_matches("upgrade").unwrap();
    assert!(upgrade.get_one::<String>("git-ref").is_none());
}

#[test]
fn build_cli_parses_auth_login() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from([
            "keel",
            "auth",
            "login",
            "--email",
            "pilot@spoke.sh",
            "--password",
            "secret",
            "--hub-url",
            "https://hub.spoke.test",
        ])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("auth"));
    let auth = matches.subcommand_matches("auth").unwrap();
    assert_eq!(auth.subcommand_name(), Some("login"));
}

#[test]
fn build_cli_parses_auth_info_and_logout() {
    let info = crate::cli::build_cli()
        .try_get_matches_from(["keel", "auth", "info", "--json"])
        .unwrap();
    assert_eq!(info.subcommand_name(), Some("auth"));
    assert_eq!(
        info.subcommand_matches("auth").unwrap().subcommand_name(),
        Some("info")
    );

    let logout = crate::cli::build_cli()
        .try_get_matches_from(["keel", "auth", "logout"])
        .unwrap();
    assert_eq!(logout.subcommand_name(), Some("auth"));
    assert_eq!(
        logout.subcommand_matches("auth").unwrap().subcommand_name(),
        Some("logout")
    );
}

#[test]
fn cli_rejects_removed_init_command() {
    let result = crate::cli::build_cli().try_get_matches_from(["keel", "init"]);
    assert!(result.is_err());
}

#[test]
fn cli_parses_routine_new() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from([
            "keel",
            "routine",
            "new",
            "Weekly Review",
            "--target-scope",
            "E1/V1",
            "--cadence",
            "cron=0 9 * * 1",
            "--cadence",
            "timezone=America/Los_Angeles",
        ])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("routine"));
    let routine = matches.subcommand_matches("routine").unwrap();
    assert_eq!(routine.subcommand_name(), Some("new"));
}

#[test]
fn cli_parses_routine_list() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from(["keel", "routine", "list"])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("routine"));
    let routine = matches.subcommand_matches("routine").unwrap();
    assert_eq!(routine.subcommand_name(), Some("list"));
}

#[test]
fn cli_parses_routine_show() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from(["keel", "routine", "show", "routine-weekly-review"])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("routine"));
    let routine = matches.subcommand_matches("routine").unwrap();
    assert_eq!(routine.subcommand_name(), Some("show"));
}

#[test]
fn cli_parses_mission_next_without_id() {
    let cli = Cli::try_parse_from(["board", "mission", "next"]).unwrap();
    assert!(matches!(
        cli.command,
        Commands::Management(ManagementCommands::Mission {
            action: MissionAction::Next { id: None, .. }
        })
    ));
}

#[test]
fn cli_parses_mission_attach_bearing_flag() {
    let cli = Cli::try_parse_from(["board", "mission", "attach", "M1", "--bearing", "B1"]).unwrap();
    assert!(matches!(
        cli.command,
        Commands::Management(ManagementCommands::Mission {
            action: MissionAction::Attach {
                mission_id,
                bearing: Some(bearing),
                epic: None,
                adr: None,
            }
        }) if mission_id == "M1" && bearing == "B1"
    ));
}

#[test]
fn cli_parses_mission_attach_epic_flag() {
    let cli = Cli::try_parse_from(["board", "mission", "attach", "M1", "--epic", "E1"]).unwrap();
    assert!(matches!(
        cli.command,
        Commands::Management(ManagementCommands::Mission {
            action: MissionAction::Attach {
                mission_id,
                bearing: None,
                epic: Some(epic),
                adr: None,
            }
        }) if mission_id == "M1" && epic == "E1"
    ));
}

#[test]
fn cli_parses_mission_attach_adr_flag() {
    let cli =
        Cli::try_parse_from(["board", "mission", "attach", "M1", "--adr", "ADR-0001"]).unwrap();
    assert!(matches!(
        cli.command,
        Commands::Management(ManagementCommands::Mission {
            action: MissionAction::Attach {
                mission_id,
                bearing: None,
                epic: None,
                adr: Some(adr),
            }
        }) if mission_id == "M1" && adr == "ADR-0001"
    ));
}

#[test]
fn cli_rejects_legacy_mission_attach_positional_target() {
    let result = Cli::try_parse_from(["board", "mission", "attach", "M1", "B1"]);
    assert!(result.is_err());
}

#[test]
fn cli_rejects_multiple_mission_attach_target_flags() {
    let result = Cli::try_parse_from([
        "board",
        "mission",
        "attach",
        "M1",
        "--bearing",
        "B1",
        "--epic",
        "E1",
    ]);
    assert!(result.is_err());
}

#[test]
fn cli_parses_mission_next_with_id() {
    let cli = Cli::try_parse_from(["board", "mission", "next", "M1"]).unwrap();
    assert!(matches!(
        cli.command,
        Commands::Management(ManagementCommands::Mission {
            action: MissionAction::Next { id: Some(id), .. }
        }) if id == "M1"
    ));
}

#[test]
fn cli_parses_mission_resume() {
    let cli = Cli::try_parse_from(["board", "mission", "resume", "M1"]).unwrap();
    assert!(matches!(
        cli.command,
        Commands::Management(ManagementCommands::Mission {
            action: MissionAction::Resume { id }
        }) if id == "M1"
    ));
}

#[test]
fn cli_parses_pulse_with_json() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from(["keel", "pulse", "--json"])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("pulse"));
    let pulse = matches.subcommand_matches("pulse").unwrap();
    assert!(*pulse.get_one::<bool>("json").unwrap());
}

#[test]
fn cli_rejects_removed_migrate_command() {
    let result = crate::cli::build_cli().try_get_matches_from(["keel", "migrate"]);
    assert!(result.is_err());
}

#[test]
fn cli_rejects_removed_status_command() {
    let result = crate::cli::build_cli().try_get_matches_from(["keel", "status"]);
    assert!(result.is_err());
}

#[test]
fn cli_rejects_removed_capacity_command() {
    let result = crate::cli::build_cli().try_get_matches_from(["keel", "capacity"]);
    assert!(result.is_err());
}

#[test]
fn cli_rejects_removed_gaps_command() {
    let result = crate::cli::build_cli().try_get_matches_from(["keel", "gaps"]);
    assert!(result.is_err());
}

#[test]
fn cli_parses_topology_command() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from([
            "keel",
            "topology",
            "--zoom",
            "story",
            "--focus",
            "e1",
            "--include-done",
            "--static",
        ])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("topology"));
    let topology = matches.subcommand_matches("topology").unwrap();
    assert_eq!(
        topology
            .get_one::<String>("zoom")
            .map(|value| value.as_str()),
        Some("story")
    );
    assert_eq!(
        topology
            .get_one::<String>("focus")
            .map(|value| value.as_str()),
        Some("e1")
    );
    assert!(*topology.get_one::<bool>("include_done").unwrap());
    assert!(*topology.get_one::<bool>("static").unwrap());
}

#[test]
fn cli_parses_roadmap_command() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from(["keel", "roadmap"])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("roadmap"));
}

#[test]
fn cli_parses_topology_command_without_focus() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from(["keel", "topology"])
        .unwrap();
    let topology = matches.subcommand_matches("topology").unwrap();
    assert_eq!(
        topology
            .get_one::<String>("zoom")
            .map(|value| value.as_str()),
        Some("world")
    );
    assert!(topology.get_one::<String>("focus").is_none());
    assert!(!topology.get_one::<bool>("static").copied().unwrap_or(false));
}

#[test]
fn cli_parses_knowledge_prune_command() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from(["keel", "knowledge", "prune"])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("knowledge"));
    let knowledge = matches.subcommand_matches("knowledge").unwrap();
    assert_eq!(knowledge.subcommand_name(), Some("prune"));
}

#[test]
fn cli_parses_knowledge_graph_command_with_static_surface_flags() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from([
            "keel",
            "knowledge",
            "graph",
            "--zoom",
            "artifact",
            "--focus",
            "story:S1",
            "--static",
        ])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("knowledge"));
    let knowledge = matches.subcommand_matches("knowledge").unwrap();
    let graph = knowledge.subcommand_matches("graph").unwrap();
    assert_eq!(
        graph.get_one::<String>("zoom").map(|value| value.as_str()),
        Some("artifact")
    );
    assert_eq!(
        graph.get_one::<String>("focus").map(|value| value.as_str()),
        Some("story:S1")
    );
    assert!(*graph.get_one::<bool>("static").unwrap());
}

#[test]
fn cli_rejects_removed_knowledge_migrate_command() {
    let result = crate::cli::build_cli().try_get_matches_from(["keel", "knowledge", "migrate"]);
    assert!(result.is_err());
}

#[test]
fn cli_parses_verify_command() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from(["keel", "verify", "run", "S1", "--all", "--json"])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("verify"));
    let sub_m = matches.subcommand_matches("verify").unwrap();
    let run_m = sub_m.subcommand_matches("run").unwrap();
    assert_eq!(
        run_m.get_one::<String>("id").map(|s| s.as_str()),
        Some("S1")
    );
    assert!(*run_m.get_one::<bool>("all").unwrap());
    assert!(*run_m.get_one::<bool>("json").unwrap());
}

#[test]
fn cli_parses_verify_recommend_command() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from(["keel", "verify", "recommend", "--json"])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("verify"));
    let sub_m = matches.subcommand_matches("verify").unwrap();
    let recommend_m = sub_m.subcommand_matches("recommend").unwrap();
    assert!(*recommend_m.get_one::<bool>("json").unwrap());
}

#[test]
fn cli_parses_verify_detect_command() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from(["keel", "verify", "detect", "--json"])
        .unwrap();
    assert_eq!(matches.subcommand_name(), Some("verify"));
    let sub_m = matches.subcommand_matches("verify").unwrap();
    let detect_m = sub_m.subcommand_matches("detect").unwrap();
    assert!(*detect_m.get_one::<bool>("json").unwrap());
}

#[test]
fn cli_rejects_verify_without_subcommand() {
    let result = crate::cli::build_cli().try_get_matches_from(["keel", "verify"]);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("verify"));
    assert!(err.contains("COMMAND"));
}

#[test]
fn cli_parses_flow_command() {
    let cli = Cli::try_parse_from(["board", "flow"]).unwrap();
    assert!(matches!(
        cli.command,
        Commands::Diagnostics(DiagnosticsCommands::Flow { .. })
    ));
}

#[test]
fn cli_parses_throughput_command() {
    let cli = Cli::try_parse_from(["board", "throughput", "--no-color"]).unwrap();
    if let Commands::Diagnostics(DiagnosticsCommands::Throughput { no_color }) = cli.command {
        assert!(no_color);
    } else {
        panic!("Expected Throughput command");
    }
}

#[test]
fn cli_parses_voyage_new_requires_epic() {
    // Should fail without --epic
    let result = Cli::try_parse_from(["board", "voyage", "new", "fast-search"]);
    assert!(result.is_err());

    // Should succeed with --epic
    let cli = Cli::try_parse_from([
        "board",
        "voyage",
        "new",
        "fast-search",
        "--epic",
        "performance",
        "--goal",
        "Improve planning visibility",
    ])
    .unwrap();
    if let Commands::Management(ManagementCommands::Voyage {
        action: VoyageAction::New { name, epic, .. },
    }) = cli.command
    {
        assert_eq!(name, "fast-search");
        assert_eq!(epic, "performance");
    } else {
        panic!("Expected Voyage New command");
    }
}

#[test]
fn cli_parses_voyage_new_requires_goal() {
    let result = Cli::try_parse_from([
        "board",
        "voyage",
        "new",
        "command-restructure",
        "--epic",
        "board-cli",
    ]);
    assert!(
        result.is_err(),
        "Expected parse error when --goal is missing"
    );
}

// ========== Entity-first command tests ==========

#[test]
fn cli_parses_story_start() {
    let cli = Cli::try_parse_from(["board", "story", "start", "FEAT0238"]).unwrap();
    if let Commands::Management(ManagementCommands::Story {
        action: StoryAction::Start { id, expect_version },
    }) = cli.command
    {
        assert_eq!(id, "FEAT0238");
        assert!(expect_version.is_none());
    } else {
        panic!("Expected Story Start command");
    }
}

#[test]
fn cli_parses_story_reflect() {
    let cli = Cli::try_parse_from(["board", "story", "reflect", "FEAT0238"]).unwrap();
    if let Commands::Management(ManagementCommands::Story {
        action: StoryAction::Reflect { id },
    }) = cli.command
    {
        assert_eq!(id, "FEAT0238");
    } else {
        panic!("Expected Story Reflect command");
    }
}

#[test]
fn cli_parses_story_accept_with_role() {
    let cli =
        Cli::try_parse_from(["board", "story", "accept", "FEAT0238", "--role", "manager"]).unwrap();
    if let Commands::Management(ManagementCommands::Story {
        action: StoryAction::Accept { id, role, reflect },
    }) = cli.command
    {
        assert_eq!(id, "FEAT0238");
        assert_eq!(role, "manager");
        assert!(reflect.is_none());
    } else {
        panic!("Expected Story Accept command");
    }
}

#[test]
fn cli_rejects_story_accept_without_role() {
    let result = Cli::try_parse_from(["board", "story", "accept", "FEAT0238"]);
    assert!(
        result.is_err(),
        "Expected parse error when --role is missing"
    );
}

#[test]
fn cli_parses_story_ice() {
    let cli = Cli::try_parse_from(["board", "story", "ice", "FEAT0001"]).unwrap();
    if let Commands::Management(ManagementCommands::Story {
        action: StoryAction::Ice { id },
    }) = cli.command
    {
        assert_eq!(id, "FEAT0001");
    } else {
        panic!("Expected Story Ice command");
    }
}

#[test]
fn cli_parses_story_thaw() {
    let cli = Cli::try_parse_from(["board", "story", "thaw", "FEAT0001"]).unwrap();
    if let Commands::Management(ManagementCommands::Story {
        action: StoryAction::Thaw { id },
    }) = cli.command
    {
        assert_eq!(id, "FEAT0001");
    } else {
        panic!("Expected Story Thaw command");
    }
}

#[test]
fn cli_parses_story_show() {
    let cli = Cli::try_parse_from(["board", "story", "show", "FEAT0001"]).unwrap();
    if let Commands::Management(ManagementCommands::Story {
        action: StoryAction::Show { id, compact: _ },
    }) = cli.command
    {
        assert_eq!(id, "FEAT0001");
    } else {
        panic!("Expected Story Show command");
    }
}

#[test]
fn cli_parses_story_list() {
    let cli = Cli::try_parse_from(["board", "story", "list"]).unwrap();
    if let Commands::Management(ManagementCommands::Story {
        action:
            StoryAction::List {
                status,
                epic,
                reflections,
            },
    }) = cli.command
    {
        assert!(status.is_empty());
        assert!(epic.is_none());
        assert!(!reflections);
    } else {
        panic!("Expected Story List command");
    }
}

#[test]
fn cli_parses_story_list_with_filters() {
    let cli = Cli::try_parse_from([
        "board",
        "story",
        "list",
        "--status",
        "backlog",
        "--status",
        "+done",
        "--epic",
        "board-cli",
    ])
    .unwrap();
    if let Commands::Management(ManagementCommands::Story {
        action:
            StoryAction::List {
                status,
                epic,
                reflections,
            },
    }) = cli.command
    {
        assert_eq!(status, vec!["backlog".to_string(), "+done".to_string()]);
        assert_eq!(epic, Some("board-cli".to_string()));
        assert!(!reflections);
    } else {
        panic!("Expected Story List command");
    }
}

#[test]
fn cli_parses_story_new() {
    let cli =
        Cli::try_parse_from(["board", "story", "new", "Add login", "--type", "feat"]).unwrap();
    if let Commands::Management(ManagementCommands::Story {
        action:
            StoryAction::New {
                title,
                r#type,
                epic,
                voyage,
            },
    }) = cli.command
    {
        assert_eq!(title, "Add login");
        assert_eq!(r#type, "feat");
        assert_eq!(epic, None);
        assert_eq!(voyage, None);
    } else {
        panic!("Expected Story New command");
    }
}

#[test]
fn cli_rejects_story_new_scope_flag() {
    let result = crate::cli::build_cli().try_get_matches_from([
        "keel",
        "story",
        "new",
        "Add login",
        "--scope",
        "web-ui/01-auth",
    ]);
    assert!(result.is_err(), "Expected parse error for removed --scope");
}

#[test]
fn cli_parses_story_new_scope_flags() {
    let cli = Cli::try_parse_from([
        "board",
        "story",
        "new",
        "Test Story",
        "--type",
        "feat",
        "--epic",
        "board",
        "--voyage",
        "09-acceptance-workflow",
    ])
    .unwrap();

    if let Commands::Management(ManagementCommands::Story {
        action:
            StoryAction::New {
                title,
                r#type,
                epic,
                voyage,
            },
    }) = cli.command
    {
        assert_eq!(title, "Test Story");
        assert_eq!(r#type, "feat");
        assert_eq!(epic.as_deref(), Some("board"));
        assert_eq!(voyage.as_deref(), Some("09-acceptance-workflow"));
    } else {
        panic!("Expected scoped Story New command");
    }
}

#[test]
fn build_cli_story_new_stays_in_sync_with_story_action_contract() {
    let matches = crate::cli::build_cli()
        .try_get_matches_from([
            "keel",
            "story",
            "new",
            "Test Story",
            "--type",
            "feat",
            "--epic",
            "board",
            "--voyage",
            "09-acceptance-workflow",
        ])
        .unwrap();
    let story_matches = matches.subcommand_matches("story").unwrap();
    let action = StoryAction::from_arg_matches(story_matches).unwrap();

    assert!(matches!(
        action,
        StoryAction::New {
            title,
            r#type,
            epic,
            voyage,
        } if title == "Test Story"
            && r#type == "feat"
            && epic.as_deref() == Some("board")
            && voyage.as_deref() == Some("09-acceptance-workflow")
    ));
}

#[test]
fn cli_rejects_story_new_voyage_without_epic_flag() {
    let result = crate::cli::build_cli().try_get_matches_from([
        "keel",
        "story",
        "new",
        "Test Story",
        "--voyage",
        "09-acceptance-workflow",
    ]);
    assert!(
        result.is_err(),
        "Expected parse error when --voyage lacks --epic"
    );
}

#[test]
fn cli_parses_config_show_json() {
    let cli = Cli::try_parse_from(["board", "config", "show", "--json"]).unwrap();
    if let Commands::Management(ManagementCommands::Config {
        action: ConfigAction::Show { json },
    }) = cli.command
    {
        assert!(json);
    } else {
        panic!("Expected Config Show command");
    }
}

#[test]
fn cli_parses_story_link() {
    let cli = Cli::try_parse_from(["board", "story", "link", "FEAT0001", "01-core"]).unwrap();
    if let Commands::Management(ManagementCommands::Story {
        action: StoryAction::Link { id, voyage },
    }) = cli.command
    {
        assert_eq!(id, "FEAT0001");
        assert_eq!(voyage, "01-core");
    } else {
        panic!("Expected Story Link command");
    }
}

#[test]
fn cli_parses_story_unlink() {
    let cli = Cli::try_parse_from(["board", "story", "unlink", "FEAT0001", "01-core"]).unwrap();
    if let Commands::Management(ManagementCommands::Story {
        action: StoryAction::Unlink { id, voyage },
    }) = cli.command
    {
        assert_eq!(id, "FEAT0001");
        assert_eq!(voyage, "01-core");
    } else {
        panic!("Expected Story Unlink command");
    }
}

#[test]
fn cli_parses_epic_new_with_required_problem() {
    let cli = Cli::try_parse_from([
        "board",
        "epic",
        "new",
        "auth-system",
        "--problem",
        "Users cannot complete login and session recovery reliably",
    ])
    .unwrap();
    if let Commands::Management(ManagementCommands::Epic {
        action: EpicAction::New { name, problem },
    }) = cli.command
    {
        assert_eq!(name, "auth-system");
        assert_eq!(
            problem,
            "Users cannot complete login and session recovery reliably"
        );
    } else {
        panic!("Expected Epic New command");
    }
}

#[test]
fn cli_parses_epic_new_requires_problem() {
    let result = Cli::try_parse_from(["board", "epic", "new", "auth-system"]);
    assert!(
        result.is_err(),
        "Expected parse error when --problem is missing"
    );
}

#[test]
fn cli_rejects_epic_new_description_flag() {
    let result = crate::cli::build_cli().try_get_matches_from([
        "keel",
        "epic",
        "new",
        "auth-system",
        "--problem",
        "Users cannot complete login and session recovery reliably",
        "--description",
        "Authentication epic",
    ]);
    assert!(
        result.is_err(),
        "Expected parse error for removed --description"
    );
}

#[test]
fn cli_creation_commands_reject_system_owned_flags() {
    let cases = vec![
        (
            vec![
                "keel",
                "epic",
                "new",
                "Auth System",
                "--problem",
                "Users cannot complete login reliably",
                "--id",
                "E-001",
            ],
            "epic new",
        ),
        (
            vec![
                "keel",
                "voyage",
                "new",
                "Command Restructure",
                "--epic",
                "board",
                "--goal",
                "Improve decomposition",
                "--status",
                "draft",
            ],
            "voyage new",
        ),
        (
            vec![
                "keel",
                "story",
                "new",
                "Add Login",
                "--type",
                "feat",
                "--created-at",
                "2026-01-01T00:00:00",
            ],
            "story new",
        ),
        (
            vec!["keel", "bearing", "new", "Discovery Spike", "--index", "1"],
            "bearing new",
        ),
        (
            vec![
                "keel",
                "adr",
                "new",
                "Store Events",
                "--updated-at",
                "2026-01-01T00:00:00",
            ],
            "adr new",
        ),
    ];

    for (args, command_name) in cases {
        let result = crate::cli::build_cli().try_get_matches_from(args);
        assert!(
            result.is_err(),
            "Expected parse error when system-owned field flag is used in {command_name}"
        );
    }
}

#[test]
fn cli_parses_epic_show() {
    let cli = Cli::try_parse_from(["board", "epic", "show", "board-cli"]).unwrap();
    if let Commands::Management(ManagementCommands::Epic {
        action: EpicAction::Show { id },
    }) = cli.command
    {
        assert_eq!(id, "board-cli");
    } else {
        panic!("Expected Epic Show command");
    }
}

#[test]
fn cli_parses_epic_list() {
    let cli = Cli::try_parse_from(["board", "epic", "list"]).unwrap();
    if let Commands::Management(ManagementCommands::Epic {
        action: EpicAction::List { status },
    }) = cli.command
    {
        assert!(status.is_empty());
    } else {
        panic!("Expected Epic List command");
    }
}

#[test]
fn bearing_research_command_contract() {
    let cli = Cli::try_parse_from(["board", "bearing", "research", "B0"]).unwrap();
    if let Commands::Management(ManagementCommands::Bearing {
        action: BearingAction::Research { id, .. },
    }) = cli.command
    {
        assert_eq!(id, "B0");
    } else {
        panic!("Expected Bearing Research command");
    }

    let capture = Cli::try_parse_from([
        "board",
        "bearing",
        "research",
        "B0",
        "--class",
        "web",
        "--provider",
        "web-search",
        "--location",
        "https://example.com",
        "--observed-at",
        "2026-03-08",
        "--retrieved-at",
        "2026-03-09",
        "--authority",
        "high",
        "--freshness",
        "medium",
        "--notes",
        "Captured claim",
    ])
    .unwrap();
    if let Commands::Management(ManagementCommands::Bearing {
        action:
            BearingAction::Research {
                id,
                class,
                provider,
                location,
                observed_at,
                retrieved_at,
                authority,
                freshness,
                notes,
            },
    }) = capture.command
    {
        assert_eq!(id, "B0");
        assert_eq!(class.as_deref(), Some("web"));
        assert_eq!(provider.as_deref(), Some("web-search"));
        assert_eq!(location.as_deref(), Some("https://example.com"));
        assert_eq!(observed_at.as_deref(), Some("2026-03-08"));
        assert_eq!(retrieved_at.as_deref(), Some("2026-03-09"));
        assert_eq!(authority.as_deref(), Some("high"));
        assert_eq!(freshness.as_deref(), Some("medium"));
        assert_eq!(notes.as_deref(), Some("Captured claim"));
    } else {
        panic!("Expected Bearing Research capture command");
    }

    let legacy = crate::cli::build_cli().try_get_matches_from(["keel", "bearing", "survey", "B0"]);
    assert!(
        legacy.is_err(),
        "legacy survey subcommand should not remain in the public CLI contract"
    );
}

#[test]
fn cli_parses_epic_list_with_filter() {
    let cli = Cli::try_parse_from([
        "board", "epic", "list", "--status", "active", "--status", "+done",
    ])
    .unwrap();
    if let Commands::Management(ManagementCommands::Epic {
        action: EpicAction::List { status },
    }) = cli.command
    {
        assert_eq!(status, vec!["active".to_string(), "+done".to_string()]);
    } else {
        panic!("Expected Epic List command");
    }
}

#[test]
fn cli_parses_adr_new_with_context_and_applies_to() {
    let cli = Cli::try_parse_from([
        "board",
        "adr",
        "new",
        "Store Events",
        "--context",
        "work-management",
        "--applies-to",
        "queue-policy",
        "--applies-to",
        "story-lifecycle",
    ])
    .unwrap();

    if let Commands::Management(ManagementCommands::Adr {
        action:
            AdrAction::New {
                title,
                context,
                applies_to,
            },
    }) = cli.command
    {
        assert_eq!(title, "Store Events");
        assert_eq!(context, Some("work-management".to_string()));
        assert_eq!(
            applies_to,
            vec!["queue-policy".to_string(), "story-lifecycle".to_string()]
        );
    } else {
        panic!("Expected ADR New command");
    }
}

#[test]
fn cli_parses_adr_new_without_optional_flags() {
    let cli = Cli::try_parse_from(["board", "adr", "new", "Store Events"]).unwrap();

    if let Commands::Management(ManagementCommands::Adr {
        action:
            AdrAction::New {
                title,
                context,
                applies_to,
            },
    }) = cli.command
    {
        assert_eq!(title, "Store Events");
        assert!(context.is_none());
        assert!(applies_to.is_empty());
    } else {
        panic!("Expected ADR New command");
    }
}

#[test]
fn cli_parses_voyage_new() {
    let cli = Cli::try_parse_from([
        "board",
        "voyage",
        "new",
        "command-restructure",
        "--epic",
        "board-cli",
        "--goal",
        "Improve command decomposition",
    ])
    .unwrap();
    if let Commands::Management(ManagementCommands::Voyage {
        action: VoyageAction::New { name, epic, .. },
    }) = cli.command
    {
        assert_eq!(name, "command-restructure");
        assert_eq!(epic, "board-cli");
    } else {
        panic!("Expected Voyage New command");
    }
}

#[test]
fn cli_parses_voyage_start() {
    let cli = Cli::try_parse_from(["board", "voyage", "start", "08"]).unwrap();
    if let Commands::Management(ManagementCommands::Voyage {
        action:
            VoyageAction::Start {
                id,
                force,
                expect_version,
            },
    }) = cli.command
    {
        assert_eq!(id, "08");
        assert!(!force);
        assert!(expect_version.is_none());
    } else {
        panic!("Expected Voyage Start command");
    }
}

#[test]
fn cli_parses_voyage_start_with_force() {
    let cli = Cli::try_parse_from(["board", "voyage", "start", "08", "--force"]).unwrap();
    if let Commands::Management(ManagementCommands::Voyage {
        action:
            VoyageAction::Start {
                id,
                force,
                expect_version,
            },
    }) = cli.command
    {
        assert_eq!(id, "08");
        assert!(force);
        assert!(expect_version.is_none());
    } else {
        panic!("Expected Voyage Start command with force");
    }
}

#[test]
fn cli_parses_voyage_plan() {
    let cli = Cli::try_parse_from(["board", "voyage", "plan", "21"]).unwrap();
    if let Commands::Management(ManagementCommands::Voyage {
        action: VoyageAction::Plan { id, no_review },
    }) = cli.command
    {
        assert_eq!(id, "21");
        assert!(!no_review);
    } else {
        panic!("Expected Voyage Plan command");
    }
}

#[test]
fn cli_parses_voyage_plan_with_no_review() {
    let cli = Cli::try_parse_from(["board", "voyage", "plan", "21", "--no-review"]).unwrap();
    if let Commands::Management(ManagementCommands::Voyage {
        action: VoyageAction::Plan { id, no_review },
    }) = cli.command
    {
        assert_eq!(id, "21");
        assert!(no_review);
    } else {
        panic!("Expected Voyage Plan command");
    }
}

#[test]
fn cli_parses_voyage_done_with_flags() {
    let cli = Cli::try_parse_from([
        "board",
        "voyage",
        "done",
        "07",
        "--well",
        "Good progress",
        "--hard",
        "Integration",
    ])
    .unwrap();
    if let Commands::Management(ManagementCommands::Voyage {
        action:
            VoyageAction::Done {
                id,
                well,
                hard,
                different,
            },
    }) = cli.command
    {
        assert_eq!(id, "07");
        assert_eq!(well, Some("Good progress".to_string()));
        assert_eq!(hard, Some("Integration".to_string()));
        assert!(different.is_none());
    } else {
        panic!("Expected Voyage Done command");
    }
}

#[test]
fn cli_parses_voyage_show() {
    let cli = Cli::try_parse_from(["board", "voyage", "show", "07"]).unwrap();
    if let Commands::Management(ManagementCommands::Voyage {
        action: VoyageAction::Show { id, compact: _ },
    }) = cli.command
    {
        assert_eq!(id, "07");
    } else {
        panic!("Expected Voyage Show command");
    }
}

#[test]
fn cli_parses_voyage_list() {
    let cli = Cli::try_parse_from(["board", "voyage", "list"]).unwrap();
    if let Commands::Management(ManagementCommands::Voyage {
        action: VoyageAction::List { epic, status },
    }) = cli.command
    {
        assert!(epic.is_none());
        assert!(status.is_empty());
    } else {
        panic!("Expected Voyage List command");
    }
}

#[test]
fn cli_parses_voyage_list_with_filters() {
    let cli = Cli::try_parse_from([
        "board",
        "voyage",
        "list",
        "--epic",
        "board-cli",
        "--status",
        "in-progress",
        "--status",
        "+done",
    ])
    .unwrap();
    if let Commands::Management(ManagementCommands::Voyage {
        action: VoyageAction::List { epic, status },
    }) = cli.command
    {
        assert_eq!(epic, Some("board-cli".to_string()));
        assert_eq!(status, vec!["in-progress".to_string(), "+done".to_string()]);
    } else {
        panic!("Expected Voyage List command");
    }
}

#[test]
fn cli_parses_bearing_list_with_filters() {
    let cli = Cli::try_parse_from([
        "board",
        "bearing",
        "list",
        "--status",
        "ready",
        "--status",
        "+declined",
    ])
    .unwrap();
    if let Commands::Management(ManagementCommands::Bearing {
        action: BearingAction::List { status },
    }) = cli.command
    {
        assert_eq!(status, vec!["ready".to_string(), "+declined".to_string()]);
    } else {
        panic!("Expected Bearing List command");
    }
}

#[test]
fn cli_rejects_legacy_story_stage_filter() {
    let result = crate::cli::build_cli().try_get_matches_from([
        "keel",
        "story",
        "list",
        "--stage",
        "ready-for-acceptance",
    ]);
    assert!(result.is_err());
}

#[test]
fn build_cli_collects_repeated_status_filters() {
    let cases = vec![
        (
            vec![
                "keel", "story", "list", "--status", "backlog", "--status", "+done",
            ],
            "story",
            vec!["backlog", "+done"],
        ),
        (
            vec![
                "keel", "epic", "list", "--status", "active", "--status", "+done",
            ],
            "epic",
            vec!["active", "+done"],
        ),
        (
            vec![
                "keel", "voyage", "list", "--status", "planned", "--status", "+done",
            ],
            "voyage",
            vec!["planned", "+done"],
        ),
        (
            vec![
                "keel",
                "bearing",
                "list",
                "--status",
                "ready",
                "--status",
                "+declined",
            ],
            "bearing",
            vec!["ready", "+declined"],
        ),
    ];

    for (args, command, expected) in cases {
        let matches = crate::cli::build_cli().try_get_matches_from(args).unwrap();
        let subcommand = matches.subcommand_matches(command).unwrap();
        let list = subcommand.subcommand_matches("list").unwrap();
        let status: Vec<_> = list
            .get_many::<String>("status")
            .unwrap()
            .map(|value| value.as_str())
            .collect();
        assert_eq!(status, expected, "unexpected status filters for {command}");
    }
}

#[test]
fn cli_rejects_legacy_epic_status_filter() {
    let result = crate::cli::build_cli().try_get_matches_from([
        "keel",
        "epic",
        "list",
        "--status",
        "strategic",
    ]);
    assert!(result.is_err());
}

#[test]
fn cli_rejects_legacy_voyage_status_filter() {
    let result = crate::cli::build_cli()
        .try_get_matches_from(["keel", "voyage", "list", "--status", "active"]);
    assert!(result.is_err());
}

#[test]
fn cli_help_displays_entity_subcommands() {
    let mut cmd = Cli::command();
    let mut help = Vec::new();
    cmd.write_long_help(&mut help).unwrap();
    let help_str = String::from_utf8(help).unwrap();

    // Verify entity subcommands are present
    assert!(help_str.contains("story"), "Missing story subcommand");
    assert!(help_str.contains("epic"), "Missing epic subcommand");
    assert!(help_str.contains("voyage"), "Missing voyage subcommand");
}

#[test]
fn cli_parses_next_with_parallel() {
    let cli = Cli::try_parse_from(["board", "next", "--role", "manager", "--parallel"]).unwrap();
    if let Commands::Diagnostics(DiagnosticsCommands::Next {
        role,
        json,
        parallel,
    }) = cli.command
    {
        assert_eq!(role, Some("manager".to_string()));
        assert!(!json);
        assert!(parallel);
    } else {
        panic!("Expected Next command");
    }
}

#[test]
fn cli_parses_next_with_required_role() {
    let cli = Cli::try_parse_from(["board", "next", "--role", "manager"]).unwrap();
    if let Commands::Diagnostics(DiagnosticsCommands::Next {
        role,
        json,
        parallel,
    }) = cli.command
    {
        assert_eq!(role, Some("manager".to_string()));
        assert!(!json);
        assert!(!parallel);
    } else {
        panic!("Expected Next command");
    }
}

#[test]
fn cli_parses_next_parallel_with_json() {
    let cli = Cli::try_parse_from(["board", "next", "--role", "manager", "--parallel", "--json"])
        .unwrap();
    if let Commands::Diagnostics(DiagnosticsCommands::Next {
        role,
        json,
        parallel,
    }) = cli.command
    {
        assert_eq!(role, Some("manager".to_string()));
        assert!(json);
        assert!(parallel);
    } else {
        panic!("Expected Next command");
    }
}

#[test]
fn cli_rejects_next_without_role() {
    let err = Cli::try_parse_from(["board", "next"])
        .expect_err("Expected parse error when --role is missing")
        .to_string();

    assert!(err.contains("--role"));
}

// ── Role flag tests ─────────────────────────────────────────────

#[test]
fn cli_parses_next_with_role() {
    let cli = Cli::try_parse_from(["board", "next", "--role", "engineer"]).unwrap();
    if let Commands::Diagnostics(DiagnosticsCommands::Next { role, .. }) = cli.command {
        assert_eq!(role, Some("engineer".to_string()));
    } else {
        panic!("Expected Next command");
    }
}

#[test]
fn cli_parses_next_with_full_taxonomy_role() {
    let cli = Cli::try_parse_from(["board", "next", "--role", "engineer/software:infra"]).unwrap();
    if let Commands::Diagnostics(DiagnosticsCommands::Next { role, .. }) = cli.command {
        assert_eq!(role, Some("engineer/software:infra".to_string()));
    } else {
        panic!("Expected Next command");
    }
}

#[test]
fn cli_rejects_legacy_next_agent_flag_with_migration_guidance() {
    let message = crate::cli::commands::management::next::legacy_next_flag_guidance(&[
        "keel".to_string(),
        "next".to_string(),
        "--agent".to_string(),
    ])
    .expect("legacy flag should produce migration guidance");

    assert!(message.contains("--agent"));
    assert!(message.contains("--role operator"));
}

#[test]
fn cli_rejects_legacy_next_human_flag_with_migration_guidance() {
    let message = crate::cli::commands::management::next::legacy_next_flag_guidance(&[
        "keel".to_string(),
        "next".to_string(),
        "--human".to_string(),
    ])
    .expect("legacy flag should produce migration guidance");

    assert!(message.contains("--human"));
    assert!(message.contains("--role manager"));
}

#[test]
fn cli_rejects_legacy_next_flags_when_role_is_already_present() {
    let message = crate::cli::commands::management::next::legacy_next_flag_guidance(&[
        "keel".to_string(),
        "next".to_string(),
        "--role".to_string(),
        "engineer/software".to_string(),
        "--agent".to_string(),
    ])
    .expect("legacy flag should conflict with --role guidance");

    assert!(message.contains("Do not combine"));
    assert!(message.contains("--role"));
}

#[test]
fn cli_rejects_legacy_story_accept_human_flag_with_migration_guidance() {
    let message =
        crate::cli::commands::management::story::accept::legacy_story_accept_flag_guidance(&[
            "keel".to_string(),
            "story".to_string(),
            "accept".to_string(),
            "S1".to_string(),
            "--human".to_string(),
        ])
        .expect("legacy accept flag should produce migration guidance");

    assert!(message.contains("--human"));
    assert!(message.contains("--role manager"));
}

#[test]
fn cli_rejects_legacy_story_accept_human_flag_when_role_is_also_present() {
    let message =
        crate::cli::commands::management::story::accept::legacy_story_accept_flag_guidance(&[
            "keel".to_string(),
            "story".to_string(),
            "accept".to_string(),
            "S1".to_string(),
            "--role".to_string(),
            "manager".to_string(),
            "--human".to_string(),
        ])
        .expect("legacy accept flag should conflict with --role guidance");

    assert!(message.contains("Do not combine"));
    assert!(message.contains("--role"));
}
