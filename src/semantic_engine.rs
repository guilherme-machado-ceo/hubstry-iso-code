//! Semantic engine module for the Hubstry-ISO_Code framework.
//! This module contains the logic for applying compliance rules and semantic analysis.

use crate::models::*;
use std::collections::HashMap;

/// The main semantic engine for compliance analysis
#[derive(Debug)]
pub struct SemanticEngine {
    config: EngineConfig,
    rules: HashMap<String, ComplianceRule>,
}

/// Result of semantic analysis
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub compliance_score: f64,
    pub violations: Vec<ComplianceViolation>,
    pub suggestions: Vec<String>,
    pub warnings: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

impl SemanticEngine {
    /// Creates a new semantic engine with the given configuration
    pub fn new(config: EngineConfig) -> Self {
        let mut engine = Self {
            config,
            rules: HashMap::new(),
        };
        engine.load_default_rules();
        engine
    }

    /// Creates a semantic engine with default configuration
    pub fn default() -> Self {
        Self::new(EngineConfig::default())
    }

    /// Loads default compliance rules for each standard
    fn load_default_rules(&mut self) {
        // Security rules (S.O.S)
        self.add_rule(ComplianceRule {
            id: "SOS_001".to_string(),
            standard: ComplianceStandard::Security,
            severity: RuleSeverity::High,
            description: "Functions handling sensitive data must be explicitly marked".to_string(),
            validation_pattern: Some("function.*password|token|key|secret".to_string()),
            remediation_hint: Some("Add S.O.S prefix to security-sensitive functions".to_string()),
        });

        self.add_rule(ComplianceRule {
            id: "SOS_002".to_string(),
            standard: ComplianceStandard::Security,
            severity: RuleSeverity::Critical,
            description: "No hardcoded secrets or credentials allowed".to_string(),
            validation_pattern: Some("(password|token|key|secret)\\s*=\\s*['\"][^'\"]+['\"]|api_key|access_token".to_string()),
            remediation_hint: Some("Use environment variables or secure configuration for secrets".to_string()),
        });

        // Privacy rules (G.D.P.R)
        self.add_rule(ComplianceRule {
            id: "GDPR_001".to_string(),
            standard: ComplianceStandard::Privacy,
            severity: RuleSeverity::High,
            description: "Personal data processing must be explicitly marked".to_string(),
            validation_pattern: Some("(email|phone|address|name|ssn|cpf)".to_string()),
            remediation_hint: Some("Add G.D.P.R prefix when processing personal data".to_string()),
        });

        self.add_rule(ComplianceRule {
            id: "GDPR_002".to_string(),
            standard: ComplianceStandard::Privacy,
            severity: RuleSeverity::Medium,
            description: "Data retention policies must be documented".to_string(),
            validation_pattern: Some("store|save|persist.*data".to_string()),
            remediation_hint: Some("Document data retention period and deletion procedures".to_string()),
        });

        // Quality rules (Q.M.S)
        self.add_rule(ComplianceRule {
            id: "QMS_001".to_string(),
            standard: ComplianceStandard::Quality,
            severity: RuleSeverity::Medium,
            description: "Functions should have proper error handling".to_string(),
            validation_pattern: Some("function.*\\{[^}]*\\}".to_string()),
            remediation_hint: Some("Add try-catch blocks or error handling mechanisms".to_string()),
        });

        // Accessibility rules (A.C.C)
        self.add_rule(ComplianceRule {
            id: "ACC_001".to_string(),
            standard: ComplianceStandard::Accessibility,
            severity: RuleSeverity::Medium,
            description: "UI elements must have accessibility attributes".to_string(),
            validation_pattern: Some("button|input|img|form".to_string()),
            remediation_hint: Some("Add aria-label, alt text, or other accessibility attributes".to_string()),
        });

        // Sustainability rules (S.U.S)
        self.add_rule(ComplianceRule {
            id: "SUS_001".to_string(),
            standard: ComplianceStandard::Sustainability,
            severity: RuleSeverity::Low,
            description: "Avoid resource-intensive operations without optimization".to_string(),
            validation_pattern: Some("while\\s*\\(true\\)|for\\s*\\(.*\\).*\\{.*\\}".to_string()),
            remediation_hint: Some("Consider algorithm optimization and resource usage".to_string()),
        });

        // Diversity rules (D.I.V)
        self.add_rule(ComplianceRule {
            id: "DIV_001".to_string(),
            standard: ComplianceStandard::Diversity,
            severity: RuleSeverity::Low,
            description: "Use inclusive language in code and comments".to_string(),
            validation_pattern: Some("(master|slave|blacklist|whitelist)".to_string()),
            remediation_hint: Some("Use inclusive alternatives: main/primary, allow/deny list".to_string()),
        });
    }

