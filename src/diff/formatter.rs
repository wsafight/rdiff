use colored::*;
use super::types::*;

pub struct DiffFormatter {
    use_color: bool,
}

impl DiffFormatter {
    pub fn new(use_color: bool) -> Self {
        Self { use_color }
    }

    /// Format diff result as unified diff
    pub fn format_unified(&self, diff_result: &DiffResult) -> String {
        let mut output = String::new();

        for file_diff in &diff_result.files {
            output.push_str(&self.format_file_diff(file_diff));
            output.push('\n');
        }

        output
    }

    /// Format a single file diff
    pub fn format_file_diff(&self, file_diff: &FileDiff) -> String {
        let mut output = String::new();

        // File header
        let header = if file_diff.is_new {
            format!("diff --git a{} b{}", file_diff.old_path, file_diff.new_path)
        } else if file_diff.is_deleted {
            format!("diff --git a{} b{}", file_diff.old_path, file_diff.new_path)
        } else {
            format!("diff --git a/{} b/{}", file_diff.old_path, file_diff.new_path)
        };

        output.push_str(&self.colorize(&header, "white", true));
        output.push('\n');

        if file_diff.is_binary {
            let binary_msg = format!("Binary files {} and {} differ", file_diff.old_path, file_diff.new_path);
            output.push_str(&self.colorize(&binary_msg, "white", false));
            output.push('\n');
            return output;
        }

        // --- and +++ lines
        let old_line = format!("--- {}", file_diff.old_path);
        let new_line = format!("+++ {}", file_diff.new_path);
        output.push_str(&self.colorize(&old_line, "red", true));
        output.push('\n');
        output.push_str(&self.colorize(&new_line, "green", true));
        output.push('\n');

        // Hunks
        for hunk in &file_diff.hunks {
            output.push_str(&self.format_hunk(hunk));
        }

        output
    }

    /// Format a hunk
    fn format_hunk(&self, hunk: &Hunk) -> String {
        let mut output = String::new();

        // Hunk header
        let header = format!(
            "@@ -{},{} +{},{} @@",
            hunk.old_start, hunk.old_count, hunk.new_start, hunk.new_count
        );
        output.push_str(&self.colorize(&header, "cyan", true));
        output.push('\n');

        // Lines
        for line in &hunk.lines {
            output.push_str(&self.format_line(line));
            output.push('\n');
        }

        output
    }

    /// Format a line change
    fn format_line(&self, line: &LineChange) -> String {
        let (prefix, color) = match line.change_type {
            ChangeType::Add => ("+", "green"),
            ChangeType::Delete => ("-", "red"),
            ChangeType::Context => (" ", "white"),
            ChangeType::Modify => ("~", "yellow"),
        };

        let line_str = format!("{}{}", prefix, line.content);
        self.colorize(&line_str, color, false)
    }

    /// Format summary statistics
    pub fn format_summary(&self, diff_result: &DiffResult) -> String {
        let summary = format!(
            "{} file(s) changed, {} insertion(s)(+), {} deletion(s)(-)",
            diff_result.total_files_changed,
            diff_result.total_additions,
            diff_result.total_deletions
        );

        self.colorize(&summary, "white", true)
    }

    /// Colorize text if color is enabled
    fn colorize(&self, text: &str, color: &str, bold: bool) -> String {
        if !self.use_color {
            return text.to_string();
        }

        let colored = match color {
            "red" => text.red(),
            "green" => text.green(),
            "yellow" => text.yellow(),
            "cyan" => text.cyan(),
            "white" => text.white(),
            _ => text.normal(),
        };

        if bold {
            colored.bold().to_string()
        } else {
            colored.to_string()
        }
    }
}
