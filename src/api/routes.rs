// src/api/routes.rs
use axum::{routing::{get, post}, Router};
use super::handlers;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/api/v1/analyze", post(handlers::analyze_code))
        .route("/api/v1/scan", post(handlers::scan_url))
        .route("/api/v1/audit", post(handlers::full_audit))
        .route("/api/v1/report/:report_id", get(handlers::get_report))
        .route("/api/v1/rules", get(handlers::list_rules))
        .route("/api/v1/info", get(handlers::engine_info))
}
