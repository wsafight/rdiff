use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use memmap2::Mmap;
use memchr::Memchr;
use similar::TextDiff;
use std::fs::{self, File};
use std::path::Path;

use super::types::*;

/// 内存映射文件，用于高效处理大文件
pub struct MmapFile {
    mmap: Mmap,
    line_offsets: Vec<usize>,  // 缓存行偏移量
}

impl MmapFile {
    /// 打开文件并创建内存映射
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        // 计算所有行的偏移量（只扫描一次）
        let line_offsets = Self::calculate_line_offsets(&mmap);

        Ok(Self { mmap, line_offsets })
    }

    /// 计算所有换行符的位置
    fn calculate_line_offsets(data: &[u8]) -> Vec<usize> {
        let mut offsets = vec![0]; // 第一行从 0 开始

        // 使用 memchr 快速找到所有换行符
        for pos in Memchr::new(b'\n', data) {
            offsets.push(pos + 1);
        }

        offsets
    }

    /// 获取文件总行数
    pub fn line_count(&self) -> usize {
        if self.line_offsets.is_empty() {
            return 0;
        }
        self.line_offsets.len()
    }

    /// 获取指定行的内容
    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        if line_num >= self.line_count() {
            return None;
        }

        let start = self.line_offsets[line_num];
        let end = if line_num + 1 < self.line_offsets.len() {
            self.line_offsets[line_num + 1] - 1  // 不包含换行符
        } else {
            self.mmap.len()
        };

        if start >= end {
            return Some("");
        }

        std::str::from_utf8(&self.mmap[start..end]).ok()
    }

    /// 获取指定范围的行
    pub fn get_lines(&self, start: usize, count: usize) -> Vec<String> {
        let end = (start + count).min(self.line_count());
        (start..end)
            .filter_map(|i| self.get_line(i).map(|s| s.to_string()))
            .collect()
    }

    /// 获取所有行（用于小文件或需要完整内容的场景）
    pub fn get_all_lines(&self) -> Vec<String> {
        self.get_lines(0, self.line_count())
    }

    /// 获取文件大小（字节）
    pub fn size(&self) -> usize {
        self.mmap.len()
    }
}

// ============================================================================
// 分块 Differ - 用于大文件
// ============================================================================

pub struct ChunkedDiffer {
    chunk_size: usize,
    options: DiffOptions,
    show_progress: bool,
}

impl ChunkedDiffer {
    pub fn new(chunk_size: usize, options: DiffOptions) -> Self {
        Self {
            chunk_size,
            options,
            show_progress: false,
        }
    }

    pub fn with_progress(chunk_size: usize, options: DiffOptions, show_progress: bool) -> Self {
        Self {
            chunk_size,
            options,
            show_progress,
        }
    }

    /// Generate full file content from new file
    fn generate_full_content(&self, file: &MmapFile) -> Vec<LineChange> {
        let lines = file.get_all_lines();
        lines.iter().enumerate().map(|(idx, line)| {
            LineChange {
                change_type: ChangeType::Context,
                old_line_num: Some(idx + 1),
                new_line_num: Some(idx + 1),
                content: line.clone(),
            }
        }).collect()
    }

    /// 分块对比大文件
    pub fn diff_large_files(
        &self,
        file1: &MmapFile,
        file2: &MmapFile,
        path1: &str,
        path2: &str,
    ) -> Result<FileDiff> {
        let total_lines1 = file1.line_count();
        let total_lines2 = file2.line_count();
        let max_lines = total_lines1.max(total_lines2);

        let mut all_hunks = Vec::new();
        let mut offset1 = 0;
        let mut offset2 = 0;

        // 创建进度条（仅当显示进度且文件较大时）
        let progress = if self.show_progress && max_lines > 50_000 {
            let pb = ProgressBar::new(max_lines as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} lines {msg}")
                    .unwrap()
                    .progress_chars("=>-")
            );
            pb.set_message("Processing...");
            Some(pb)
        } else {
            None
        };

