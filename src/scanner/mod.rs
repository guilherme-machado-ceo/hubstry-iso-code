// src/scanner/mod.rs
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Clone, serde::Serialize)]
pub struct WebScanResult {
    pub url: String,
    pub scan_date: String,
    pub violations: Vec<WebViolation>,
    pub score: f64,
    pub pages_scanned: usize,
    pub elements_analyzed: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct WebViolation {
    pub rule_id: String,
    pub severity: String,
    pub business_description: String,
    pub technical_description: String,
    pub element_selector: Option<String>,
    pub element_html: Option<String>,
    pub page_url: String,
    pub remediation_business: String,
    pub remediation_technical: String,
}

pub struct ScanConfig {
    pub max_pages: usize,
    pub follow_links: bool,
    pub check_subpages: Vec<String>,
    pub rules_path: String,
}

#[async_trait]
pub trait WebScanner: Send + Sync {
    async fn scan(&self, url: &str, config: &ScanConfig) -> Result<WebScanResult>;
}

/// Stub implementation for the MVP
pub struct StaticDomScanner;

impl StaticDomScanner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StaticDomScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WebScanner for StaticDomScanner {
    async fn scan(&self, url: &str, _config: &ScanConfig) -> Result<WebScanResult> {
        // Here we would use reqwest and scraper to fetch HTML and apply rules
        // based on the selectors defined in the rule YAML.
        Ok(WebScanResult {
            url: url.to_string(),
            scan_date: chrono::Utc::now().to_rfc3339(),
            violations: vec![], // Empty for the stub
            score: 100.0,
            pages_scanned: 1,
            elements_analyzed: 0,
        })
    }
}
