// src/api/handlers.rs
use axum::{Json, extract::Path};
use super::models::*;
use serde_json::json;

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        uptime_seconds: 0,
    })
}

pub async fn analyze_code(Json(_req): Json<AnalyzeRequest>) -> Json<AnalysisResponse> {
    // Stub
    Json(AnalysisResponse {
        report_id: "stub-id".into(),
        status: "completed".into(),
        score: 100.0,
        classification: "CONFORME".into(),
        total_violations: 0,
        violations_by_severity: ViolationSummary { critical: 0, high: 0, medium: 0, low: 0 },
        violations: vec![],
        categories: vec![],
        report_html: None,
        report_pdf_base64: None,
        disclaimer: "AVISO LEGAL...".into(),
        engine_version: env!("CARGO_PKG_VERSION").into(),
        rules_version: "1.0.0".into(),
        analyzed_at: chrono::Utc::now().to_rfc3339(),
        valid_until: (chrono::Utc::now() + chrono::Duration::days(90)).to_rfc3339(),
    })
}

pub async fn scan_url(Json(_req): Json<ScanRequest>) -> Json<serde_json::Value> {
    // Stub
    Json(json!({"status": "scan_completed"}))
}

pub async fn full_audit(Json(_req): Json<AuditRequest>) -> Json<serde_json::Value> {
    // Stub
    Json(json!({"status": "audit_completed"}))
}

pub async fn get_report(Path(report_id): Path<String>) -> Json<serde_json::Value> {
    // Stub
    Json(json!({"report_id": report_id}))
}

pub async fn list_rules() -> Json<serde_json::Value> {
    // Stub
    Json(json!({"rules": []}))
}

pub async fn engine_info() -> Json<EngineInfoResponse> {
    Json(EngineInfoResponse {
        version: env!("CARGO_PKG_VERSION").into(),
        supported_languages: vec!["Rust".into(), "JavaScript".into(), "Python".into()],
        supported_jurisdictions: vec!["ECA_Digital".into(), "LGPD".into()],
        total_rules: 18,
    })
}
