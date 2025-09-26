//! Semantic engine module for the Hubstry-ISO_Code framework.
//! This module dispatches analysis to the appropriate jurisdiction-specific modules.

use crate::models::{AnalysisResult, ComplianceViolation, EngineConfig, Jurisdiction, RuleSeverity};
use crate::prefix_manager::{self, PrefixError};
use std::collections::HashMap;
use syn::{Attribute, Expr, File, Item, Lit, Meta};

/// The main semantic engine for compliance analysis.
#[derive(Debug)]
pub struct SemanticEngine {
    config: EngineConfig,
}

impl SemanticEngine {
    /// Creates a new semantic engine with the given configuration.
    pub fn new(config: EngineConfig) -> Self {
        SemanticEngine { config }
    }

    /// Creates a semantic engine with default configuration.
    pub fn default() -> Self {
        Self::new(EngineConfig::default())
    }

    /// Analyzes a `syn::File` AST for compliance violations.
    /// This function can fail if the prefix configuration cannot be loaded.
    pub fn analyze(&self, file_ast: &File) -> Result<AnalysisResult, PrefixError> {
        let mut violations = Vec::new();
        // Load the prefix map once at the beginning.
        let prefix_map = prefix_manager::try_get_prefix_map()?;

        for item in &file_ast.items {
            if let Item::Fn(func) = item {
                // Find all compliance prefixes in the function's doc attributes
                for attr in &func.attrs {
                    if let Some(prefix) = self.get_potential_prefix(attr) {
                        // Look up the prefix to find its jurisdiction
                        if let Some(prefix_info) = prefix_map.get(&prefix) {
                            let jurisdiction = match prefix_info.standard.as_str() {
                                "Eca" => Jurisdiction::Eca,
                                _ => Jurisdiction::Generic,
                            };

                            // Dispatch to the correct jurisdiction if it's enabled
                            if self.config.enabled_jurisdictions.contains(&jurisdiction) {
                                match jurisdiction {
                                    Jurisdiction::Eca => {
                                            violations.extend(crate::jurisdictions::eca::validate(func, &prefix));
                                    }
                                        _ => {} // Other jurisdictions are not handled
                                }
                            }
                        }
                    }
                }
            }
        }

        let compliance_score = self.calculate_compliance_score(&violations);

        Ok(AnalysisResult {
            compliance_score,
            violations,
            suggestions: Vec::new(), // Placeholder for future implementation
            warnings: Vec::new(),    // Placeholder for future implementation
            metadata: HashMap::new(), // Placeholder for future implementation
        })
    }

    /// Extracts a potential prefix string from a `#[doc = "..."]` attribute.
    fn get_potential_prefix(&self, attr: &Attribute) -> Option<String> {
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(nv) = &attr.meta {
                if let Expr::Lit(expr_lit) = &nv.value {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        let comment_text = lit_str.value();
                        let parts: Vec<&str> = comment_text.trim().splitn(2, ':').collect();
                        if parts.len() > 1 {
                            return Some(parts[0].trim().to_string());
                        }
                    }
                }
            }
        }
        None
    }

    /// Calculates a compliance score based on violations.
    fn calculate_compliance_score(&self, violations: &[ComplianceViolation]) -> f64 {
        if violations.is_empty() {
            100.0
        } else {
            let total_weight: f64 = violations
                .iter()
                .map(|v| match v.severity {
                    RuleSeverity::Critical => 10.0,
                    RuleSeverity::High => 5.0,
                    RuleSeverity::Medium => 2.0,
                    RuleSeverity::Low => 1.0,
                    RuleSeverity::Info => 0.5,
                })
                .sum();
            (100.0 - (total_weight * 2.0)).max(0.0)
        }
    }

    /// Generates a compliance report.
    pub fn generate_report(&self, result: &AnalysisResult) -> String {
        let mut report = String::new();
        report.push_str(&format!("# Hubstry-ISO_Code Compliance Report\n\n"));
        report.push_str(&format!("**Compliance Score:** {:.1}%\n\n", result.compliance_score));

        if !result.violations.is_empty() {
            report.push_str(&format!("## Violations ({})\n\n", result.violations.len()));
            for violation in &result.violations {
                report.push_str(&format!(
                    "- **{}** [{}]: {}\n",
                    violation.severity, violation.rule_id, violation.message
                ));
                if let Some(suggestion) = &violation.suggestion {
                    report.push_str(&format!("  *Suggestion: {}*\n", suggestion));
                }
                report.push('\n');
            }
        }
        report
    }
}