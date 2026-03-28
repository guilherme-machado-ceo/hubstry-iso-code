//! Abstract Syntax Tree (AST) representations.
//! This module provides a generic, language-agnostic representation of an AST
//! to decouple the core validation logic from any specific parser like `syn`.

/// Represents a source code file.
#[derive(Debug, Clone)]
pub struct FileAst {
    pub functions: Vec<FunctionAst>,
}

/// Represents a function or method.
#[derive(Debug, Clone)]
pub struct FunctionAst {
    pub name: String,
    pub doc_comments: Vec<String>,
    pub called_functions: Vec<String>,
    pub line: usize,
    pub column: usize,
}
