// Governance & Consent Layer (GL): Manages rights and licensing engine, consent envelopes,
// attribution resolver, and policy enforcement.
// This layer ensures ethical and legal compliance in the CAPCF system.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents a right associated with an artifact or user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Right {
    pub id: Uuid,
    pub holder: String,
    pub resource: String,
    pub permissions: Vec<String>, // e.g., ["read", "modify", "distribute"]
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// License for using artifacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub id: Uuid,
    pub artifact_id: Uuid,
    pub licensee: String,
    pub terms: String,
    pub issued_at: DateTime<Utc>,
}

/// Consent envelope for user agreements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentEnvelope {
    pub id: Uuid,
    pub user_id: String,
    pub purpose: String,
    pub data: serde_json::Value,
    pub consented_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

/// Attribution for creators or contributors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribution {
    pub id: Uuid,
    pub artifact_id: Uuid,
    pub contributor: String,
    pub role: String,
    pub contribution: String,
}

/// Policy for enforcement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: Uuid,
    pub name: String,
    pub rules: serde_json::Value, // OPA-style rules
    pub active: bool,
}

/// Interface for the Governance Service.
#[async_trait::async_trait]
pub trait GovernanceService: Send + Sync {
    /// Enforces a policy on a given context.
    async fn enforce_policy(&self, policy: &Policy, context: serde_json::Value) -> Result<bool, GovernanceError>;

    /// Resolves attribution for an artifact.
    async fn resolve_attribution(&self, artifact_id: Uuid) -> Result<Vec<Attribution>, GovernanceError>;

    /// Grants a right.
    async fn grant_right(&mut self, right: Right) -> Result<(), GovernanceError>;

    /// Checks consent for a user and purpose.
    async fn check_consent(&self, user_id: String, purpose: String) -> Result<bool, GovernanceError>;

    /// Issues a license.
    async fn issue_license(&mut self, license: License) -> Result<(), GovernanceError>;
}

/// Errors in the Governance Layer.
#[derive(Debug, thiserror::Error)]
pub enum GovernanceError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Policy violation")]
    PolicyViolation,
    #[error("Consent not found")]
    ConsentNotFound,
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}