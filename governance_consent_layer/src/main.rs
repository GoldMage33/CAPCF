// REST API for the Governance & Consent Layer.

use axum::{
    extract::{Json, Path},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use serde_json::json;
use governance_consent_layer::*;
use std::sync::Arc;

mod impl;

use impl::SledGovernanceService;

#[tokio::main]
async fn main() {
    let service = Arc::new(SledGovernanceService::new().await.expect("Failed to initialize service"));

    let app = Router::new()
        .route("/policies", post({
            let service = service.clone();
            move |Json(payload): Json<Policy>| async move {
                // For simplicity, enforce policy
                match service.enforce_policy(&payload, json!({})).await {
                    Ok(result) => (axum::http::StatusCode::OK, Json(json!({"enforced": result}))),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "policy enforcement failed"}))),
                }
            }
        }))
        .route("/attributions/:artifact_id", get({
            let service = service.clone();
            move |Path(artifact_id): Path<String>| async move {
                let id = uuid::Uuid::parse_str(&artifact_id).unwrap();
                match service.resolve_attribution(id).await {
                    Ok(attributions) => (axum::http::StatusCode::OK, Json(attributions)),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
                }
            }
        }))
        .route("/rights", post({
            let service = service.clone();
            move |Json(payload): Json<Right>| async move {
                let mut svc = service.as_ref().clone();
                match svc.grant_right(payload).await {
                    Ok(_) => (axum::http::StatusCode::OK, Json(json!({"status": "right granted"}))),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "failed to grant right"}))),
                }
            }
        }))
        .route("/consent", post({
            let service = service.clone();
            move |Json(payload): Json<ConsentEnvelope>| async move {
                // For check_consent, but since it's check, perhaps GET
                // Placeholder
                (axum::http::StatusCode::OK, Json(json!({"status": "consent checked"})))
            }
        }))
        .route("/licenses", post({
            let service = service.clone();
            move |Json(payload): Json<License>| async move {
                let mut svc = service.as_ref().clone();
                match svc.issue_license(payload).await {
                    Ok(_) => (axum::http::StatusCode::OK, Json(json!({"status": "license issued"}))),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "failed to issue license"}))),
                }
            }
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Governance Layer listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}