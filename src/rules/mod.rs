// src/rules/mod.rs
pub mod loader;
pub mod models;

use crate::models::{ComplianceViolation, RuleSeverity};
use crate::parser::FunctionInfo;
use crate::rules::models::RuleDefinition;
use regex::Regex;

/// Checks if a function satisfies or violates a specific rule based on pattern matching.
pub fn validate_rule_against_function(
    func: &FunctionInfo,
    rule: &RuleDefinition,
) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();

    // The validation here is naive but implements the structure for Phase 2 expanded schema matching
    if let Some(code_patterns) = &rule.detection.code_patterns {
        let mut has_annotation = false;

        // 1. Check if the function is annotated with any of the required annotations
        for annotation in &code_patterns.function_annotations {
             if func.annotations.iter().any(|a: &String| a.contains(annotation)) {
                  has_annotation = true;
                  break;
             }
        }

        // If the rule expects an annotation and we don't have one, we don't apply this specific logic.
        // If it does not expect an annotation, we assume the rule applies universally (e.g. banning patterns everywhere).
        let rule_applies = has_annotation || code_patterns.function_annotations.is_empty();

        if rule_applies {
             // 2. Check for forbidden patterns in body or calls
             for forbidden in &code_patterns.forbidden_patterns {
                  if let Ok(re) = Regex::new(&forbidden.pattern) {
                      let matched = func.calls.iter().any(|c| re.is_match(&c.full_expression))
                                 || re.is_match(&func.body_source);

                      if matched {
                          violations.push(ComplianceViolation {
                              rule_id: rule.id.clone(),
                              severity: match rule.severity.as_str() {
                                  "CRITICAL" => RuleSeverity::Critical,
                                  "HIGH" => RuleSeverity::High,
                                  "MEDIUM" => RuleSeverity::Medium,
                                  "LOW" => RuleSeverity::Low,
                                  _ => RuleSeverity::Info,
                              },
                              message: forbidden.description.clone()
                                  .unwrap_or_else(|| "Forbidden pattern matched".to_string()),
                              line: Some(func.start_line),
                              column: Some(func.start_column),
                              suggestion: rule.remediation.technical.clone()
                                  .or_else(|| Some(rule.remediation.business.clone())),
                          });
                      }
                  }
             }

             // 3. Check for required calls
             if !code_patterns.required_calls.is_empty() {
                 let mut found_required = false;
                 for req in &code_patterns.required_calls {
                     if let Ok(re) = Regex::new(&req.pattern) {
                         if func.calls.iter().any(|c| re.is_match(&c.full_expression)) {
                             found_required = true;
                             break;
                         }
                     }
                 }

                 if !found_required {
                     violations.push(ComplianceViolation {
                         rule_id: rule.id.clone(),
                         severity: match rule.severity.as_str() {
                             "CRITICAL" => RuleSeverity::Critical,
                             "HIGH" => RuleSeverity::High,
                             "MEDIUM" => RuleSeverity::Medium,
                             "LOW" => RuleSeverity::Low,
                             _ => RuleSeverity::Info,
                         },
                         message: rule.technical_description.clone()
                                  .unwrap_or_else(|| rule.business_description.clone()),
                         line: Some(func.start_line),
                         column: Some(func.start_column),
                         suggestion: rule.remediation.technical.clone()
                                     .or_else(|| Some(rule.remediation.business.clone())),
                     });
                 }
             }
        }
    }

    violations
}
