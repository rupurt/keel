//! Domain events emitted by application use cases.

/// Event stream for cross-aggregate lifecycle coordination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainEvent {
    /// Emitted after a story is moved into `in-progress`.
    StoryStarted {
        story_id: String,
        scope: Option<String>,
    },
    /// Emitted after a story is accepted to `done`.
    StoryAccepted {
        story_id: String,
        scope: Option<String>,
    },
    /// Emitted after a voyage is completed.
    VoyageCompleted { voyage_id: String, epic_id: String },
}

impl DomainEvent {
    pub fn name(&self) -> &'static str {
        match self {
            Self::StoryStarted { .. } => "story.started",
            Self::StoryAccepted { .. } => "story.accepted",
            Self::VoyageCompleted { .. } => "voyage.completed",
        }
    }
}
