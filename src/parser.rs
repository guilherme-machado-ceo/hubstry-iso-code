//! Parser module for the Hubstry-ISO_Code framework.
//! This module now uses the `syn` crate to parse Rust source code into a robust AST.

use syn::{File, Result};

/// Parses a string of source code into a `syn::File` AST.
///
/// # Arguments
///
/// * `code` - A string slice that holds the source code to be parsed.
///
/// # Returns
///
/// A `Result` containing the parsed `syn::File` on success, or a `syn::Error` on failure.
pub fn parse(code: &str) -> Result<File> {
    syn::parse_file(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syn_parser_success() {
        let code = r#"
            // ECA.AGE.VERIFY: Test function
            fn my_function() {
                println!("Hello, world!");
            }
        "#;
        let result = parse(code);
        assert!(result.is_ok());
    }

    #[test]
    fn test_syn_parser_failure() {
        let code = "fn my_function() {"; // Missing closing brace
        let result = parse(code);
        assert!(result.is_err());
    }
}