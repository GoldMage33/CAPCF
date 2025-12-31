// Implementation of ProvenanceService using Sled for storage.

use async_trait::async_trait;
use sled::{Db, Tree};
use std::sync::Arc;
use tokio::sync::Mutex;
use provenance_layer::*;

pub struct SledProvenanceService {
    db: Arc<Db>,
    events_tree: Tree,
    artifacts_tree: Tree,
}

impl Clone for SledProvenanceService {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            events_tree: self.db.open_tree("events").unwrap(),
            artifacts_tree: self.db.open_tree("artifacts").unwrap(),
        }
    }
}

impl SledProvenanceService {
    pub async fn new() -> Result<Self, ProvenanceError> {
        let db = sled::open("provenance_db")?;
        let events_tree = db.open_tree("events")?;
        let artifacts_tree = db.open_tree("artifacts")?;
        Ok(Self {
            db: Arc::new(db),
            events_tree,
            artifacts_tree,
        })
    }
}

#[async_trait]
impl ProvenanceService for SledProvenanceService {
    async fn log_event(&mut self, event: Event) -> Result<(), ProvenanceError> {
        let key = event.id.to_string();
        let value = serde_json::to_vec(&event)?;
        self.events_tree.insert(key, value)?;
        self.events_tree.flush()?;
        Ok(())
    }

    async fn register_artifact(&mut self, artifact: Artifact) -> Result<(), ProvenanceError> {
        let key = artifact.id.to_string();
        let value = serde_json::to_vec(&artifact)?;
        self.artifacts_tree.insert(key, value)?;
        self.artifacts_tree.flush()?;
        Ok(())
    }

    async fn verify_signature(&self, data: &[u8], signature: &Signature) -> Result<bool, ProvenanceError> {
        // Mock verification: in real implementation, use ring to verify
        // For now, assume valid if signature is present
        Ok(signature.signature.len() > 0)
    }

    async fn get_lineage(&self, artifact_id: Uuid) -> Result<Lineage, ProvenanceError> {
        if let Some(value) = self.artifacts_tree.get(artifact_id.to_string())? {
            let artifact: Artifact = serde_json::from_slice(&value)?;
            Ok(Lineage {
                artifact_id,
                parent_ids: artifact.lineage,
                changes: "Version update".to_string(), // Placeholder
            })
        } else {
            Err(ProvenanceError::ArtifactNotFound)
        }
    }

    async fn get_events(&self, filter: Option<EventFilter>) -> Result<Vec<Event>, ProvenanceError> {
        let mut events = Vec::new();
        for result in self.events_tree.iter() {
            let (_key, value) = result?;
            let event: Event = serde_json::from_slice(&value)?;
            if let Some(f) = &filter {
                if let Some(et) = &f.event_type {
                    if event.event_type != *et {
                        continue;
                    }
                }
                if let Some(st) = f.start_time {
                    if event.timestamp < st {
                        continue;
                    }
                }
                if let Some(et) = f.end_time {
                    if event.timestamp > et {
                        continue;
                    }
                }
            }
            events.push(event);
        }
        Ok(events)
    }
}