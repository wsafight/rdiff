pub mod cli;
pub mod diff;
pub mod utils;
pub mod web;

// 重新导出主要类型
pub use diff::types::{DiffOptions, DiffResult, FileDiff};
pub use diff::file::FileDiffer;
pub use diff::directory::DirectoryDiffer;
