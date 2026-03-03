//! Shared ANSI theme configuration for CLI presentation rendering.

/// Color theme used by terminal presentation renderers.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Color for human actor (cyan)
    pub human: &'static str,
    /// Color for agent actor (green)
    pub agent: &'static str,
    /// Color for warnings (yellow)
    pub warning: &'static str,
    /// Color for muted/secondary text (gray)
    pub muted: &'static str,
    /// Bold text
    pub bold: &'static str,
    /// Reset all formatting
    pub reset: &'static str,
}

impl Theme {
    /// Check if colors are enabled based on --no-color flag and NO_COLOR env var.
    pub fn should_use_color(no_color_flag: bool) -> bool {
        if no_color_flag {
            return false;
        }
        // Per no-color.org: NO_COLOR env var presence (any value) disables color.
        std::env::var("NO_COLOR").is_err()
    }

    /// Get the appropriate theme based on color settings.
    pub fn for_color_mode(use_color: bool) -> Self {
        if use_color {
            Self::default()
        } else {
            Self::no_color()
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            human: "\x1b[36m",   // Cyan
            agent: "\x1b[32m",   // Green
            warning: "\x1b[33m", // Yellow
            muted: "\x1b[90m",   // Gray
            bold: "\x1b[1m",     // Bold
            reset: "\x1b[0m",    // Reset
        }
    }
}

impl Theme {
    /// Theme with no colors (empty strings).
    pub fn no_color() -> Self {
        Self {
            human: "",
            agent: "",
            warning: "",
            muted: "",
            bold: "",
            reset: "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_struct_has_all_fields() {
        let theme = Theme::default();
        assert!(!theme.human.is_empty());
        assert!(!theme.agent.is_empty());
        assert!(!theme.warning.is_empty());
        assert!(!theme.muted.is_empty());
        assert!(!theme.bold.is_empty());
        assert!(!theme.reset.is_empty());
    }

    #[test]
    fn default_theme_has_colors() {
        let theme = Theme::default();
        assert!(theme.human.contains("\x1b["));
        assert!(theme.agent.contains("\x1b["));
        assert!(theme.warning.contains("\x1b["));
        assert!(theme.muted.contains("\x1b["));
        assert!(theme.bold.contains("\x1b["));
        assert!(theme.reset.contains("\x1b["));
    }

    #[test]
    fn no_color_theme_is_empty() {
        let theme = Theme::no_color();
        assert!(theme.human.is_empty());
        assert!(theme.agent.is_empty());
        assert!(theme.warning.is_empty());
        assert!(theme.muted.is_empty());
        assert!(theme.bold.is_empty());
        assert!(theme.reset.is_empty());
    }

    #[test]
    fn should_use_color_respects_flag() {
        assert!(!Theme::should_use_color(true));
    }

    #[test]
    fn for_color_mode_returns_correct_theme() {
        let with_color = Theme::for_color_mode(true);
        assert!(!with_color.human.is_empty());

        let without_color = Theme::for_color_mode(false);
        assert!(without_color.human.is_empty());
    }
}
