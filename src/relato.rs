//! Lógica de validação para o padrão R.E.L.A.T.O (Relatórios Semestrais).

use crate::models::{AstNode, ComplianceViolation, NodeType, RuleSeverity};

/// Valida um nó da AST em busca de conformidade com as regras do R.E.L.A.T.O.
pub fn validate(node: &AstNode) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let report_keywords = ["public", "transparency", "semester", "biannual"];

    let called_functions: Vec<String> = find_call_expressions(node).iter().map(|s| s.to_lowercase()).collect();
    let mut all_relevant_names = called_functions;
    all_relevant_names.push(node.content.to_lowercase()); // Include the function's own name

    let has_report_context = all_relevant_names.iter().any(|name| report_keywords.iter().any(|kw| name.contains(kw)));

    if !has_report_context {
        violations.push(ComplianceViolation {
            rule_id: "RELATO_001".to_string(),
            severity: RuleSeverity::High,
            message: "The report function does not seem to specify its public or periodic nature in its name or sub-calls.".to_string(),
            line: None,
            column: None,
            suggestion: Some("Ensure the function's name or its sub-calls include keywords like 'public', 'transparency', 'semester', or 'biannual'.".to_string()),
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