//! Lógica de validação para o padrão A.L.G.O.R.I.T.H.M (Auditoria de Algoritmos).

use crate::models::{AstNode, ComplianceViolation, NodeType, RuleSeverity};

/// Valida um nó da AST em busca de conformidade com as regras do A.L.G.O.R.I.T.H.M.
pub fn validate(node: &AstNode) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let audit_keywords = ["fairness", "bias", "explainability", "audit", "transparency"];

    let called_functions: Vec<String> = find_call_expressions(node).iter().map(|s| s.to_lowercase()).collect();
    let mut all_relevant_names = called_functions;
    all_relevant_names.push(node.content.to_lowercase()); // Include the function's own name

    let has_audit_context = all_relevant_names.iter().any(|name| audit_keywords.iter().any(|kw| name.contains(kw)));

    if !has_audit_context {
        violations.push(ComplianceViolation {
            rule_id: "ALGORITHM_001".to_string(),
            severity: RuleSeverity::High,
            message: "The algorithm function does not seem to include considerations for fairness, bias, or transparency in its name or sub-calls.".to_string(),
            line: None,
            column: None,
            suggestion: Some("Ensure the function's name or its sub-calls include keywords like 'fairness', 'bias', 'explainability', or 'audit'.".to_string()),
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