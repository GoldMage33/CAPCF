// Cross-Domain Integration Layer (CDIL): Handles ethics, provenance, identity harmonization,
// interoperability schemas, and system-wide consistency checks.
// This layer integrates all CAPCF components seamlessly.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Harmonization of ethics, provenance, identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Harmonization {
    pub id: Uuid,
    pub domains: Vec<String>, // e.g., ["ethics", "provenance", "identity"]
    pub harmonized_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Interoperability schema (JSON-LD, gRPC, GraphQL).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteroperabilitySchema {
    pub id: Uuid,
    pub schema_type: String, // "json-ld", "grpc", "graphql"
    pub definition: serde_json::Value,
    pub version: String,
}

/// System-wide consistency check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyCheck {
    pub id: Uuid,
    pub check_type: String,
    pub result: bool,
    pub details: String,
    pub checked_at: DateTime<Utc>,
}

/// Interface for the Integration Service.
#[async_trait::async_trait]
pub trait IntegrationService: Send + Sync {
    /// Harmonizes data across domains.
    async fn harmonize(&self, domains: Vec<String>, data: serde_json::Value) -> Result<Harmonization, IntegrationError>;

    /// Validates against interoperability schema.
    async fn validate_schema(&self, schema: &InteroperabilitySchema, data: serde_json::Value) -> Result<bool, IntegrationError>;

    /// Performs system-wide consistency check.
    async fn check_consistency(&self, check_type: String) -> Result<ConsistencyCheck, IntegrationError>;

    /// Translates data between schemas.
    async fn translate(&self, from_schema: &InteroperabilitySchema, to_schema: &InteroperabilitySchema, data: serde_json::Value) -> Result<serde_json::Value, IntegrationError>;
}

/// Errors in the Integration Layer.
#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    #[error("Harmonization failed: {0}")]
    HarmonizationError(String),
    #[error("Schema validation failed")]
    SchemaValidationError,
    #[error("Consistency check failed")]
    ConsistencyError,
    #[error("Translation error: {0}")]
    TranslationError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}