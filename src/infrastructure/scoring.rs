//! EV (Expected Value) scoring system for bearings
//!
//! Calculates a composite score based on impact, confidence, effort, and risk factors
//! parsed from ASSESSMENT.md files, with weights applied based on the current mode.

use anyhow::{Context, Result, anyhow};
use std::path::Path;

use crate::infrastructure::config::ModeWeights;

/// Assessment factors for EV scoring (values 1-5)
#[derive(Debug, Clone, Default)]
pub struct AssessmentFactors {
    pub impact: Option<u8>,
    pub confidence: Option<u8>,
    pub effort: Option<u8>,
    pub risk: Option<u8>,
}

impl AssessmentFactors {
    /// Check if all required factors are present and valid
    pub fn is_complete(&self) -> bool {
        self.impact.is_some()
            && self.confidence.is_some()
            && self.effort.is_some()
            && self.risk.is_some()
    }

    /// Get a list of missing factors
    pub fn missing_factors(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if self.impact.is_none() {
            missing.push("impact");
        }
        if self.confidence.is_none() {
            missing.push("confidence");
        }
        if self.effort.is_none() {
            missing.push("effort");
        }
        if self.risk.is_none() {
            missing.push("risk");
        }
        missing
    }
}

/// EV score result
#[derive(Debug, Clone)]
pub struct EvScore {
    /// Raw score before weights
    #[allow(dead_code)] // Available for detailed score breakdown display
    pub raw_score: f64,
    /// Weighted score
    pub weighted_score: f64,
    /// The factors used
    #[allow(dead_code)] // Available for detailed score breakdown display
    pub factors: AssessmentFactors,
}

impl EvScore {
    /// Format score for display
    #[allow(dead_code)] // Available for CLI score display
    pub fn display(&self) -> String {
        format!("{:.2}", self.weighted_score)
    }
}

/// Calculate EV score from factors and weights
///
/// Formula: score = (impact × confidence) / (effort + risk_penalty)
/// Where risk_penalty = (risk - 1) / 2 (so risk 1 = 0, risk 5 = 2)
pub fn calculate_score(factors: &AssessmentFactors, weights: &ModeWeights) -> Result<EvScore> {
    let impact = factors
        .impact
        .ok_or_else(|| anyhow!("Missing impact factor"))? as f64;
    let confidence = factors
        .confidence
        .ok_or_else(|| anyhow!("Missing confidence factor"))? as f64;
    let effort = factors
        .effort
        .ok_or_else(|| anyhow!("Missing effort factor"))? as f64;
    let risk = factors.risk.ok_or_else(|| anyhow!("Missing risk factor"))? as f64;

    // Risk penalty: converts 1-5 risk to 0-2 penalty
    let risk_penalty = (risk - 1.0) / 2.0;

    // Raw formula: (impact × confidence) / (effort + risk_penalty)
    let raw_numerator = impact * confidence;
    let raw_denominator = effort + risk_penalty;
    let raw_score = if raw_denominator > 0.0 {
        raw_numerator / raw_denominator
    } else {
        raw_numerator
    };

    // Weighted formula: (impact*w1 × confidence*w2) / (effort*w3 + risk_penalty*w4)
    let weighted_numerator =
        (impact * weights.impact_weight) * (confidence * weights.confidence_weight);
    let weighted_denominator =
        (effort * weights.effort_weight) + (risk_penalty * weights.risk_weight);
    let weighted_score = if weighted_denominator > 0.0 {
        weighted_numerator / weighted_denominator
    } else {
        weighted_numerator
    };

    Ok(EvScore {
        raw_score,
        weighted_score,
        factors: factors.clone(),
    })
}

/// Parse assessment factors from ASSESSMENT.md content
///
/// Looks for a markdown table with columns for Factor and Value/Score
pub fn parse_assessment(content: &str) -> AssessmentFactors {
    let mut factors = AssessmentFactors::default();

    // Parse table rows looking for factor values
    for line in content.lines() {
        let line = line.trim();

        // Skip non-table lines and header separators
        if !line.starts_with('|') || line.contains("---") {
            continue;
        }

        // Parse table row
        let cells: Vec<&str> = line
            .split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if cells.len() >= 2 {
            let factor_name = cells[0].to_lowercase();
            let value_str = cells[1];

            // Try to extract a numeric value (valid range 1-5)
            if let Some(value) = extract_score(value_str).filter(|v| (1..=5).contains(v)) {
                if factor_name.contains("impact") {
                    factors.impact = Some(value);
                } else if factor_name.contains("confidence") {
                    factors.confidence = Some(value);
                } else if factor_name.contains("effort") {
                    factors.effort = Some(value);
                } else if factor_name.contains("risk") {
                    factors.risk = Some(value);
                }
            }
        }
    }

    factors
}

/// Extract a score value from a string like "4", "4/5", "High (4)", etc.
fn extract_score(s: &str) -> Option<u8> {
    // Try direct parse first
    if let Ok(n) = s.trim().parse::<u8>() {
        return Some(n);
    }

    // Try parsing first number in "X/Y" format
    if let Some(pos) = s.find('/')
        && let Ok(n) = s[..pos].trim().parse::<u8>()
    {
        return Some(n);
    }

    // Try finding a digit in parentheses like "(4)"
    if let Some(start) = s.find('(')
        && let Some(end) = s.find(')')
        && let Ok(n) = s[start + 1..end].trim().parse::<u8>()
    {
        return Some(n);
    }

    // Try finding any single digit
    s.chars().find_map(|c| c.to_digit(10).map(|d| d as u8))
}

