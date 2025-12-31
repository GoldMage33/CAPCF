// Example implementation of the Provenance Layer service.
// This provides a REST API for logging events and registering artifacts.

use axum::{
    extract::{Json, Path},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use serde_json::json;
use provenance_layer::*;

mod provenance_impl;

use provenance_impl::SledProvenanceService;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let service = Arc::new(SledProvenanceService::new().await.expect("Failed to initialize service"));

    let app = Router::new()
        .route("/events", post({
            let service = service.clone();
            move |Json(payload): Json<Event>| async move {
                let mut svc = service.as_ref().clone();
                match svc.log_event(payload).await {
                    Ok(_) => (axum::http::StatusCode::OK, Json(json!({"status": "event logged"}))),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "failed to log event"}))),
                }
            }
        }))
        .route("/artifacts", post({
            let service = service.clone();
            move |Json(payload): Json<Artifact>| async move {
                let mut svc = service.as_ref().clone();
                match svc.register_artifact(payload).await {
                    Ok(_) => (axum::http::StatusCode::OK, Json(json!({"status": "artifact registered"}))),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "failed to register artifact"}))),
                }
            }
        }))
        .route("/events", get({
            let service = service.clone();
            move || async move {
                let svc = service.as_ref();
                match svc.get_events(None).await {
                    Ok(events) => (axum::http::StatusCode::OK, Json(events)),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
                }
            }
        }))
        .route("/events/:id", get({
            let service = service.clone();
            move |Path(id): Path<String>| async move {
                let svc = service.as_ref();
                match svc.get_events(Some(id)).await {
                    Ok(events) => {
                        if events.is_empty() {
                            (axum::http::StatusCode::NOT_FOUND, Json(json!({"error": "event not found"})))
                        } else {
                            (axum::http::StatusCode::OK, Json(events[0].clone()))
                        }
                    },
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "failed to get event"}))),
                }
            }
        }))
        .route("/artifacts/:id", get({
            let service = service.clone();
            move |Path(id): Path<String>| async move {
                let svc = service.as_ref();
                match svc.get_artifacts(Some(id)).await {
                    Ok(artifacts) => {
                        if artifacts.is_empty() {
                            (axum::http::StatusCode::NOT_FOUND, Json(json!({"error": "artifact not found"})))
                        } else {
                            (axum::http::StatusCode::OK, Json(artifacts[0].clone()))
                        }
                    },
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "failed to get artifact"}))),
                }
            }
        }))
        .route("/artifacts/:id/lineage", get({
            let service = service.clone();
            move |Path(id): Path<String>| async move {
                let svc = service.as_ref();
                match svc.get_lineage(id).await {
                    Ok(lineage) => (axum::http::StatusCode::OK, Json(lineage)),
                    Err(_) => (axum::http::StatusCode::NOT_FOUND, Json(json!({"error": "lineage not found"}))),
                }
            }
        }))
        .route("/blocks", get({
            let service = service.clone();
            move || async move {
                let svc = service.as_ref();
                match svc.get_blocks().await {
                    Ok(blocks) => (axum::http::StatusCode::OK, Json(blocks)),
                    Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
                }
            }
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Provenance Layer listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}