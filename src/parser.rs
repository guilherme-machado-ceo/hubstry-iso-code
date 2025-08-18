//! Parser module for the Hubstry-ISO_Code framework.
//! This module is responsible for parsing the custom ISO-Code syntax with compliance prefixes.

use crate::models::*;
use std::collections::HashMap;

/// Lexer for tokenizing ISO-Code input
#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

/// Token types for the ISO-Code language
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Compliance prefixes
    CompliancePrefix(String),
    // Identifiers and literals
    Identifier(String),
    String(String),
    Number(f64),
    // Symbols
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Dot,
    Colon,
    // Keywords
    Function,
    Let,
    If,
    Else,
    Return,
    // Special
    Newline,
    Eof,
    Invalid(String),
}

/// Parser for ISO-Code syntax
#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Lexer {
    /// Creates a new lexer with the given input
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            current_char: None,
        };
        lexer.read_char();
        lexer
    }

    /// Reads the next character from input
    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = self.input.chars().nth(self.position);
        }
        self.position += 1;
    }



    /// Skips whitespace characters (except newlines)
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() && ch != '\n' {
                self.read_char();
            } else {
                break;
            }
        }
    }

    /// Reads an identifier or keyword
    fn read_identifier(&mut self) -> String {
        let start_pos = self.position - 1;
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start_pos..self.position - 1].to_string()
    }

    /// Reads a string literal
    fn read_string(&mut self) -> String {
        self.read_char(); // Skip opening quote
        let start_pos = self.position - 1;
        while let Some(ch) = self.current_char {
            if ch == '"' {
                let result = self.input[start_pos..self.position - 1].to_string();
                self.read_char(); // Skip closing quote
                return result;
            }
            self.read_char();
        }
        // Unterminated string
        self.input[start_pos..].to_string()
    }

    /// Reads a number
    fn read_number(&mut self) -> f64 {
        let start_pos = self.position - 1;
        while let Some(ch) = self.current_char {
            if ch.is_numeric() || ch == '.' {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start_pos..self.position - 1]
            .parse()
            .unwrap_or(0.0)
    }

    /// Gets the next token from the input
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char {
            None => Token::Eof,
            Some('\n') => {
                self.read_char();
                Token::Newline
            }
            Some('(') => {
                self.read_char();
                Token::LeftParen
            }
            Some(')') => {
                self.read_char();
                Token::RightParen
            }
            Some('{') => {
                self.read_char();
                Token::LeftBrace
            }
            Some('}') => {
                self.read_char();
                Token::RightBrace
            }
            Some(';') => {
                self.read_char();
                Token::Semicolon
            }
            Some(',') => {
                self.read_char();
                Token::Comma
            }
            Some('.') => {
                self.read_char();
                Token::Dot
            }
            Some(':') => {
                self.read_char();
                Token::Colon
            }
            Some('"') => Token::String(self.read_string()),
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let identifier = self.read_identifier();
                
                // Check if it's a compliance prefix
                if self.is_compliance_prefix(&identifier) {
                    Token::CompliancePrefix(identifier)
                } else {
                    // Check if it's a keyword
                    match identifier.as_str() {
                        "function" | "fn" => Token::Function,
                        "let" => Token::Let,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Return,
                        _ => Token::Identifier(identifier),
                    }
                }
            }
            Some(ch) if ch.is_numeric() => Token::Number(self.read_number()),
            Some(ch) => {
                self.read_char();
                Token::Invalid(ch.to_string())
            }
        }
    }

    /// Checks if an identifier is a compliance prefix
    fn is_compliance_prefix(&self, identifier: &str) -> bool {
        matches!(
            identifier,
            "S.O.S" | "G.D.P.R" | "Q.M.S" | "A.C.C" | "S.U.S" | "D.I.V"
        )
    }
}

