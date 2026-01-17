use anyhow::Result;
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::diff::types::{DiffResult, LineChange};
use super::templates::HtmlTemplate;

#[derive(Clone)]
pub struct AppState {
    pub diff_result: Arc<DiffResult>,
}

pub struct WebServer {
    port: u16,
    state: AppState,
}

impl WebServer {
    pub fn new(diff_result: DiffResult, port: Option<u16>) -> Self {
        Self {
            port: port.unwrap_or(0), // 0 means random port
            state: AppState {
                diff_result: Arc::new(diff_result),
            },
        }
    }

    /// Start the web server and open browser
    pub async fn run(self) -> Result<()> {
        let router = Self::create_router(self.state);

        // Find available port
        let port = if self.port == 0 {
            Self::find_available_port()
        } else {
            self.port
        };

        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        println!("🚀 Starting web server at http://{}", addr);
        println!("📂 Open your browser to view the diff");
        println!("💡 Press Ctrl+C to stop the server\n");

        // Open browser
        let url = format!("http://{}", addr);
        if let Err(e) = open::that(&url) {
            eprintln!("⚠️  Could not open browser automatically: {}", e);
            println!("Please open this URL manually: {}", url);
        }

        // Start server
        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, router).await?;

        Ok(())
    }

    /// Create router
    fn create_router(state: AppState) -> Router {
        Router::new()
            .route("/", get(index_handler))
            .route("/api/diff", get(api_handler))
            .route("/api/diff/paginated", get(paginated_api_handler))
            .with_state(state)
    }

    /// Find available port
    fn find_available_port() -> u16 {
        // Try ports from 8080 to 8090
        for port in 8080..8100 {
            if std::net::TcpListener::bind(("127.0.0.1", port)).is_ok() {
                return port;
            }
        }
        // Fallback to random port
        0
    }
}

/// Handler for index page
async fn index_handler(State(state): State<AppState>) -> impl IntoResponse {
    let html = HtmlTemplate::generate(&state.diff_result);
    Html(html)
}

/// Handler for API endpoint (return JSON)
async fn api_handler(State(state): State<AppState>) -> impl IntoResponse {
    axum::Json((*state.diff_result).clone())
}

/// Query parameters for paginated API
#[derive(Deserialize)]
struct PaginationParams {
    #[serde(default)]
    page: usize,
    #[serde(default = "default_page_size")]
    page_size: usize,
}

fn default_page_size() -> usize {
    100
}

/// Response for paginated API
#[derive(Serialize)]
struct PaginatedDiffResponse {
    lines: Vec<LineChange>,
    total_lines: usize,
    page: usize,
    page_size: usize,
    total_pages: usize,
    has_more: bool,
}

/// Handler for paginated API endpoint
async fn paginated_api_handler(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let page = params.page;
    let page_size = params.page_size.min(1000); // 最大 1000 行每页

    // 收集所有行
    let all_lines: Vec<LineChange> = state
        .diff_result
        .files
        .iter()
        .flat_map(|file| {
            file.hunks
                .iter()
                .flat_map(|hunk| hunk.lines.iter())
        })
        .cloned()
        .collect();

    let total_lines = all_lines.len();
    let total_pages = (total_lines + page_size - 1) / page_size;
    let start = page * page_size;
    let end = (start + page_size).min(total_lines);
    let has_more = end < total_lines;

    let lines = if start < total_lines {
        all_lines[start..end].to_vec()
    } else {
        Vec::new()
    };

    axum::Json(PaginatedDiffResponse {
        lines,
        total_lines,
        page,
        page_size,
        total_pages,
        has_more,
    })
}
