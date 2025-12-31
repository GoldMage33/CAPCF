// Implementation of ProvenanceService using Sled for storage.
// Corresponds to State_PL = (E, A, G_P, B)

use async_trait::async_trait;
use sled::{Db, Tree};
use std::sync::Arc;
use tokio::sync::Mutex;
use provenance_layer::*;
use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519};
use ring::rand::SystemRandom;

pub struct SledProvenanceService {
    db: Arc<Db>,
    events_tree: Tree,
    artifacts_tree: Tree,
    blocks_tree: Tree,
    graph: Arc<Mutex<HashMap<Uuid, Vec<Uuid>>>>, // Simple adjacency list for G_P
    pending_events: Arc<Mutex<Vec<Event>>>,
    last_block_hash: Arc<Mutex<String>>,
}

impl Clone for SledProvenanceService {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            events_tree: self.db.open_tree("events").unwrap(),
            artifacts_tree: self.db.open_tree("artifacts").unwrap(),
            blocks_tree: self.db.open_tree("blocks").unwrap(),
            graph: self.graph.clone(),
            pending_events: self.pending_events.clone(),
            last_block_hash: self.last_block_hash.clone(),
        }
    }
}

impl SledProvenanceService {
    pub async fn new() -> Result<Self, ProvenanceError> {
        let db = sled::open("provenance_db")?;
        let events_tree = db.open_tree("events")?;
        let artifacts_tree = db.open_tree("artifacts")?;
        let blocks_tree = db.open_tree("blocks")?;
        let graph = Arc::new(Mutex::new(HashMap::new()));
        let pending_events = Arc::new(Mutex::new(Vec::new()));
        let last_block_hash = Arc::new(Mutex::new("genesis".to_string()));

        // Load existing graph from events
        let mut g = graph.lock().await;
        for result in events_tree.iter() {
            let (_key, value) = result?;
            let event: Event = serde_json::from_slice(&value)?;
            for &out in &event.out_artifacts {
                g.entry(out).or_insert(Vec::new());
                for &inp in &event.in_artifacts {
                    if let Some(children) = g.get_mut(&inp) {
                        if !children.contains(&out) {
                            children.push(out);
                        }
                    }
                }
            }
        }

        Ok(Self {
            db: Arc::new(db),
            events_tree,
            artifacts_tree,
            blocks_tree,
            graph,
            pending_events,
            last_block_hash,
        })
    }
}

#[async_trait]
impl ProvenanceService for SledProvenanceService {
    async fn log_event(&mut self, mut event: Event) -> Result<(), ProvenanceError> {
        event.id = Uuid::new_v4();
        event.timestamp = Utc::now();

        // Mock signature for now (use ring in production)
        event.signature = Some(Signature {
            signer: event.actor.clone(),
            signature: vec![0; 64], // Placeholder
            algorithm: "Ed25519".to_string(),
        });

        let key = event.id.to_string();
        let value = serde_json::to_vec(&event)?;
        self.events_tree.insert(key, value)?;
        self.events_tree.flush()?;

        // Update graph
        let mut g = self.graph.lock().await;
        for &out in &event.out_artifacts {
            g.entry(out).or_insert(Vec::new());
            for &inp in &event.in_artifacts {
                if let Some(children) = g.get_mut(&inp) {
                    if !children.contains(&out) {
                        children.push(out);
                    }
                }
            }
        }

        // Add to pending for block
        let mut pending = self.pending_events.lock().await;
        pending.push(event);

        Ok(())
    }

    async fn register_artifact(&mut self, mut artifact: Artifact) -> Result<(), ProvenanceError> {
        artifact.id = Uuid::new_v4();
        artifact.registered_at = Utc::now();

        let key = artifact.id.to_string();
        let value = serde_json::to_vec(&artifact)?;
        self.artifacts_tree.insert(key, value)?;
        self.artifacts_tree.flush()?;
        Ok(())
    }

    async fn verify_signature(&self, data: &[u8], signature: &Signature) -> Result<bool, ProvenanceError> {
        // Mock verification
        Ok(signature.signature.len() == 64)
    }

    async fn get_lineage(&self, artifact_id: Uuid) -> Result<Lineage, ProvenanceError> {
        let g = self.graph.lock().await;
        let parents = self.find_parents(&g, artifact_id).await;
        let children = g.get(&artifact_id).cloned().unwrap_or_default();
        Ok(Lineage {
            artifact_id,
            parent_ids: parents,
            child_ids: children,
            changes: "Version update".to_string(),
        })
    }

    async fn get_events(&self, filter: Option<EventFilter>) -> Result<Vec<Event>, ProvenanceError> {
        let mut events = Vec::new();
        for result in self.events_tree.iter() {
            let (_key, value) = result?;
            let event: Event = serde_json::from_slice(&value)?;
            if let Some(f) = &filter {
                if let Some(et) = &f.event_type {
                    if event.operation != *et {
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

    async fn create_block(&mut self) -> Result<Block, ProvenanceError> {
        let mut pending = self.pending_events.lock().await;
        if pending.is_empty() {
            return Err(ProvenanceError::BlockError);
        }

        let events = pending.drain(..).collect::<Vec<_>>();
        let block_id = Uuid::new_v4();
        let mut last_hash = self.last_block_hash.lock().await.clone();
        let block_data = serde_json::to_string(&events)?;
        let hash = format!("{:x}", ring::digest::digest(&ring::digest::SHA256, block_data.as_bytes()));

        let block = Block {
            id: block_id,
            events,
            hash: hash.clone(),
            previous_hash: last_hash.clone(),
            created_at: Utc::now(),
        };

        let key = block_id.to_string();
        let value = serde_json::to_vec(&block)?;
        self.blocks_tree.insert(key, value)?;
        self.blocks_tree.flush()?;

        *self.last_block_hash.lock().await = hash;

        Ok(block)
    }
}

impl SledProvenanceService {
    async fn find_parents(&self, g: &HashMap<Uuid, Vec<Uuid>>, artifact_id: Uuid) -> Vec<Uuid> {
        let mut parents = Vec::new();
        for (parent, children) in g.iter() {
            if children.contains(&artifact_id) {
                parents.push(*parent);
            }
        }
        parents
    }
}