//! # sysml-api
//!
//! REST API server for SysML v2 models.
//!
//! This crate provides a minimal HTTP API for model storage and retrieval.
//!
//! ## Endpoints
//!
//! - `GET /health` - Health check
//! - `GET /projects` - List all projects
//! - `GET /projects/:id/commits` - List commits for a project
//! - `GET /projects/:id/commits/:commit/model` - Get model snapshot
//! - `POST /projects/:id/commits/:commit/model` - Store model snapshot

use std::sync::Arc;
use tokio::sync::RwLock;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use sysml_canon::{from_json_str, to_json_string};
use sysml_core::ModelGraph;
use sysml_id::{CommitId, ProjectId};
use sysml_store::{InMemoryStore, SnapshotMeta, Store, StoreError};

/// Application state.
pub struct AppState {
    store: RwLock<InMemoryStore>,
}

impl AppState {
    /// Create new application state with an empty store.
    pub fn new() -> Self {
        AppState {
            store: RwLock::new(InMemoryStore::new()),
        }
    }

    /// Create application state with an existing store.
    pub fn with_store(store: InMemoryStore) -> Self {
        AppState {
            store: RwLock::new(store),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Health check response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Error response.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Project list response.
#[derive(Debug, Serialize)]
pub struct ProjectsResponse {
    pub projects: Vec<String>,
}

/// Commits list response.
#[derive(Debug, Serialize)]
pub struct CommitsResponse {
    pub commits: Vec<CommitInfo>,
}

/// Commit information.
#[derive(Debug, Serialize)]
pub struct CommitInfo {
    pub id: String,
    pub parent: Option<String>,
    pub message: String,
    pub timestamp: u64,
}

/// Request body for storing a model.
#[derive(Debug, Deserialize)]
pub struct StoreModelRequest {
    pub message: String,
    #[serde(default)]
    pub parent: Option<String>,
    pub model: serde_json::Value,
}

/// Response for storing a model.
#[derive(Debug, Serialize)]
pub struct StoreModelResponse {
    pub commit: String,
    pub project: String,
}

/// Health check endpoint.
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

/// List all projects.
async fn list_projects(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let store = state.store.read().await;
    match store.list_projects() {
        Ok(projects) => {
            let project_ids: Vec<String> = projects.iter().map(|p| p.as_str().to_string()).collect();
            (StatusCode::OK, Json(ProjectsResponse { projects: project_ids })).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

/// List commits for a project.
async fn list_commits(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    let store = state.store.read().await;
    let project = ProjectId::new(&project_id);

    match store.list_commits(&project) {
        Ok(commits) => {
            let commit_infos: Vec<CommitInfo> = commits
                .iter()
                .map(|c| CommitInfo {
                    id: c.commit.as_str().to_string(),
                    parent: c.parent.as_ref().map(|p| p.as_str().to_string()),
                    message: c.message.clone(),
                    timestamp: c.timestamp,
                })
                .collect();
            (StatusCode::OK, Json(CommitsResponse { commits: commit_infos })).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

/// Get a model snapshot.
async fn get_model(
    State(state): State<Arc<AppState>>,
    Path((project_id, commit_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let store = state.store.read().await;
    let project = ProjectId::new(&project_id);
    let commit = CommitId::new(&commit_id);

    match store.get_snapshot(&project, &commit) {
        Ok(Some(snapshot)) => {
            match snapshot.graph() {
                Ok(graph) => {
                    let json = to_json_string(&graph);
                    (
                        StatusCode::OK,
                        [("content-type", "application/json")],
                        json,
                    )
                        .into_response()
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: e.to_string(),
                    }),
                )
                    .into_response(),
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Snapshot not found".to_string(),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

/// Store a model snapshot.
async fn store_model(
    State(state): State<Arc<AppState>>,
    Path((project_id, commit_id)): Path<(String, String)>,
    Json(request): Json<StoreModelRequest>,
) -> impl IntoResponse {
    // Parse the model
    let model_json = serde_json::to_string(&request.model).unwrap_or_default();
    let graph = match from_json_str(&model_json) {
        Ok(g) => g,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Invalid model: {}", e),
                }),
            )
                .into_response();
        }
    };

    let project = ProjectId::new(&project_id);
    let commit = CommitId::new(&commit_id);

    let mut meta = SnapshotMeta::new(commit.clone(), request.message);
    if let Some(parent) = request.parent {
        meta = meta.with_parent(CommitId::new(parent));
    }

    let mut store = state.store.write().await;
    match store.put_snapshot(&project, meta, &graph) {
        Ok(()) => (
            StatusCode::CREATED,
            Json(StoreModelResponse {
                commit: commit_id,
                project: project_id,
            }),
        )
            .into_response(),
        Err(StoreError::Conflict(msg)) => (
            StatusCode::CONFLICT,
            Json(ErrorResponse { error: msg }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

/// Create the API router.
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/projects", get(list_projects))
        .route("/projects/:project_id/commits", get(list_commits))
        .route(
            "/projects/:project_id/commits/:commit_id/model",
            get(get_model).post(store_model),
        )
        .with_state(state)
}

/// Run the API server.
pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AppState::new());
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_endpoint() {
        let state = Arc::new(AppState::new());
        let app = create_router(state);

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn list_projects_empty() {
        let state = Arc::new(AppState::new());
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/projects")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_nonexistent_model() {
        let state = Arc::new(AppState::new());
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/projects/test/commits/v1/model")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
