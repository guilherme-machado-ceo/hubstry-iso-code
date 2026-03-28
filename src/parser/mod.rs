// src/parser/mod.rs
pub mod rust_parser;
pub mod js_parser;
pub mod python_parser;
pub mod tree_sitter_utils;

use anyhow::Result;
use tree_sitter::{Language, Node, Tree};

/// Informações extraídas de uma função/método
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub annotations: Vec<String>, // Doc comments, decorators, JSDoc
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub start_column: usize,
    pub end_column: usize,
    pub body_source: String, // Código-fonte do corpo
    pub calls: Vec<CallInfo>,
    pub string_literals: Vec<StringLiteral>,
}

#[derive(Debug, Clone)]
pub struct CallInfo {
    pub function_name: String,
    pub full_expression: String, // Ex: "serpro_client.verify_age(cpf)"
    pub arguments: Vec<String>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
    pub line: usize,
    pub column: usize,
}

/// Trait que todo parser de linguagem deve implementar
pub trait LanguageParser: Send + Sync {
    fn language(&self) -> Language;
    fn language_name(&self) -> &str;
    fn file_extensions(&self) -> &[&str];
    fn parse_source(&self, source: &[u8]) -> Result<Tree>;
    fn extract_functions(
        &self,
        tree: &Tree,
        source: &[u8],
        file_path: &str,
    ) -> Result<Vec<FunctionInfo>>;
    fn extract_calls(&self, node: &Node, source: &[u8]) -> Vec<CallInfo>;
    fn extract_string_literals(&self, node: &Node, source: &[u8]) -> Vec<StringLiteral>;
}

/// Registry de parsers disponíveis
pub struct ParserRegistry {
    parsers: Vec<Box<dyn LanguageParser>>,
}

impl ParserRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            parsers: Vec::new(),
        };
        reg.register(Box::new(rust_parser::RustParser::new()));
        reg.register(Box::new(js_parser::JsParser::new()));
        reg.register(Box::new(python_parser::PythonParser::new()));
        reg
    }

    pub fn register(&mut self, parser: Box<dyn LanguageParser>) {
        self.parsers.push(parser);
    }

    pub fn parser_for_extension(&self, ext: &str) -> Option<&dyn LanguageParser> {
        self.parsers
            .iter()
            .find(|p| p.file_extensions().contains(&ext))
            .map(|p| p.as_ref())
    }

    pub fn parser_for_language(&self, lang: &str) -> Option<&dyn LanguageParser> {
        self.parsers
            .iter()
            .find(|p| p.language_name().eq_ignore_ascii_case(lang))
            .map(|p| p.as_ref())
    }
}

impl Default for ParserRegistry {
    fn default() -> Self {
        Self::new()
    }
}