    /// Adds a compliance rule to the engine
    pub fn add_rule(&mut self, rule: ComplianceRule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    /// Analyzes an AST for compliance violations
    pub fn analyze(&self, ast: &AstNode) -> AnalysisResult {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();
        let mut metadata = HashMap::new();

        // Perform analysis
        self.analyze_node(ast, &mut violations, &mut warnings, &mut suggestions);

        // Calculate compliance score
        let compliance_score = self.calculate_compliance_score(&violations);

        // Add metadata
        metadata.insert("analysis_version".to_string(), "0.1.0".to_string());
        metadata.insert("analyzed_at".to_string(), chrono::Utc::now().to_rfc3339());
        metadata.insert("total_violations".to_string(), violations.len().to_string());
        metadata.insert("enabled_standards".to_string(), 
            self.config.enabled_standards.iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        AnalysisResult {
            compliance_score,
            violations,
            suggestions,
            warnings,
            metadata,
        }
    }

    /// Recursively analyzes AST nodes
    fn analyze_node(
        &self,
        node: &AstNode,
        violations: &mut Vec<ComplianceViolation>,
        warnings: &mut Vec<String>,
        suggestions: &mut Vec<String>,
    ) {
        // Check if node has compliance context
        if !node.compliance_context.is_empty() {
            self.validate_compliance_context(node, violations, suggestions);
        } else {
            // Check if node should have compliance context
            self.check_missing_compliance(node, violations, warnings);
        }

        // Apply content-based rules
        self.apply_content_rules(node, violations, warnings);

        // Recursively analyze children
        for child in &node.children {
            self.analyze_node(child, violations, warnings, suggestions);
        }
    }

    /// Validates existing compliance context
    fn validate_compliance_context(
        &self,
        node: &AstNode,
        violations: &mut Vec<ComplianceViolation>,
        suggestions: &mut Vec<String>,
    ) {
        for context in &node.compliance_context {
            // Check if the standard is enabled
            if !self.config.enabled_standards.contains(&context.standard) {
                suggestions.push(format!(
                    "Compliance standard {} is not enabled in configuration",
                    context.standard
                ));
            }

            // Validate context parameters
            if context.rules.is_empty() && context.parameters.is_empty() {
                violations.push(ComplianceViolation {
                    rule_id: "CONTEXT_001".to_string(),
                    severity: RuleSeverity::Low,
                    message: format!(
                        "Compliance context {} lacks specific rules or parameters",
                        context.prefix
                    ),
                    line: None,
                    column: None,
                    suggestion: Some("Add specific compliance rules or parameters".to_string()),
                });
            }
        }
    }

    /// Checks for missing compliance context where it should be present
    fn check_missing_compliance(
        &self,
        node: &AstNode,
        violations: &mut Vec<ComplianceViolation>,
        warnings: &mut Vec<String>,
    ) {
        // Check each enabled standard's rules
        for standard in &self.config.enabled_standards {
            let standard_rules: Vec<_> = self.rules.values()
                .filter(|rule| rule.standard == *standard)
                .collect();

            for rule in standard_rules {
                if let Some(pattern) = &rule.validation_pattern {
                    if self.matches_pattern(&node.content, pattern) {
                        let violation = ComplianceViolation {
                            rule_id: rule.id.clone(),
                            severity: rule.severity.clone(),
                            message: format!(
                                "Missing {} compliance annotation: {}",
                                standard, rule.description
                            ),
                            line: None,
                            column: None,
                            suggestion: rule.remediation_hint.clone(),
                        };

                        if self.config.strict_mode || rule.severity == RuleSeverity::Critical {
                            violations.push(violation);
                        } else {
                            warnings.push(format!(
                                "Warning: {} (Rule: {})",
                                rule.description, rule.id
                            ));
                        }
                    }
                }
            }
        }
    }

    /// Applies content-based rules to detect violations
    fn apply_content_rules(
        &self,
        node: &AstNode,
        violations: &mut Vec<ComplianceViolation>,
        warnings: &mut Vec<String>,
    ) {
        // Apply rules based on node content
        for rule in self.rules.values() {
            if !self.config.enabled_standards.contains(&rule.standard) {
                continue;
            }

            if let Some(pattern) = &rule.validation_pattern {
                if self.matches_pattern(&node.content, pattern) {
                    // Check if this violation is already covered by compliance context
                    let has_relevant_context = node.compliance_context.iter()
                        .any(|ctx| ctx.standard == rule.standard);

                    if !has_relevant_context {
                        let violation = ComplianceViolation {
                            rule_id: rule.id.clone(),
                            severity: rule.severity.clone(),
                            message: rule.description.clone(),
                            line: None,
                            column: None,
                            suggestion: rule.remediation_hint.clone(),
                        };

                        if rule.severity == RuleSeverity::Critical || self.config.strict_mode {
                            violations.push(violation);
                        } else {
                            warnings.push(format!(
                                "Potential issue: {} (Rule: {})",
                                rule.description, rule.id
                            ));
                        }
                    }
                }
            }
        }
    }

    /// Simple pattern matching (in a real implementation, this would use regex)
    fn matches_pattern(&self, content: &str, pattern: &str) -> bool {
        // Simplified pattern matching - in production, use proper regex
        let content_lower = content.to_lowercase();
        let pattern_lower = pattern.to_lowercase();
        
        // Basic keyword matching
        if pattern_lower.contains("|") {
            pattern_lower.split('|').any(|p| content_lower.contains(p.trim()))
        } else {
            content_lower.contains(&pattern_lower)
        }
    }

    /// Calculates a compliance score based on violations
    fn calculate_compliance_score(&self, violations: &[ComplianceViolation]) -> f64 {
        if violations.is_empty() {
            return 100.0;
        }

        let total_weight: f64 = violations.iter().map(|v| {
            match v.severity {
                RuleSeverity::Critical => 10.0,
                RuleSeverity::High => 5.0,
                RuleSeverity::Medium => 2.0,
                RuleSeverity::Low => 1.0,
                RuleSeverity::Info => 0.5,
            }
        }).sum();

        // Calculate score (100 - penalty, minimum 0)
        let penalty = total_weight * 2.0; // Adjust multiplier as needed
        (100.0 - penalty).max(0.0)
    }

    /// Generates a compliance report
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
        
        if !result.warnings.is_empty() {
            report.push_str(&format!("## Warnings ({})\n\n", result.warnings.len()));
            for warning in &result.warnings {
                report.push_str(&format!("- {}\n", warning));
            }
            report.push('\n');
        }
        
        if !result.suggestions.is_empty() {
            report.push_str(&format!("## Suggestions ({})\n\n", result.suggestions.len()));
            for suggestion in &result.suggestions {
                report.push_str(&format!("- {}\n", suggestion));
            }
        }
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[test]
    fn test_semantic_engine_creation() {
        let engine = SemanticEngine::default();
        assert!(!engine.rules.is_empty());
    }

    #[test]
    fn test_compliance_score_calculation() {
        let engine = SemanticEngine::default();
        let violations = vec![
            ComplianceViolation {
                rule_id: "TEST_001".to_string(),
                severity: RuleSeverity::High,
                message: "Test violation".to_string(),
                line: None,
                column: None,
                suggestion: None,
            }
        ];
        let score = engine.calculate_compliance_score(&violations);
        assert!(score < 100.0);
    }

    #[test]
    fn test_pattern_matching() {
        let engine = SemanticEngine::default();
        assert!(engine.matches_pattern("password = 'secret'", "password"));
        assert!(!engine.matches_pattern("username = 'john'", "password"));
    }
}


