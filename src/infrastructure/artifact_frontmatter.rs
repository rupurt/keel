//! Helpers for canonical frontmatter datetime formatting.

use chrono::NaiveDateTime;

pub fn format_datetime(value: NaiveDateTime) -> String {
    value.format("%Y-%m-%dT%H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::format_datetime;

    #[test]
    fn format_datetime_uses_strict_board_format() {
        let value =
            chrono::NaiveDateTime::parse_from_str("2026-03-05T10:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap();
        assert_eq!(format_datetime(value), "2026-03-05T10:30:00");
    }
}
