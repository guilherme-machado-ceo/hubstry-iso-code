//! Advanced compliance analysis example for Hubstry-ISO_Code
//!
//! This example demonstrates more complex compliance scenarios with multiple standards,
//! custom rules, and batch analysis capabilities.

use hubstry_iso_code::models::*;
use hubstry_iso_code::semantic_engine::*;
use std::collections::HashMap;

fn main() {
    println!("🚀 Advanced Compliance Analysis Demo");
    println!("{}", "=".repeat(50));
    
    // Run different compliance scenarios
    demo_multiple_standards();
    demo_custom_rules();
    demo_batch_analysis();
    
    println!("\n✅ Advanced compliance analysis completed!");
}

fn demo_multiple_standards() {
    println!("\n📋 Demo 1: Multiple Compliance Standards");
    println!("{}", "-".repeat(40));
    
    // Create a function with security annotation
    let secure_function = AstNode {
        node_type: NodeType::Function,
        content: "// S.O.S: Secure authentication\nfunction authenticate(user, pass) { return validateCredentials(user, pass); }".to_string(),
        raw_body: None,
        children: Vec::new(),
        compliance_context: vec![ComplianceContext {
            prefix: "S.O.S".to_string(),
            standard: ComplianceStandard::Security,
            rules: vec!["secure_auth".to_string()],
            parameters: HashMap::new(),
        }],
        metadata: HashMap::new(),
    };
    
    // Configure engine with multiple standards
    let config = EngineConfig {
        enabled_standards: vec![
            ComplianceStandard::Security,
            ComplianceStandard::Privacy,
            ComplianceStandard::Quality,
        ],
        strict_mode: true,
        output_format: OutputFormat::Json,
        custom_rules: Vec::new(),
    };
    
    let engine = SemanticEngine::new(config);
    let result = engine.analyze(&secure_function);
    
    println!("Compliance Score: {:.1}%", result.compliance_score);
    println!("Violations: {}", result.violations.len());
    
    for violation in &result.violations {
        println!("  - [{}] {}", violation.severity, violation.message);
    }
}

fn demo_custom_rules() {
    println!("\n🔧 Demo 2: Custom Compliance Rules");
    println!("{}", "-".repeat(40));
    
    // Create a simple function node
    let test_function = AstNode {
        node_type: NodeType::Function,
        content: "function adminFunction() { return true; }".to_string(),
        raw_body: None,
        children: Vec::new(),
        compliance_context: Vec::new(),
        metadata: HashMap::new(),
    };
    
    // Create custom rule
    let custom_rule = ComplianceRule {
        id: "admin_security".to_string(),
        standard: ComplianceStandard::Security,
        severity: RuleSeverity::High,
        description: "Admin functions require security review".to_string(),
        validation_pattern: Some("admin".to_string()),
        remediation_hint: Some("Add security documentation".to_string()),
    };
    
    let config = EngineConfig {
         enabled_standards: vec![ComplianceStandard::Security],
         strict_mode: false,
         output_format: OutputFormat::PlainText,
         custom_rules: vec![custom_rule],
     };
    
    let engine = SemanticEngine::new(config);
    let result = engine.analyze(&test_function);
    
    println!("Custom Rule Analysis:");
    println!("Score: {:.1}%", result.compliance_score);
    println!("Violations: {}", result.violations.len());
}

fn demo_batch_analysis() {
    println!("\n📊 Demo 3: Batch Analysis");
    println!("{}", "-".repeat(40));
    
    // Create multiple functions for batch analysis
    let functions = vec![
        AstNode {
            node_type: NodeType::Function,
            content: "function secureLogin() { /* secure implementation */ }".to_string(),
            raw_body: None,
            children: Vec::new(),
            compliance_context: vec![ComplianceContext {
                prefix: "S.O.S".to_string(),
                standard: ComplianceStandard::Security,
                rules: vec!["secure_function".to_string()],
                parameters: HashMap::new(),
            }],
            metadata: HashMap::new(),
        },
        AstNode {
            node_type: NodeType::Function,
            content: "function unsafeFunction() { return password; }".to_string(),
            raw_body: None,
            children: Vec::new(),
            compliance_context: Vec::new(),
            metadata: HashMap::new(),
        },
    ];
    
    let config = EngineConfig::default();
     let engine = SemanticEngine::new(config);
    
    println!("Analyzing {} functions:", functions.len());
    
    for (i, function) in functions.iter().enumerate() {
        let result = engine.analyze(function);
        println!("  Function {}: Score {:.1}%, Violations: {}", 
                 i + 1, result.compliance_score, result.violations.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_advanced_compliance() {
        demo_multiple_standards();
        demo_custom_rules();
        demo_batch_analysis();
    }
}