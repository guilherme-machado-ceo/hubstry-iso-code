//! Lógica de validação para o padrão S.D.K.S.C.A.N (Scanner de SDKs).

use crate::models::{AstNode, ComplianceViolation, RuleSeverity};

/// Valida um nó da AST em busca de conformidade com as regras do S.D.K.S.C.A.N.
pub fn validate(node: &AstNode) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let review_keywords = ["audited", "security review", "privacy vetted", "vetted"];

    let has_review_note = if let Some(raw_body) = &node.raw_body {
        let body_lower = raw_body.to_lowercase();
        review_keywords.iter().any(|&kw| body_lower.contains(kw))
    } else {
        false
    };

    if !has_review_note {
        violations.push(ComplianceViolation {
            rule_id: "SDKSCAN_001".to_string(),
            severity: RuleSeverity::Medium,
            message: "The function using an external SDK or API does not appear to have a security review note in its body.".to_string(),
            line: None,
            column: None,
            suggestion: Some("Add a comment inside the function body noting the security and privacy review status of the external dependency.".to_string()),
        });
    }

    violations
}