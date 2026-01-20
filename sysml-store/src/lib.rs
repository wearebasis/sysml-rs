//! # sysml-store
//!
//! Storage trait and types for SysML v2 model snapshots.
//!
//! This crate defines the interface for storing and retrieving model
//! snapshots with version control.

use std::collections::HashMap;
use sysml_canon::{from_json_str, to_json_string};
use sysml_core::ModelGraph;
use sysml_id::{CommitId, ProjectId};
use thiserror::Error;

/// Errors that can occur during storage operations.
#[derive(Debug, Error)]
pub enum StoreError {
    /// The requested project was not found.
    #[error("project not found: {0}")]
    ProjectNotFound(String),

    /// The requested commit was not found.
    #[error("commit not found: {0}")]
    CommitNotFound(String),

    /// Serialization failed.
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// Deserialization failed.
    #[error("deserialization error: {0}")]
    DeserializationError(String),

    /// Database error.
    #[error("database error: {0}")]
    DatabaseError(String),

    /// Conflict (e.g., commit already exists).
    #[error("conflict: {0}")]
    Conflict(String),
}

/// Metadata about a snapshot.
#[derive(Debug, Clone)]
pub struct SnapshotMeta {
    /// The commit ID.
    pub commit: CommitId,
    /// The parent commit ID (None for initial commit).
    pub parent: Option<CommitId>,
    /// Commit message.
    pub message: String,
    /// Timestamp (Unix epoch seconds).
    pub timestamp: u64,
}

impl SnapshotMeta {
    /// Create new snapshot metadata.
    pub fn new(commit: CommitId, message: impl Into<String>) -> Self {
        SnapshotMeta {
            commit,
            parent: None,
            message: message.into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }

    /// Set the parent commit.
    pub fn with_parent(mut self, parent: CommitId) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Set the timestamp.
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }
}

/// A stored snapshot containing metadata and model data.
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// Metadata about this snapshot.
    pub meta: SnapshotMeta,
    /// The serialized model data (JSON).
    pub data: String,
}

impl Snapshot {
    /// Create a new snapshot from a model graph.
    pub fn new(meta: SnapshotMeta, graph: &ModelGraph) -> Self {
        Snapshot {
            meta,
            data: to_json_string(graph),
        }
    }

    /// Deserialize the model graph.
    pub fn graph(&self) -> Result<ModelGraph, StoreError> {
        from_json_str(&self.data).map_err(|e| StoreError::DeserializationError(e.to_string()))
    }
}

/// Trait for model storage backends.
pub trait Store {
    /// Store a model snapshot.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID
    /// * `commit` - The commit ID for this snapshot
    /// * `graph` - The model graph to store
    /// * `meta` - Snapshot metadata
    ///
    /// # Returns
    ///
    /// Ok(()) on success, or a StoreError on failure.
    fn put_snapshot(
        &mut self,
        project: &ProjectId,
        meta: SnapshotMeta,
        graph: &ModelGraph,
    ) -> Result<(), StoreError>;

    /// Retrieve a model snapshot.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID
    /// * `commit` - The commit ID to retrieve
    ///
    /// # Returns
    ///
    /// The model graph if found, None otherwise.
    fn get_snapshot(
        &self,
        project: &ProjectId,
        commit: &CommitId,
    ) -> Result<Option<Snapshot>, StoreError>;

    /// Get the latest commit ID for a project.
    fn latest(&self, project: &ProjectId) -> Result<Option<CommitId>, StoreError>;

    /// List all commits for a project (most recent first).
    fn list_commits(&self, project: &ProjectId) -> Result<Vec<SnapshotMeta>, StoreError>;

    /// List all projects.
    fn list_projects(&self) -> Result<Vec<ProjectId>, StoreError>;
}

/// An in-memory store implementation.
#[derive(Debug, Default)]
pub struct InMemoryStore {
    /// Snapshots indexed by (project, commit).
    snapshots: HashMap<(String, String), Snapshot>,
    /// Latest commit for each project.
    latest: HashMap<String, CommitId>,
    /// All commits for each project (in order).
    commits: HashMap<String, Vec<SnapshotMeta>>,
}

impl InMemoryStore {
    /// Create a new empty in-memory store.
    pub fn new() -> Self {
        InMemoryStore {
            snapshots: HashMap::new(),
            latest: HashMap::new(),
            commits: HashMap::new(),
        }
    }
}

