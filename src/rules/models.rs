use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuleConfig {
    pub jurisdiction: String,
    pub law_reference: Option<String>,
    pub decree_reference: Option<String>,
    pub enforcement_body: Option<String>,
    pub effective_date: Option<String>,
    pub max_penalty: Option<String>,
    #[serde(default)]
    pub categories: Vec<RuleCategory>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuleCategory {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub rules: Vec<RuleDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuleDefinition {
    pub id: String,
    pub severity: String,
    pub law_article: Option<String>,
    pub business_description: String,
    pub technical_description: Option<String>,
    #[serde(default)]
    pub detection: RuleDetection,
    #[serde(default)]
    pub remediation: RuleRemediation,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RuleDetection {
    pub code_patterns: Option<CodePatterns>,
    pub web_patterns: Option<WebPatterns>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CodePatterns {
    #[serde(default)]
    pub function_annotations: Vec<String>,
    #[serde(default)]
    pub required_calls: Vec<PatternDefinition>,
    #[serde(default)]
    pub forbidden_patterns: Vec<PatternDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PatternDefinition {
    pub pattern: String,
    pub description: Option<String>,
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct WebPatterns {
    #[serde(default)]
    pub required_elements: Vec<WebSelectorDefinition>,
    #[serde(default)]
    pub forbidden_elements: Vec<WebSelectorDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebSelectorDefinition {
    pub selector: String,
    pub description: Option<String>,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RuleRemediation {
    pub business: String,
    pub technical: Option<String>,
}
