use serde::Serialize;
use std::path::PathBuf;

/// Severity of a validation problem.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
pub enum Severity {
    /// Must be rejected by runtime transition paths.
    Error,
    /// Should be surfaced to humans, may or may not be blocking at runtime.
    Warning,
    /// Informational message, never blocking.
    #[allow(dead_code)]
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "Error"),
            Self::Warning => write!(f, "Warning"),
            Self::Info => write!(f, "Info"),
        }
    }
}

/// A specific problem found on the board.
///
/// This type is used by both `keel doctor` for reporting and the transition
/// engine for gating.
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct Problem {
    /// Problem severity.
    pub severity: Severity,
    /// Source path for diagnostics.
    pub path: PathBuf,
    /// Human-readable summary.
    pub message: String,
    /// Optional automated fix.
    pub fix: Option<Fix>,
    /// Optional scope (epic/voyage path) where the problem occurred.
    pub scope: Option<String>,
    /// Classification of the problem (for gaps and doctor grouping).
    pub category: Option<GapCategory>,
    /// Identifier for the specific check that failed.
    pub check_id: CheckId,
}

impl Problem {
    /// Create a new Error severity problem.
    pub fn error(path: PathBuf, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            path,
            message: message.into(),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        }
    }

    /// Create a new Warning severity problem.
    pub fn warning(path: PathBuf, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            path,
            message: message.into(),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        }
    }

    /// Set the scope for this problem.
    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// Set the category for this problem.
    #[allow(dead_code)]
    pub fn with_category(mut self, category: GapCategory) -> Self {
        self.category = Some(category);
        self
    }

    /// Set the check ID for this problem.
    pub fn with_check_id(mut self, check_id: CheckId) -> Self {
        self.check_id = check_id;
        self
    }

    /// Set the fix for this problem.
    #[allow(dead_code)]
    pub fn with_fix(mut self, fix: Fix) -> Self {
        self.fix = Some(fix);
        self
    }

    /// Returns true if this problem should block runtime transitions.
    pub fn blocks_runtime(&self, strict: bool) -> bool {
        strict || matches!(self.severity, Severity::Error)
    }
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum Fix {
    MigrateAnnotationToTest {
        path: PathBuf,
    },
    MigrateAnnotationToGrep {
        path: PathBuf,
    },
    #[allow(dead_code)]
    UpdateVoyageStatus {
        path: PathBuf,
        new_status: String,
    },
    #[allow(dead_code)]
    StartVoyage {
        path: PathBuf,
        voyage_id: String,
    },
    #[allow(dead_code)]
    MigrateVoyageId {
        epic_id: String,
        old_voyage_id: String,
        new_voyage_id: String,
        voyage_dir: PathBuf,
    },
    UpdateTitle {
        path: PathBuf,
        new_title: String,
    },
    RemoveFile {
        path: PathBuf,
    },
    RenameFile {
        old_path: PathBuf,
        new_path: PathBuf,
    },
    #[allow(dead_code)]
    UpdateFrontmatterId {
        path: PathBuf,
        new_id: String,
    },
    ClearPlaceholder {
        path: PathBuf,
        pattern: String,
    },
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
pub enum GapCategory {
    #[allow(dead_code)]
    Structural,
    #[allow(dead_code)]
    Coherence,
    #[allow(dead_code)]
    Drift,
    #[allow(dead_code)]
    Coverage,
    #[allow(dead_code)]
    Convention,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash, Default)]
pub enum CheckId {
    #[default]
    Unknown,
    VoyagesReadmeStructure,
    VoyagesSrsExists,
    VoyagesSddExists,
    #[allow(dead_code)]
    VoyagesGeneratedVoyageIds,
    StoryMissingFrontmatter,
    StoryInvalidYaml,
    StoryMissingId,
    StoryMissingTitle,
    StoryDuplicateId,
    VoyageDuplicateId,
    BearingDuplicateId,
    AdrDuplicateId,
    StoryFilenameInconsistent,
    StoryDeprecatedFields,
    StoryInvalidRole,
    StoryOrphanedScope,
    StoryIndexGap,
    StoryIndexDuplicate,
    StoryIncompleteAcceptance,
    StoryMissingVerification,
    StoryMalformedVerification,
    StoryMissingSrsRef,
    StoryDependencyCycle,
    StoryParallelConflictCoherence,
    StoryUnexpectedReflection,
    StoryPlanningScaffold,
    StoryTerminalScaffold,
    StoryMissingManifest,
    StoryManifestTampered,
    VoyageStatusDrift,
    EpicStatusDrift,
    EpicMissingReadme,
    EpicMissingPrd,
    EpicMissingPressRelease,
    EpicPressReleaseIncomplete,
    EpicInvalidFrontmatter,
    EpicDuplicateId,
    EpicDateConsistency,
    VoyageDateConsistency,
    StoryDateConsistency,
    BearingDateConsistency,
    AdrDateConsistency,
    TitleCaseViolation,
    IdInconsistency,
}
