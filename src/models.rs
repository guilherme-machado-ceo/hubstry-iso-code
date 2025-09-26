//! Data models representing ISO codes, rules, and analysis results.
//! This module defines the core data structures used throughout the Hubstry-ISO_Code engine.

use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

/// Represents different legal jurisdictions for compliance.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Jurisdiction {
    Eca,
    Generic,
}

/// Represents a compliance rule with its metadata
#[derive(Debug, Clone)]
pub struct ComplianceRule {
    pub id: String,
    pub jurisdiction: Jurisdiction,
    pub severity: RuleSeverity,
    pub description: String,
    pub validation_pattern: Option<String>,
    pub remediation_hint: Option<String>,
}

/// Severity levels for compliance rules
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum RuleSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Represents compliance context attached to code elements
#[derive(Debug, Clone)]
pub struct ComplianceContext {
    pub prefix: String,
    pub jurisdiction: Jurisdiction,
    pub rules: Vec<String>,
    pub parameters: HashMap<String, String>,
}

/// Represents a compliance violation found during analysis
#[derive(Debug, Clone)]
pub struct ComplianceViolation {
    pub rule_id: String,
    pub severity: RuleSeverity,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub suggestion: Option<String>,
}

/// Configuration for the semantic engine
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub enabled_jurisdictions: Vec<Jurisdiction>,
    pub strict_mode: bool,
    pub output_format: OutputFormat,
    pub custom_rules: Vec<ComplianceRule>,
}

/// Output formats supported by the engine
#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Json,
    Yaml,
    Xml,
    PlainText,
    Markdown,
}

impl fmt::Display for Jurisdiction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Jurisdiction::Eca => write!(f, "ECA Digital"),
            Jurisdiction::Generic => write!(f, "Generic"),
        }
    }
}

/// Represents the result of a semantic analysis.
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub compliance_score: f64,
    pub violations: Vec<ComplianceViolation>,
    pub suggestions: Vec<String>,
    pub warnings: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl fmt::Display for RuleSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleSeverity::Critical => write!(f, "CRITICAL"),
            RuleSeverity::High => write!(f, "HIGH"),
            RuleSeverity::Medium => write!(f, "MEDIUM"),
            RuleSeverity::Low => write!(f, "LOW"),
            RuleSeverity::Info => write!(f, "INFO"),
        }
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            enabled_jurisdictions: vec![Jurisdiction::Eca],
            strict_mode: false,
            output_format: OutputFormat::Json,
            custom_rules: Vec::new(),
        }
    }
}