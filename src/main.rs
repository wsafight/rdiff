use anyhow::Result;
use clap::Parser;
use std::path::Path;

mod cli;
mod diff;
mod utils;
mod web;

use cli::args::Args;
use diff::{
    directory::DirectoryDiffer,
    formatter::DiffFormatter,
    large_file::AdaptiveDiffer,
    types::{DiffOptions, DiffResult},
};
use web::server::WebServer;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // 解析命令行参数
    let args = Args::parse();

    // 创建 diff 选项
    let diff_options = DiffOptions {
        context_lines: args.unified_lines,
        ignore_whitespace: args.ignore_whitespace,
        ignore_case: args.ignore_case,
    };

    // 检查路径是否存在
    let path1 = Path::new(&args.path1);
    let path2 = Path::new(&args.path2);

    if !path1.exists() && !path2.exists() {
        eprintln!("❌ Error: Both paths do not exist");
        std::process::exit(1);
    }

    // 执行 diff
    let diff_result = if path1.is_dir() || path2.is_dir() {
        // 目录对比
        let differ = DirectoryDiffer::new(diff_options);
        differ.compare_directories(&args.path1, &args.path2)?
    } else {
        // 文件对比 - 使用自适应策略自动优化大文件性能
        let differ = AdaptiveDiffer::new(diff_options);
        let file_diff = differ.diff_files(&args.path1, &args.path2)?;

        // 计算统计信息
        let (total_additions, total_deletions) = count_changes(&file_diff);

        DiffResult {
            files: vec![file_diff],
            total_additions,
            total_deletions,
            total_files_changed: 1,
        }
    };

    // 输出结果
    if args.web {
        // Web 模式
        println!("🌐 Opening web browser to display diff...\n");
        let server = WebServer::new(diff_result, args.port);
        server.run().await?;
    } else {
        // 命令行模式
        let use_color = match args.color.as_str() {
            "always" => true,
            "never" => false,
            _ => {
                // auto: 检测是否是 TTY
                atty::is(atty::Stream::Stdout)
            }
        };

        let formatter = DiffFormatter::new(use_color);

        if args.brief {
            // 仅显示文件名
            if diff_result.files.is_empty() {
                println!("✅ Files are identical");
            } else {
                for file in &diff_result.files {
                    println!("Files {} and {} differ", file.old_path, file.new_path);
                }
            }
        } else {
            // 显示完整 diff
            if diff_result.files.is_empty() {
                println!("✅ No differences found");
            } else {
                let output = formatter.format_unified(&diff_result);
                print!("{}", output);

                // 显示统计信息
                println!();
                let summary = formatter.format_summary(&diff_result);
                println!("{}", summary);
            }
        }
    }

    Ok(())
}

/// Count total additions and deletions in a file diff
fn count_changes(file_diff: &diff::types::FileDiff) -> (usize, usize) {
    let mut additions = 0;
    let mut deletions = 0;

    for hunk in &file_diff.hunks {
        for line in &hunk.lines {
            match line.change_type {
                diff::types::ChangeType::Add => additions += 1,
                diff::types::ChangeType::Delete => deletions += 1,
                _ => {}
            }
        }
    }

    (additions, deletions)
}

// 需要添加 atty 依赖来检测 TTY
mod atty {
    pub enum Stream {
        Stdout,
    }

    pub fn is(_stream: Stream) -> bool {
        // 简单实现：假设总是在 TTY 中
        true
    }
}
