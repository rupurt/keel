//! Canonical metadata for Keel's public command surface.
//!
//! This catalog is the single descriptive source for command families,
//! capability classification, turn-loop phase hints, docs slugs, and
//! scene-support metadata. It intentionally describes the product surface
//! rather than the underlying clap parser shape.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CommandFamily {
    Fixer,
    Operator,
    Architect,
    Discovery,
    Comms,
    Setup,
}

impl CommandFamily {
    pub const ALL: [Self; 6] = [
        Self::Fixer,
        Self::Operator,
        Self::Architect,
        Self::Discovery,
        Self::Comms,
        Self::Setup,
    ];

    pub const fn help_heading(self) -> &'static str {
        match self {
            Self::Fixer => "1. The Fixer (Orientation and integrity)",
            Self::Operator => "2. The Operator (Pull and close delivery slices)",
            Self::Architect => "3. The Architect (Shape missions, plans, and constraints)",
            Self::Discovery => "Discovery & Automation:",
            Self::Comms => "Comms:",
            Self::Setup => "Setup:",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandCapability {
    Actionable,
    Informational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnPhase {
    Orient,
    Inspect,
    Pull,
    Ship,
    Close,
}

impl TurnPhase {
    pub const ALL: [Self; 5] = [
        Self::Orient,
        Self::Inspect,
        Self::Pull,
        Self::Ship,
        Self::Close,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            Self::Orient => "Orient",
            Self::Inspect => "Inspect",
            Self::Pull => "Pull",
            Self::Ship => "Ship",
            Self::Close => "Close",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SceneSupport {
    pub surface_id: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandSurfaceId {
    Doctor,
    Health,
    Heartbeat,
    Flow,
    Screen,
    Workshop,
    Turn,
    Roles,
    Next,
    Story,
    Verify,
    Audit,
    Mission,
    Epic,
    Voyage,
    Routine,
    Adr,
    Roadmap,
    Finance,
    Play,
    Bearing,
    Knowledge,
    Pulse,
    Topology,
    Ping,
    Poke,
    Inbox,
    Outbox,
    New,
    Upgrade,
    Config,
    Generate,
    Hooks,
    MissionShow,
    MissionNext,
    StoryStart,
    StoryRecord,
    StorySubmit,
    StoryAccept,
    AdrList,
    AdrShow,
    AdrAccept,
    AdrReject,
    AdrDeprecate,
    AdrSupersede,
    BearingNew,
    BearingList,
    BearingShow,
    BearingResearch,
    BearingAssess,
    BearingPark,
    BearingDecline,
    BearingLay,
    PlaySuggest,
    PlayExplore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandDescriptor {
    pub id: CommandSurfaceId,
    pub command: &'static str,
    pub mode: Option<&'static str>,
    pub family: CommandFamily,
    pub capability: CommandCapability,
    pub turn_phase: Option<TurnPhase>,
    pub docs_slug: &'static str,
    pub scene_support: Option<SceneSupport>,
    pub atlas_visible: bool,
}

impl CommandDescriptor {
    pub const fn full_path(self) -> &'static str {
        match self.id {
            CommandSurfaceId::MissionShow => "mission show",
            CommandSurfaceId::MissionNext => "mission next",
            CommandSurfaceId::StoryStart => "story start",
            CommandSurfaceId::StoryRecord => "story record",
            CommandSurfaceId::StorySubmit => "story submit",
            CommandSurfaceId::StoryAccept => "story accept",
            CommandSurfaceId::AdrList => "adr list",
            CommandSurfaceId::AdrShow => "adr show",
            CommandSurfaceId::AdrAccept => "adr accept",
            CommandSurfaceId::AdrReject => "adr reject",
            CommandSurfaceId::AdrDeprecate => "adr deprecate",
            CommandSurfaceId::AdrSupersede => "adr supersede",
            CommandSurfaceId::BearingNew => "bearing new",
            CommandSurfaceId::BearingList => "bearing list",
            CommandSurfaceId::BearingShow => "bearing show",
            CommandSurfaceId::BearingResearch => "bearing research",
            CommandSurfaceId::BearingAssess => "bearing assess",
            CommandSurfaceId::BearingPark => "bearing park",
            CommandSurfaceId::BearingDecline => "bearing decline",
            CommandSurfaceId::BearingLay => "bearing lay",
            CommandSurfaceId::PlaySuggest => "play suggest",
            CommandSurfaceId::PlayExplore => "play explore",
            _ => self.command,
        }
    }

    pub const fn supports_scene(self) -> bool {
        self.scene_support.is_some()
    }

    pub const fn help_summary(self) -> Option<&'static str> {
        match self.id {
            CommandSurfaceId::Doctor => Some("Validate board health and optionally fix issues"),
            CommandSurfaceId::Health => Some("Subsystem status check and bio-scan (The Med-Bay)"),
            CommandSurfaceId::Heartbeat => {
                Some("Show repository activity heartbeat and wake state")
            }
            CommandSurfaceId::Flow => Some("Show workflow lane dashboard from configured topology"),
            CommandSurfaceId::Screen => Some("Show a visual representation of the project state"),
            CommandSurfaceId::Workshop => {
                Some("Focus on items requiring human attention (The Workbench)")
            }
            CommandSurfaceId::Turn => {
                Some("Inspect the canonical Orient/Inspect/Pull/Ship/Close loop")
            }
            CommandSurfaceId::Roles => Some("Inspect configured roles, lanes, and contracts"),
            CommandSurfaceId::Next => {
                Some("Pull the next item using explicit role-based queue routing")
            }
            CommandSurfaceId::Story => Some("Implementation units and acceptance criteria"),
            CommandSurfaceId::Verify => Some("Execute verification proofs"),
            CommandSurfaceId::Audit => Some("Rich evidence and traceability report"),
            CommandSurfaceId::Mission => Some("Strategic objectives and charters"),
            CommandSurfaceId::Epic => Some("Strategic grouping and PRD management"),
            CommandSurfaceId::Voyage => Some("Tactical planning (SRS/SDD) and execution"),
            CommandSurfaceId::Routine => Some("Scheduled strategic work (Routines)"),
            CommandSurfaceId::Adr => Some("Architecture Decision Records (The Physics)"),
            CommandSurfaceId::Roadmap => Some("Strategic management timeline"),
            CommandSurfaceId::Finance => Some("Work capital and system solvency (The Vault)"),
            CommandSurfaceId::Play => Some("Invite play-driven discovery (The Sandbox)"),
            CommandSurfaceId::Bearing => Some("Research phase and fog reduction"),
            CommandSurfaceId::Knowledge => Some("Manage institutional memory"),
            CommandSurfaceId::Pulse => Some("Run one non-interactive automation cycle"),
            CommandSurfaceId::Topology => Some("Show a zoomable world map of the board"),
            CommandSurfaceId::Ping => Some("Send a message to the inbox"),
            CommandSurfaceId::Poke => Some("Respond to or re-evaluate a ping in the inbox"),
            CommandSurfaceId::Inbox => Some("List messages in the inbox"),
            CommandSurfaceId::Outbox => Some("List messages in the outbox"),
            CommandSurfaceId::New => Some("Create a new Keel project scaffold"),
            CommandSurfaceId::Upgrade => Some("Upgrade keel from the latest release or a git ref"),
            CommandSurfaceId::Config => Some("Configuration and technique inventory"),
            CommandSurfaceId::Generate => Some("Regenerate board artifacts"),
            CommandSurfaceId::Hooks => Some("Install git hooks for pacemaker protocol"),
            _ => None,
        }
    }
}

pub const COMMAND_CATALOG: &[CommandDescriptor] = &[
    CommandDescriptor {
        id: CommandSurfaceId::Doctor,
        command: "doctor",
        mode: None,
        family: CommandFamily::Fixer,
        capability: CommandCapability::Informational,
        turn_phase: Some(TurnPhase::Orient),
        docs_slug: "cli/scene-surfaces",
        scene_support: Some(SceneSupport {
            surface_id: "doctor-scene",
        }),
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Health,
        command: "health",
        mode: None,
        family: CommandFamily::Fixer,
        capability: CommandCapability::Informational,
        turn_phase: Some(TurnPhase::Orient),
        docs_slug: "cli/scene-surfaces",
        scene_support: Some(SceneSupport {
            surface_id: "med-bay",
        }),
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Heartbeat,
        command: "heartbeat",
        mode: None,
        family: CommandFamily::Fixer,
        capability: CommandCapability::Informational,
        turn_phase: Some(TurnPhase::Orient),
        docs_slug: "cli/heartbeat-and-pacemaker",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Flow,
        command: "flow",
        mode: None,
        family: CommandFamily::Fixer,
        capability: CommandCapability::Informational,
        turn_phase: Some(TurnPhase::Orient),
        docs_slug: "cli/scene-surfaces",
        scene_support: Some(SceneSupport {
            surface_id: "power-rack",
        }),
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Screen,
        command: "screen",
        mode: None,
        family: CommandFamily::Fixer,
        capability: CommandCapability::Informational,
        turn_phase: Some(TurnPhase::Inspect),
        docs_slug: "cli/scene-surfaces",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Workshop,
        command: "workshop",
        mode: None,
        family: CommandFamily::Operator,
        capability: CommandCapability::Informational,
        turn_phase: Some(TurnPhase::Inspect),
        docs_slug: "cli/scene-surfaces",
        scene_support: Some(SceneSupport {
            surface_id: "workbench",
        }),
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Turn,
        command: "turn",
        mode: None,
        family: CommandFamily::Operator,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "workflows/turn-loop",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Roles,
        command: "roles",
        mode: None,
        family: CommandFamily::Operator,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "roles-and-lanes/role-routing-and-next",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Next,
        command: "next",
        mode: None,
        family: CommandFamily::Operator,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Pull),
        docs_slug: "roles-and-lanes/role-routing-and-next",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Story,
        command: "story",
        mode: None,
        family: CommandFamily::Operator,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Ship),
        docs_slug: "foundations/planning-and-verification",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Verify,
        command: "verify",
        mode: None,
        family: CommandFamily::Operator,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Ship),
        docs_slug: "foundations/planning-and-verification",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Audit,
        command: "audit",
        mode: None,
        family: CommandFamily::Operator,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Close),
        docs_slug: "foundations/planning-and-verification",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Mission,
        command: "mission",
        mode: None,
        family: CommandFamily::Architect,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "foundations/board-model",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Epic,
        command: "epic",
        mode: None,
        family: CommandFamily::Architect,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "foundations/planning-and-verification",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Voyage,
        command: "voyage",
        mode: None,
        family: CommandFamily::Architect,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "foundations/planning-and-verification",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Routine,
        command: "routine",
        mode: None,
        family: CommandFamily::Architect,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "workflows/routines-and-pulse",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Adr,
        command: "adr",
        mode: None,
        family: CommandFamily::Architect,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "foundations/planning-and-verification",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Roadmap,
        command: "roadmap",
        mode: None,
        family: CommandFamily::Architect,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Finance,
        command: "finance",
        mode: None,
        family: CommandFamily::Architect,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "cli/scene-surfaces",
        scene_support: Some(SceneSupport {
            surface_id: "vault",
        }),
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Play,
        command: "play",
        mode: None,
        family: CommandFamily::Discovery,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Bearing,
        command: "bearing",
        mode: None,
        family: CommandFamily::Discovery,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Knowledge,
        command: "knowledge",
        mode: None,
        family: CommandFamily::Discovery,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Pulse,
        command: "pulse",
        mode: None,
        family: CommandFamily::Discovery,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Inspect),
        docs_slug: "workflows/routines-and-pulse",
        scene_support: Some(SceneSupport {
            surface_id: "clocktower",
        }),
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Topology,
        command: "topology",
        mode: None,
        family: CommandFamily::Discovery,
        capability: CommandCapability::Informational,
        turn_phase: Some(TurnPhase::Inspect),
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Ping,
        command: "ping",
        mode: None,
        family: CommandFamily::Comms,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "workflows/downstream-project-contracts",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Poke,
        command: "poke",
        mode: None,
        family: CommandFamily::Comms,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "workflows/downstream-project-contracts",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Inbox,
        command: "inbox",
        mode: None,
        family: CommandFamily::Comms,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "workflows/downstream-project-contracts",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Outbox,
        command: "outbox",
        mode: None,
        family: CommandFamily::Comms,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "workflows/downstream-project-contracts",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Upgrade,
        command: "upgrade",
        mode: None,
        family: CommandFamily::Setup,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "workflows/upgrading-keel-and-syncing-instructions",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::New,
        command: "new",
        mode: None,
        family: CommandFamily::Setup,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "start-here/install-keel",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Config,
        command: "config",
        mode: None,
        family: CommandFamily::Setup,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "start-here/install-keel",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Generate,
        command: "generate",
        mode: None,
        family: CommandFamily::Setup,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "workflows/downstream-project-contracts",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::Hooks,
        command: "hooks",
        mode: None,
        family: CommandFamily::Setup,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "start-here/install-keel",
        scene_support: None,
        atlas_visible: true,
    },
    CommandDescriptor {
        id: CommandSurfaceId::MissionShow,
        command: "mission",
        mode: Some("show"),
        family: CommandFamily::Architect,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "cli/scene-surfaces",
        scene_support: Some(SceneSupport {
            surface_id: "constraint-watch",
        }),
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::MissionNext,
        command: "mission",
        mode: Some("next"),
        family: CommandFamily::Operator,
        capability: CommandCapability::Informational,
        turn_phase: Some(TurnPhase::Inspect),
        docs_slug: "workflows/turn-loop",
        scene_support: Some(SceneSupport {
            surface_id: "mission-radar",
        }),
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::StoryStart,
        command: "story",
        mode: Some("start"),
        family: CommandFamily::Operator,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Ship),
        docs_slug: "workflows/turn-loop",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::StoryRecord,
        command: "story",
        mode: Some("record"),
        family: CommandFamily::Operator,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Ship),
        docs_slug: "workflows/turn-loop",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::StorySubmit,
        command: "story",
        mode: Some("submit"),
        family: CommandFamily::Operator,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Ship),
        docs_slug: "workflows/turn-loop",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::StoryAccept,
        command: "story",
        mode: Some("accept"),
        family: CommandFamily::Operator,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Close),
        docs_slug: "workflows/turn-loop",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::AdrList,
        command: "adr",
        mode: Some("list"),
        family: CommandFamily::Architect,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "foundations/planning-and-verification",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::AdrShow,
        command: "adr",
        mode: Some("show"),
        family: CommandFamily::Architect,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "foundations/planning-and-verification",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::AdrAccept,
        command: "adr",
        mode: Some("accept"),
        family: CommandFamily::Architect,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Close),
        docs_slug: "foundations/planning-and-verification",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::AdrReject,
        command: "adr",
        mode: Some("reject"),
        family: CommandFamily::Architect,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Close),
        docs_slug: "foundations/planning-and-verification",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::AdrDeprecate,
        command: "adr",
        mode: Some("deprecate"),
        family: CommandFamily::Architect,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Close),
        docs_slug: "foundations/planning-and-verification",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::AdrSupersede,
        command: "adr",
        mode: Some("supersede"),
        family: CommandFamily::Architect,
        capability: CommandCapability::Actionable,
        turn_phase: Some(TurnPhase::Close),
        docs_slug: "foundations/planning-and-verification",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::BearingNew,
        command: "bearing",
        mode: Some("new"),
        family: CommandFamily::Discovery,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::BearingList,
        command: "bearing",
        mode: Some("list"),
        family: CommandFamily::Discovery,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::BearingShow,
        command: "bearing",
        mode: Some("show"),
        family: CommandFamily::Discovery,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::BearingResearch,
        command: "bearing",
        mode: Some("research"),
        family: CommandFamily::Discovery,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::BearingAssess,
        command: "bearing",
        mode: Some("assess"),
        family: CommandFamily::Discovery,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::BearingPark,
        command: "bearing",
        mode: Some("park"),
        family: CommandFamily::Discovery,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::BearingDecline,
        command: "bearing",
        mode: Some("decline"),
        family: CommandFamily::Discovery,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::BearingLay,
        command: "bearing",
        mode: Some("lay"),
        family: CommandFamily::Discovery,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::PlaySuggest,
        command: "play",
        mode: Some("suggest"),
        family: CommandFamily::Discovery,
        capability: CommandCapability::Actionable,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: false,
    },
    CommandDescriptor {
        id: CommandSurfaceId::PlayExplore,
        command: "play",
        mode: Some("explore"),
        family: CommandFamily::Discovery,
        capability: CommandCapability::Informational,
        turn_phase: None,
        docs_slug: "cli/overview",
        scene_support: None,
        atlas_visible: false,
    },
];

