//! Canonical turn-loop projection for the public operating rhythm.

use super::command_catalog::{CommandSurfaceId, TurnPhase, descriptor_for_id};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnCommandHint {
    pub id: CommandSurfaceId,
    pub example: String,
}

impl TurnCommandHint {
    pub fn descriptor(&self) -> &'static super::command_catalog::CommandDescriptor {
        descriptor_for_id(self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnPhaseProjection {
    pub phase: TurnPhase,
    pub title: &'static str,
    pub purpose: &'static str,
    pub commands: Vec<TurnCommandHint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnLoopProjection {
    pub phases: Vec<TurnPhaseProjection>,
}

pub fn project() -> TurnLoopProjection {
    let phases = TurnPhase::ALL
        .into_iter()
        .map(|phase| TurnPhaseProjection {
            phase,
            title: phase.title(),
            purpose: phase_purpose(phase),
            commands: phase_command_ids(phase)
                .iter()
                .copied()
                .map(|id| TurnCommandHint {
                    id,
                    example: turn_example_for(id),
                })
                .collect(),
        })
        .collect();

    TurnLoopProjection { phases }
}

fn phase_command_ids(phase: TurnPhase) -> &'static [CommandSurfaceId] {
    match phase {
        TurnPhase::Orient => &[
            CommandSurfaceId::Health,
            CommandSurfaceId::Heartbeat,
            CommandSurfaceId::Flow,
            CommandSurfaceId::Doctor,
        ],
        TurnPhase::Inspect => &[CommandSurfaceId::MissionNext, CommandSurfaceId::Pulse],
        TurnPhase::Pull => &[CommandSurfaceId::Next],
        TurnPhase::Ship => &[
            CommandSurfaceId::StoryStart,
            CommandSurfaceId::StoryRecord,
            CommandSurfaceId::StorySubmit,
        ],
        TurnPhase::Close => &[CommandSurfaceId::StoryAccept],
    }
}

fn phase_purpose(phase: TurnPhase) -> &'static str {
    match phase {
        TurnPhase::Orient => "Read the board before you move it.",
        TurnPhase::Inspect => "Read what the board thinks matters now.",
        TurnPhase::Pull => "Pull one role-scoped slice from the correct lane.",
        TurnPhase::Ship => "Execute the slice and attach proof while the work is fresh.",
        TurnPhase::Close => "Accept the slice explicitly and absorb the state change upward.",
    }
}

fn turn_example_for(id: CommandSurfaceId) -> String {
    match id {
        CommandSurfaceId::Doctor => "keel doctor".to_string(),
        CommandSurfaceId::Health => "keel health --scene".to_string(),
        CommandSurfaceId::Heartbeat => "keel heartbeat".to_string(),
        CommandSurfaceId::Flow => "keel flow --scene".to_string(),
        CommandSurfaceId::MissionNext => "keel mission next --status".to_string(),
        CommandSurfaceId::Pulse => "keel pulse".to_string(),
        CommandSurfaceId::Next => "keel next --role manager".to_string(),
        CommandSurfaceId::StoryStart => "keel story start STORY-ID".to_string(),
        CommandSurfaceId::StoryRecord => {
            "keel story record STORY-ID --ac 1 --cmd \"your-check-here\"".to_string()
        }
        CommandSurfaceId::StorySubmit => "keel story submit STORY-ID".to_string(),
        CommandSurfaceId::StoryAccept => "keel story accept --role manager STORY-ID".to_string(),
        _ => format!("keel {}", descriptor_for_id(id).full_path()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_loop_projection_preserves_documented_phase_order() {
        let projection = project();
        let phases: Vec<_> = projection.phases.iter().map(|phase| phase.phase).collect();
        assert_eq!(
            phases,
            vec![
                TurnPhase::Orient,
                TurnPhase::Inspect,
                TurnPhase::Pull,
                TurnPhase::Ship,
                TurnPhase::Close
            ]
        );
    }

    #[test]
    fn turn_loop_projection_carries_documented_examples() {
        let projection = project();

        let orient = &projection.phases[0];
        let orient_examples: Vec<_> = orient
            .commands
            .iter()
            .map(|command| command.example.as_str())
            .collect();
        assert!(orient_examples.contains(&"keel health --scene"));
        assert!(orient_examples.contains(&"keel flow --scene"));
        assert!(orient_examples.contains(&"keel doctor"));

        let inspect = &projection.phases[1];
        let inspect_examples: Vec<_> = inspect
            .commands
            .iter()
            .map(|command| command.example.as_str())
            .collect();
        assert!(inspect_examples.contains(&"keel mission next --status"));
        assert!(inspect_examples.contains(&"keel pulse"));
        assert_eq!(inspect_examples.len(), 2);

        let ship = &projection.phases[3];
        let ship_examples: Vec<_> = ship
            .commands
            .iter()
            .map(|command| command.example.as_str())
            .collect();
        assert!(ship_examples.contains(&"keel story start STORY-ID"));
        assert!(
            ship_examples.contains(&"keel story record STORY-ID --ac 1 --cmd \"your-check-here\"")
        );
        assert!(ship_examples.contains(&"keel story submit STORY-ID"));

        let close = &projection.phases[4];
        let close_examples: Vec<_> = close
            .commands
            .iter()
            .map(|command| command.example.as_str())
            .collect();
        assert!(close_examples.contains(&"keel story accept --role manager STORY-ID"));
    }

    #[test]
    fn curated_turn_examples_still_match_catalog_phase_hints() {
        let projection = project();

        for phase in &projection.phases {
            for command in &phase.commands {
                assert_eq!(command.descriptor().turn_phase, Some(phase.phase));
            }
        }
    }
}
