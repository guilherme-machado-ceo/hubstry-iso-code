//! Lógica de validação de conformidade para a jurisdição do ECA Digital.

use crate::models::{ComplianceViolation, RuleSeverity};
use crate::prefix_manager::PrefixInfo;
use syn::{visit::Visit, Expr, ExprCall};

use std::collections::HashMap;

/// Um Grafo de Chamadas básico (Call Graph) para funções no mesmo arquivo.
pub struct CallGraph {
    pub function_calls: HashMap<String, Vec<String>>,
}

impl CallGraph {
    pub fn new() -> Self {
        CallGraph {
            function_calls: HashMap::new(),
        }
    }
}

impl Default for CallGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl CallGraph {
    pub fn build(file_ast: &syn::File) -> Self {
        let mut graph = CallGraph::new();

        for item in &file_ast.items {
            if let syn::Item::Fn(func) = item {
                let func_name = func.sig.ident.to_string();
                let mut call_finder = CallFinder::new();
                call_finder.visit_item_fn(func);
                graph.function_calls.insert(func_name, call_finder.calls);
            }
        }

        graph
    }

    pub fn build_from_generic(file_ast: &crate::ast::FileAst) -> Self {
        let mut graph = CallGraph::new();

        for func in &file_ast.functions {
            let func_name = func.name.clone();
            graph
                .function_calls
                .insert(func_name, func.called_functions.clone());
        }

        graph
    }

    pub fn calls(&self, func_name: &str, target_keyword: &str) -> bool {
        let mut visited = std::collections::HashSet::new();
        self.calls_recursive(func_name, target_keyword, &mut visited)
    }

    fn calls_recursive(
        &self,
        func_name: &str,
        target_keyword: &str,
        visited: &mut std::collections::HashSet<String>,
    ) -> bool {
        if !visited.insert(func_name.to_string()) {
            return false;
        }

        if let Some(calls) = self.function_calls.get(func_name) {
            for call in calls {
                if call.to_lowercase().contains(&target_keyword.to_lowercase()) {
                    return true;
                }
                if self.calls_recursive(call, target_keyword, visited) {
                    return true;
                }
            }
        }

        false
    }
}

/// Valida uma função da AST de acordo com uma regra específica do ECA Digital.
pub fn validate(
    func: &crate::ast::FunctionAst,
    prefix_info: &PrefixInfo,
    call_graph: &CallGraph,
) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();

    match prefix_info.prefix.as_str() {
        "ECA.AGE.VERIFY" => {
            violations.extend(validate_age_verification(func, prefix_info, call_graph));
        }
        "ECA.PARENT.CONSENT" => {
            violations.extend(validate_parental_consent(func, prefix_info, call_graph));
        }
        "ECA.LOOTBOX.BLOCK" => {
            violations.extend(validate_lootbox_block(func, prefix_info, call_graph));
        }
        "ECA.AD.NO_RETENTION" => {
            violations.extend(validate_ad_retention(func, prefix_info, call_graph));
        }
        "ECA.AD.NO_TARGETING" => {
            violations.extend(validate_ad_targeting(func, prefix_info, call_graph));
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
fn validate_age_verification(
    func: &crate::ast::FunctionAst,
    prefix_info: &PrefixInfo,
    call_graph: &CallGraph,
) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let keywords = if !prefix_info.expected_calls.is_empty() {
        prefix_info.expected_calls.clone()
    } else {
        vec![
            "age".to_string(),
            "birthdate".to_string(),
            "dob".to_string(),
            "verify_age".to_string(),
            "check_age".to_string(),
        ]
    };

    let func_name = func.name.clone();
    let has_verification_call = keywords.iter().any(|kw| call_graph.calls(&func_name, kw));

    if !has_verification_call {
        violations.push(ComplianceViolation {
            rule_id: "ECA.AGE.VERIFY.1".to_string(),
            severity: RuleSeverity::High,
            message: "Function is annotated for age verification, but does not appear to call a relevant verification function.".to_string(),
            line: Some(func.line),
            column: Some(func.column),
            suggestion: Some("Ensure the function calls a service or helper for age verification (e.g., 'verify_age_with_id()' or 'serpro_datavalid.verify_age()').".to_string()),
        });
    }

    violations
}

/// Valida se uma função que coleta dados também obtém o consentimento parental.
fn validate_parental_consent(
    func: &crate::ast::FunctionAst,
    prefix_info: &PrefixInfo,
    call_graph: &CallGraph,
) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let data_collection_keywords = if !prefix_info.data_collection_keywords.is_empty() {
        prefix_info.data_collection_keywords.clone()
    } else {
        vec![
            "collect".to_string(),
            "save".to_string(),
            "store".to_string(),
            "get_data".to_string(),
            "user_profile".to_string(),
        ]
    };

    let consent_keywords = if !prefix_info.expected_calls.is_empty() {
        prefix_info.expected_calls.clone()
    } else {
        vec![
            "consent".to_string(),
            "permission".to_string(),
            "authorization".to_string(),
            "parent_ok".to_string(),
            "get_parental_consent".to_string(),
        ]
    };

    let func_name = func.name.clone();
    let mentions_data_collection = data_collection_keywords
        .iter()
        .any(|kw| call_graph.calls(&func_name, kw));
    let mentions_consent = consent_keywords
        .iter()
        .any(|kw| call_graph.calls(&func_name, kw));

    if mentions_data_collection && !mentions_consent {
        violations.push(ComplianceViolation {
            rule_id: "ECA.PARENT.CONSENT.1".to_string(),
            severity: RuleSeverity::High,
            message: "Function appears to collect user data but lacks a call to a parental consent function.".to_string(),
            line: Some(func.line),
            column: Some(func.column),
            suggestion: Some("Ensure that any data collection from minors is preceded by a call to a verifiable parental consent mechanism (e.g., 'get_parental_consent()').".to_string()),
        });
    }

    violations
}

