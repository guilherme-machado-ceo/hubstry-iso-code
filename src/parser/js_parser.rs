// src/parser/js_parser.rs
use super::{CallInfo, FunctionInfo, LanguageParser, StringLiteral};
use crate::parser::tree_sitter_utils::get_node_source;
use anyhow::{anyhow, Result};
use tree_sitter::{Language, Node, Parser, Tree};

pub struct JsParser;

impl JsParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsParser {
    fn default() -> Self {
        Self::new()
    }
}

impl JsParser {
    fn collect_functions<'a>(
        &self,
        node: Node<'a>,
        source: &[u8],
        file_path: &str,
        funcs: &mut Vec<FunctionInfo>,
    ) {
        let is_function = node.kind() == "function_declaration"
            || node.kind() == "arrow_function"
            || node.kind() == "method_definition";

        if is_function {
            let name = if let Some(name_node) = node.child_by_field_name("name") {
                get_node_source(&name_node, source).to_string()
            } else {
                "anonymous".to_string()
            };

            // Collect preceding comments (JSDoc, etc.)
            let mut annotations = Vec::new();
            let mut prev = node.prev_sibling();
            while let Some(p) = prev {
                if matches!(p.kind(), "comment" | "decorator") {
                    annotations.push(get_node_source(&p, source).to_string());
                    prev = p.prev_sibling();
                } else {
                    break;
                }
            }

            let body_source = if let Some(body_node) = node.child_by_field_name("body") {
                get_node_source(&body_node, source).to_string()
            } else {
                get_node_source(&node, source).to_string() // For expression body arrow functions
            };

            let calls = self.extract_calls(&node, source);
            let string_literals = self.extract_string_literals(&node, source);

            funcs.push(FunctionInfo {
                name,
                annotations,
                file_path: file_path.to_string(),
                start_line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
                start_column: node.start_position().column + 1,
                end_column: node.end_position().column + 1,
                body_source,
                calls,
                string_literals,
            });
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.collect_functions(child, source, file_path, funcs);
        }
    }

    fn extract_calls_recursive(&self, node: &Node, source: &[u8], calls: &mut Vec<CallInfo>) {
        if node.kind() == "call_expression" {
            if let Some(func_node) = node.child_by_field_name("function") {
                let function_name = get_node_source(&func_node, source).to_string();
                let full_expression = get_node_source(node, source).to_string();
                let mut arguments = Vec::new();

                if let Some(args_node) = node.child_by_field_name("arguments") {
                    let mut cursor = args_node.walk();
                    for child in args_node.children(&mut cursor) {
                        if child.is_named() {
                            arguments.push(get_node_source(&child, source).to_string());
                        }
                    }
                }

                calls.push(CallInfo {
                    function_name,
                    full_expression,
                    arguments,
                    line: node.start_position().row + 1,
                    column: node.start_position().column + 1,
                });
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_calls_recursive(&child, source, calls);
        }
    }

    fn extract_strings_recursive(
        &self,
        node: &Node,
        source: &[u8],
        strings: &mut Vec<StringLiteral>,
    ) {
        if node.kind() == "string" || node.kind() == "template_string" {
            strings.push(StringLiteral {
                value: get_node_source(node, source).to_string(),
                line: node.start_position().row + 1,
                column: node.start_position().column + 1,
            });
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_strings_recursive(&child, source, strings);
        }
    }
}

impl LanguageParser for JsParser {
    fn language(&self) -> Language {
        tree_sitter_javascript::LANGUAGE.into()
    }

    fn language_name(&self) -> &str {
        "JavaScript"
    }

    fn file_extensions(&self) -> &[&str] {
        &["js", "jsx", "ts", "tsx", "mjs"]
    }

    fn parse_source(&self, source: &[u8]) -> Result<Tree> {
        let mut parser = Parser::new();
        // Since TS syntax is mostly a superset with type annotations, we might
        // technically need tree-sitter-typescript for deep TS files, but JS parses most correctly.
        // For the sake of the MVP JS parser, this acts as the base.
        parser
            .set_language(&self.language())
            .map_err(|e| anyhow!("Failed to set JS language: {}", e))?;
        parser
            .parse(source, None)
            .ok_or_else(|| anyhow!("Failed to parse JS source code"))
    }

    fn extract_functions(
        &self,
        tree: &Tree,
        source: &[u8],
        file_path: &str,
    ) -> Result<Vec<FunctionInfo>> {
        let mut functions = Vec::new();
        self.collect_functions(tree.root_node(), source, file_path, &mut functions);
        Ok(functions)
    }

    fn extract_calls(&self, node: &Node, source: &[u8]) -> Vec<CallInfo> {
        let mut calls = Vec::new();
        self.extract_calls_recursive(node, source, &mut calls);
        calls
    }

    fn extract_string_literals(&self, node: &Node, source: &[u8]) -> Vec<StringLiteral> {
        let mut strings = Vec::new();
        self.extract_strings_recursive(node, source, &mut strings);
        strings
    }
}
