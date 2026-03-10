//! Canonical doctor check catalog.
//!
//! These stable ids back `keel.toml` doctor check overrides and the
//! `keel doctor` disabled rendering.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoctorCheckDefinition {
    pub id: &'static str,
    pub section: &'static str,
    pub name: &'static str,
}

pub const ALL_DOCTOR_CHECKS: &[DoctorCheckDefinition] = &[
    DoctorCheckDefinition {
        id: "story-id-uniqueness",
        section: "Stories",
        name: "ID uniqueness",
    },
    DoctorCheckDefinition {
        id: "story-id-consistency",
        section: "Stories",
        name: "ID consistency",
    },
    DoctorCheckDefinition {
        id: "story-title-convention",
        section: "Stories",
        name: "Title convention",
    },
    DoctorCheckDefinition {
        id: "story-acceptance-criteria-completion",
        section: "Stories",
        name: "Acceptance criteria completion",
    },
    DoctorCheckDefinition {
        id: "story-verification-annotations",
        section: "Stories",
        name: "Verification annotations",
    },
    DoctorCheckDefinition {
        id: "story-srs-traceability",
        section: "Stories",
        name: "SRS traceability",
    },
    DoctorCheckDefinition {
        id: "story-implementation-dependency-cycles",
        section: "Stories",
        name: "Implementation dependency cycles",
    },
    DoctorCheckDefinition {
        id: "story-parallel-conflict-coherence",
        section: "Stories",
        name: "Parallel conflict coherence",
    },
    DoctorCheckDefinition {
        id: "story-scoped-evidence-coverage",
        section: "Stories",
        name: "Scoped evidence coverage",
    },
    DoctorCheckDefinition {
        id: "story-frontmatter-drift",
        section: "Stories",
        name: "Story frontmatter drift",
    },
    DoctorCheckDefinition {
        id: "story-reflection-coherence",
        section: "Stories",
        name: "Reflection coherence",
    },
    DoctorCheckDefinition {
        id: "story-active-story-coherence",
        section: "Stories",
        name: "Active story coherence",
    },
    DoctorCheckDefinition {
        id: "story-terminal-artifact-coherence",
        section: "Stories",
        name: "Terminal artifact coherence",
    },
    DoctorCheckDefinition {
        id: "story-knowledge-catalog-integrity",
        section: "Stories",
        name: "Knowledge catalog integrity",
    },
    DoctorCheckDefinition {
        id: "story-verification-manifest-integrity",
        section: "Stories",
        name: "Verification manifest integrity",
    },
    DoctorCheckDefinition {
        id: "story-date-consistency",
        section: "Stories",
        name: "Story date consistency",
    },
    DoctorCheckDefinition {
        id: "voyage-structure",
        section: "Voyages",
        name: "Voyage structure",
    },
    DoctorCheckDefinition {
        id: "voyage-id-uniqueness",
        section: "Voyages",
        name: "ID uniqueness",
    },
    DoctorCheckDefinition {
        id: "voyage-id-consistency",
        section: "Voyages",
        name: "ID consistency",
    },
    DoctorCheckDefinition {
        id: "voyage-title-convention",
        section: "Voyages",
        name: "Title convention",
    },
    DoctorCheckDefinition {
        id: "voyage-status-drift",
        section: "Voyages",
        name: "Voyage status drift",
    },
    DoctorCheckDefinition {
        id: "voyage-scope-authored-content",
        section: "Voyages",
        name: "Scope authored content",
    },
    DoctorCheckDefinition {
        id: "voyage-srs-authored-requirements",
        section: "Voyages",
        name: "SRS authored requirements",
    },
    DoctorCheckDefinition {
        id: "voyage-sdd-authored-content",
        section: "Voyages",
        name: "SDD authored content",
    },
    DoctorCheckDefinition {
        id: "voyage-prd-lineage-coherence",
        section: "Voyages",
        name: "PRD lineage coherence",
    },
    DoctorCheckDefinition {
        id: "voyage-scope-lineage-coherence",
        section: "Voyages",
        name: "Scope lineage coherence",
    },
    DoctorCheckDefinition {
        id: "voyage-legacy-scope-headings",
        section: "Voyages",
        name: "Legacy scope headings",
    },
    DoctorCheckDefinition {
        id: "voyage-artifact-contract",
        section: "Voyages",
        name: "Voyage artifact contract",
    },
    DoctorCheckDefinition {
        id: "voyage-evidence-chains",
        section: "Voyages",
        name: "Evidence chains",
    },
    DoctorCheckDefinition {
        id: "voyage-date-consistency",
        section: "Voyages",
        name: "Voyage date consistency",
    },
    DoctorCheckDefinition {
        id: "epic-structure",
        section: "Epics",
        name: "Epic structure",
    },
    DoctorCheckDefinition {
        id: "epic-id-uniqueness",
        section: "Epics",
        name: "ID uniqueness",
    },
    DoctorCheckDefinition {
        id: "epic-id-consistency",
        section: "Epics",
        name: "ID consistency",
    },
    DoctorCheckDefinition {
        id: "epic-title-convention",
        section: "Epics",
        name: "Title convention",
    },
    DoctorCheckDefinition {
        id: "epic-status-drift",
        section: "Epics",
        name: "Epic status drift",
    },
    DoctorCheckDefinition {
        id: "epic-completion-gates",
        section: "Epics",
        name: "Epic completion gates",
    },
    DoctorCheckDefinition {
        id: "epic-goal-lineage-coherence",
        section: "Epics",
        name: "Goal lineage coherence",
    },
    DoctorCheckDefinition {
        id: "epic-press-release-coherence",
        section: "Epics",
        name: "Press release coherence (optional)",
    },
    DoctorCheckDefinition {
        id: "epic-date-consistency",
        section: "Epics",
        name: "Epic date consistency",
    },
    DoctorCheckDefinition {
        id: "bearing-structure",
        section: "Bearings",
        name: "Bearing structure",
    },
    DoctorCheckDefinition {
        id: "bearing-id-consistency",
        section: "Bearings",
        name: "ID consistency",
    },
    DoctorCheckDefinition {
        id: "bearing-id-format",
        section: "Bearings",
        name: "ID format",
    },
    DoctorCheckDefinition {
        id: "bearing-id-uniqueness",
        section: "Bearings",
        name: "ID uniqueness",
    },
    DoctorCheckDefinition {
        id: "bearing-title-convention",
        section: "Bearings",
        name: "Title convention",
    },
    DoctorCheckDefinition {
        id: "bearing-coherence",
        section: "Bearings",
        name: "Bearing coherence",
    },
    DoctorCheckDefinition {
        id: "bearing-content-completion",
        section: "Bearings",
        name: "Bearing content completion",
    },
    DoctorCheckDefinition {
        id: "bearing-epic-coherence",
        section: "Bearings",
        name: "Bearing-Epic coherence",
    },
    DoctorCheckDefinition {
        id: "bearing-date-consistency",
        section: "Bearings",
        name: "Bearing date consistency",
    },
    DoctorCheckDefinition {
        id: "bearing-recommendation",
        section: "Bearings",
        name: "Bearing decision readiness",
    },
    DoctorCheckDefinition {
        id: "bearing-lineage-epic",
        section: "Bearings",
        name: "Bearing lineage epic",
    },
    DoctorCheckDefinition {
        id: "bearing-lineage-goals",
        section: "Bearings",
        name: "Bearing lineage goals",
    },
    DoctorCheckDefinition {
        id: "adr-structure",
        section: "ADRs",
        name: "ADR structure",
    },
    DoctorCheckDefinition {
        id: "adr-id-uniqueness",
        section: "ADRs",
        name: "ID uniqueness",
    },
    DoctorCheckDefinition {
        id: "adr-id-consistency",
        section: "ADRs",
        name: "ID consistency",
    },
    DoctorCheckDefinition {
        id: "adr-title-convention",
        section: "ADRs",
        name: "Title convention",
    },
    DoctorCheckDefinition {
        id: "adr-proposed-usage",
        section: "ADRs",
        name: "Proposed ADR usage",
    },
    DoctorCheckDefinition {
        id: "adr-content-completion",
        section: "ADRs",
        name: "ADR content completion",
    },
    DoctorCheckDefinition {
        id: "adr-date-consistency",
        section: "ADRs",
        name: "ADR date consistency",
    },
    DoctorCheckDefinition {
        id: "mission-id-uniqueness",
        section: "Missions",
        name: "ID uniqueness",
    },
    DoctorCheckDefinition {
        id: "mission-goal-achievement",
        section: "Missions",
        name: "Goal achievement",
    },
    DoctorCheckDefinition {
        id: "mission-active-work-coherence",
        section: "Missions",
        name: "Active mission work coherence",
    },
    DoctorCheckDefinition {
        id: "mission-orphaned-lineage",
        section: "Missions",
        name: "Orphaned mission lineage",
    },
    DoctorCheckDefinition {
        id: "mission-date-consistency",
        section: "Missions",
        name: "Mission date consistency",
    },
    DoctorCheckDefinition {
        id: "mission-completion-evidence",
        section: "Missions",
        name: "Mission completion evidence",
    },
];
