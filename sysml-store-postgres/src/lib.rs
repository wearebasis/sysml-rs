//! # sysml-store-postgres
//!
//! PostgreSQL storage backend for SysML v2 model snapshots.
//!
//! This crate provides a PostgreSQL implementation of the Store trait.
//! When the `postgres` feature is disabled, it provides an in-memory stub.
//!
//! ## Features
//!
//! - `postgres`: Enable PostgreSQL support via sqlx

pub use sysml_store::{Snapshot, SnapshotMeta, Store, StoreError};

// Re-export in-memory store as fallback
pub use sysml_store::InMemoryStore;

#[cfg(feature = "postgres")]
mod postgres_impl {
    use super::*;
    use sysml_canon::{from_json_str, to_json_string};
    use sysml_core::ModelGraph;
    use sysml_id::{CommitId, ProjectId};
    use sqlx::postgres::PgPool;

    /// PostgreSQL-backed store.
    pub struct PostgresStore {
        pool: PgPool,
    }

    impl PostgresStore {
        /// Create a new PostgreSQL store.
        pub async fn new(database_url: &str) -> Result<Self, StoreError> {
            let pool = PgPool::connect(database_url)
                .await
                .map_err(|e| StoreError::DatabaseError(e.to_string()))?;

            Ok(PostgresStore { pool })
        }

        /// Initialize the database schema.
        pub async fn init_schema(&self) -> Result<(), StoreError> {
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS snapshots (
                    project_id TEXT NOT NULL,
                    commit_id TEXT NOT NULL,
                    parent_id TEXT,
                    message TEXT NOT NULL,
                    timestamp BIGINT NOT NULL,
                    data JSONB NOT NULL,
                    PRIMARY KEY (project_id, commit_id)
                )
                "#,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| StoreError::DatabaseError(e.to_string()))?;

            sqlx::query(
                r#"
                CREATE INDEX IF NOT EXISTS idx_snapshots_project
                ON snapshots (project_id, timestamp DESC)
                "#,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| StoreError::DatabaseError(e.to_string()))?;

            Ok(())
        }

        /// Store a snapshot asynchronously.
        pub async fn put_snapshot_async(
            &self,
            project: &ProjectId,
            meta: SnapshotMeta,
            graph: &ModelGraph,
        ) -> Result<(), StoreError> {
            let data = to_json_string(graph);

            sqlx::query(
                r#"
                INSERT INTO snapshots (project_id, commit_id, parent_id, message, timestamp, data)
                VALUES ($1, $2, $3, $4, $5, $6::jsonb)
                "#,
            )
            .bind(project.as_str())
            .bind(meta.commit.as_str())
            .bind(meta.parent.as_ref().map(|p| p.as_str().to_string()))
            .bind(&meta.message)
            .bind(meta.timestamp as i64)
            .bind(&data)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("duplicate key") {
                    StoreError::Conflict(format!("commit {} already exists", meta.commit))
                } else {
                    StoreError::DatabaseError(e.to_string())
                }
            })?;

            Ok(())
        }

        /// Get a snapshot asynchronously.
        pub async fn get_snapshot_async(
            &self,
            project: &ProjectId,
            commit: &CommitId,
        ) -> Result<Option<Snapshot>, StoreError> {
            let row: Option<(String, Option<String>, String, i64, String)> = sqlx::query_as(
                r#"
                SELECT commit_id, parent_id, message, timestamp, data::text
                FROM snapshots
                WHERE project_id = $1 AND commit_id = $2
                "#,
            )
            .bind(project.as_str())
            .bind(commit.as_str())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| StoreError::DatabaseError(e.to_string()))?;

            match row {
                Some((commit_id, parent_id, message, timestamp, data)) => {
                    let mut meta = SnapshotMeta::new(CommitId::new(commit_id), message)
                        .with_timestamp(timestamp as u64);
                    if let Some(parent) = parent_id {
                        meta = meta.with_parent(CommitId::new(parent));
                    }
                    Ok(Some(Snapshot { meta, data }))
                }
                None => Ok(None),
            }
        }

        /// Get the latest commit ID asynchronously.
        pub async fn latest_async(
            &self,
            project: &ProjectId,
        ) -> Result<Option<CommitId>, StoreError> {
            let row: Option<(String,)> = sqlx::query_as(
                r#"
                SELECT commit_id
                FROM snapshots
                WHERE project_id = $1
                ORDER BY timestamp DESC
                LIMIT 1
                "#,
            )
            .bind(project.as_str())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| StoreError::DatabaseError(e.to_string()))?;

            Ok(row.map(|(id,)| CommitId::new(id)))
        }
    }
}

#[cfg(feature = "postgres")]
pub use postgres_impl::PostgresStore;

/// Create an in-memory store (fallback when postgres feature is disabled).
pub fn create_in_memory_store() -> InMemoryStore {
    InMemoryStore::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_core::{Element, ElementKind, ModelGraph};
    use sysml_id::{CommitId, ProjectId};

    #[test]
    fn in_memory_store_basic() {
        let mut store = create_in_memory_store();
        let project = ProjectId::new("test");
        let commit = CommitId::new("v1");

        let mut graph = ModelGraph::new();
        graph.add_element(Element::new_with_kind(ElementKind::Package).with_name("Test"));

        let meta = SnapshotMeta::new(commit.clone(), "Test commit");
        store.put_snapshot(&project, meta, &graph).unwrap();

        let snapshot = store.get_snapshot(&project, &commit).unwrap().unwrap();
        let restored = snapshot.graph().unwrap();
        assert_eq!(restored.element_count(), 1);
    }
}
