// Implementation of the Governance Service using Sled for persistence.

use async_trait::async_trait;
use sled::Db;
use serde_json;
use crate::{GovernanceService, GovernanceError, Right, License, ConsentEnvelope, Attribution, Policy};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct SledGovernanceService {
    db: Db,
}

impl SledGovernanceService {
    pub async fn new() -> Result<Self, GovernanceError> {
        let db = sled::open("governance_db").map_err(|e| GovernanceError::DatabaseError(e.to_string()))?;
        Ok(Self { db })
    }
}

#[async_trait]
impl GovernanceService for SledGovernanceService {
    async fn enforce_policy(&self, policy: &Policy, context: serde_json::Value) -> Result<bool, GovernanceError> {
        // Simple enforcement: check if context matches rules
        // In real impl, use OPA or similar
        Ok(true) // Placeholder
    }

    async fn resolve_attribution(&self, artifact_id: Uuid) -> Result<Vec<Attribution>, GovernanceError> {
        let key = format!("attribution_{}", artifact_id);
        if let Some(data) = self.db.get(key).map_err(|e| GovernanceError::DatabaseError(e.to_string()))? {
            let attributions: Vec<Attribution> = serde_json::from_slice(&data)?;
            Ok(attributions)
        } else {
            Ok(vec![])
        }
    }

    async fn grant_right(&mut self, right: Right) -> Result<(), GovernanceError> {
        let key = format!("right_{}", right.id);
        let data = serde_json::to_vec(&right)?;
        self.db.insert(key, data).map_err(|e| GovernanceError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn check_consent(&self, user_id: String, purpose: String) -> Result<bool, GovernanceError> {
        let key = format!("consent_{}_{}", user_id, purpose);
        if let Some(data) = self.db.get(key).map_err(|e| GovernanceError::DatabaseError(e.to_string()))? {
            let envelope: ConsentEnvelope = serde_json::from_slice(&data)?;
            Ok(envelope.revoked_at.is_none())
        } else {
            Err(GovernanceError::ConsentNotFound)
        }
    }

    async fn issue_license(&mut self, license: License) -> Result<(), GovernanceError> {
        let key = format!("license_{}", license.id);
        let data = serde_json::to_vec(&license)?;
        self.db.insert(key, data).map_err(|e| GovernanceError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}