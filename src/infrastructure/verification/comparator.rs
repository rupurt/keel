//! Logic for comparing actual command output against expected criteria

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct CompareResult {
    pub passed: bool,
    pub actual: String,
    pub expected: String,
    pub requires_human_review: bool,
    pub error: Option<String>,
}

impl CompareResult {
    pub fn passed(&self) -> bool {
        self.passed
    }

    pub fn requires_human_review(&self) -> bool {
        self.requires_human_review
    }
}

pub fn compare(
    comparison: &super::parser::Comparison,
    exit_code: i32,
    stdout: &str,
    stderr: &str,
) -> CompareResult {
    use super::parser::Comparison;

    match comparison {
        Comparison::Success => CompareResult {
            passed: exit_code == 0,
            actual: format!("exit code {}", exit_code),
            expected: "exit code 0".to_string(),
            requires_human_review: false,
            error: if exit_code != 0 {
                Some(stderr.to_string())
            } else {
                None
            },
        },
        Comparison::Equals(expected) => {
            let actual = stdout.trim().to_string();
            CompareResult {
                passed: actual == *expected,
                actual,
                expected: expected.clone(),
                requires_human_review: false,
                error: None,
            }
        }
        Comparison::Contains(expected) => {
            let actual = stdout.trim().to_string();
            CompareResult {
                passed: actual.contains(expected),
                actual,
                expected: format!("contains '{}'", expected),
                requires_human_review: false,
                error: None,
            }
        }
        Comparison::Manual => CompareResult {
            passed: false,
            actual: "requires manual verification".to_string(),
            expected: "manual verification".to_string(),
            requires_human_review: true,
            error: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_result_methods() {
        let res = CompareResult {
            passed: true,
            actual: "ok".into(),
            expected: "ok".into(),
            requires_human_review: false,
            error: None,
        };
        assert!(res.passed());
        assert!(!res.requires_human_review());
    }
}
