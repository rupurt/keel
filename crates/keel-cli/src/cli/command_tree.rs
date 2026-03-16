//! Shared CLI command tree definition.

use clap::{Arg, ArgAction, Command, Subcommand};

const HELP_GROUPS: &str = r#"
The Ramping Path (Your Moves):

1. The Fixer (Learning by Healing)
  doctor      Validate board health and optionally fix issues
  health      Subsystem status check and bio-scan (The Med-Bay)
  flow        Show workflow lane dashboard from configured topology
  screen      Visual representation of the current board (Strategic Radar)

2. The Operator (Learning by Building)
  workshop    Focus on items requiring human attention (The Workbench)
  next        Pull the next item using explicit role-based queue routing
  story       Implementation units and acceptance criteria
  verify      Execute verification proofs
  audit       Rich evidence/traceability report

3. The Architect (Learning by Constraining)
  mission     Strategic objectives and charters
  epic        Strategic grouping and PRD management
  voyage      Tactical planning (SRS/SDD) and execution
  routine     Scheduled strategic work (Routines)
  adr         Architecture Decision Records (The Physics)
  roadmap     Strategic management timeline
  finance     Work capital and system solvency (The Vault)

Discovery & Automation:
  play        Invite play-driven discovery (The Sandbox)
  bearing     Research phase and fog reduction
  knowledge   Manage institutional memory
  pulse       Run one non-interactive automation cycle
  topology    Show a zoomable world map of the board

Comms:
  ping        Send a message to the inbox
  poke        Respond to or re-evaluate a ping in the inbox
  inbox       List messages in the inbox
  outbox      List messages in the outbox

Setup:
  init        Initialize a new keel board
  config      Configuration and technique inventory
  generate    Regenerate board artifacts
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
        .arg(
            Arg::new("auth-file")
                .long("auth-file")
                .global(true)
                .help("Path to a JWT file to authenticate the actor")
                .value_parser(clap::value_parser!(std::path::PathBuf)),
        )
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
                )
                .arg(
                    Arg::new("scene")
                        .long("scene")
                        .help("Render a visual representation of the current board scene")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("status")
                        .long("status")
                        .help("Print a 3-bullet importance summary suitable for commit messages")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("health")
                .about("Subsystem status check and bio-scan (The Med-Bay)")
                .hide(true)
                .arg(
                    Arg::new("scene")
                        .long("scene")
                        .help("Render the system health as a visual bio-scan scene")
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
                )
                .arg(
                    Arg::new("hide-routines")
                        .long("hide-routines")
                        .help("Hide scheduled routines in the flow output")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("scene")
                        .long("scene")
                        .help("Render a visual representation of the current board scene")
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
            Command::new("workshop")
                .about("Focus on items requiring human attention (The Workbench)")
                .arg(
                    Arg::new("scene")
                        .long("scene")
                        .action(ArgAction::SetTrue)
                        .help("Render the workshop as a visual scene"),
                )
                .hide(true),
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
                )
                .arg(
                    Arg::new("scene")
                        .long("scene")
                        .help("Render a visual representation of the current board scene")
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
            Command::new("screen")
                .about("Show a visual representation of the project state")
                .arg(
                    Arg::new("zoom")
                        .long("zoom")
                        .value_name("LEVEL")
                        .default_value("mission")
                        .value_parser(["world", "mission", "epic", "voyage", "story"])
                        .help("Zoom level to render"),
                )
                .arg(
                    Arg::new("focus")
                        .long("focus")
                        .value_name("ID")
                        .help("Center the screen on a specific entity branch"),
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
                .after_help(
                    "Examples:
  keel play --theater --theme drama
  keel play --theater --theme comedy --persona standup
  keel play --theater --theme action --persona shakespeare --mood poetic
  keel play --theater VDlzABC123
  keel play VDlzABC123 --prop bard
  keel play --cross VDlzABC123 VDlzDEF456",
                )
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
                    Arg::new("theater")
                        .long("theater")
                        .help("Launch the theater runtime and render the scene setup")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("theme")
                        .long("theme")
                        .help("Select a theater theme (for example: drama, comedy, action)")
                        .num_args(1),
                )
                .arg(
                    Arg::new("persona")
                        .long("persona")
                        .help("Select a theater persona (for example: neutral, standup, shakespeare)")
                        .num_args(1),
                )
                .arg(
                    Arg::new("mood")
                        .long("mood")
                        .help("Select a theater mood (for example: adaptive, playful, poetic, spectacular, focused)")
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
            Command::new("ping")
                .about("Send a message to the inbox")
                .hide(true)
                .arg(Arg::new("message").required(true).help("The message to send").num_args(1..))
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help("Output as JSON for scripting")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("poke")
                .about("Respond to or re-evaluate a ping in the inbox or spark the system")
                .hide(true)
                .arg(Arg::new("message").help("Message to the system or a specific ping").num_args(1..))
                .arg(Arg::new("id").long("id").help("The ID of the ping if targeting a specific one"))
                .arg(Arg::new("self").long("self").help("Explicitly poke the system").action(ArgAction::SetTrue))
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help("Output as JSON for scripting")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("inbox")
                .about("List messages in the inbox")
                .hide(true)
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help("Output as JSON for scripting")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("outbox")
                .about("List messages in the outbox")
                .hide(true)
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help("Output as JSON for scripting")
                        .action(ArgAction::SetTrue),
                ),
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
            crate::cli::commands::management::watch::WatchAction,
        >("watch", "Watch (time constraint) commands"))
        .subcommand(hidden_subcommand_group::<
            crate::cli::commands::management::story::StoryAction,
        >("story", "Story commands"))
        .subcommand(
            Command::new("roadmap")
                .about("Render the management roadmap")
                .hide(true),
        )
        .subcommand(
            Command::new("finance")
                .about("Work capital and system solvency (The Vault)")
                .arg(
                    Arg::new("scene")
                        .long("scene")
                        .action(ArgAction::SetTrue)
                        .help("Render the financial state as a visual scene"),
                )
                .hide(true),
        )
        .subcommand(hidden_subcommand_group::<
            crate::cli::commands::setup::config::ConfigAction,
        >("config", "Configuration commands"))
}
