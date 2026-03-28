//! Semantic engine module for the Hubstry-ISO_Code framework.
//! This module dispatches analysis to the appropriate jurisdiction-specific modules.

use crate::ast::{FileAst, FunctionAst};
use crate::models::{
    AnalysisResult, ComplianceViolation, EngineConfig, Jurisdiction, RuleSeverity,
};
use crate::prefix_manager::{self, PrefixError};
use std::collections::HashMap;
use syn::{spanned::Spanned, Expr, File, Item, Lit, Meta};

/// The main semantic engine for compliance analysis.
#[derive(Debug)]
pub struct SemanticEngine {
    config: EngineConfig,
}

impl SemanticEngine {
    /// Creates a new semantic engine with the given configuration.
    pub fn new(config: EngineConfig) -> Self {
        SemanticEngine { config }
    }
}

impl Default for SemanticEngine {
    fn default() -> Self {
        Self::new(EngineConfig::default())
    }
}

impl SemanticEngine {
    /// Helper to convert a `syn::File` to our generic `FileAst`.
    fn syn_to_generic_ast(file_ast: &File) -> FileAst {
        let mut functions = Vec::new();

        for item in &file_ast.items {
            if let Item::Fn(func) = item {
                let name = func.sig.ident.to_string();
                let line = func.span().start().line;
                let column = func.span().start().column;

                let mut doc_comments = Vec::new();
                for attr in &func.attrs {
                    if attr.path().is_ident("doc") {
                        if let Meta::NameValue(nv) = &attr.meta {
                            if let Expr::Lit(expr_lit) = &nv.value {
                                if let Lit::Str(lit_str) = &expr_lit.lit {
                                    doc_comments.push(lit_str.value().trim().to_string());
                                }
                            }
                        }
                    }
                }

                let mut call_finder = crate::jurisdictions::eca::CallFinder::new();
                syn::visit::Visit::visit_item_fn(&mut call_finder, func);

                functions.push(FunctionAst {
                    name,
                    doc_comments,
                    called_functions: call_finder.calls,
                    line,
                    column,
                });
            }
        }

        FileAst { functions }
    }

    /// Analyzes a `syn::File` AST for compliance violations.
    /// This function can fail if the prefix configuration cannot be loaded.
    pub fn analyze(&self, file_ast: &File) -> Result<AnalysisResult, PrefixError> {
        let generic_ast = Self::syn_to_generic_ast(file_ast);
        self.analyze_generic(&generic_ast)
    }

    /// Analyzes a generic `FileAst` for compliance violations.
    pub fn analyze_generic(&self, file_ast: &FileAst) -> Result<AnalysisResult, PrefixError> {
        let mut violations = Vec::new();
        // Load the prefix map once at the beginning.
        let prefix_map = prefix_manager::try_get_prefix_map()?;

        // Build the basic call graph for inter-procedural analysis
        let call_graph = crate::jurisdictions::eca::CallGraph::build_from_generic(file_ast);

        for func in &file_ast.functions {
            // Find all compliance prefixes in the function's doc comments
            for comment in &func.doc_comments {
                if let Some(prefix) = self.get_potential_prefix_from_string(comment) {
                    // Look up the prefix to find its jurisdiction
                    if let Some(prefix_info) = prefix_map.get(&prefix) {
                        let jurisdiction = match prefix_info.standard.as_str() {
                            "Eca" => Jurisdiction::Eca,
                            "Lgpd" => Jurisdiction::Lgpd,
                            _ => Jurisdiction::Generic,
                        };

                        // Dispatch to the correct jurisdiction if it's enabled
                        if self.config.enabled_jurisdictions.contains(&jurisdiction) {
                            match jurisdiction {
                                Jurisdiction::Eca => {
                                    violations.extend(crate::jurisdictions::eca::validate(
                                        func,
                                        prefix_info,
                                        &call_graph,
                                    ));
                                }
                                Jurisdiction::Lgpd => {
                                    violations.extend(crate::jurisdictions::lgpd::validate(
                                        func,
                                        prefix_info,
                                        &call_graph,
                                    ));
                                }
                                _ => {} // Other jurisdictions are not handled
                            }
                        }
                    }
                }
            }
        }

        let compliance_score = self.calculate_compliance_score(&violations);

        Ok(AnalysisResult {
            compliance_score,
            violations,
            suggestions: Vec::new(),  // Placeholder for future implementation
            warnings: Vec::new(),     // Placeholder for future implementation
            metadata: HashMap::new(), // Placeholder for future implementation
        })
    }

    /// Extracts a potential prefix string from a doc comment string.
    fn get_potential_prefix_from_string(&self, comment_text: &str) -> Option<String> {
        let parts: Vec<&str> = comment_text.trim().splitn(2, ':').collect();
        if parts.len() > 1 {
            return Some(parts[0].trim().to_string());
        }
        None
    }