pub fn atlas_command_descriptors() -> impl Iterator<Item = &'static CommandDescriptor> {
    COMMAND_CATALOG
        .iter()
        .filter(|descriptor| descriptor.atlas_visible)
}

pub fn descriptors_for_family(
    family: CommandFamily,
) -> impl Iterator<Item = &'static CommandDescriptor> {
    COMMAND_CATALOG
        .iter()
        .filter(move |descriptor| descriptor.family == family && descriptor.atlas_visible)
}

pub fn descriptors_for_turn_phase(
    phase: TurnPhase,
) -> impl Iterator<Item = &'static CommandDescriptor> {
    COMMAND_CATALOG
        .iter()
        .filter(move |descriptor| descriptor.turn_phase == Some(phase))
}

pub fn scene_command_descriptors() -> impl Iterator<Item = &'static CommandDescriptor> {
    COMMAND_CATALOG
        .iter()
        .filter(|descriptor| descriptor.supports_scene())
}

pub fn descriptor_for_id(id: CommandSurfaceId) -> &'static CommandDescriptor {
    COMMAND_CATALOG
        .iter()
        .find(|descriptor| descriptor.id == id)
        .expect("command catalog must contain every declared surface id")
}

pub fn descriptor_for_path(
    command: &str,
    mode: Option<&str>,
) -> Option<&'static CommandDescriptor> {
    COMMAND_CATALOG
        .iter()
        .find(|descriptor| descriptor.command == command && descriptor.mode == mode)
}

