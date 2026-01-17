use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Add,      // 新增行
    Delete,   // 删除行
    Modify,   // 修改行
    Context,  // 上下文行（未修改）
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineChange {
    pub change_type: ChangeType,
    pub old_line_num: Option<usize>,
    pub new_line_num: Option<usize>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<LineChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    pub path: String,
    pub old_path: String,
    pub new_path: String,
    pub is_binary: bool,
    pub is_new: bool,
    pub is_deleted: bool,
    pub hunks: Vec<Hunk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_content: Option<Vec<LineChange>>, // Complete file content for full view
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    pub files: Vec<FileDiff>,
    pub total_additions: usize,
    pub total_deletions: usize,
    pub total_files_changed: usize,
}

#[derive(Debug, Clone)]
pub struct DiffOptions {
    pub context_lines: usize,
    pub ignore_whitespace: bool,
    pub ignore_case: bool,
}

impl Default for DiffOptions {
    fn default() -> Self {
        Self {
            context_lines: 3,
            ignore_whitespace: false,
            ignore_case: false,
        }
    }
}
