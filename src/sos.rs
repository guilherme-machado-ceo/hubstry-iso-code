//! Lógica de validação para o padrão S.O.S (Segurança).

use crate::models::{AstNode, ComplianceViolation, NodeType, RuleSeverity};

/// Valida um nó da AST em busca de conformidade com as regras do S.O.S.
pub fn validate(node: &AstNode) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();

    let security_keywords = ["authenticate", "password", "credentials", "token", "key", "secret", "jwt", "crypto", "encrypt", "decrypt", "hash", "auth"];
    let called_functions: Vec<String> = find_call_expressions(node).iter().map(|s| s.to_lowercase()).collect();
    let mut all_relevant_names = called_functions;
    all_relevant_names.push(node.content.to_lowercase()); // Include the function's own name

    let is_context_relevant = all_relevant_names.iter().any(|name| security_keywords.iter().any(|kw| name.contains(kw)));

    if !is_context_relevant {
        violations.push(ComplianceViolation {
            rule_id: "SOS_CONTEXT_001".to_string(),
            severity: RuleSeverity::Medium,
            message: "Potential misuse of S.O.S prefix. The code does not appear to call any security-sensitive functions.".to_string(),
            line: None,
            column: None,
            suggestion: Some("Ensure the function or its sub-calls handle authentication, authorization, encryption, or secrets.".to_string()),
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