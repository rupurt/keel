//! Turn command - inspect the canonical operating loop.

use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct TurnPayload {
    phases: Vec<TurnPhasePayload>,
}

#[derive(Debug, Serialize)]
struct TurnPhasePayload {
    phase: String,
    title: String,
    purpose: String,
    commands: Vec<TurnCommandPayload>,
}

#[derive(Debug, Serialize)]
struct TurnCommandPayload {
    path: String,
    example: String,
    docs_slug: String,
}

pub fn run(json: bool) -> Result<()> {
    let projection = keel::read_model::turn_loop::project();

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&payload_for(&projection))?
        );
    } else {
        print!("{}", render_text(&projection));
    }

    Ok(())
}

fn payload_for(projection: &keel::read_model::turn_loop::TurnLoopProjection) -> TurnPayload {
    TurnPayload {
        phases: projection
            .phases
            .iter()
            .map(|phase| TurnPhasePayload {
                phase: phase.title.to_lowercase(),
                title: phase.title.to_string(),
                purpose: phase.purpose.to_string(),
                commands: phase
                    .commands
                    .iter()
                    .map(|command| {
                        let descriptor = command.descriptor();
                        TurnCommandPayload {
                            path: descriptor.full_path().to_string(),
                            example: command.example.to_string(),
                            docs_slug: descriptor.docs_slug.to_string(),
                        }
                    })
                    .collect(),
            })
            .collect(),
    }
}

fn render_text(projection: &keel::read_model::turn_loop::TurnLoopProjection) -> String {
    let mut output = String::from("The Turn Loop\n\n");

    for (index, phase) in projection.phases.iter().enumerate() {
        output.push_str(&format!(
            "{}. {}  {}\n",
            index + 1,
            phase.title,
            phase.purpose
        ));
        for command in &phase.commands {
            output.push_str(&format!("   - {}\n", command.example));
        }
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::{payload_for, render_text};

    #[test]
    fn turn_text_surface_contains_documented_examples() {
        let projection = keel::read_model::turn_loop::project();
        let rendered = render_text(&projection);

        assert!(rendered.contains("1. Orient  Read the board before you move it."));
        assert!(rendered.contains("keel mission next --status"));
        assert!(rendered.contains("keel story accept --role manager STORY-ID"));
    }

    #[test]
    fn turn_json_surface_is_stable() {
        let projection = keel::read_model::turn_loop::project();
        let payload = payload_for(&projection);
        let json = serde_json::to_value(payload).expect("turn payload should serialize");

        assert_eq!(json["phases"][0]["phase"], "orient");
        assert_eq!(json["phases"][1]["phase"], "inspect");
        assert!(
            json["phases"][1]["commands"]
                .as_array()
                .expect("inspect commands")
                .iter()
                .any(|command| command["path"] == "mission next")
        );
        assert!(
            json["phases"][3]["commands"]
                .as_array()
                .expect("ship commands")
                .iter()
                .any(|command| command["example"] == "keel story submit STORY-ID")
        );
    }
}
