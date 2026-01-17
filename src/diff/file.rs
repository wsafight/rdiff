use anyhow::Result;
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::path::Path;

use super::types::*;
use crate::utils::fs as utils_fs;

pub struct FileDiffer {
    options: DiffOptions,
}

impl FileDiffer {
    pub fn new(options: DiffOptions) -> Self {
        Self { options }
    }

    /// Compare two files and return diff result
    pub fn compare_files(&self, path1: &str, path2: &str) -> Result<FileDiff> {
        let path1_obj = Path::new(path1);
        let path2_obj = Path::new(path2);

        // Check if files exist
        let exists1 = path1_obj.exists();
        let exists2 = path2_obj.exists();

        if !exists1 && !exists2 {
            anyhow::bail!("Both files do not exist");
        }

        // Handle new or deleted files
        if !exists1 {
            let content2 = fs::read_to_string(path2)?;
            return Ok(self.create_new_file_diff(path2, &content2));
        }

        if !exists2 {
            let content1 = fs::read_to_string(path1)?;
            return Ok(self.create_deleted_file_diff(path1, &content1));
        }

        // Check if files are binary
        if utils_fs::is_binary_file(path1)? || utils_fs::is_binary_file(path2)? {
            return Ok(FileDiff {
                path: path1.to_string(),
                old_path: path1.to_string(),
                new_path: path2.to_string(),
                is_binary: true,
                is_new: false,
                is_deleted: false,
                hunks: vec![],
                full_content: None,
            });
        }

        // Read file contents
        let content1 = fs::read_to_string(path1)?;
        let content2 = fs::read_to_string(path2)?;

        Ok(self.compare_text(&content1, &content2, path1, path2))
    }

    /// Compare two text contents
    pub fn compare_text(
        &self,
        old_text: &str,
        new_text: &str,
        old_path: &str,
        new_path: &str,
    ) -> FileDiff {
        let diff = TextDiff::from_lines(old_text, new_text);

        let mut hunks = Vec::new();
        let mut current_hunk: Option<Hunk> = None;
        let mut old_line = 1;
        let mut new_line = 1;

        for change in diff.iter_all_changes() {
            let line_content = change.value().trim_end_matches(&['\n', '\r'][..]).to_string();

            match change.tag() {
                ChangeTag::Equal => {
                    let line_change = LineChange {
                        change_type: ChangeType::Context,
                        old_line_num: Some(old_line),
                        new_line_num: Some(new_line),
                        content: line_content,
                    };

                    if let Some(ref mut hunk) = current_hunk {
                        hunk.lines.push(line_change);
                        hunk.old_count += 1;
                        hunk.new_count += 1;
                    }

                    old_line += 1;
                    new_line += 1;
                }
                ChangeTag::Delete => {
                    let line_change = LineChange {
                        change_type: ChangeType::Delete,
                        old_line_num: Some(old_line),
                        new_line_num: None,
                        content: line_content,
                    };

                    if current_hunk.is_none() {
                        current_hunk = Some(Hunk {
                            old_start: old_line,
                            old_count: 0,
                            new_start: new_line,
                            new_count: 0,
                            lines: Vec::new(),
                        });
                    }

                    if let Some(ref mut hunk) = current_hunk {
                        hunk.lines.push(line_change);
                        hunk.old_count += 1;
                    }

                    old_line += 1;
                }
                ChangeTag::Insert => {
                    let line_change = LineChange {
                        change_type: ChangeType::Add,
                        old_line_num: None,
                        new_line_num: Some(new_line),
                        content: line_content,
                    };

                    if current_hunk.is_none() {
                        current_hunk = Some(Hunk {
                            old_start: old_line,
                            old_count: 0,
                            new_start: new_line,
                            new_count: 0,
                            lines: Vec::new(),
                        });
                    }

                    if let Some(ref mut hunk) = current_hunk {
                        hunk.lines.push(line_change);
                        hunk.new_count += 1;
                    }

                    new_line += 1;
                }
            }
        }

        // Add last hunk if exists
        if let Some(hunk) = current_hunk {
            hunks.push(hunk);
        }

        // Generate full content from new_text for web full view
        let full_content = Some(
            new_text
                .lines()
                .enumerate()
                .map(|(idx, line)| LineChange {
                    change_type: ChangeType::Context,
                    old_line_num: Some(idx + 1),
                    new_line_num: Some(idx + 1),
                    content: line.to_string(),
                })
                .collect()
        );

        FileDiff {
            path: old_path.to_string(),
            old_path: old_path.to_string(),
            new_path: new_path.to_string(),
            is_binary: false,
            is_new: false,
            is_deleted: false,
            hunks,
            full_content,
        }
    }

    fn create_new_file_diff(&self, path: &str, content: &str) -> FileDiff {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut line_changes = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            line_changes.push(LineChange {
                change_type: ChangeType::Add,
                old_line_num: None,
                new_line_num: Some(idx + 1),
                content: line.clone(),
            });
        }

        // For new files, full content is all the lines (marked as Context for full view)
        let full_content = Some(
            lines
                .iter()
                .enumerate()
                .map(|(idx, line)| LineChange {
                    change_type: ChangeType::Context,
                    old_line_num: Some(idx + 1),
                    new_line_num: Some(idx + 1),
                    content: line.clone(),
                })
                .collect()
        );

        FileDiff {
            path: path.to_string(),
            old_path: "/dev/null".to_string(),
            new_path: path.to_string(),
            is_binary: false,
            is_new: true,
            is_deleted: false,
            hunks: vec![Hunk {
                old_start: 0,
                old_count: 0,
                new_start: 1,
                new_count: lines.len(),
                lines: line_changes,
            }],
            full_content,
        }
    }

    fn create_deleted_file_diff(&self, path: &str, content: &str) -> FileDiff {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut line_changes = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            line_changes.push(LineChange {
                change_type: ChangeType::Delete,
                old_line_num: Some(idx + 1),
                new_line_num: None,
                content: line.clone(),
            });
        }

        FileDiff {
            path: path.to_string(),
            old_path: path.to_string(),
            new_path: "/dev/null".to_string(),
            is_binary: false,
            is_new: false,
            is_deleted: true,
            hunks: vec![Hunk {
                old_start: 1,
                old_count: lines.len(),
                new_start: 0,
                new_count: 0,
                lines: line_changes,
            }],
            full_content: None, // Deleted files have no new content
        }
    }
}