        // 分块处理
        while offset1 < total_lines1 || offset2 < total_lines2 {
            let chunk_size1 = self.chunk_size.min(total_lines1 - offset1);
            let chunk_size2 = self.chunk_size.min(total_lines2 - offset2);

            if chunk_size1 == 0 && chunk_size2 == 0 {
                break;
            }

            let chunk1 = file1.get_lines(offset1, chunk_size1);
            let chunk2 = file2.get_lines(offset2, chunk_size2);

            // 对小块进行 diff
            if let Some(chunk_hunks) = self.diff_chunk(&chunk1, &chunk2, offset1, offset2) {
                all_hunks.push(chunk_hunks);
            }

            offset1 += chunk_size1;
            offset2 += chunk_size2;

            // 更新进度
            if let Some(ref pb) = progress {
                pb.set_position(offset1.max(offset2) as u64);
            }
        }

        // 完成进度条
        if let Some(pb) = progress {
            pb.finish_with_message("Complete!");
        }

        // 合并相邻的 hunks
        let merged_hunks = self.merge_hunks(all_hunks);

        // 计算统计信息
        let (additions, deletions) = self.count_changes(&merged_hunks);

        // 生成完整文件内容（用于 Web 全文展示）
        let full_content = Some(self.generate_full_content(file2));

        Ok(FileDiff {
            path: path1.to_string(),
            old_path: path1.to_string(),
            new_path: path2.to_string(),
            is_binary: false,
            is_new: false,
            is_deleted: false,
            hunks: merged_hunks,
            full_content,
        })
    }

    fn diff_chunk(
        &self,
        chunk1: &[String],
        chunk2: &[String],
        offset1: usize,
        offset2: usize,
    ) -> Option<Hunk> {
        if chunk1.is_empty() && chunk2.is_empty() {
            return None;
        }

        let text1 = chunk1.join("\n");
        let text2 = chunk2.join("\n");

        let diff = TextDiff::from_lines(&text1, &text2);

        let mut lines = Vec::new();
        let mut old_line = offset1 + 1;
        let mut new_line = offset2 + 1;

        for change in diff.iter_all_changes() {
            let content = change.value().trim_end_matches(&['\n', '\r'][..]).to_string();

            match change.tag() {
                similar::ChangeTag::Equal => {
                    lines.push(LineChange {
                        change_type: ChangeType::Context,
                        old_line_num: Some(old_line),
                        new_line_num: Some(new_line),
                        content,
                    });
                    old_line += 1;
                    new_line += 1;
                }
                similar::ChangeTag::Delete => {
                    lines.push(LineChange {
                        change_type: ChangeType::Delete,
                        old_line_num: Some(old_line),
                        new_line_num: None,
                        content,
                    });
                    old_line += 1;
                }
                similar::ChangeTag::Insert => {
                    lines.push(LineChange {
                        change_type: ChangeType::Add,
                        old_line_num: None,
                        new_line_num: Some(new_line),
                        content,
                    });
                    new_line += 1;
                }
            }
        }

        if lines.is_empty() {
            return None;
        }

        Some(Hunk {
            old_start: offset1 + 1,
            old_count: chunk1.len(),
            new_start: offset2 + 1,
            new_count: chunk2.len(),
            lines,
        })
    }

    fn merge_hunks(&self, hunks: Vec<Hunk>) -> Vec<Hunk> {
        // 简单实现：直接返回所有 hunks
        // TODO: 实现智能合并相邻的 hunks
        hunks
    }

    fn count_changes(&self, hunks: &[Hunk]) -> (usize, usize) {
        let mut additions = 0;
        let mut deletions = 0;

        for hunk in hunks {
            for line in &hunk.lines {
                match line.change_type {
                    ChangeType::Add => additions += 1,
                    ChangeType::Delete => deletions += 1,
                    _ => {}
                }
            }
        }

        (additions, deletions)
    }
}

