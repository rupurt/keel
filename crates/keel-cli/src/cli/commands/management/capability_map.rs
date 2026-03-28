//! Canonical command capability classification for management guidance rendering.

use crate::cli::command_catalog::{CommandCapability, CommandSurfaceId, descriptor_for_id};
use crate::cli::commands::management::guidance::{
    CanonicalGuidance, CommandGuidance, render_command_guidance,
};

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
    BearingResearch,
    BearingAssess,
    BearingPark,
    BearingDecline,
    BearingLay,
    PlaySuggest,
    PlayExplore,
    VerifyStory,
    AuditStory,
}

fn command_surface_id(command: ManagementCommand) -> CommandSurfaceId {
    match command {
        ManagementCommand::AdrList => CommandSurfaceId::AdrList,
        ManagementCommand::AdrShow => CommandSurfaceId::AdrShow,
        ManagementCommand::AdrAccept => CommandSurfaceId::AdrAccept,
        ManagementCommand::AdrReject => CommandSurfaceId::AdrReject,
        ManagementCommand::AdrDeprecate => CommandSurfaceId::AdrDeprecate,
        ManagementCommand::AdrSupersede => CommandSurfaceId::AdrSupersede,
        ManagementCommand::BearingNew => CommandSurfaceId::BearingNew,
        ManagementCommand::BearingList => CommandSurfaceId::BearingList,
        ManagementCommand::BearingShow => CommandSurfaceId::BearingShow,
        ManagementCommand::BearingResearch => CommandSurfaceId::BearingResearch,
        ManagementCommand::BearingAssess => CommandSurfaceId::BearingAssess,
        ManagementCommand::BearingPark => CommandSurfaceId::BearingPark,
        ManagementCommand::BearingDecline => CommandSurfaceId::BearingDecline,
        ManagementCommand::BearingLay => CommandSurfaceId::BearingLay,
        ManagementCommand::PlaySuggest => CommandSurfaceId::PlaySuggest,
        ManagementCommand::PlayExplore => CommandSurfaceId::PlayExplore,
        ManagementCommand::VerifyStory => CommandSurfaceId::Verify,
        ManagementCommand::AuditStory => CommandSurfaceId::Audit,
    }
}

/// Canonical command capability classification map.
pub fn classify_command(command: ManagementCommand) -> CommandCapability {
    descriptor_for_id(command_surface_id(command)).capability
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
            classify_command(ManagementCommand::BearingResearch),
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
                Some(CommandGuidance::next("keel next --role manager/product")),
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
            Some(CommandGuidance::next("keel next --role manager/product")),
        )
        .unwrap();
        assert_eq!(
            next.next_step.unwrap().command,
            "keel next --role manager/product"
        );
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
