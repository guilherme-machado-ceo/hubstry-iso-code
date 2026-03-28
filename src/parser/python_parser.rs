// src/parser/python_parser.rs
use super::{CallInfo, FunctionInfo, LanguageParser, StringLiteral};
use crate::parser::tree_sitter_utils::get_node_source;
use anyhow::{anyhow, Result};
use tree_sitter::{Language, Node, Parser, Tree};

pub struct PythonParser;

impl PythonParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PythonParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for PythonParser {
    fn language(&self) -> Language {
        tree_sitter_python::LANGUAGE.into()
    }

    fn language_name(&self) -> &str {
        "Python"
    }

    fn file_extensions(&self) -> &[&str] {
        &["py"]
    }

    fn parse_source(&self, source: &[u8]) -> Result<Tree> {
        let mut parser = Parser::new();
        parser
            .set_language(&self.language())
            .map_err(|e| anyhow!("Failed to set language: {}", e))?;
        parser
            .parse(source, None)
            .ok_or_else(|| anyhow!("Failed to parse Python source code"))
    }

    fn extract_functions(
        &self,
        tree: &Tree,
        source: &[u8],
        file_path: &str,
    ) -> Result<Vec<FunctionInfo>> {
        let mut funcs = Vec::new();
        let root = tree.root_node();
        let mut cursor = root.walk();

        for child in root.children(&mut cursor) {
             if child.kind() == "function_definition" {
                 let name = if let Some(n) = child.child_by_field_name("name") {
                     get_node_source(&n, source).to_string()
                 } else {
                     "anonymous".to_string()
                 };
                 funcs.push(FunctionInfo {
                     name,
                     annotations: vec![],
                     file_path: file_path.to_string(),
                     start_line: child.start_position().row + 1,
                     end_line: child.end_position().row + 1,
                     start_column: child.start_position().column + 1,
                     end_column: child.end_position().column + 1,
                     body_source: get_node_source(&child, source).to_string(),
                     calls: vec![],
                     string_literals: vec![],
                 });
             }
        }
        Ok(funcs)
    }

    fn extract_calls(&self, _node: &Node, _source: &[u8]) -> Vec<CallInfo> {
        vec![]
    }

    fn extract_string_literals(&self, _node: &Node, _source: &[u8]) -> Vec<StringLiteral> {
        vec![]
    }
}
