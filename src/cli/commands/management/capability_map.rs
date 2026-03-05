//! Canonical command capability classification for management guidance rendering.

use crate::cli::commands::management::guidance::{
    CanonicalGuidance, CommandGuidance, render_command_guidance,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandCapability {
    Actionable,
    Informational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagementCommand {
    AdrList,
    AdrShow,
    AdrAccept,
    AdrReject,
    AdrDeprecate,
    AdrSupersede,
    BearingNew,
    BearingList,
    BearingShow,
    BearingSurvey,
    BearingAssess,
    BearingPark,
    BearingDecline,
    BearingLay,
    PlaySuggest,
    PlayExplore,
    VerifyStory,
    AuditStory,
}

/// Canonical command capability classification map.
pub fn classify_command(command: ManagementCommand) -> CommandCapability {
    match command {
        ManagementCommand::AdrList
        | ManagementCommand::AdrShow
        | ManagementCommand::BearingList
        | ManagementCommand::BearingShow
        | ManagementCommand::PlayExplore => CommandCapability::Informational,

        ManagementCommand::AdrAccept
        | ManagementCommand::AdrReject
        | ManagementCommand::AdrDeprecate
        | ManagementCommand::AdrSupersede
        | ManagementCommand::BearingNew
        | ManagementCommand::BearingSurvey
        | ManagementCommand::BearingAssess
        | ManagementCommand::BearingPark
        | ManagementCommand::BearingDecline
        | ManagementCommand::BearingLay
        | ManagementCommand::PlaySuggest
        | ManagementCommand::VerifyStory
        | ManagementCommand::AuditStory => CommandCapability::Actionable,
    }
}

/// Render canonical guidance based on command capability classification.
pub fn render_guidance_for_command(
    command: ManagementCommand,
    guidance: Option<CommandGuidance>,
) -> Option<CanonicalGuidance> {
    match classify_command(command) {
        CommandCapability::Actionable => render_command_guidance(guidance),
        CommandCapability::Informational => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classification_map_labels_representative_commands() {
        assert_eq!(
            classify_command(ManagementCommand::AdrList),
            CommandCapability::Informational
        );
        assert_eq!(
            classify_command(ManagementCommand::BearingShow),
            CommandCapability::Informational
        );
        assert_eq!(
            classify_command(ManagementCommand::PlayExplore),
            CommandCapability::Informational
        );

        assert_eq!(
            classify_command(ManagementCommand::AdrAccept),
            CommandCapability::Actionable
        );
        assert_eq!(
            classify_command(ManagementCommand::BearingSurvey),
            CommandCapability::Actionable
        );
        assert_eq!(
            classify_command(ManagementCommand::BearingNew),
            CommandCapability::Actionable
        );
        assert_eq!(
            classify_command(ManagementCommand::PlaySuggest),
            CommandCapability::Actionable
        );
        assert_eq!(
            classify_command(ManagementCommand::VerifyStory),
            CommandCapability::Actionable
        );
        assert_eq!(
            classify_command(ManagementCommand::AuditStory),
            CommandCapability::Actionable
        );
    }

    #[test]
    fn informational_commands_suppress_guidance_payload() {
        for command in [
            ManagementCommand::AdrList,
            ManagementCommand::AdrShow,
            ManagementCommand::BearingList,
            ManagementCommand::BearingShow,
            ManagementCommand::PlayExplore,
        ] {
            let guidance = render_guidance_for_command(
                command,
                Some(CommandGuidance::next("keel next --human")),
            );
            assert!(
                guidance.is_none(),
                "{command:?} should not emit actionable guidance"
            );
        }
    }

    #[test]
    fn actionable_commands_emit_canonical_next_or_recovery_payload() {
        let next = render_guidance_for_command(
            ManagementCommand::AdrAccept,
            Some(CommandGuidance::next("keel next --human")),
        )
        .unwrap();
        assert_eq!(next.next_step.unwrap().command, "keel next --human");
        assert!(next.recovery_step.is_none());

        let recovery = render_guidance_for_command(
            ManagementCommand::VerifyStory,
            Some(CommandGuidance::recovery("keel story audit S1")),
        )
        .unwrap();
        assert_eq!(
            recovery.recovery_step.unwrap().command,
            "keel story audit S1"
        );
        assert!(recovery.next_step.is_none());
    }
}
