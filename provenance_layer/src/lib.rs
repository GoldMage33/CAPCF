// Provenance Layer (PL): Handles append-only event logs, cryptographic signatures,
// artifact registry, and versioned lineage tracking.
// This layer ensures the integrity and traceability of all artifacts and events in the CAPCF system.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents an event in the append-only log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub data: serde_json::Value,
    pub signature: Option<Signature>,
}

/// Cryptographic signature for events and artifacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub signer: String,
    pub signature: Vec<u8>,
    pub algorithm: String,
}

/// Represents an artifact in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub content_hash: String,
    pub lineage: Vec<Uuid>, // References to previous versions or related artifacts
    pub metadata: serde_json::Value,
    pub registered_at: DateTime<Utc>,
}

/// Versioned lineage tracking for artifacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lineage {
    pub artifact_id: Uuid,
    pub parent_ids: Vec<Uuid>,
    pub changes: String,
}

/// Interface for the Provenance Service.
/// Provides methods to log events, register artifacts, verify signatures, and track lineage.
#[async_trait::async_trait]
pub trait ProvenanceService: Send + Sync {
    /// Logs an event to the append-only log.
    async fn log_event(&mut self, event: Event) -> Result<(), ProvenanceError>;

    /// Registers a new artifact in the registry.
    async fn register_artifact(&mut self, artifact: Artifact) -> Result<(), ProvenanceError>;

    /// Verifies the cryptographic signature of an event or artifact.
    async fn verify_signature(&self, data: &[u8], signature: &Signature) -> Result<bool, ProvenanceError>;

    /// Retrieves the lineage for a given artifact.
    async fn get_lineage(&self, artifact_id: Uuid) -> Result<Lineage, ProvenanceError>;

    /// Retrieves events from the log, optionally filtered.
    async fn get_events(&self, filter: Option<EventFilter>) -> Result<Vec<Event>, ProvenanceError>;
}

/// Filter for querying events.
#[derive(Debug, Clone)]
pub struct EventFilter {
    pub event_type: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
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
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}