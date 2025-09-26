//! Lógica de validação de conformidade para a jurisdição do ECA Digital.

use crate::models::{ComplianceViolation, RuleSeverity};
use syn::{spanned::Spanned, visit::Visit, Expr, ExprCall, ItemFn};

/// Valida uma função da AST de acordo com uma regra específica do ECA Digital.
pub fn validate(func: &ItemFn, prefix: &str) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();

    match prefix {
        "ECA.AGE.VERIFY" => {
            violations.extend(validate_age_verification(func));
        }
        "ECA.PARENT.CONSENT" => {
            violations.extend(validate_parental_consent(func));
        }
        "ECA.LOOTBOX.BLOCK" => {
            violations.extend(validate_lootbox_block(func));
        }
        // Outros prefixos do ECA podem ser adicionados aqui.
        _ => {}
    }

    violations
}

// A função `get_compliance_prefix` foi removida. A responsabilidade de encontrar
// e validar o prefixo agora é inteiramente do `semantic_engine`, que passará
// o prefixo validado para a função `validate`.

/// Valida se uma função anotada com `ECA.AGE.VERIFY` realmente executa uma verificação de idade.
fn validate_age_verification(func: &ItemFn) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let keywords = ["age", "birthdate", "dob", "verify_age", "check_age"];

    let mut call_finder = CallFinder::new();
    call_finder.visit_item_fn(func);

    let has_verification_call = call_finder.calls.iter().any(|call|
        keywords.iter().any(|kw| call.to_lowercase().contains(kw))
    );

    if !has_verification_call {
        violations.push(ComplianceViolation {
            rule_id: "ECA.AGE.VERIFY.1".to_string(),
            severity: RuleSeverity::High,
            message: "Function is annotated for age verification, but does not appear to call a relevant verification function.".to_string(),
            line: Some(func.span().start().line),
            column: Some(func.span().start().column),
            suggestion: Some("Ensure the function calls a service or helper for age verification (e.g., 'verify_age_with_id()').".to_string()),
        });
    }

    violations
}

/// Valida se uma função que coleta dados também obtém o consentimento parental.
fn validate_parental_consent(func: &ItemFn) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let data_collection_keywords = ["collect", "save", "store", "get_data", "user_profile"];
    let consent_keywords = ["consent", "permission", "authorization", "parent_ok", "get_parental_consent"];

    let mut call_finder = CallFinder::new();
    call_finder.visit_item_fn(func);

    let mentions_data_collection = call_finder.calls.iter().any(|call|
        data_collection_keywords.iter().any(|kw| call.contains(kw))
    );
    let mentions_consent = call_finder.calls.iter().any(|call|
        consent_keywords.iter().any(|kw| call.contains(kw))
    );

    if mentions_data_collection && !mentions_consent {
        violations.push(ComplianceViolation {
            rule_id: "ECA.PARENT.CONSENT.1".to_string(),
            severity: RuleSeverity::High,
            message: "Function appears to collect user data but lacks a call to a parental consent function.".to_string(),
            line: Some(func.span().start().line),
            column: Some(func.span().start().column),
            suggestion: Some("Ensure that any data collection from minors is preceded by a call to a verifiable parental consent mechanism (e.g., 'get_parental_consent()').".to_string()),
        });
    }

    violations
}

/// Valida se uma função que implementa uma loot box está protegida por verificação de idade.
fn validate_lootbox_block(func: &ItemFn) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let lootbox_keywords = ["lootbox", "crate", "pack", "random_reward", "gacha", "open_box"];
    let age_check_keywords = ["age", "birthdate", "dob", "verify_age", "check_age"];

    let mut call_finder = CallFinder::new();
    call_finder.visit_item_fn(func);

    let mentions_lootbox = call_finder.calls.iter().any(|call|
        lootbox_keywords.iter().any(|kw| call.contains(kw))
    );
    let mentions_age_check = call_finder.calls.iter().any(|call|
        age_check_keywords.iter().any(|kw| call.contains(kw))
    );

    if mentions_lootbox && !mentions_age_check {
        violations.push(ComplianceViolation {
            rule_id: "ECA.LOOTBOX.BLOCK.1".to_string(),
            severity: RuleSeverity::High,
            message: "Function appears to implement a loot box mechanic without an age verification check.".to_string(),
            line: Some(func.span().start().line),
            column: Some(func.span().start().column),
            suggestion: Some("Ensure that access to loot box mechanics is protected by a call to an age verification function.".to_string()),
        });
    }

    violations
}


// --- AST Visitor para encontrar chamadas de função ---

struct CallFinder {
    calls: Vec<String>,
}

impl CallFinder {
    fn new() -> Self {
        CallFinder { calls: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for CallFinder {
    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Expr::Path(expr_path) = &*node.func {
            if let Some(path_segment) = expr_path.path.segments.last() {
                self.calls.push(path_segment.ident.to_string());
            }
        }
        // Continue visiting children of the expression, like arguments
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        // Add the method name to our list of calls
        self.calls.push(node.method.to_string());

        // Continue visiting children, like arguments
        syn::visit::visit_expr_method_call(self, node);
    }
}