pub fn render_help_groups() -> String {
    let mut output = String::from("\nThe Ramping Path (Your Moves):\n");

    for family in CommandFamily::ALL {
        output.push('\n');
        output.push_str(family.help_heading());
        output.push('\n');

        for descriptor in descriptors_for_family(family) {
            let summary = descriptor
                .help_summary()
                .expect("atlas-visible command must provide help summary");
            output.push_str(&format!("  {:<11} {}\n", descriptor.command, summary));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn atlas_commands_cover_each_documented_family() {
        for family in CommandFamily::ALL {
            let commands: Vec<_> = descriptors_for_family(family).collect();
            assert!(
                !commands.is_empty(),
                "family {:?} should have atlas-visible commands",
                family
            );
        }
    }

    #[test]
    fn atlas_commands_are_unique_by_path() {
        let mut seen = BTreeSet::new();
        for descriptor in atlas_command_descriptors() {
            assert!(
                seen.insert(descriptor.full_path()),
                "duplicate atlas path {}",
                descriptor.full_path()
            );
        }
    }

    #[test]
    fn scene_commands_are_queryable_without_a_separate_list() {
        let scene_commands: Vec<_> = scene_command_descriptors()
            .map(|descriptor| descriptor.full_path())
            .collect();

        assert_eq!(
            scene_commands,
            vec![
                "doctor",
                "health",
                "flow",
                "workshop",
                "finance",
                "pulse",
                "mission show",
                "mission next",
            ]
        );
    }

    #[test]
    fn descriptor_lookup_supports_guided_management_surfaces() {
        let descriptor = descriptor_for_path("bearing", Some("research"))
            .expect("bearing research descriptor should exist");
        assert_eq!(descriptor.id, CommandSurfaceId::BearingResearch);
        assert_eq!(descriptor.capability, CommandCapability::Actionable);

        let play_explore =
            descriptor_for_path("play", Some("explore")).expect("play explore descriptor");
        assert_eq!(play_explore.capability, CommandCapability::Informational);
    }

    #[test]
    fn help_renderer_uses_family_metadata_and_catalog_entries() {
        let help = render_help_groups();

        assert!(help.contains("1. The Fixer (Orientation and integrity)"));
        assert!(help.contains("  doctor      Validate board health and optionally fix issues"));
        assert!(
            help.contains(
                "  turn        Inspect the canonical Orient/Inspect/Pull/Ship/Close loop"
            )
        );
        assert!(help.contains("Discovery & Automation:"));
        assert!(help.contains("  pulse       Run one non-interactive automation cycle"));
    }

    #[test]
    fn turn_phase_descriptors_cover_documented_ship_and_close_subcommands() {
        let ship_commands: Vec<_> = descriptors_for_turn_phase(TurnPhase::Ship)
            .map(|descriptor| descriptor.full_path())
            .collect();
        assert!(ship_commands.contains(&"story start"));
        assert!(ship_commands.contains(&"story record"));
        assert!(ship_commands.contains(&"story submit"));

        let close_commands: Vec<_> = descriptors_for_turn_phase(TurnPhase::Close)
            .map(|descriptor| descriptor.full_path())
            .collect();
        assert!(close_commands.contains(&"story accept"));
    }
}
