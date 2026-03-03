//! Shared CLI command tree definition.

use clap::{Arg, ArgAction, Command};

const HELP_GROUPS: &str = r#"
These are common Keel commands:

Setup
  init        Initialize a new keel board in the current directory
  config      Configuration commands
  generate    Regenerate all README files

Management
  next        Pull from human queue (default) or agent queue (--agent)
  play        Invite play-driven discovery
  audit       Rich evidence/traceability report
  verify      Execute verification proofs
  knowledge   Manage institutional knowledge
  adr         ADR commands (architecture decisions)
  bearing     Bearing commands (research phase)
  epic        Epic commands
  voyage      Voyage commands
  story       Story commands

Diagnostics
  doctor      Validate board health and optionally fix issues
  status      Show board status summary
  flow        Show two-actor flow dashboard (human queue vs agent queue)
  throughput  Show weekly throughput and timing sparklines
  capacity    Show per-epic capacity breakdown with parallel potential
  gaps        Show gap classification summary (runs doctor, shows only gap counts)
"#;
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
            Command::new("status")
                .about("Show board status summary")
                .hide(true),
        )
        .subcommand(
            Command::new("flow")
                .about("Show two-actor flow dashboard (human queue vs agent queue)")
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
                .about("Pull the next item from the selected queue (human by default, --agent for implementation queue)")
                .hide(true)
                .arg(
                    Arg::new("role")
                        .long("role")
                        .value_name("TAXONOMY")
                        .help("Filter by role taxonomy (e.g., \"engineer/software:infrastructure\")")
                        .num_args(1)
                        .conflicts_with_all(["agent", "human"]),
                )
                .arg(
                    Arg::new("agent")
                        .long("agent")
                        .help("Pull from the agent implementation queue (in-progress/backlog)")
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["role", "human"]),
                )
                .arg(
                    Arg::new("human")
                        .long("human")
                        .help("Pull from the human queue only (never returns implementation work)")
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["role", "agent"]),
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
            Command::new("capacity")
                .about("Show per-epic capacity breakdown with parallel potential")
                .hide(true)
                .arg(
                    Arg::new("all")
                        .long("all")
                        .help("Show all epics including those with zero capacity")
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
            Command::new("gaps")
                .about("Show gap classification summary (runs doctor, shows only gap counts)")
                .hide(true),
        )
        .subcommand(
            Command::new("play")
                .about("Invite play-driven discovery")
                .hide(true)
                .arg(
                    Arg::new("bearing")
                        .help("Bearing ID to generate a play scenario from")
                        .value_name("BEARING")
                        .index(1),
                )
                .arg(
                    Arg::new("prop")
                        .long("prop")
                        .help("Start with a specific prop equipped")
                        .num_args(1),
                )
                .arg(
                    Arg::new("cross")
                        .long("cross")
                        .help("Cross two bearings for a paired play session")
                        .num_args(2)
                        .value_names(["id1", "id2"]),
                )
                .arg(
                    Arg::new("list_props")
                        .long("list-props")
                        .help("List available props by category")
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
                .about("Execute verification proofs")
                .hide(true)
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
                ),
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
                        .about("Visualize the knowledge graph"),
                )
                .subcommand(
                    Command::new("impact")
                        .about("Impact/Drift analysis"),
                )
                .subcommand_required(true),
        )
        .subcommand(
            Command::new("adr")
                .about("ADR commands (architecture decisions)")
                .hide(true)
                .subcommand(Command::new("new").about("Create a new ADR").arg(
                    Arg::new("title").required(true).value_name("TITLE"),
                ))
                .subcommand(
                    Command::new("list")
                        .about("List all ADRs")
                        .arg(Arg::new("status").long("status").value_name("STATUS")),
                )
                .subcommand(
                    Command::new("show")
                        .about("Show ADR details")
                        .arg(Arg::new("id").required(true).value_name("ID")),
                )
                .subcommand(
                    Command::new("accept")
                        .about("Accept a proposed ADR")
                        .arg(Arg::new("id").required(true).value_name("ID")),
                )
                .subcommand(
                    Command::new("reject")
                        .about("Reject a proposed ADR")
                        .arg(Arg::new("id").required(true).value_name("ID"))
                        .arg(Arg::new("reason").required(true).value_name("REASON")),
                )
                .subcommand(
                    Command::new("deprecate")
                        .about("Deprecate an accepted ADR (no longer recommended)")
                        .arg(Arg::new("id").required(true).value_name("ID"))
                        .arg(Arg::new("reason").required(true).value_name("REASON")),
                )
                .subcommand(
                    Command::new("supersede")
                        .about("Supersede an ADR with a newer one")
                        .arg(Arg::new("new_id").required(true).value_name("NEW_ID"))
                        .arg(Arg::new("old_id").required(true).value_name("OLD_ID")),
                )
                .subcommand_required(true),
        )
        .subcommand(
            Command::new("bearing")
                .about("Bearing commands (research phase)")
                .hide(true)
                .subcommand(
                    Command::new("new").about("Create a new bearing").arg(
                        Arg::new("name").required(true).value_name("NAME"),
                    ),
                )
                .subcommand(
                    Command::new("survey")
                        .about("Add SURVEY.md to a bearing")
                        .arg(Arg::new("name").required(true).value_name("NAME")),
                )
                .subcommand(
                    Command::new("assess")
                        .about("Add ASSESSMENT.md to a bearing")
                        .arg(Arg::new("name").required(true).value_name("NAME")),
                )
                .subcommand(
                    Command::new("list")
                        .about("List all bearings")
                        .arg(Arg::new("status").long("status").value_name("STATUS")),
                )
                .subcommand(
                    Command::new("show")
                        .about("Show bearing details")
                        .arg(Arg::new("name").required(true).value_name("NAME")),
                )
                .subcommand(
                    Command::new("park")
                        .about("Park a bearing for later")
                        .arg(Arg::new("name").required(true).value_name("NAME")),
                )
                .subcommand(
                    Command::new("decline")
                        .about("Decline a bearing with reason")
                        .arg(Arg::new("name").required(true).value_name("NAME"))
                        .arg(Arg::new("reason").required(true).value_name("REASON")),
                )
                .subcommand(
                    Command::new("lay").about("Graduate bearing to epic").arg(
                        Arg::new("name").required(true).value_name("NAME"),
                    ),
                )
                .subcommand_required(true),
        )
        .subcommand(
            Command::new("epic")
                .about("Epic commands")
                .hide(true)
                .subcommand(
                    Command::new("new")
                        .about("Create a new epic")
                        .arg(Arg::new("name").required(true).value_name("NAME"))
                        .arg(
                            Arg::new("goal")
                                .long("goal")
                                .short('g')
                                .value_name("GOAL")
                                .required(true),
                        ),
                )
                .subcommand(
                    Command::new("show")
                        .about("Show epic details")
                        .arg(Arg::new("id").required(true)),
                )
                .subcommand(
                    Command::new("list")
                        .about("List epics")
                        .arg(
                            Arg::new("status")
                                .long("status")
                                .short('s')
                                .help("Filter by derived epic state")
                                .value_parser(["draft", "active", "done"])
                                .value_name("STATUS"),
                        ),
                )
                .subcommand_required(true),
        )
        .subcommand(
            Command::new("voyage")
                .about("Voyage commands")
                .hide(true)
                .subcommand(
                    Command::new("new")
                        .about("Create a new voyage")
                        .arg(Arg::new("name").required(true).value_name("NAME"))
                        .arg(Arg::new("epic").long("epic").required(true).value_name("EPIC"))
                        .arg(
                            Arg::new("goal")
                                .long("goal")
                                .short('g')
                                .value_name("GOAL")
                                .required(true),
                        ),
                )
                .subcommand(
                    Command::new("start")
                        .about("Start a voyage")
                        .arg(Arg::new("id").required(true).value_name("ID"))
                        .arg(Arg::new("force").long("force").action(ArgAction::SetTrue))
                        .arg(
                            Arg::new("expect_version")
                                .long("expect-version")
                                .value_parser(clap::value_parser!(u64)),
                        ),
                )
                .subcommand(
                    Command::new("plan")
                        .about("Plan a voyage (move from draft to planned)")
                        .arg(Arg::new("id").required(true).value_name("ID"))
                        .arg(Arg::new("no_review").long("no-review").action(ArgAction::SetTrue)),
                )
                .subcommand(
                    Command::new("done")
                        .about("Complete a voyage")
                        .arg(Arg::new("id").required(true).value_name("ID"))
                        .arg(Arg::new("well").long("well").value_name("WELL"))
                        .arg(Arg::new("hard").long("hard").value_name("HARD"))
                        .arg(Arg::new("different").long("different").value_name("DIFFERENT")),
                )
                .subcommand(
                    Command::new("show")
                        .about("Show voyage details")
                        .arg(Arg::new("id").required(true).value_name("ID")),
                )
                .subcommand(
                    Command::new("list")
                        .about("List voyages")
                        .arg(Arg::new("epic").long("epic").value_name("EPIC"))
                        .arg(
                            Arg::new("status")
                                .long("status")
                                .help("Filter by canonical voyage state")
                                .value_parser(["draft", "planned", "in-progress", "done"])
                                .value_name("STATUS"),
                        ),
                )
                .subcommand_required(true),
        )
        .subcommand(
            Command::new("story")
                .about("Story commands")
                .hide(true)
                .subcommand(
                    Command::new("new")
                        .about("Create a new story")
                        .arg(Arg::new("title").required(true).value_name("TITLE"))
                        .arg(
                            Arg::new("type")
                                .long("type")
                                .short('t')
                                .value_name("TYPE")
                                .default_value("feat"),
                        ),
                )
                .subcommand(
                    Command::new("start")
                        .about("Start working on a story")
                        .arg(Arg::new("id").required(true).value_name("ID"))
                        .arg(
                            Arg::new("expect_version")
                                .long("expect-version")
                                .value_parser(clap::value_parser!(u64)),
                        ),
                )
                .subcommand(
                    Command::new("submit")
                        .about("Submit a story for review")
                        .arg(Arg::new("id").required(true).value_name("ID")),
                )
                .subcommand(
                    Command::new("accept")
                        .about("Accept a story (move to done)")
                        .arg(Arg::new("id").required(true).value_name("ID"))
                        .arg(Arg::new("human").long("human").action(ArgAction::SetTrue))
                        .arg(Arg::new("reflect").long("reflect").value_name("REFLECT")),
                )
                .subcommand(
                    Command::new("reflect")
                        .about("Create REFLECT.md from the reflection template")
                        .arg(Arg::new("id").required(true).value_name("ID")),
                )
                .subcommand(
                    Command::new("reject")
                        .about("Reject a story (move to rejected)")
                        .arg(Arg::new("id").required(true).value_name("ID"))
                        .arg(Arg::new("reason").required(true).value_name("REASON")),
                )
                .subcommand(
                    Command::new("ice")
                        .about("Move a story to icebox")
                        .arg(Arg::new("id").required(true).value_name("ID")),
                )
                .subcommand(
                    Command::new("thaw")
                        .about("Move a story from icebox to backlog")
                        .arg(Arg::new("id").required(true).value_name("ID")),
                )
                .subcommand(
                    Command::new("show")
                        .about("Show story details")
                        .arg(Arg::new("id").required(true).value_name("ID")),
                )
                .subcommand(
                    Command::new("list")
                        .about("List stories")
                        .arg(
                            Arg::new("stage")
                                .long("stage")
                                .help("Filter by canonical story stage")
                                .value_parser([
                                    "backlog",
                                    "in-progress",
                                    "needs-human-verification",
                                    "done",
                                    "rejected",
                                    "icebox",
                                ])
                                .value_name("STAGE"),
                        )
                        .arg(Arg::new("epic").long("epic").value_name("EPIC"))
                        .arg(
                            Arg::new("reflections")
                                .long("reflections")
                                .action(ArgAction::SetTrue),
                        ),
                )
                .subcommand(
                    Command::new("link")
                        .about("Link a story to a voyage")
                        .arg(Arg::new("id").required(true).value_name("ID"))
                        .arg(Arg::new("voyage").required(true).value_name("VOYAGE")),
                )
                .subcommand(
                    Command::new("unlink")
                        .about("Unlink a story from a voyage")
                        .arg(Arg::new("id").required(true).value_name("ID"))
                        .arg(Arg::new("voyage").required(true).value_name("VOYAGE")),
                )
                .subcommand(
                    Command::new("record")
                        .about("Record proof for an acceptance criterion")
                        .arg(Arg::new("id").required(true).value_name("ID"))
                        .arg(
                            Arg::new("ac")
                                .long("ac")
                                .short('a')
                                .value_name("NUMBER")
                                .value_parser(clap::value_parser!(usize)),
                        )
                        .arg(Arg::new("cmd").long("cmd").short('c').value_name("COMMAND"))
                        .arg(Arg::new("msg").long("msg").short('m').value_name("MESSAGE"))
                        .arg(
                            Arg::new("judge")
                                .long("judge")
                                .help("Trigger LLM-Judge verification")
                                .action(ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("files")
                                .long("file")
                                .short('f')
                                .value_name("FILE")
                                .action(ArgAction::Append),
                        ),
                )
                .subcommand_required(true),
        )
        .subcommand(
            Command::new("config")
                .about("Configuration commands")
                .hide(true)
                .subcommand(
                    Command::new("show").about("Show resolved configuration and source"),
                )
                .subcommand(
                    Command::new("mode")
                        .about("Show or change scoring mode")
                        .arg(Arg::new("name").required(false).value_name("NAME")),
                )
                .subcommand_required(true),
        )
}
