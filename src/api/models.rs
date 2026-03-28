// src/api/models.rs
use serde::{Deserialize, Serialize};

// === REQUESTS ===

#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub source_code: Option<String>,
    pub language: Option<String>,
    pub filename: Option<String>,
    pub jurisdiction: Option<String>,
    pub report_format: Option<String>,
    pub license_key: Option<String>,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    pub url: String,
    pub max_pages: Option<usize>,
    pub jurisdiction: Option<String>,
    pub report_format: Option<String>,
    pub license_key: Option<String>,
}

#[derive(Deserialize)]
pub struct AuditRequest {
    pub source_code: Option<String>,
    pub language: Option<String>,
    pub filename: Option<String>,
    pub url: Option<String>,
    pub max_pages: Option<usize>,
    pub jurisdiction: Option<String>,
    pub report_format: Option<String>,
    pub license_key: Option<String>,
}

// === RESPONSES ===

#[derive(Serialize)]
pub struct AnalysisResponse {
    pub report_id: String,
    pub status: String,
    pub score: f64,
    pub classification: String,
    pub total_violations: usize,
    pub violations_by_severity: ViolationSummary,
    pub violations: Vec<ViolationDetail>,
    pub categories: Vec<CategoryScore>,
    pub report_html: Option<String>,
    pub report_pdf_base64: Option<String>,
    pub disclaimer: String,
    pub engine_version: String,
    pub rules_version: String,
    pub analyzed_at: String,
    pub valid_until: String,
}

#[derive(Serialize)]
pub struct ViolationSummary {
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
}

#[derive(Serialize)]
pub struct ViolationDetail {
    pub rule_id: String,
    pub severity: String,
    pub business_description: String,
    pub business_remediation: String,
    pub law_article: Option<String>,
    pub technical_description: String,
    pub technical_remediation: String,
    pub location: ViolationLocation,
    pub code_snippet: Option<String>,
    pub suggested_fix: Option<String>,
}

#[derive(Serialize)]
pub struct ViolationLocation {
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub url: Option<String>,
    pub element_selector: Option<String>,
    pub element_html: Option<String>,
}

#[derive(Serialize)]
pub struct CategoryScore {
    pub category_id: String,
    pub category_name: String,
    pub score: f64,
    pub total_rules: usize,
    pub violations_found: usize,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
}

#[derive(Serialize)]
pub struct EngineInfoResponse {
    pub version: String,
    pub supported_languages: Vec<String>,
    pub supported_jurisdictions: Vec<String>,
    pub total_rules: usize,
}