// ============================================================================
// 并行 Differ - 利用多核加速
// ============================================================================

use rayon::prelude::*;

pub struct ParallelDiffer {
    chunk_size: usize,
    options: DiffOptions,
    show_progress: bool,
}

impl ParallelDiffer {
    pub fn new(chunk_size: usize, options: DiffOptions) -> Self {
        Self {
            chunk_size,
            options,
            show_progress: false,
        }
    }

    pub fn with_progress(chunk_size: usize, options: DiffOptions, show_progress: bool) -> Self {
        Self {
            chunk_size,
            options,
            show_progress,
        }
    }

    /// 并行对比大文件
    pub fn diff_parallel(
        &self,
        file1: &MmapFile,
        file2: &MmapFile,
        path1: &str,
        path2: &str,
    ) -> Result<FileDiff> {
        let total_lines1 = file1.line_count();
        let total_lines2 = file2.line_count();
        let max_lines = total_lines1.max(total_lines2);

        // 计算分块数量
        let num_chunks = (max_lines + self.chunk_size - 1) / self.chunk_size;

        // 创建进度条（仅当显示进度且文件较大时）
        let progress = if self.show_progress && max_lines > 50_000 {
            let pb = ProgressBar::new(num_chunks as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.green/blue} {pos:>4}/{len:4} chunks {msg}")
                    .unwrap()
                    .progress_chars("##-")
            );
            pb.set_message("Parallel processing...");
            Some(pb)
        } else {
            None
        };

        // 并行处理每个块
        let hunks: Vec<Option<Hunk>> = (0..num_chunks)
            .into_par_iter()
            .map(|i| {
                let start1 = (i * self.chunk_size).min(total_lines1);
                let start2 = (i * self.chunk_size).min(total_lines2);

                let count1 = self.chunk_size.min(total_lines1 - start1);
                let count2 = self.chunk_size.min(total_lines2 - start2);

                if count1 == 0 && count2 == 0 {
                    return None;
                }

                let chunk1 = file1.get_lines(start1, count1);
                let chunk2 = file2.get_lines(start2, count2);

                let chunked = ChunkedDiffer::new(self.chunk_size, self.options.clone());
                let result = chunked.diff_chunk(&chunk1, &chunk2, start1, start2);

                // 更新进度（线程安全）
                if let Some(ref pb) = progress {
                    pb.inc(1);
                }

                result
            })
            .collect();

        // 完成进度条
        if let Some(pb) = progress {
            pb.finish_with_message("Parallel processing complete!");
        }

        // 合并结果
        let merged_hunks: Vec<Hunk> = hunks.into_iter().flatten().collect();

        // 计算统计信息
        let chunked = ChunkedDiffer::new(self.chunk_size, self.options.clone());
        let (additions, deletions) = chunked.count_changes(&merged_hunks);

        // 生成完整文件内容（用于 Web 全文展示）
        let full_content = Some(chunked.generate_full_content(file2));

        Ok(FileDiff {
            path: path1.to_string(),
            old_path: path1.to_string(),
            new_path: path2.to_string(),
            is_binary: false,
            is_new: false,
            is_deleted: false,
            hunks: merged_hunks,
            full_content,
        })
    }
}

// ============================================================================
// 自适应 Differ - 根据文件大小自动选择策略
// ============================================================================

pub struct AdaptiveDiffer {
    small_file_threshold: u64,   // 10 MB
    medium_file_threshold: u64,  // 100 MB
    chunk_size: usize,           // 10,000 行
    options: DiffOptions,
    show_progress: bool,
}

impl AdaptiveDiffer {
    pub fn new(options: DiffOptions) -> Self {
        Self {
            small_file_threshold: 10 * 1024 * 1024,     // 10 MB
            medium_file_threshold: 100 * 1024 * 1024,   // 100 MB
            chunk_size: 10_000,
            options,
            show_progress: true,  // 默认显示进度条
        }
    }