    /// Calculates a compliance score based on violations.
    fn calculate_compliance_score(&self, violations: &[ComplianceViolation]) -> f64 {
        if violations.is_empty() {
            100.0
        } else {
            let total_weight: f64 = violations
                .iter()
                .map(|v| match v.severity {
                    RuleSeverity::Critical => 10.0,
                    RuleSeverity::High => 5.0,
                    RuleSeverity::Medium => 2.0,
                    RuleSeverity::Low => 1.0,
                    RuleSeverity::Info => 0.5,
                })
                .sum();
            (100.0 - (total_weight * 2.0)).max(0.0)
        }
    }

    /// Generates a compliance report in Markdown text format.
    pub fn generate_report(&self, result: &AnalysisResult) -> String {
        let mut report = String::new();
        report.push_str("# Hubstry-ISO_Code Compliance Report\n\n");
        report.push_str(&format!(
            "**Compliance Score:** {:.1}%\n\n",
            result.compliance_score
        ));

        if !result.violations.is_empty() {
            report.push_str(&format!("## Violations ({})\n\n", result.violations.len()));
            for violation in &result.violations {
                report.push_str(&format!(
                    "- **{}** [{}]: {}\n",
                    violation.severity, violation.rule_id, violation.message
                ));
                let line = violation.line.unwrap_or(0);
                let col = violation.column.unwrap_or(0);
                report.push_str(&format!("  *Location: Line {}, Column {}*\n", line, col));
                if let Some(suggestion) = &violation.suggestion {
                    report.push_str(&format!("  *Suggestion: {}*\n", suggestion));
                }
                report.push('\n');
            }
        }
        report
    }

    /// Generates a compliance report in JSON format.
    pub fn generate_json_report(&self, result: &AnalysisResult) -> String {
        serde_json::to_string_pretty(result).unwrap_or_else(|_| "{}".to_string())
    }

    /// Generates a compliance report in HTML format suitable for C-Levels.
    pub fn generate_html_report(&self, result: &AnalysisResult) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html lang=\"pt-BR\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("<title>Relatório de Compliance - Hubstry CaaS</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 0; padding: 20px; background-color: #f9f9fb; color: #333; }\n");
        html.push_str(".container { max-width: 800px; margin: auto; background: #fff; padding: 30px; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }\n");
        html.push_str(
            "h1 { color: #2c3e50; border-bottom: 2px solid #ecf0f1; padding-bottom: 10px; }\n",
        );
        html.push_str(".score-board { text-align: center; margin: 30px 0; padding: 20px; border-radius: 8px; }\n");
        html.push_str(".score-board.good { background-color: #d4edda; color: #155724; }\n");
        html.push_str(".score-board.warning { background-color: #fff3cd; color: #856404; }\n");
        html.push_str(".score-board.danger { background-color: #f8d7da; color: #721c24; }\n");
        html.push_str(".score-value { font-size: 48px; font-weight: bold; }\n");
        html.push_str(".violation { border: 1px solid #e2e8f0; border-left: 5px solid #e53e3e; margin-bottom: 20px; padding: 15px; border-radius: 4px; }\n");
        html.push_str(".violation h3 { margin-top: 0; color: #e53e3e; }\n");
        html.push_str(".meta { font-size: 0.9em; color: #718096; margin-bottom: 10px; }\n");
        html.push_str(".suggestion { background-color: #edf2f7; padding: 10px; border-radius: 4px; font-style: italic; }\n");
        html.push_str("</style>\n</head>\n<body>\n");

        html.push_str("<div class=\"container\">\n");
        html.push_str("<h1>Relatório Executivo de Compliance</h1>\n");

        let score_class = if result.compliance_score >= 90.0 {
            "good"
        } else if result.compliance_score >= 70.0 {
            "warning"
        } else {
            "danger"
        };

        html.push_str(&format!("<div class=\"score-board {}\">\n", score_class));
        html.push_str("<h2>Score Geral de Conformidade</h2>\n");
        html.push_str(&format!(
            "<div class=\"score-value\">{:.1}%</div>\n",
            result.compliance_score
        ));
        html.push_str("</div>\n");

        if !result.violations.is_empty() {
            html.push_str(&format!(
                "<h2>Violações Detectadas ({})</h2>\n",
                result.violations.len()
            ));
            for violation in &result.violations {
                html.push_str("<div class=\"violation\">\n");
                html.push_str(&format!(
                    "<h3>[{}] {}</h3>\n",
                    violation.rule_id, violation.severity
                ));
                html.push_str(&format!(
                    "<p><strong>Problema:</strong> {}</p>\n",
                    violation.message
                ));

                let line = violation.line.unwrap_or(0);
                let col = violation.column.unwrap_or(0);
                html.push_str(&format!(
                    "<div class=\"meta\">Localização: Linha {}, Coluna {}</div>\n",
                    line, col
                ));

                if let Some(suggestion) = &violation.suggestion {
                    html.push_str(&format!("<div class=\"suggestion\"><strong>Sugestão de Mitigação:</strong> {}</div>\n", suggestion));
                }
                html.push_str("</div>\n");
            }
        } else {
            html.push_str("<p>Nenhuma violação detectada. O código está em conformidade com as regras verificadas.</p>\n");
        }

        html.push_str("</div>\n</body>\n</html>\n");
        html
    }
}
