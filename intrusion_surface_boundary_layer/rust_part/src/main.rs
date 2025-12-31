// REST API for the Intrusion Surface & Boundary Layer.

use axum::{
    extract::Json,
    routing::post,
    Router,
};
use std::net::SocketAddr;
use serde_json::json;
use intrusion_surface_boundary_layer::*;
use std::sync::Arc;

mod impl;

use impl::SimpleIntrusionService;

#[tokio::main]
async fn main() {
    let service = Arc::new(SimpleIntrusionService::new());

    let app = Router::new()
        .route("/detect_mimicry", post({
            let service = service.clone();
            move |Json(payload): Json<(String, String)>| async move {
                let (target, mimic) = payload;
                match service.detect_mimicry(target, mimic).await {
                    Ok(detection) => (axum::http::StatusCode::OK, Json(detection)),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "detection failed"}))),
                }
            }
        }))
        .route("/check_identity", post({
            let service = service.clone();
            move |Json(user_id): Json<String>| async move {
                match service.check_identity(user_id).await {
                    Ok(check) => (axum::http::StatusCode::OK, Json(check)),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "check failed"}))),
                }
            }
        }))
        .route("/enforce_boundary", post({
            let service = service.clone();
            move |Json(payload): Json<BoundaryEnforcement>| async move {
                let mut svc = service.as_ref().clone();
                match svc.enforce_boundary(payload).await {
                    Ok(_) => (axum::http::StatusCode::OK, Json(json!({"status": "boundary enforced"}))),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "enforcement failed"}))),
                }
            }
        }))
        .route("/detect_derivative", post({
            let service = service.clone();
            move |Json(payload): Json<(String, String)>| async move {
                let (original_str, derivative_str) = payload;
                let original = uuid::Uuid::parse_str(&original_str).unwrap();
                let derivative = uuid::Uuid::parse_str(&derivative_str).unwrap();
                match service.detect_derivative(original, derivative).await {
                    Ok(detection) => (axum::http::StatusCode::OK, Json(detection)),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "detection failed"}))),
                }
            }
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    println!("Intrusion Layer listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}