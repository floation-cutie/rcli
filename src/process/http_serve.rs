use std::fmt::{self, Display};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use axum::extract::{Path as AxumPath, State};
use axum::http::StatusCode;
use axum::routing::get;
use tracing::{info, warn};

#[derive(Clone, Debug)]
struct HttpServeState {
    dir: PathBuf,
}

impl Display for HttpServeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpServeState(dir: {})", self.dir.display())
    }
}

pub async fn process_http_serve(path: &PathBuf, port: u16) -> Result<()> {
    let state = HttpServeState { dir: path.clone() };
    let router = Router::new()
        .route("/", get(root_handler))
        .route("/{*path}", get(file_handler))
        .with_state(Arc::new(state));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving files from '{:?}' on {}", path, addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn root_handler(State(_state): State<Arc<HttpServeState>>) -> String {
    "index.html".to_string()
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    AxumPath(path): AxumPath<String>,
) -> (StatusCode, String) {
    info!("state: {} path: {}", state, path);
    let file_path = state.dir.join(&path);
    info!("Trying to read file: {:?}", file_path);

    if file_path.is_file() {
        serve_file(&file_path).await
    } else if file_path.is_dir() {
        list_directory(&file_path).await
    } else {
        (StatusCode::NOT_FOUND, format!("File not found: {}", path))
    }
}

/// Serves a file by reading its contents
async fn serve_file(file_path: &Path) -> (StatusCode, String) {
    match tokio::fs::read_to_string(file_path).await {
        Ok(content) => {
            info!("Successfully read {} bytes from file: {:?}", content.len(), file_path);
            (StatusCode::OK, content)
        }
        Err(e) => {
            warn!("Failed to read file: {:?}, error: {}", file_path, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

/// Lists all files and directories in the given directory
async fn list_directory(dir_path: &Path) -> (StatusCode, String) {
    match tokio::fs::read_dir(dir_path).await {
        Ok(mut entries) => match collect_directory_entries(&mut entries).await {
            Ok(file_list) => {
                info!("Directory listing for {:?}:\n{}", dir_path, file_list);
                (StatusCode::OK, file_list)
            }
            Err(e) => {
                warn!("Failed to read directory entry: {:?}, error: {}", dir_path, e);
                (StatusCode::INTERNAL_SERVER_ERROR, e)
            }
        },
        Err(e) => {
            warn!("Failed to read directory: {:?}, error: {}", dir_path, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

/// Collects and formats directory entries
async fn collect_directory_entries(entries: &mut tokio::fs::ReadDir) -> Result<String, String> {
    let mut file_list = String::new();

    loop {
        match entries.next_entry().await {
            Ok(Some(entry)) => {
                let entry_path = entry.path();
                let formatted_entry = format_directory_entry(&entry, &entry_path);
                file_list.push_str(&formatted_entry);
            }
            Ok(None) => break,
            Err(e) => {
                return Err(format!("Failed to read directory entry: {}", e));
            }
        }
    }

    Ok(file_list)
}

/// Formats a single directory entry for display
fn format_directory_entry(entry: &tokio::fs::DirEntry, entry_path: &Path) -> String {
    if entry_path.is_file() {
        format!("{}\n", entry.file_name().to_string_lossy())
    } else if entry_path.is_dir() {
        format!("{}/\n", entry.file_name().to_string_lossy())
    } else {
        String::new()
    }
}
