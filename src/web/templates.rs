use crate::diff::types::*;
use super::assets::{get_css, get_js};

pub struct HtmlTemplate;

impl HtmlTemplate {
    /// Generate complete HTML page
    pub fn generate(diff_result: &DiffResult) -> String {
        let diff_data_json = serde_json::to_string(diff_result).unwrap_or_else(|_| "{}".to_string());

        format!(
            r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Diff Viewer - Rust Diff Tool</title>
    <style>{}</style>
</head>
<body>
    <header>
        <h1>📊 Diff Viewer</h1>
        <div class="controls">
            <button id="toggle-view" class="btn">Switch to Side-by-Side</button>
            <span class="stats">
                {} file(s) changed, {} insertion(s)(+), {} deletion(s)(-)
            </span>
        </div>
    </header>
    <main id="diff-container">
        <div class="loading">Loading diff...</div>
    </main>
    <script>
        const diffData = {};
        {}
    </script>
</body>
</html>"#,
            get_css(),
            diff_result.total_files_changed,
            diff_result.total_additions,
            diff_result.total_deletions,
            diff_data_json,
            get_js()
        )
    }
}
