//! Lógica de validação para o padrão L.O.O.T.B.O.X (Loot Box).

use crate::models::{AstNode, ComplianceViolation, NodeType, RuleSeverity};

/// Valida um nó da AST em busca de conformidade com as regras do L.O.O.T.B.O.X.
pub fn validate(node: &AstNode) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();

    let reward_keywords = ["random", "chance", "gacha", "box", "reward", "crate", "pack"];
    let disclosure_keywords = ["odds", "probabilities", "rates", "chance_of"];

    let called_functions: Vec<String> = find_call_expressions(node).iter().map(|s| s.to_lowercase()).collect();
    let mut all_relevant_names = called_functions;
    all_relevant_names.push(node.content.to_lowercase());

    let mentions_reward = all_relevant_names.iter().any(|name| reward_keywords.iter().any(|kw| name.contains(kw)));
    let mentions_disclosure = all_relevant_names.iter().any(|name| disclosure_keywords.iter().any(|kw| name.contains(kw)));

    if mentions_reward && !mentions_disclosure {
        violations.push(ComplianceViolation {
            rule_id: "LOOTBOX_001".to_string(),
            severity: RuleSeverity::High,
            message: "The loot box mechanic does not appear to disclose item probabilities in its name or sub-calls.".to_string(),
            line: None,
            column: None,
            suggestion: Some("Ensure that the probabilities or odds for each random reward are clearly stated in the function's name or sub-calls.".to_string()),
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