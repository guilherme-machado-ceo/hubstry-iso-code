//! Basic compliance analysis example for Hubstry-ISO_Code
//! 
//! This example demonstrates how to:
//! - Create an AST node with compliance context
//! - Configure the semantic engine
//! - Perform compliance analysis
//! - Generate a compliance report

use hubstry_iso_code::models::*;
use hubstry_iso_code::semantic_engine::*;
use std::collections::HashMap;

fn main() {
    println!("🔍 Hubstry-ISO_Code Basic Analysis Example\n");

    // Create a sample AST node representing a function that handles passwords
    let password_function = AstNode {
        node_type: NodeType::Function,
        content: "function validatePassword(password) { return password.length > 8; }".to_string(),
        raw_body: Some("{ return password.length > 8; }".to_string()),
        children: vec![],
        compliance_context: vec![
            // Properly annotated with S.O.S (Security) compliance
            ComplianceContext {
                prefix: "S.O.S".to_string(),
                standard: ComplianceStandard::Security,
                rules: vec!["password_validation".to_string()],
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("min_length".to_string(), "8".to_string());
                    params.insert("complexity".to_string(), "medium".to_string());
                    params
                },
            }
        ],
        metadata: HashMap::new(),
    };

    // Create another node without proper compliance annotation
    let user_data_function = AstNode {
        node_type: NodeType::Function,
        content: "function storeUserEmail(email) { database.save(email); }".to_string(),
        raw_body: Some("{ database.save(email); }".to_string()),
        children: vec![],
        compliance_context: vec![], // Missing G.D.P.R annotation!
        metadata: HashMap::new(),
    };

    // Create a root AST node containing both functions
    let root_ast = AstNode {
        node_type: NodeType::Module,
        content: "User authentication module".to_string(),
        raw_body: None,
        children: vec![password_function, user_data_function],
        compliance_context: vec![],
        metadata: HashMap::new(),
    };

    // Configure the semantic engine
    let config = EngineConfig {
        enabled_standards: vec![
            ComplianceStandard::Security,
            ComplianceStandard::Privacy,
            ComplianceStandard::Quality,
        ],
        strict_mode: false,
        output_format: OutputFormat::Json,
        custom_rules: Vec::new(),
    };

    // Create and configure the semantic engine
    let engine = SemanticEngine::new(config);

    println!("📋 Analyzing code for compliance violations...\n");

    // Perform compliance analysis
    let analysis_result = engine.analyze(&root_ast);

    // Display results
    println!("📊 Analysis Results:");
    println!("   Compliance Score: {:.1}%", analysis_result.compliance_score);
    println!("   Violations Found: {}", analysis_result.violations.len());
    println!("   Warnings: {}", analysis_result.warnings.len());
    println!("   Suggestions: {}", analysis_result.suggestions.len());
    println!();

    // Show violations in detail
    if !analysis_result.violations.is_empty() {
        println!("🚨 Compliance Violations:");
        for (i, violation) in analysis_result.violations.iter().enumerate() {
            println!("   {}. [{}] {} - {}", 
                i + 1, 
                violation.severity, 
                violation.rule_id, 
                violation.message
            );
            if let Some(suggestion) = &violation.suggestion {
                println!("      💡 Suggestion: {}", suggestion);
            }
            println!();
        }
    }

    // Show warnings
    if !analysis_result.warnings.is_empty() {
        println!("⚠️  Warnings:");
        for (i, warning) in analysis_result.warnings.iter().enumerate() {
            println!("   {}. {}", i + 1, warning);
        }
        println!();
    }

    // Generate and display compliance report
    println!("📄 Generating Compliance Report...\n");
    let report = engine.generate_report(&analysis_result);
    println!("{}", report);

    // Show metadata
    println!("📋 Analysis Metadata:");
    for (key, value) in &analysis_result.metadata {
        println!("   {}: {}", key, value);
    }

    println!("\n✅ Analysis complete! Use the suggestions above to improve compliance.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_analysis_example() {
        // This test ensures the example code runs without panicking
        let config = EngineConfig::default();
        let engine = SemanticEngine::new(config);
        
        let test_node = AstNode {
            node_type: NodeType::Function,
            content: "test function".to_string(),
            raw_body: None,
            children: vec![],
            compliance_context: vec![],
            metadata: HashMap::new(),
        };
        
        let result = engine.analyze(&test_node);
        assert!(result.compliance_score >= 0.0);
        assert!(result.compliance_score <= 100.0);
    }
}