use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

use super::file::FileDiffer;
use super::types::*;

pub struct DirectoryDiffer {
    file_differ: FileDiffer,
}

impl DirectoryDiffer {
    pub fn new(options: DiffOptions) -> Self {
        Self {
            file_differ: FileDiffer::new(options),
        }
    }

    /// Compare two directories recursively
    pub fn compare_directories(&self, dir1: &str, dir2: &str) -> Result<DiffResult> {
        let dir1_path = Path::new(dir1);
        let dir2_path = Path::new(dir2);

        if !dir1_path.exists() {
            anyhow::bail!("Directory {} does not exist", dir1);
        }

        if !dir2_path.exists() {
            anyhow::bail!("Directory {} does not exist", dir2);
        }

        // Collect all files in both directories
        let files1 = self.collect_files(dir1)?;
        let files2 = self.collect_files(dir2)?;

        let mut all_paths: HashSet<String> = HashSet::new();
        all_paths.extend(files1.keys().cloned());
        all_paths.extend(files2.keys().cloned());

        let mut files = Vec::new();
        let mut total_additions = 0;
        let mut total_deletions = 0;

        for rel_path in all_paths.iter() {
            let full_path1 = files1.get(rel_path);
            let full_path2 = files2.get(rel_path);

            let file_diff = match (full_path1, full_path2) {
                (Some(p1), Some(p2)) => {
                    // File exists in both directories
                    self.file_differ.compare_files(p1, p2)?
                }
                (Some(p1), None) => {
                    // File only in dir1 (deleted)
                    let content = std::fs::read_to_string(p1)?;
                    self.create_deleted_diff(rel_path, &content)
                }
                (None, Some(p2)) => {
                    // File only in dir2 (new)
                    let content = std::fs::read_to_string(p2)?;
                    self.create_new_diff(rel_path, &content)
                }
                (None, None) => unreachable!(),
            };

            // Count additions and deletions
            for hunk in &file_diff.hunks {
                for line in &hunk.lines {
                    match line.change_type {
                        ChangeType::Add => total_additions += 1,
                        ChangeType::Delete => total_deletions += 1,
                        _ => {}
                    }
                }
            }

            // Only add if there are actual changes
            if !file_diff.hunks.is_empty() {
                files.push(file_diff);
            }
        }

        Ok(DiffResult {
            total_files_changed: files.len(),
            files,
            total_additions,
            total_deletions,
        })
    }

    /// Collect all files in directory with relative paths
    fn collect_files(&self, dir: &str) -> Result<HashMap<String, String>> {
        let dir_path = Path::new(dir);
        let mut files = HashMap::new();

        for entry in WalkDir::new(dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let full_path = entry.path();
                let rel_path = full_path
                    .strip_prefix(dir_path)
                    .unwrap_or(full_path)
                    .to_string_lossy()
                    .to_string();

                files.insert(rel_path, full_path.to_string_lossy().to_string());
            }
        }

        Ok(files)
    }

    fn create_new_diff(&self, rel_path: &str, content: &str) -> FileDiff {
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
            path: rel_path.to_string(),
            old_path: format!("/dev/null"),
            new_path: rel_path.to_string(),
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

    fn create_deleted_diff(&self, rel_path: &str, content: &str) -> FileDiff {
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
            path: rel_path.to_string(),
            old_path: rel_path.to_string(),
            new_path: format!("/dev/null"),
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