/// Load and parse assessment factors from an ASSESSMENT.md file
pub fn load_assessment(path: &Path) -> Result<AssessmentFactors> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read assessment file: {}", path.display()))?;

    Ok(parse_assessment(&content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_assessment_extracts_factors() {
        let content = r#"
# Assessment

| Factor | Score |
|--------|-------|
| Impact | 4 |
| Confidence | 3 |
| Effort | 2 |
| Risk | 1 |
"#;

        let factors = parse_assessment(content);
        assert_eq!(factors.impact, Some(4));
        assert_eq!(factors.confidence, Some(3));
        assert_eq!(factors.effort, Some(2));
        assert_eq!(factors.risk, Some(1));
    }

    #[test]
    fn parse_assessment_handles_fraction_format() {
        let content = r#"
| Factor | Score |
|--------|-------|
| Impact | 4/5 |
| Confidence | 3/5 |
| Effort | 2/5 |
| Risk | 1/5 |
"#;

        let factors = parse_assessment(content);
        assert_eq!(factors.impact, Some(4));
        assert_eq!(factors.confidence, Some(3));
    }

    #[test]
    fn parse_assessment_handles_parentheses_format() {
        let content = r#"
| Factor | Score |
|--------|-------|
| Impact | High (4) |
| Confidence | Medium (3) |
| Effort | Low (2) |
| Risk | Minimal (1) |
"#;

        let factors = parse_assessment(content);
        assert_eq!(factors.impact, Some(4));
        assert_eq!(factors.risk, Some(1));
    }

    #[test]
    fn parse_assessment_handles_missing_factors() {
        let content = r#"
| Factor | Score |
|--------|-------|
| Impact | 4 |
| Effort | 2 |
"#;

        let factors = parse_assessment(content);
        assert_eq!(factors.impact, Some(4));
        assert_eq!(factors.confidence, None);
        assert_eq!(factors.effort, Some(2));
        assert_eq!(factors.risk, None);
        assert!(!factors.is_complete());
        assert_eq!(factors.missing_factors(), vec!["confidence", "risk"]);
    }

    #[test]
    fn calculate_score_uses_formula() {
        let factors = AssessmentFactors {
            impact: Some(4),
            confidence: Some(4),
            effort: Some(2),
            risk: Some(1),
        };

        let weights = ModeWeights {
            impact_weight: 1.0,
            confidence_weight: 1.0,
            effort_weight: 1.0,
            risk_weight: 1.0,
        };

        let score = calculate_score(&factors, &weights).unwrap();

        // Raw: (4 × 4) / (2 + 0) = 16 / 2 = 8.0
        assert!((score.raw_score - 8.0).abs() < 0.01);
        assert!((score.weighted_score - 8.0).abs() < 0.01);
    }

    #[test]
    fn calculate_score_applies_risk_penalty() {
        let factors = AssessmentFactors {
            impact: Some(4),
            confidence: Some(4),
            effort: Some(2),
            risk: Some(5), // Max risk
        };

        let weights = ModeWeights {
            impact_weight: 1.0,
            confidence_weight: 1.0,
            effort_weight: 1.0,
            risk_weight: 1.0,
        };

        let score = calculate_score(&factors, &weights).unwrap();

        // Risk penalty for 5 = (5-1)/2 = 2
        // Raw: (4 × 4) / (2 + 2) = 16 / 4 = 4.0
        assert!((score.raw_score - 4.0).abs() < 0.01);
    }

    #[test]
    fn calculate_score_applies_weights() {
        let factors = AssessmentFactors {
            impact: Some(4),
            confidence: Some(3),
            effort: Some(2),
            risk: Some(1),
        };

        // Constrained mode emphasizes effort
        let constrained = ModeWeights::constrained();
        let score = calculate_score(&factors, &constrained).unwrap();

        // Growth mode de-emphasizes effort
        let growth = ModeWeights::growth();
        let growth_score = calculate_score(&factors, &growth).unwrap();

        // Growth score should be higher (effort less penalized)
        assert!(growth_score.weighted_score > score.weighted_score);
    }

    #[test]
    fn calculate_score_errors_on_missing_factors() {
        let factors = AssessmentFactors {
            impact: Some(4),
            confidence: None,
            effort: Some(2),
            risk: Some(1),
        };

        let weights = ModeWeights::constrained();
        let result = calculate_score(&factors, &weights);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("confidence"));
    }

    #[test]
    fn extract_score_handles_various_formats() {
        assert_eq!(extract_score("4"), Some(4));
        assert_eq!(extract_score("  4  "), Some(4));
        assert_eq!(extract_score("4/5"), Some(4));
        assert_eq!(extract_score("High (4)"), Some(4));
        assert_eq!(extract_score("Medium 3"), Some(3));
        assert_eq!(extract_score(""), None);
        assert_eq!(extract_score("None"), None);
    }

    #[test]
    fn factors_is_complete() {
        let complete = AssessmentFactors {
            impact: Some(4),
            confidence: Some(3),
            effort: Some(2),
            risk: Some(1),
        };
        assert!(complete.is_complete());

        let incomplete = AssessmentFactors {
            impact: Some(4),
            confidence: None,
            effort: Some(2),
            risk: Some(1),
        };
        assert!(!incomplete.is_complete());
    }
}