impl Store for InMemoryStore {
    fn put_snapshot(
        &mut self,
        project: &ProjectId,
        meta: SnapshotMeta,
        graph: &ModelGraph,
    ) -> Result<(), StoreError> {
        let project_key = project.as_str().to_string();
        let commit_key = meta.commit.as_str().to_string();
        let key = (project_key.clone(), commit_key);

        if self.snapshots.contains_key(&key) {
            return Err(StoreError::Conflict(format!(
                "commit {} already exists",
                meta.commit
            )));
        }

        let snapshot = Snapshot::new(meta.clone(), graph);
        self.snapshots.insert(key, snapshot);
        self.latest.insert(project_key.clone(), meta.commit.clone());

        self.commits
            .entry(project_key)
            .or_default()
            .push(meta);

        Ok(())
    }

    fn get_snapshot(
        &self,
        project: &ProjectId,
        commit: &CommitId,
    ) -> Result<Option<Snapshot>, StoreError> {
        let key = (project.as_str().to_string(), commit.as_str().to_string());
        Ok(self.snapshots.get(&key).cloned())
    }

    fn latest(&self, project: &ProjectId) -> Result<Option<CommitId>, StoreError> {
        Ok(self.latest.get(project.as_str()).cloned())
    }

    fn list_commits(&self, project: &ProjectId) -> Result<Vec<SnapshotMeta>, StoreError> {
        Ok(self
            .commits
            .get(project.as_str())
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .rev()
            .collect())
    }

    fn list_projects(&self) -> Result<Vec<ProjectId>, StoreError> {
        Ok(self
            .commits
            .keys()
            .map(|k| ProjectId::new(k.clone()))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_core::{Element, ElementKind};

    fn create_test_graph() -> ModelGraph {
        let mut graph = ModelGraph::new();
        let elem = Element::new_with_kind(ElementKind::Package).with_name("Test");
        graph.add_element(elem);
        graph
    }

    #[test]
    fn in_memory_store_put_get() {
        let mut store = InMemoryStore::new();
        let project = ProjectId::new("test-project");
        let commit = CommitId::new("v1");
        let graph = create_test_graph();
        let meta = SnapshotMeta::new(commit.clone(), "Initial commit");

        store.put_snapshot(&project, meta, &graph).unwrap();

        let snapshot = store.get_snapshot(&project, &commit).unwrap().unwrap();
        let restored = snapshot.graph().unwrap();

        assert_eq!(graph.element_count(), restored.element_count());
    }

    #[test]
    fn in_memory_store_latest() {
        let mut store = InMemoryStore::new();
        let project = ProjectId::new("test-project");
        let graph = create_test_graph();

        let meta1 = SnapshotMeta::new(CommitId::new("v1"), "First");
        store.put_snapshot(&project, meta1, &graph).unwrap();

        let meta2 = SnapshotMeta::new(CommitId::new("v2"), "Second")
            .with_parent(CommitId::new("v1"));
        store.put_snapshot(&project, meta2, &graph).unwrap();

        let latest = store.latest(&project).unwrap().unwrap();
        assert_eq!(latest.as_str(), "v2");
    }

    #[test]
    fn in_memory_store_list_commits() {
        let mut store = InMemoryStore::new();
        let project = ProjectId::new("test-project");
        let graph = create_test_graph();

        store
            .put_snapshot(
                &project,
                SnapshotMeta::new(CommitId::new("v1"), "First"),
                &graph,
            )
            .unwrap();
        store
            .put_snapshot(
                &project,
                SnapshotMeta::new(CommitId::new("v2"), "Second"),
                &graph,
            )
            .unwrap();

        let commits = store.list_commits(&project).unwrap();
        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].commit.as_str(), "v2"); // Most recent first
    }

    #[test]
    fn in_memory_store_list_projects() {
        let mut store = InMemoryStore::new();
        let graph = create_test_graph();

        store
            .put_snapshot(
                &ProjectId::new("project-a"),
                SnapshotMeta::new(CommitId::new("v1"), "A"),
                &graph,
            )
            .unwrap();
        store
            .put_snapshot(
                &ProjectId::new("project-b"),
                SnapshotMeta::new(CommitId::new("v1"), "B"),
                &graph,
            )
            .unwrap();

        let projects = store.list_projects().unwrap();
        assert_eq!(projects.len(), 2);
    }

    #[test]
    fn in_memory_store_conflict() {
        let mut store = InMemoryStore::new();
        let project = ProjectId::new("test-project");
        let graph = create_test_graph();

        let meta = SnapshotMeta::new(CommitId::new("v1"), "First");
        store.put_snapshot(&project, meta.clone(), &graph).unwrap();

        let result = store.put_snapshot(&project, meta, &graph);
        assert!(matches!(result, Err(StoreError::Conflict(_))));
    }

    #[test]
    fn snapshot_meta_with_parent() {
        let meta = SnapshotMeta::new(CommitId::new("v2"), "Second")
            .with_parent(CommitId::new("v1"))
            .with_timestamp(1234567890);

        assert_eq!(meta.parent.unwrap().as_str(), "v1");
        assert_eq!(meta.timestamp, 1234567890);
    }
}
