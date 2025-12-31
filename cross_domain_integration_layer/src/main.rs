// REST API for the Cross-Domain Integration Layer.

use axum::{
    extract::Json,
    routing::post,
    Router,
};
use std::net::SocketAddr;
use serde_json::json;
use cross_domain_integration_layer::*;
use std::sync::Arc;

mod impl;

use impl::SimpleIntegrationService;

#[tokio::main]
async fn main() {
    let service = Arc::new(SimpleIntegrationService::new());

    let app = Router::new()
        .route("/harmonize", post({
            let service = service.clone();
            move |Json(payload): Json<(Vec<String>, serde_json::Value)>| async move {
                let (domains, data) = payload;
                match service.harmonize(domains, data).await {
                    Ok(harm) => (axum::http::StatusCode::OK, Json(harm)),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "harmonization failed"}))),
                }
            }
        }))
        .route("/validate_schema", post({
            let service = service.clone();
            move |Json(payload): Json<(InteroperabilitySchema, serde_json::Value)>| async move {
                let (schema, data) = payload;
                match service.validate_schema(&schema, data).await {
                    Ok(valid) => (axum::http::StatusCode::OK, Json(json!({"valid": valid}))),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "validation failed"}))),
                }
            }
        }))
        .route("/check_consistency", post({
            let service = service.clone();
            move |Json(check_type): Json<String>| async move {
                match service.check_consistency(check_type).await {
                    Ok(check) => (axum::http::StatusCode::OK, Json(check)),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "check failed"}))),
                }
            }
        }))
        .route("/translate", post({
            let service = service.clone();
            move |Json(payload): Json<(InteroperabilitySchema, InteroperabilitySchema, serde_json::Value)>| async move {
                let (from_schema, to_schema, data) = payload;
                match service.translate(&from_schema, &to_schema, data).await {
                    Ok(translated) => (axum::http::StatusCode::OK, Json(translated)),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "translation failed"}))),
                }
            }
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3003));
    println!("Integration Layer listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}