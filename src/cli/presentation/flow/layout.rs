//! Pipeline layout configuration and ASCII flow rendering

/// Configuration for pipeline visual layout
pub struct LayoutConfig {
    pub total_width: usize,
    pub stage_width: usize,
    pub last_stage_width: usize,
}

impl LayoutConfig {
    /// Create layout from terminal width
    pub fn from_terminal_width(width: usize) -> Self {
        // Adjust for potential small widths
        let effective_width = width.max(60);
        let stage_width = effective_width / 6;
        let last_stage_width = effective_width - (stage_width * 5);
        Self {
            total_width: effective_width,
            stage_width,
            last_stage_width,
        }
    }

    /// Render stage labels with coloring, centered in each column
    pub fn render_stage_labels(&self, human_color: &str, agent_color: &str, reset: &str) -> String {
        let mut out = String::new();
        let stages = [
            "Governance",
            "Research",
            "Planning",
            "Execution",
            "Verification",
            "Done",
        ];

        for (i, stage) in stages.iter().enumerate() {
            let color = if i == 0 || i == 1 || i == 2 || i == 4 {
                human_color
            } else if i == 3 {
                agent_color
            } else {
                reset // Done is neutral/reset color
            };
            let width = if i < 5 {
                self.stage_width
            } else {
                self.last_stage_width
            };

            let padding = width.saturating_sub(stage.len());
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;

            out.push_str(color);
            out.push_str(&" ".repeat(left_pad));
            out.push_str(stage);
            out.push_str(&" ".repeat(right_pad));
            out.push_str(reset);
        }
        out
    }

    /// Render ASCII flow diagram spanning 100% width with arrows at boundaries
    pub fn render_flow_diagram(&self) -> String {
        let mut out = String::new();
        for _ in 0..5 {
            out.push_str(&"─".repeat(self.stage_width - 1));
            out.push('▶');
        }
        // Last stage line (no outgoing arrow shown in this view)
        out.push_str(&"─".repeat(self.last_stage_width));
        out
    }

    /// Render item counts per stage, centered in each column
    pub fn render_stage_counts(
        &self,
        governance_count: usize,
        human_counts: &[usize],
        agent_counts: &[usize],
        done_count: usize,
    ) -> String {
        let mut out = String::new();
        // Combined for display
        let all_counts = [
            governance_count,                                    // Governance (proposed ADRs)
            human_counts[0] + human_counts[1] + human_counts[2], // Research (exploring + surveying + assessing)
            human_counts[3] + human_counts[4],                   // Planning (draft + planned)
            agent_counts[0] + agent_counts[1], // Execution (backlog + in_progress)
            human_counts[6],                   // Verification (needs_verification)
            done_count,                        // Done (completed stories)
        ];

        for (i, count) in all_counts.iter().enumerate() {
            let width = if i < 5 {
                self.stage_width
            } else {
                self.last_stage_width
            };
            let s = count.to_string();
            let padding = width.saturating_sub(s.len());
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;

            out.push_str(&" ".repeat(left_pad));
            out.push_str(&s);
            out.push_str(&" ".repeat(right_pad));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_config_from_width() {
        let config = LayoutConfig::from_terminal_width(120);
        assert_eq!(config.stage_width, 20);
        assert_eq!(config.last_stage_width, 20);

        let config_small = LayoutConfig::from_terminal_width(30);
        assert_eq!(config_small.total_width, 60); // min width
    }

    #[test]
    fn render_stage_labels() {
        let config = LayoutConfig::from_terminal_width(120);
        let labels = config.render_stage_labels("<H>", "<A>", "<R>");
        // Labels are centered, so they will have spaces around them
        assert!(labels.contains("Governance"));
        assert!(labels.contains("Research"));
        assert!(labels.contains("Planning"));
        assert!(labels.contains("Execution"));
        assert!(labels.contains("Verification"));
        assert!(labels.contains("Done"));
        assert!(labels.contains("<H>"));
        assert!(labels.contains("<A>"));
    }

    #[test]
    fn render_flow_diagram() {
        let config = LayoutConfig::from_terminal_width(120);
        let flow = config.render_flow_diagram();
        // 5 arrows for 6 stages
        assert_eq!(flow.matches('▶').count(), 5);
        assert!(flow.starts_with('─'));
    }

    #[test]
    fn render_stage_counts() {
        let config = LayoutConfig::from_terminal_width(120);
        let human_counts = [1, 2, 3, 4, 5, 0, 7];
        let agent_counts = [10, 20];
        let counts = config.render_stage_counts(100, &human_counts, &agent_counts, 50);

        // Governance: 100
        assert!(counts.contains("100"));
        // Research: 1+2+3 = 6
        assert!(counts.contains("6"));
        // Planning: 4+5 = 9
        assert!(counts.contains("9"));
        // Execution: 10+20 = 30
        assert!(counts.contains("30"));
        // Verification: 7
        assert!(counts.contains("7"));
        // Done: 50
        assert!(counts.contains("50"));
    }
}