    pub fn with_progress(options: DiffOptions, show_progress: bool) -> Self {
        let mut differ = Self::new(options);
        differ.show_progress = show_progress;
        differ
    }

    pub fn with_thresholds(
        options: DiffOptions,
        small_threshold: u64,
        medium_threshold: u64,
        chunk_size: usize,
    ) -> Self {
        Self {
            small_file_threshold: small_threshold,
            medium_file_threshold: medium_threshold,
            chunk_size,
            options,
            show_progress: true,
        }
    }

    /// 自适应对比文件
    pub fn diff_files(&self, path1: &str, path2: &str) -> Result<FileDiff> {
        let size1 = fs::metadata(path1)?.len();
        let size2 = fs::metadata(path2)?.len();
        let max_size = size1.max(size2);

        tracing::info!(
            "Comparing files: {} ({} bytes) vs {} ({} bytes)",
            path1, size1, path2, size2
        );

        match max_size {
            // 小文件：使用现有的快速方法
            s if s < self.small_file_threshold => {
                tracing::info!("Using fast diff for small files");
                self.diff_small_files(path1, path2)
            }

            // 中等文件：内存映射 + 分块
            s if s < self.medium_file_threshold => {
                tracing::info!("Using chunked diff for medium files");
                self.diff_medium_files(path1, path2)
            }

            // 大文件：内存映射 + 分块 + 并行
            _ => {
                tracing::info!("Using parallel diff for large files");
                self.diff_large_files(path1, path2)
            }
        }
    }

    fn diff_small_files(&self, path1: &str, path2: &str) -> Result<FileDiff> {
        // 使用现有的 FileDiffer
        use super::file::FileDiffer;
        let differ = FileDiffer::new(self.options.clone());
        differ.compare_files(path1, path2)
    }

    fn diff_medium_files(&self, path1: &str, path2: &str) -> Result<FileDiff> {
        let file1 = MmapFile::open(path1)?;
        let file2 = MmapFile::open(path2)?;

        let chunked = ChunkedDiffer::with_progress(
            self.chunk_size,
            self.options.clone(),
            self.show_progress
        );
        chunked.diff_large_files(&file1, &file2, path1, path2)
    }

    fn diff_large_files(&self, path1: &str, path2: &str) -> Result<FileDiff> {
        let file1 = MmapFile::open(path1)?;
        let file2 = MmapFile::open(path2)?;

        let parallel = ParallelDiffer::with_progress(
            self.chunk_size,
            self.options.clone(),
            self.show_progress
        );
        parallel.diff_parallel(&file1, &file2, path1, path2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_mmap_file_basic() {
        // 创建测试文件
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "line 1").unwrap();
        writeln!(temp_file, "line 2").unwrap();
        writeln!(temp_file, "line 3").unwrap();
        temp_file.flush().unwrap();

        // 测试内存映射
        let mmap_file = MmapFile::open(temp_file.path()).unwrap();

        assert_eq!(mmap_file.line_count(), 4);  // 3 行 + 最后的空行
        assert_eq!(mmap_file.get_line(0), Some("line 1"));
        assert_eq!(mmap_file.get_line(1), Some("line 2"));
        assert_eq!(mmap_file.get_line(2), Some("line 3"));
    }

    #[test]
    fn test_get_lines_range() {
        let mut temp_file = NamedTempFile::new().unwrap();
        for i in 1..=10 {
            writeln!(temp_file, "line {}", i).unwrap();
        }
        temp_file.flush().unwrap();

        let mmap_file = MmapFile::open(temp_file.path()).unwrap();
        let lines = mmap_file.get_lines(2, 3);

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line 3");
        assert_eq!(lines[1], "line 4");
        assert_eq!(lines[2], "line 5");
    }
}
