//! Shared CLI command tree definition.

use clap::{Arg, ArgAction, Command, Subcommand};

const HELP_GROUPS: &str = r#"
These are common Keel commands:

Setup
  init        Initialize a new keel board in the current directory
  config      Configuration commands
  generate    Regenerate all README files

Management
  next        Pull the next item using explicit role-based queue routing
  pulse       Run one non-interactive automation cycle
  topology    Show a zoomable world map of the board
  play        Invite play-driven discovery
  audit       Rich evidence/traceability report
  verify      Execute verification proofs
  knowledge   Manage institutional knowledge
  mission     Mission commands (long-running objectives)
  adr         ADR commands (architecture decisions)
  bearing     Bearing commands (research phase)
  epic        Epic commands
  routine     Routine commands
  voyage      Voyage commands
  story       Story commands (new, start, submit, accept, reject, ice, thaw, show, list, link, unlink, record, audit)
  roadmap     Render the management roadmap

Diagnostics
  doctor      Validate board health and optionally fix issues
  flow        Show workflow lane dashboard from configured topology
  throughput  Show weekly throughput and timing sparklines
"#;

fn hidden_subcommand_group<T>(name: &'static str, about: &'static str) -> Command
where
    T: Subcommand,
{
    T::augment_subcommands(Command::new(name).about(about).hide(true)).subcommand_required(true)
}

