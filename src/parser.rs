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
    Equals,
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

    /// Peeks at the next character without consuming it
    fn peek_char(&self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.position)
        }
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

    /// Reads the content of a comment line until a newline
    fn read_comment(&mut self) -> String {
        let start_pos = self.position;
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.read_char();
        }
        self.input[start_pos..self.position - 1].trim().to_string()
    }

    /// Extracts a compliance prefix (e.g., "S.O.S") from a comment string.
    fn extract_prefix_from_comment(&self, comment: &str) -> Option<String> {
        let parts: Vec<&str> = comment.split(':').collect();
        if parts.len() > 1 {
            let potential_prefix = parts[0].trim();
            if self.is_compliance_prefix(potential_prefix) {
                return Some(potential_prefix.to_string());
            }
        }
        None
    }

    /// Gets the next token from the input
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char {
            None => Token::Eof,
            Some('/') => {
                if self.peek_char() == Some('/') {
                    // This is a comment, so we process it.
                    self.read_char(); // Consume the first '/'
                    self.read_char(); // Consume the second '/'
                    let comment_content = self.read_comment();

                    // Check if the comment contains a compliance prefix.
                    if let Some(prefix) = self.extract_prefix_from_comment(&comment_content) {
                        return Token::CompliancePrefix(prefix);
                    } else {
                        // It's a regular comment, so we ignore it and get the next real token.
                        return self.next_token();
                    }
                } else {
                    // It's something else, maybe division in the future. For now, invalid.
                    self.read_char();
                    Token::Invalid("/".to_string())
                }
            }
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
            Some('=') => {
                self.read_char();
                Token::Equals
            }
            Some('"') => Token::String(self.read_string()),
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let identifier = self.read_identifier();
                
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
            "S.O.S"
                | "G.D.P.R"
                | "Q.M.S"
                | "A.C.C"
                | "S.U.S"
                | "D.I.V"
                | "P.A.I.N.E.L"
                | "R.E.L.A.T.O"
                | "A.L.G.O.R.I.T.H.M"
                | "L.O.O.T.B.O.X"
                | "S.D.K.S.C.A.N"
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
            Token::If => self.parse_if_statement(),
            Token::Return => self.parse_return_statement(),
            Token::Identifier(_) => self.parse_expression_statement(),
            _ => {
                let error = format!("Unexpected token: {:?}", self.current_token);
                self.next_token();
                Err(error)
            }
        }
    }

    /// Parses a compliance-annotated statement, attaching the context to the statement itself.
    fn parse_compliance_statement(&mut self, prefix: String) -> Result<AstNode, String> {
        // Consume the prefix token.
        self.next_token();

        // Skip any newlines between the compliance comment and the actual statement.
        while self.current_token == Token::Newline {
            self.next_token();
        }

        // Now, parse the actual statement that is being annotated.
        match self.parse_statement() {
            Ok(mut statement_node) => {
                // Create the compliance context from the prefix.
                if let Some(context) = ComplianceContext::from_prefix(&prefix) {
                    // Add the context directly to the parsed statement node.
                    statement_node.add_compliance_context(context);
                }
                // Return the modified statement node, now with compliance context.
                Ok(statement_node)
            }
            Err(e) => Err(format!(
                "Expected a statement after compliance prefix '{}', but found error: {}",
                prefix, e
            )),
        }
    }

    /// Parses a function declaration
    fn parse_function(&mut self) -> Result<AstNode, String> {
        // The node's content will be the function name.
        let mut node;
        self.next_token(); // consume 'function'

        // Parse function name
        if let Token::Identifier(name) = &self.current_token {
            node = AstNode::new(NodeType::Function, name.clone());
            self.next_token();
        } else {
            return Err("Expected function name".to_string());
        }

        // Parse parameters (simplified)
        if self.current_token == Token::LeftParen {
            self.next_token();
            while self.current_token != Token::RightParen && self.current_token != Token::Eof {
                self.next_token();
            }
            if self.current_token == Token::RightParen {
                self.next_token();
            }
        }

        // Parse function body
        if self.current_token == Token::LeftBrace {
            let body_start_pos = self.lexer.position; // Position after '{'
            self.next_token(); // consume '{'

            let mut body_block = AstNode::new(NodeType::Block, "body".to_string());
            
            while self.current_token != Token::RightBrace && self.current_token != Token::Eof {
                while self.current_token == Token::Newline {
                    self.next_token();
                }
                if self.current_token == Token::RightBrace { break; }

                match self.parse_statement() {
                    Ok(stmt) => body_block.add_child(stmt),
                    Err(_) => self.next_token(),
                }
            }
            
            let body_end_pos = self.lexer.position - 1; // Position of '}'

            if self.current_token == Token::RightBrace {
                self.next_token(); // consume '}'
            }

            // Capture the raw text of the function body for textual analysis
            if body_end_pos > body_start_pos {
                node.raw_body = Some(self.lexer.input[body_start_pos..body_end_pos].to_string());
            }
            
            node.add_child(body_block);
        }

        Ok(node)
    }

    /// Parses a return statement
    fn parse_return_statement(&mut self) -> Result<AstNode, String> {
        self.next_token(); // consume 'return'
        let mut node = AstNode::new(NodeType::ReturnStatement, "return".to_string());

        // Parse the expression that is being returned.
        match self.parse_expression_statement() {
            Ok(expr_node) => node.add_child(expr_node),
            Err(e) => return Err(e),
        }

        // Semicolon is optional after a return statement in some contexts.
        if self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(node)
    }

    /// Parses an if statement
    fn parse_if_statement(&mut self) -> Result<AstNode, String> {
        self.next_token(); // consume 'if'
        let mut node = AstNode::new(NodeType::IfStatement, "if".to_string());

        // Parse condition and look for call expressions inside it
        if self.current_token == Token::LeftParen {
            self.next_token(); // consume '('
            while self.current_token != Token::RightParen && self.current_token != Token::Eof {
                // If we find an identifier that is part of a function call, parse it.
                if let Token::Identifier(_) = self.current_token {
                    if self.peek_token == Token::LeftParen {
                        if let Ok(call_expr) = self.parse_expression_statement() {
                            // Add the call expression from the condition as a child of the IfStatement node.
                            node.add_child(call_expr);
                            continue; // Continue parsing the rest of the condition
                        }
                    }
                }
                self.next_token();
            }
            if self.current_token == Token::RightParen {
                self.next_token(); // consume ')'
            }
        }

        // Parse 'then' block
        if self.current_token == Token::LeftBrace {
            self.next_token(); // consume '{'
            let mut body_block = AstNode::new(NodeType::Block, "then_block".to_string());
            while self.current_token != Token::RightBrace && self.current_token != Token::Eof {
                // Skip any newlines inside the block
                while self.current_token == Token::Newline {
                    self.next_token();
                }
                if self.current_token == Token::RightBrace { break; }

                match self.parse_statement() {
                    Ok(stmt) => body_block.add_child(stmt),
                    Err(_) => self.next_token(),
                }
            }
            if self.current_token == Token::RightBrace {
                self.next_token(); // consume '}'
            }
            node.add_child(body_block);
        }

        Ok(node)
    }

    /// Parses a variable declaration
    fn parse_variable_declaration(&mut self) -> Result<AstNode, String> {
        self.next_token(); // consume 'let'

        // The node's content is the variable name.
        let node;
        if let Token::Identifier(name) = &self.current_token {
            node = AstNode::new(NodeType::Variable, name.clone());
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

    /// Parses an expression statement, now with the ability to detect call expressions and assignments.
    fn parse_expression_statement(&mut self) -> Result<AstNode, String> {
        if let Token::Identifier(name) = self.current_token.clone() {
            // Check if it's a function call
            if self.peek_token == Token::LeftParen {
                self.next_token(); // consume identifier
                self.next_token(); // consume '('

                let node = AstNode::new(NodeType::CallExpression, name);

                // Skip arguments for now
                while self.current_token != Token::RightParen && self.current_token != Token::Eof {
                    self.next_token();
                }
                if self.current_token == Token::RightParen {
                    self.next_token();
                }

                if self.current_token == Token::Semicolon { self.next_token(); }
                return Ok(node);
            }
            // Check if it's an assignment by looking for the '=' token.
            else if self.peek_token == Token::Equals {
                self.next_token(); // consume identifier
                self.next_token(); // consume '='

                let node = AstNode::new(NodeType::AssignmentExpression, name);

                // Skip the rest of the expression
                while self.current_token != Token::Semicolon && self.current_token != Token::Newline && self.current_token != Token::Eof {
                    self.next_token();
                }
                if self.current_token == Token::Semicolon { self.next_token(); }
                return Ok(node);
            }
        }

        // Fallback for other expression types (currently just skips them)
        let node = AstNode::new(NodeType::Expression, "expression".to_string());
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
        let mut lexer = Lexer::new("// S.O.S: A security function\nfunction test() {}".to_string());
        assert_eq!(lexer.next_token(), Token::CompliancePrefix("S.O.S".to_string()));
        // The lexer should now see the newline after processing the comment line
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::Function);
    }

    #[test]
    fn test_parser_compliance_statement() {
        let mut parser = Parser::new("S.O.S let secure_var = 'test';".to_string());
        let result = parser.parse();
        assert!(!result.ast.children.is_empty());
    }
}