/// Valida se uma função que implementa uma loot box está protegida por verificação de idade.
fn validate_lootbox_block(
    func: &crate::ast::FunctionAst,
    prefix_info: &PrefixInfo,
    call_graph: &CallGraph,
) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let lootbox_keywords = if !prefix_info.data_collection_keywords.is_empty() {
        prefix_info.data_collection_keywords.clone()
    } else {
        vec![
            "lootbox".to_string(),
            "crate".to_string(),
            "pack".to_string(),
            "random_reward".to_string(),
            "gacha".to_string(),
            "open_box".to_string(),
        ]
    };
    let age_check_keywords = if !prefix_info.expected_calls.is_empty() {
        prefix_info.expected_calls.clone()
    } else {
        vec![
            "age".to_string(),
            "birthdate".to_string(),
            "dob".to_string(),
            "verify_age".to_string(),
            "check_age".to_string(),
        ]
    };

    let func_name = func.name.clone();
    let mentions_lootbox = lootbox_keywords
        .iter()
        .any(|kw| call_graph.calls(&func_name, kw));
    let mentions_age_check = age_check_keywords
        .iter()
        .any(|kw| call_graph.calls(&func_name, kw));

    if mentions_lootbox && !mentions_age_check {
        violations.push(ComplianceViolation {
            rule_id: "ECA.LOOTBOX.BLOCK.1".to_string(),
            severity: RuleSeverity::High,
            message: "Function appears to implement a loot box mechanic without an age verification check.".to_string(),
            line: Some(func.line),
            column: Some(func.column),
            suggestion: Some("Ensure that access to loot box mechanics is protected by a call to an age verification function.".to_string()),
        });
    }

    violations
}

/// Valida se uma função não retém dados para publicidade.
fn validate_ad_retention(
    func: &crate::ast::FunctionAst,
    prefix_info: &PrefixInfo,
    call_graph: &CallGraph,
) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let keywords = if !prefix_info.expected_calls.is_empty() {
        prefix_info.expected_calls.clone()
    } else {
        vec![
            "disable_ad_tracking".to_string(),
            "prevent_data_retention".to_string(),
        ]
    };

    let func_name = func.name.clone();
    let has_verification_call = keywords.iter().any(|kw| call_graph.calls(&func_name, kw));

    if !has_verification_call {
        violations.push(ComplianceViolation {
            rule_id: "ECA.AD.NO_RETENTION.1".to_string(),
            severity: RuleSeverity::High,
            message: "Function handles advertising data without a mechanism to prevent data retention.".to_string(),
            line: Some(func.line),
            column: Some(func.column),
            suggestion: Some("Ensure you are calling a function to explicitly disable tracking or prevent ad data retention.".to_string()),
        });
    }

    violations
}

/// Valida se uma função não realiza publicidade direcionada.
fn validate_ad_targeting(
    func: &crate::ast::FunctionAst,
    prefix_info: &PrefixInfo,
    call_graph: &CallGraph,
) -> Vec<ComplianceViolation> {
    let mut violations = Vec::new();
    let keywords = if !prefix_info.expected_calls.is_empty() {
        prefix_info.expected_calls.clone()
    } else {
        vec![
            "disable_targeted_ads".to_string(),
            "serve_generic_ads".to_string(),
        ]
    };

    let func_name = func.name.clone();
    let has_verification_call = keywords.iter().any(|kw| call_graph.calls(&func_name, kw));

    if !has_verification_call {
        violations.push(ComplianceViolation {
            rule_id: "ECA.AD.NO_TARGETING.1".to_string(),
            severity: RuleSeverity::High,
            message: "Function appears to serve ads without explicitly serving generic, non-targeted ads.".to_string(),
            line: Some(func.line),
            column: Some(func.column),
            suggestion: Some("Ensure the ad delivery explicitly limits to generic ads without user targeting.".to_string()),
        });
    }

    violations
}

// --- AST Visitor para encontrar chamadas de função ---

pub struct CallFinder {
    pub calls: Vec<String>,
}

impl CallFinder {
    pub fn new() -> Self {
        CallFinder { calls: Vec::new() }
    }
}

impl Default for CallFinder {
    fn default() -> Self {
        Self::new()
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