pub fn build_cli() -> Command {
    Command::new("keel")
        .about("Agentic SDLC management — minimize drift through planning, execution, and verification")
        .version(env!("CARGO_PKG_VERSION"))
        .after_help(HELP_GROUPS)
        .disable_help_subcommand(true)
        .subcommand(
            Command::new("doctor")
                .about("Validate board health and optionally fix issues")
                .hide(true)
                .arg(
                    Arg::new("fix")
                        .long("fix")
                        .help("Auto-fix safe issues without prompting")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("evidence")
                        .long("evidence")
                        .help("Show detailed evidence chains for requirement traceability")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("watch")
                        .long("watch")
                        .help("Watch mode: continuously monitor for changes and re-validate (SRS-06)")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("quick")
                        .long("quick")
                        .help("Quick mode: run fast structural checks only (SRS-08, SRS-09)")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("flow")
                .about("Show workflow lane dashboard from configured topology")
                .hide(true)
                .arg(
                    Arg::new("no_color")
                        .long("no-color")
                        .help("Disable color output (also respects NO_COLOR env var)")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("throughput")
                .about("Show weekly throughput and timing sparklines")
                .hide(true)
                .arg(
                    Arg::new("no_color")
                        .long("no-color")
                        .help("Disable color output (also respects NO_COLOR env var)")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("next")
                .about("Pull the next item for an explicit role-selected queue")
                .hide(true)
                .arg(
                    Arg::new("role")
                        .long("role")
                        .value_name("TAXONOMY")
                        .required(true)
                        .help("Required role taxonomy controlling queue selection (e.g., \"manager\" or \"operator/software:infrastructure\")")
                        .num_args(1),
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help("Output as JSON for scripting")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("parallel")
                        .long("parallel")
                        .help("Return all parallel-safe stories for batch dispatch")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("pulse")
                .about("Run one non-interactive automation cycle")
                .hide(true)
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help("Output as JSON for scripting")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("topology")
                .about("Show a zoomable world map of the board")
                .hide(true)
                .arg(
                    Arg::new("zoom")
                        .long("zoom")
                        .value_name("LEVEL")
                        .default_value("world")
                        .value_parser(["world", "mission", "epic", "voyage", "story"])
                        .help("Zoom level to render"),
                )
                .arg(
                    Arg::new("focus")
                        .long("focus")
                        .value_name("ID")
                        .help("Center the map on a specific entity branch"),
                )
                .arg(
                    Arg::new("include_done")
                        .long("include-done")
                        .help("Include done voyages and stories")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("static")
                        .long("static")
                        .help("Render once and exit even in an interactive terminal")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("play")
                .about("Run a marionette-style discovery scene")
                .hide(true)
                .arg(
                    Arg::new("bearing")
                        .help("Bearing ID to cast in a discovery scenario")
                        .value_name("BEARING")
                        .index(1),
                )
                .arg(
                    Arg::new("prop")
                        .long("prop")
                        .help("Start with a specific prop in hand")
                        .num_args(1),
                )
                .arg(
                    Arg::new("cross")
                        .long("cross")
                        .help("Cross two bearings in one paired scene")
                        .num_args(2)
                        .value_names(["id1", "id2"]),
                )
                .arg(
                    Arg::new("list_props")
                        .long("list-props")
                        .help("List available props and marionette functions by category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("suggest")
                        .long("suggest")
                        .help("Suggest a mask for a bearing based on its content")
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("audit")
                .about("Rich evidence/traceability report")
                .hide(true)
                .arg(
                    Arg::new("id")
                        .help("ID of story, voyage, or epic to audit (default: all)")
                        .value_name("ID")
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("verify")
                .about("Verification operations")
                .hide(true)
                .subcommand(
                    Command::new("run")
                        .about("Execute verification proofs")
                        .arg(
                            Arg::new("id")
                                .help("ID of story, voyage, or epic to verify (default: all)")
                                .value_name("ID")
                                .index(1),
                        )
                        .arg(
                            Arg::new("all")
                                .long("all")
                                .help("Verify all stories on the board")
                                .action(ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("json")
                                .long("json")
                                .help("Output as JSON for scripting")
                                .action(ArgAction::SetTrue),
                        ),
                )
                .subcommand(
                    Command::new("recommend")
                        .about("Show detected+active verification techniques")
                        .arg(
                            Arg::new("json")
                                .long("json")
                                .help("Output as JSON for scripting")
                                .action(ArgAction::SetTrue),
                        ),
                )
                .subcommand(
                    Command::new("detect")
                        .about("Show verification detection signals and status")
                        .arg(
                            Arg::new("json")
                                .long("json")
                                .help("Output as JSON for scripting")
                                .action(ArgAction::SetTrue),
                        ),
                )
                .subcommand_required(true),
        )
        .subcommand(
            Command::new("generate")
                .about("Regenerate all README files")
                .hide(true),
        )
        .subcommand(
            Command::new("init")
                .about("Initialize a new keel board in the current directory")
                .hide(true),
        )
        .subcommand(
            Command::new("knowledge")
                .about("Manage institutional knowledge")
                .hide(true)
                .subcommand(
                    Command::new("list")
                        .about("List all knowledge units")
                        .arg(Arg::new("category").long("category").short('c').value_name("CATEGORY"))
                        .arg(
                            Arg::new("sort")
                                .long("sort")
                                .value_name("MODE")
                                .value_parser(["id", "story"])
                                .default_value("id")
                                .help("Sort mode: id | story"),
                        )
                        .arg(Arg::new("pending").long("pending").short('p').help("Only show pending (unapplied) knowledge").action(ArgAction::SetTrue)),
                )
                .subcommand(
                    Command::new("show")
                        .about("Show detailed knowledge unit")
                        .arg(Arg::new("id").required(true).value_name("ID")),
                )
                .subcommand(
                    Command::new("explore")
                        .about("Explore thematic threads and rising patterns"),
                )
                .subcommand(
                    Command::new("graph")
                        .about("Visualize the knowledge graph")
                        .arg(
                            Arg::new("zoom")
                                .long("zoom")
                                .value_name("LEVEL")
                                .value_parser(["world", "delivery", "artifact", "source"])
                                .default_value("world")
                                .help("Zoom level to render"),
                        )
                        .arg(
                            Arg::new("focus")
                                .long("focus")
                                .value_name("ID")
                                .help("Center the graph on a specific node branch"),
                        )
                        .arg(
                            Arg::new("static")
                                .long("static")
                                .help("Render once and exit even in an interactive terminal")
                                .action(ArgAction::SetTrue),
                        ),
                )
                .subcommand(
                    Command::new("impact")
                        .about("Impact/Drift analysis"),
                )
                .subcommand(
                    Command::new("prune")
                        .about("Prune duplicate knowledge and refresh canonical files"),
                )
                .subcommand_required(true),
        )
        .subcommand(hidden_subcommand_group::<
            crate::cli::commands::management::adr::AdrAction,
        >("adr", "ADR commands (architecture decisions)"))
        .subcommand(hidden_subcommand_group::<
            crate::cli::commands::management::mission::MissionAction,
        >("mission", "Mission commands (long-running objectives)"))
        .subcommand(hidden_subcommand_group::<
            crate::cli::commands::management::bearing::BearingAction,
        >("bearing", "Bearing commands (research phase)"))
        .subcommand(hidden_subcommand_group::<
            crate::cli::commands::management::epic::EpicAction,
        >("epic", "Epic commands"))
        .subcommand(hidden_subcommand_group::<
            crate::cli::commands::management::routine::RoutineAction,
        >("routine", "Routine commands"))
        .subcommand(hidden_subcommand_group::<
            crate::cli::commands::management::voyage::VoyageAction,
        >("voyage", "Voyage commands"))
        .subcommand(hidden_subcommand_group::<
            crate::cli::commands::management::story::StoryAction,
        >("story", "Story commands"))
        .subcommand(
            Command::new("roadmap")
                .about("Render the management roadmap")
                .hide(true),
        )
        .subcommand(hidden_subcommand_group::<
            crate::cli::commands::setup::config::ConfigAction,
        >("config", "Configuration commands"))
}
