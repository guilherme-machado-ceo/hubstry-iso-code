//! Lógica de validação para o padrão P.A.I.N.E.L (Painel Parental).

use crate::models::{AstNode, ComplianceViolation, NodeType, RuleSeverity};

/// Valida um nó da AST em busca de conformidade com as regras do P.A.I.N.E.L.
pub fn validate(node: &AstNode) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();

    let called_functions: Vec<String> = find_call_expressions(node)
        .iter()
        .map(|s| s.to_lowercase())
        .collect();
    let assigned_variables: Vec<String> = find_assignment_expressions(node)
        .iter()
        .map(|s| s.to_lowercase())
        .collect();

    // Rule PAINEL_001: Check for essential features by looking for specific function calls.
    let essential_keywords = ["block", "limit", "filter", "report"];
    let has_essential_feature = called_functions.iter().any(|call|
        essential_keywords.iter().any(|kw| call.contains(kw))
    );
    if !has_essential_feature {
        violations.push(ComplianceViolation {
            rule_id: "PAINEL_001".to_string(),
            severity: RuleSeverity::High,
            message: "Parental control function does not appear to call any essential feature functions (e.g., block_content, set_time_limit).".to_string(),
            line: None,
            column: None,
            suggestion: Some("Ensure the function calls subroutines for content blocking, time limits, filtering, or activity reporting.".to_string()),
        });
    }

    // Rule PAINEL_002: Check for insecure deactivation via function calls or assignments.
    let deactivation_call_keywords = ["disable", "deactivate", "turn_off", "remove_control"];
    let deactivation_assign_keywords = ["enable_control", "parental_lock"];
    let auth_keywords = ["password", "pin", "authentication", "auth"];

    let mentions_deactivation_call = called_functions.iter().any(|call|
        deactivation_call_keywords.iter().any(|kw| call.contains(kw))
    );
    let mentions_deactivation_assign = assigned_variables.iter().any(|var|
        deactivation_assign_keywords.iter().any(|kw| var.contains(kw))
    );
    let mentions_deactivation = mentions_deactivation_call || mentions_deactivation_assign;

    let mentions_auth = called_functions.iter().any(|call|
        auth_keywords.iter().any(|kw| call.contains(kw))
    );

    if mentions_deactivation && !mentions_auth {
        violations.push(ComplianceViolation {
            rule_id: "PAINEL_002".to_string(),
            severity: RuleSeverity::Medium,
            message: "A function that appears to disable parental controls was found, but no call to an authentication function (e.g., check_password) was detected.".to_string(),
            line: None,
            column: None,
            suggestion: Some("Ensure any function that disables parental controls is protected by a call to a password or PIN verification function.".to_string()),
        });
    }

    violations
}

/// Helper function to find all call expressions within a node and its children.
fn find_call_expressions(node: &AstNode) -> Vec<String> {
    let mut calls = Vec::new();
    if node.node_type == NodeType::CallExpression {
        calls.push(node.content.clone());
    }
    for child in &node.children {
        calls.extend(find_call_expressions(child));
    }
    calls
}

/// Helper function to find all assignment expressions within a node and its children.
fn find_assignment_expressions(node: &AstNode) -> Vec<String> {
    let mut assignments = Vec::new();
    if node.node_type == NodeType::AssignmentExpression {
        assignments.push(node.content.clone());
    }
    for child in &node.children {
        assignments.extend(find_assignment_expressions(child));
    }
    assignments
}