use sha2::{Digest, Sha256};
use std::env;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tier {
    Community,
    Starter,
    Pro,
    Enterprise,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Feature {
    CodeAnalysis,
    WebScanning,
    ReportTerminal,
    ReportHtml,
    ReportPdf,
    ReportJson,
    ApiAccess,
    PremiumRules,
    CustomRules,
}

impl Tier {
    pub fn has_access(&self, feature: Feature) -> bool {
        match self {
            Tier::Community => matches!(feature, Feature::CodeAnalysis | Feature::ReportTerminal),
            Tier::Starter => matches!(
                feature,
                Feature::CodeAnalysis
                    | Feature::ReportTerminal
                    | Feature::ReportHtml
                    | Feature::ReportPdf
                    | Feature::PremiumRules
            ),
            Tier::Pro => matches!(
                feature,
                Feature::CodeAnalysis
                    | Feature::ReportTerminal
                    | Feature::ReportHtml
                    | Feature::ReportPdf
                    | Feature::PremiumRules
                    | Feature::WebScanning
                    | Feature::ReportJson
                    | Feature::ApiAccess
            ),
            Tier::Enterprise => true,
        }
    }
}

pub fn validate_license_key(key: Option<&str>) -> Tier {
    let key = match key {
        Some(k) if !k.is_empty() => k,
        _ => return Tier::Community,
    };

    let salt = env::var("HUBSTRY_LICENSE_SALT").unwrap_or_else(|_| "default_salt".to_string());

    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", key, salt));
    let hash = hex::encode(hasher.finalize());

    // Basic hash matching simulation for MVP. In prod this would be an API check/DB query.
    if hash.starts_with("st_") || key.starts_with("starter_") {
        Tier::Starter
    } else if hash.starts_with("pr_") || key.starts_with("pro_") {
        Tier::Pro
    } else if hash.starts_with("ent_") || key.starts_with("enterprise_") {
        Tier::Enterprise
    } else {
        Tier::Community
    }
}