impl Parser {
    /// Creates a new parser with the given input
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            peek_token,
        }
    }

    /// Advances to the next token
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    /// Parses the input and returns a ParseResult
    pub fn parse(&mut self) -> ParseResult {
        let mut ast = AstNode::new(NodeType::Root, "root".to_string());
        let violations = Vec::new();
        let mut warnings = Vec::new();
        let mut metadata = HashMap::new();

        // Parse statements until EOF
        while self.current_token != Token::Eof {
            match self.parse_statement() {
                Ok(stmt) => ast.add_child(stmt),
                Err(error) => warnings.push(error),
            }
            
            // Skip newlines
            while self.current_token == Token::Newline {
                self.next_token();
            }
        }

        metadata.insert("parser_version".to_string(), "0.1.0".to_string());
        metadata.insert("parsed_at".to_string(), chrono::Utc::now().to_rfc3339());

        ParseResult {
            ast,
            compliance_violations: violations,
            warnings,
            metadata,
        }
    }

    /// Parses a statement
    fn parse_statement(&mut self) -> Result<AstNode, String> {
        match &self.current_token {
            Token::CompliancePrefix(prefix) => self.parse_compliance_statement(prefix.clone()),
            Token::Function => self.parse_function(),
            Token::Let => self.parse_variable_declaration(),
            Token::Identifier(_) => self.parse_expression_statement(),
            _ => {
                let error = format!("Unexpected token: {:?}", self.current_token);
                self.next_token();
                Err(error)
            }
        }
    }

    /// Parses a compliance-annotated statement
    fn parse_compliance_statement(&mut self, prefix: String) -> Result<AstNode, String> {
        let mut node = AstNode::new(NodeType::ComplianceComment, prefix.clone());
        
        // Add compliance context
        if let Some(context) = ComplianceContext::from_prefix(&prefix) {
            node.add_compliance_context(context);
        }

        self.next_token(); // consume prefix
        
        // Parse the actual statement that follows
        if self.current_token != Token::Eof {
            match self.parse_statement() {
                Ok(stmt) => node.add_child(stmt),
                Err(e) => return Err(e),
            }
        }

        Ok(node)
    }

    /// Parses a function declaration
    fn parse_function(&mut self) -> Result<AstNode, String> {
        let mut node = AstNode::new(NodeType::Function, "function".to_string());
        self.next_token(); // consume 'function'

        // Parse function name
        if let Token::Identifier(name) = &self.current_token {
            node.metadata.insert("name".to_string(), name.clone());
            self.next_token();
        } else {
            return Err("Expected function name".to_string());
        }

        // Parse parameters (simplified)
        if self.current_token == Token::LeftParen {
            self.next_token();
            // Skip parameter parsing for now
            while self.current_token != Token::RightParen && self.current_token != Token::Eof {
                self.next_token();
            }
            if self.current_token == Token::RightParen {
                self.next_token();
            }
        }

        // Parse function body
        if self.current_token == Token::LeftBrace {
            self.next_token();
            let mut body = AstNode::new(NodeType::Block, "body".to_string());
            
            while self.current_token != Token::RightBrace && self.current_token != Token::Eof {
                match self.parse_statement() {
                    Ok(stmt) => body.add_child(stmt),
                    Err(_) => self.next_token(), // Skip invalid statements
                }
            }
            
            if self.current_token == Token::RightBrace {
                self.next_token();
            }
            
            node.add_child(body);
        }

        Ok(node)
    }

    /// Parses a variable declaration
    fn parse_variable_declaration(&mut self) -> Result<AstNode, String> {
        let mut node = AstNode::new(NodeType::Variable, "variable".to_string());
        self.next_token(); // consume 'let'

        // Parse variable name
        if let Token::Identifier(name) = &self.current_token {
            node.metadata.insert("name".to_string(), name.clone());
            self.next_token();
        } else {
            return Err("Expected variable name".to_string());
        }

        // Skip assignment for now
        while self.current_token != Token::Semicolon && self.current_token != Token::Newline && self.current_token != Token::Eof {
            self.next_token();
        }

        if self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(node)
    }

    /// Parses an expression statement
    fn parse_expression_statement(&mut self) -> Result<AstNode, String> {
        let node = AstNode::new(NodeType::Expression, "expression".to_string());
        
        // Skip to next statement for now
        while self.current_token != Token::Semicolon && self.current_token != Token::Newline && self.current_token != Token::Eof {
            self.next_token();
        }

        if self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_compliance_prefix() {
        let mut lexer = Lexer::new("S.O.S function test() {}".to_string());
        assert_eq!(lexer.next_token(), Token::CompliancePrefix("S.O.S".to_string()));
        assert_eq!(lexer.next_token(), Token::Function);
    }

    #[test]
    fn test_parser_compliance_statement() {
        let mut parser = Parser::new("S.O.S let secure_var = 'test';".to_string());
        let result = parser.parse();
        assert!(!result.ast.children.is_empty());
    }
}


