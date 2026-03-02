//! Table rendering utilities for CLI output
//!
//! Provides dynamic column width calculation to prevent column bleeding.

/// A simple table for CLI output with auto-calculated column widths
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    widths: Vec<usize>,
}

impl Table {
    /// Create a new table with the given column headers
    pub fn new(headers: &[&str]) -> Self {
        let headers: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
        let widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
        Self {
            headers,
            rows: Vec::new(),
            widths,
        }
    }

    /// Add a row to the table, updating column widths as needed
    pub fn row(&mut self, values: &[&str]) {
        let row: Vec<String> = values.iter().map(|s| s.to_string()).collect();

        // Update max widths using visible width (ignoring ANSI codes)
        for (i, value) in row.iter().enumerate() {
            if i < self.widths.len() {
                let width = crate::infrastructure::utils::visible_width(value);
                self.widths[i] = self.widths[i].max(width);
            }
        }

        self.rows.push(row);
    }

    /// Print the table with proper column alignment
    pub fn print(&self) {
        // Print header
        self.print_row(&self.headers);

        // Print separator
        let total_width: usize = self.widths.iter().sum::<usize>() + (self.widths.len() - 1) * 2;
        println!("{}", "-".repeat(total_width));

        // Print rows
        for row in &self.rows {
            self.print_row(row);
        }
    }

    /// Print a single row with proper column widths
    fn print_row(&self, values: &[String]) {
        let mut row_str = String::new();

        for (i, v) in values.iter().enumerate() {
            if i > 0 {
                row_str.push_str("  ");
            }

            let target_width = self.widths.get(i).copied().unwrap_or(0);
            let current_width = crate::infrastructure::utils::visible_width(v);

            row_str.push_str(v);
            if target_width > current_width {
                row_str.push_str(&" ".repeat(target_width - current_width));
            }
        }

        println!("{}", row_str);
    }

    /// Check if the table has any rows
    #[allow(dead_code)] // Utility method for conditional rendering
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Get the number of rows
    #[allow(dead_code)] // Utility method for conditional rendering
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_calculates_widths() {
        let mut table = Table::new(&["ID", "NAME"]);
        table.row(&["1", "Alice"]);
        table.row(&["123", "Bob"]);

        // ID column should be 3 (from "123"), NAME should be 5 (from "Alice")
        assert_eq!(table.widths, vec![3, 5]);
    }

    #[test]
    fn table_header_sets_min_width() {
        let mut table = Table::new(&["IDENTIFIER", "N"]);
        table.row(&["1", "Alice"]);

        // IDENTIFIER is longer than "1", N is shorter than "Alice"
        assert_eq!(table.widths, vec![10, 5]);
    }

    #[test]
    fn table_is_empty() {
        let table = Table::new(&["A", "B"]);
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn table_counts_rows() {
        let mut table = Table::new(&["A"]);
        table.row(&["1"]);
        table.row(&["2"]);
        assert!(!table.is_empty());
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn table_alignment_with_ansi() {
        use owo_colors::OwoColorize;
        let mut table = Table::new(&["ID", "STATUS"]);
        // "DONE" in green is longer in raw bytes but 4 in visible width
        let done = "DONE".green().to_string();
        table.row(&["1", &done]);
        table.row(&["123", "TODO"]);

        assert_eq!(table.widths[0], 3); // from "123"
        assert_eq!(table.widths[1], 6); // from "STATUS" (6) vs "DONE" (4) or "TODO" (4)
    }
}
