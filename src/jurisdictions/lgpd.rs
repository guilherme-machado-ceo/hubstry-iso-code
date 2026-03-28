//! Lógica de validação de conformidade para a jurisdição LGPD.

use crate::ast::FunctionAst;
use crate::jurisdictions::eca::CallGraph;
use crate::models::{ComplianceViolation, RuleSeverity};
use crate::prefix_manager::PrefixInfo; // Podemos reusar o CallGraph já que é genérico

/// Valida uma função da AST de acordo com regras da LGPD
pub fn validate(
    func: &FunctionAst,
    prefix_info: &PrefixInfo,
    call_graph: &CallGraph,
) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();

    match prefix_info.prefix.as_str() {
        "LGPD.DATA.COLLECTION" => {
            violations.extend(validate_data_collection(func, prefix_info, call_graph));
        }
        "LGPD.CONSENT.REQUIRED" => {
            violations.extend(validate_consent_required(func, prefix_info, call_graph));
        }
        _ => {}
    }

    violations
}

/// Valida a coleta de dados e espera uma anonimização em conjunto
fn validate_data_collection(
    func: &FunctionAst,
    prefix_info: &PrefixInfo,
    call_graph: &CallGraph,
) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let data_collection_keywords = if !prefix_info.data_collection_keywords.is_empty() {
        prefix_info.data_collection_keywords.clone()
    } else {
        vec![
            "collect_personal_data".to_string(),
            "save_user_info".to_string(),
        ]
    };
    let anonymization_calls = if !prefix_info.expected_calls.is_empty() {
        prefix_info.expected_calls.clone()
    } else {
        vec!["anonymize_data".to_string()]
    };

    let func_name = func.name.clone();
    let mentions_collection = data_collection_keywords
        .iter()
        .any(|kw| call_graph.calls(&func_name, kw));
    let mentions_anonymization = anonymization_calls
        .iter()
        .any(|kw| call_graph.calls(&func_name, kw));

    if mentions_collection && !mentions_anonymization {
        violations.push(ComplianceViolation {
            rule_id: "LGPD.DATA.COLLECTION.1".to_string(),
            severity: RuleSeverity::Medium,
            message: "Data collection function does not anonymize data as expected under LGPD minimization principles.".to_string(),
            line: Some(func.line),
            column: Some(func.column),
            suggestion: Some("Ensure data is anonymized immediately upon collection or specify explicit consent mechanisms.".to_string()),
        });
    }

    violations
}

/// Valida requisição de consentimento
fn validate_consent_required(
    func: &FunctionAst,
    prefix_info: &PrefixInfo,
    call_graph: &CallGraph,
) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let expected_calls = if !prefix_info.expected_calls.is_empty() {
        prefix_info.expected_calls.clone()
    } else {
        vec![
            "check_user_consent".to_string(),
            "require_opt_in".to_string(),
        ]
    };

    let func_name = func.name.clone();
    let has_consent = expected_calls
        .iter()
        .any(|kw| call_graph.calls(&func_name, kw));

    if !has_consent {
        violations.push(ComplianceViolation {
            rule_id: "LGPD.CONSENT.REQUIRED.1".to_string(),
            severity: RuleSeverity::High,
            message: "Function requires explicit consent but lacks a verifiable consent check (e.g., 'check_user_consent').".to_string(),
            line: Some(func.line),
            column: Some(func.column),
            suggestion: Some("Implement an explicit opt-in verification or a consent management mechanism before proceeding.".to_string()),
        });
    }

    violations
}
