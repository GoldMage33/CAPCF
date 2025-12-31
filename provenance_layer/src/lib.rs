// Provenance Layer (PL): Handles append-only event logs, cryptographic signatures,
// artifact registry, and versioned lineage tracking.
// This layer ensures the integrity and traceability of all artifacts and events in the CAPCF system.
// Corresponds to formal model: PL with state (E, A, G_P, B)

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Represents an event in the append-only log.
/// Corresponds to e = (id_e, t_e, actor_e, in_e, op_e, out_e, ctx_e, sig_e)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub actor: String,
    pub in_artifacts: Vec<Uuid>,
    pub operation: String,
    pub out_artifacts: Vec<Uuid>,
    pub context: serde_json::Value,
    pub signature: Option<Signature>,
}

/// Cryptographic signature for events and artifacts.
/// Uses ring for ECDSA or similar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub signer: String,
    pub signature: Vec<u8>,
    pub algorithm: String,
}

/// Represents an artifact in the registry.
/// Corresponds to a = (id_a, h_a, meta_a)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub content_hash: String,
    pub metadata: serde_json::Value,
    pub registered_at: DateTime<Utc>,
}

/// Versioned lineage tracking for artifacts.
/// Corresponds to lineage queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lineage {
    pub artifact_id: Uuid,
    pub parent_ids: Vec<Uuid>,
    pub child_ids: Vec<Uuid>,
    pub changes: String,
}

/// Block for tamper-evidence.
/// Corresponds to b_i = (id_{b_i}, events_{b_i}, h_{b_i})
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: Uuid,
    pub events: Vec<Event>,
    pub hash: String,
    pub previous_hash: String,
    pub created_at: DateTime<Utc>,
}

/// Filter for querying events.
/// Corresponds to EventFilter in formal model.
#[derive(Debug, Clone)]
pub struct EventFilter {
    pub event_type: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

/// Interface for the Provenance Service.
/// Corresponds to ProvenanceService in formal model.
#[async_trait::async_trait]
pub trait ProvenanceService: Send + Sync {
    /// Logs an event to the append-only log.
    /// Corresponds to createEvent + appendEvent
    async fn log_event(&mut self, event: Event) -> Result<(), ProvenanceError>;

    /// Registers a new artifact in the registry.
    async fn register_artifact(&mut self, artifact: Artifact) -> Result<(), ProvenanceError>;

    /// Verifies the cryptographic signature.
    async fn verify_signature(&self, data: &[u8], signature: &Signature) -> Result<bool, ProvenanceError>;

    /// Retrieves the lineage for a given artifact.
    /// Corresponds to lineage^{-} and lineage^{+}
    async fn get_lineage(&self, artifact_id: Uuid) -> Result<Lineage, ProvenanceError>;

    /// Retrieves events from the log, optionally filtered.
    async fn get_events(&self, filter: Option<EventFilter>) -> Result<Vec<Event>, ProvenanceError>;

    /// Creates a new block with pending events.
    /// Corresponds to block creation for tamper-evidence.
    async fn create_block(&mut self) -> Result<Block, ProvenanceError>;
}

/// Errors that can occur in the Provenance Layer.
#[derive(Debug, thiserror::Error)]
pub enum ProvenanceError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Signature verification failed")]
    SignatureError,
    #[error("Artifact not found")]
    ArtifactNotFound,
    #[error("Block creation failed")]
    BlockError,
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}