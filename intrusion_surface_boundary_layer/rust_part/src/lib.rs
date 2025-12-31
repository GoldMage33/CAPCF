// Intrusion Surface & Boundary Layer (IL): Handles stylistic mimicry detection,
// identity integrity checks, narrative boundary enforcement, and unauthorized derivative detection.
// This layer protects the system from malicious intrusions and maintains boundaries in the CAPCF framework.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Detection of stylistic mimicry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MimicryDetection {
    pub id: Uuid,
    pub target: String,
    pub mimic: String,
    pub confidence: f64,
    pub detected_at: DateTime<Utc>,
}

/// Identity integrity check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityCheck {
    pub id: Uuid,
    pub user_id: String,
    pub integrity_score: f64,
    pub checked_at: DateTime<Utc>,
}

/// Narrative boundary enforcement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundaryEnforcement {
    pub id: Uuid,
    pub narrative_id: Uuid,
    pub boundary: String,
    pub enforced_at: DateTime<Utc>,
}

/// Unauthorized derivative detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivativeDetection {
    pub id: Uuid,
    pub original: Uuid,
    pub derivative: Uuid,
    pub similarity: f64,
    pub detected_at: DateTime<Utc>,
}

/// Interface for the Intrusion Service.
#[async_trait::async_trait]
pub trait IntrusionService: Send + Sync {
    /// Detects stylistic mimicry.
    async fn detect_mimicry(&self, target: String, mimic: String) -> Result<MimicryDetection, IntrusionError>;

    /// Checks identity integrity.
    async fn check_identity(&self, user_id: String) -> Result<IdentityCheck, IntrusionError>;

    /// Enforces narrative boundary.
    async fn enforce_boundary(&mut self, boundary: BoundaryEnforcement) -> Result<(), IntrusionError>;

    /// Detects unauthorized derivatives.
    async fn detect_derivative(&self, original: Uuid, derivative: Uuid) -> Result<DerivativeDetection, IntrusionError>;
}

/// Errors in the Intrusion Layer.
#[derive(Debug, thiserror::Error)]
pub enum IntrusionError {
    #[error("Detection failed: {0}")]
    DetectionError(String),
    #[error("Integrity check failed")]
    IntegrityError,
    #[error("Boundary violation")]
    BoundaryViolation,
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}