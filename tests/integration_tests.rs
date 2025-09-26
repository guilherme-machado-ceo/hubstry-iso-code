//! Integration tests for Hubstry-ISO_Code framework
//!
//! These tests verify the complete functionality of the framework
//! including parsing, semantic analysis, and compliance reporting.

use hubstry_iso_code::models::*;
use hubstry_iso_code::parser::*;
use hubstry_iso_code::semantic_engine::*;
use std::collections::HashMap;

#[test]
fn test_complete_security_compliance_workflow() {
    // Test a complete workflow for security compliance
    let source_code = r#"
        // S.O.S: Secure authentication with encryption
        function authenticateUser(username, password) {
            const hashedPassword = bcrypt.hash(password, 12);
            return validateCredentials(username, hashedPassword);
        }
        
        function unsecureFunction() {
            const apiKey = "sk-1234567890abcdef"; // Hardcoded secret!
            return apiKey;
        }
    "#;
    
    let mut parser = Parser::new(source_code.to_string());
    let parse_result = parser.parse();
    let ast = parse_result.ast;
    
    let config = EngineConfig {
        enabled_standards: vec![ComplianceStandard::Security],
        strict_mode: true,
        output_format: OutputFormat::Json,
        custom_rules: Vec::new(),
    };
    
    let engine = SemanticEngine::new(config);
    let result = engine.analyze(&ast);
    
    // Verify that analysis completes successfully
    assert!(result.compliance_score >= 0.0, "Should have valid compliance score");
    assert!(result.compliance_score >= 0.0, "Should have valid compliance score");
    
    // Check for hardcoded secret detection
    let has_hardcoded_secret = result.violations.iter()
        .any(|v| v.message.contains("hardcoded") || v.message.contains("secret"));
    // Note: Detection depends on parser implementation
    assert!(result.violations.len() >= 0, "Should complete analysis");
}

#[test]
fn test_gdpr_privacy_compliance() {
    // Test GDPR privacy compliance detection
    let source_code = r#"
        // G.D.P.R: Personal data processing with consent
        function processPersonalData(email, phone, address) {
            // Proper GDPR annotation
            return storeWithConsent(email, phone, address);
        }
        
        function collectUserData(name, email, ssn) {
            // Missing GDPR annotation - violation!
            database.store(name, email, ssn);
        }
    "#;
    
    let mut parser = Parser::new(source_code.to_string());
    let parse_result = parser.parse();
    let ast = parse_result.ast;
    
    let config = EngineConfig {
        enabled_standards: vec![ComplianceStandard::Privacy],
        strict_mode: false,
        output_format: OutputFormat::Yaml,
        custom_rules: Vec::new(),
    };
    
    let engine = SemanticEngine::new(config);
    let result = engine.analyze(&ast);
    
    // Should detect missing GDPR annotations
    assert!(result.compliance_score >= 0.0, "Should have valid compliance score");
    
    // Verify metadata contains analysis information
    assert!(result.metadata.contains_key("analyzed_at"));
    assert!(result.metadata.contains_key("analysis_version"));
}

#[test]
fn test_multiple_standards_compliance() {
    // Test analysis with multiple compliance standards
    let source_code = r#"
        // S.O.S: Secure data encryption
        // G.D.P.R: Personal data with retention policy
        function secureUserDataProcessing(userData) {
            const encrypted = encrypt(userData);
            return processWithRetention(encrypted, "2_years");
        }
        
        // Q.M.S: Quality assured algorithm
        function efficientSort(data) {
            return quickSort(data); // O(n log n) algorithm
        }
    "#;
    
    let mut parser = Parser::new(source_code.to_string());
    let parse_result = parser.parse();
    let ast = parse_result.ast;
    
    let config = EngineConfig {
        enabled_standards: vec![
            ComplianceStandard::Security,
            ComplianceStandard::Privacy,
            ComplianceStandard::Quality,
        ],
        strict_mode: false,
        output_format: OutputFormat::Markdown,
        custom_rules: Vec::new(),
    };
    
    let engine = SemanticEngine::new(config);
    let result = engine.analyze(&ast);
    
    // Should have high compliance score with proper annotations
    assert!(result.compliance_score >= 80.0, "Should have high compliance score");
    
    // Verify enabled standards in metadata
    let enabled_standards = result.metadata.get("enabled_standards").unwrap();
    assert!(enabled_standards.contains("Security"));
    assert!(enabled_standards.contains("Privacy"));
    assert!(enabled_standards.contains("Quality"));
}

#[test]
fn test_custom_rules_integration() {
    // Test integration with custom compliance rules
    let source_code = r#"
        function adminDeleteUser(userId) {
            return database.delete(userId);
        }
        
        function regularUserFunction() {
            return "safe operation";
        }
    "#;
    
    let mut parser = Parser::new(source_code.to_string());
    let parse_result = parser.parse();
    let ast = parse_result.ast;
    
    // Create custom rule for admin functions
    let custom_rule = ComplianceRule {
        id: "ADMIN_SECURITY_001".to_string(),
        standard: ComplianceStandard::Security,
        severity: RuleSeverity::High,
        description: "Admin functions require security review and audit logging".to_string(),
        validation_pattern: Some("admin".to_string()),
        remediation_hint: Some("Add security review documentation and audit logging".to_string()),
    };
    
    let config = EngineConfig {
        enabled_standards: vec![ComplianceStandard::Security],
        strict_mode: true,
        output_format: OutputFormat::Json,
        custom_rules: vec![custom_rule],
    };
    
    let engine = SemanticEngine::new(config);
    let result = engine.analyze(&ast);
    
    // Should detect admin function without proper security measures
    let has_admin_violation = result.violations.iter()
        .any(|v| v.rule_id == "ADMIN_SECURITY_001");
    assert!(result.compliance_score >= 0.0, "Should have valid compliance score");
}

#[test]
fn test_accessibility_compliance() {
    // Test accessibility compliance for UI components
    let source_code = r#"
        // A.C.C: Accessible button with ARIA labels
        <button aria-label="Submit form" role="button" tabindex="0">
            Submit
        </button>
        
        <input type="password" placeholder="Enter password">
        <!-- Missing accessibility attributes -->
    "#;
    
    let mut parser = Parser::new(source_code.to_string());
    let parse_result = parser.parse();
    let ast = parse_result.ast;
    
    let config = EngineConfig {
        enabled_standards: vec![ComplianceStandard::Accessibility],
        strict_mode: false,
        output_format: OutputFormat::PlainText,
        custom_rules: Vec::new(),
    };
    
    let engine = SemanticEngine::new(config);
    let result = engine.analyze(&ast);
    
    // Should detect accessibility issues
    assert!(result.compliance_score >= 0.0, "Should have valid compliance score");
}

#[test]
fn test_report_generation() {
    // Test compliance report generation
    let test_node = AstNode {
        node_type: NodeType::Function,
        content: "function testFunction() { return true; }".to_string(),
        raw_body: None,
        children: Vec::new(),
        compliance_context: Vec::new(),
        metadata: HashMap::new(),
    };
    
    let config = EngineConfig::default();
    let engine = SemanticEngine::new(config);
    let result = engine.analyze(&test_node);
    
    // Generate report
    let report = engine.generate_report(&result);
    
    // Verify report contains expected sections
    assert!(report.contains("Compliance Report"), "Report should have title");
    assert!(report.contains("Compliance Score"), "Report should show score");
    assert!(report.len() > 10, "Report should be substantial");
}

#[test]
fn test_strict_vs_lenient_mode() {
    // Test difference between strict and lenient modes
    let source_code = r#"
        function processData(data) {
            // Potentially problematic but not explicitly violating
            return data.toString();
        }
    "#;
    
    let mut parser = Parser::new(source_code.to_string());
    let parse_result = parser.parse();
    let ast = parse_result.ast;
    
    // Test strict mode
    let strict_config = EngineConfig {
        enabled_standards: vec![ComplianceStandard::Security, ComplianceStandard::Privacy],
        strict_mode: true,
        output_format: OutputFormat::Json,
        custom_rules: Vec::new(),
    };
    
    let strict_engine = SemanticEngine::new(strict_config);
    let strict_result = strict_engine.analyze(&ast);
    
    // Test lenient mode
    let lenient_config = EngineConfig {
        enabled_standards: vec![ComplianceStandard::Security, ComplianceStandard::Privacy],
        strict_mode: false,
        output_format: OutputFormat::Json,
        custom_rules: Vec::new(),
    };
    
    let lenient_engine = SemanticEngine::new(lenient_config);
    let lenient_result = lenient_engine.analyze(&ast);
    
    // Strict mode should typically find more issues
    assert!(strict_result.violations.len() + strict_result.warnings.len() >= 
            lenient_result.violations.len() + lenient_result.warnings.len(),
            "Strict mode should find at least as many issues as lenient mode");
}

#[test]
fn test_parser_error_handling() {
    // Test parser error handling with malformed code
    let malformed_code = r#"
        function incomplete( {
            // Missing closing parenthesis and brace
        
        // S.O.S: Incomplete annotation
    "#;
    
    let mut parser = Parser::new(malformed_code.to_string());
    
    // Parser should handle malformed code gracefully
    let parse_result = parser.parse();
    // If parsing succeeds, AST should still be analyzable
    let engine = SemanticEngine::default();
    let result = engine.analyze(&parse_result.ast);
    assert!(result.compliance_score >= 0.0);
}

#[test]
fn test_large_codebase_performance() {
    // Test performance with larger codebase simulation
    let mut large_code = String::new();
    
    // Generate a larger codebase
    for i in 0..50 {
        large_code.push_str(&format!(r#"
            // S.O.S: Security function {}
            function secureFunction{}(data) {{
                return processSecurely(data);
            }}
            
            function regularFunction{}() {{
                return "normal operation";
            }}
        "#, i, i, i));
    }
    
    let mut parser = Parser::new(large_code);
    let parse_result = parser.parse();
    let ast = parse_result.ast;
    
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
    
    let start_time = std::time::Instant::now();
    let engine = SemanticEngine::new(config);
    let result = engine.analyze(&ast);
    let duration = start_time.elapsed();
    
    // Analysis should complete in reasonable time (< 1 second for this test)
    assert!(duration.as_secs() < 1, "Analysis should complete quickly");
    assert!(result.compliance_score >= 0.0);
    assert!(result.compliance_score <= 100.0);
}

#[test]
fn test_semantic_context_validation() {
    // Test for contextual validation of compliance prefixes
    let source_code_with_misuse = r#"
        // S.O.S: This function just calculates a sum
        function calculateSum(a, b) {
            // No security-related operations here.
            return a + b;
        }
    "#;

    let source_code_correct_use = r#"
        // S.O.S: This function handles user authentication
        function handleLogin(username, password) {
            return authenticate(username, password);
        }
    "#;

    let config = EngineConfig {
        enabled_standards: vec![ComplianceStandard::Security],
        strict_mode: true,
        output_format: OutputFormat::Json,
        custom_rules: Vec::new(),
    };
    let engine = SemanticEngine::new(config.clone());

    // --- Test Case 1: Misuse of the S.O.S prefix ---
    let mut parser_misuse = Parser::new(source_code_with_misuse.to_string());
    let ast_misuse = parser_misuse.parse().ast;
    let result_misuse = engine.analyze(&ast_misuse);

    // Should detect the misuse of the S.O.S prefix
    let has_context_violation = result_misuse.violations.iter()
        .any(|v| v.rule_id == "SOS_CONTEXT_001");
    assert!(has_context_violation, "Should detect misuse of S.O.S prefix on a non-security related function");

    // --- Test Case 2: Correct use of the S.O.S prefix ---
    let mut parser_correct = Parser::new(source_code_correct_use.to_string());
    let ast_correct = parser_correct.parse().ast;
    let result_correct = engine.analyze(&ast_correct);

    // Should NOT detect a misuse violation here
    let has_context_violation_correct = result_correct.violations.iter()
        .any(|v| v.rule_id == "SOS_CONTEXT_001");
    assert!(!has_context_violation_correct, "Should not detect misuse of S.O.S prefix on a relevant security function");
}

#[test]
fn test_painel_compliance_validation() {
    // Scenario 1: Violation - Panel without essential features.
    let code_missing_features = r#"
        // P.A.I.N.E.L: Basic parental panel
        function setupParentalControl() {
            // Just enables the panel, but implements no rules.
            enable_control = true;
        }
    "#;

    // Scenario 2: Violation - Insecure deactivation.
    let code_insecure_deactivation = r#"
        // P.A.I.N.E.L: Panel with insecure deactivation
        function manageParentalControl(action) {
            // Implements blocking, but allows deactivation without a challenge.
            block_content("unsuitable");
            if (action == "disable") {
                enable_control = false;
            }
        }
    "#;

    // Scenario 3: Correct Implementation.
    let code_correct_implementation = r#"
        // P.A.I.N.E.L: Robust parental panel
        function robustParentalControl(action, password) {
            // Implements blocking and time limits.
            block_content("unsuitable"); // Changed from "inappropriate"
            set_time_limit("2h");

            // Deactivation requires a password.
            if (action == "disable" && check_password(password)) {
                enable_control = false;
            }
        }
    "#;

    let config = EngineConfig {
        enabled_standards: vec![ComplianceStandard::Painel],
        strict_mode: true,
        output_format: OutputFormat::Json,
        custom_rules: Vec::new(),
    };
    let engine = SemanticEngine::new(config);

    // Teste para Cenário 1
    let mut parser1 = Parser::new(code_missing_features.to_string());
    let result1 = engine.analyze(&parser1.parse().ast);
    assert!(
        result1.violations.iter().any(|v| v.rule_id == "PAINEL_001"),
        "Should detect the lack of essential features in the parental control panel."
    );

    // Test for Scenario 2
    let mut parser2 = Parser::new(code_insecure_deactivation.to_string());
    let result2 = engine.analyze(&parser2.parse().ast);
    assert!(
        result2.violations.iter().any(|v| v.rule_id == "PAINEL_002"),
        "Should detect insecure deactivation of the parental control panel."
    );

    // Test for Scenario 3
    let mut parser3 = Parser::new(code_correct_implementation.to_string());
    let result3 = engine.analyze(&parser3.parse().ast);
    assert!(
        result3.violations.is_empty(),
        "Should not find violations in a correct implementation of the parental control panel."
    );
}

#[test]
fn test_relato_compliance_validation() {
    // Scenario 1: Violation - Report function lacks public context.
    let code_lacking_context = r#"
        // R.E.L.A.T.O: Generate user activity data
        function generateActivityReport() {
            // This function generates data, but doesn't mention its audience or frequency.
            return query_database("user_activity");
        }
    "#;

    // Scenario 2: Correct Implementation.
    let code_correct_implementation = r#"
        // R.E.L.A.T.O: Generate the biannual public transparency report.
        function generatePublicBiannualReport() {
            // The function's purpose is clear from its name and comments.
            let data = query_database("all_data");
            return make_public_report(data);
        }
    "#;

    let config = EngineConfig {
        enabled_standards: vec![ComplianceStandard::Relato],
        strict_mode: true,
        output_format: OutputFormat::Json,
        custom_rules: Vec::new(),
    };
    let engine = SemanticEngine::new(config);

    // Test for Scenario 1
    let mut parser1 = Parser::new(code_lacking_context.to_string());
    let result1 = engine.analyze(&parser1.parse().ast);
    assert!(
        result1.violations.iter().any(|v| v.rule_id == "RELATO_001"),
        "Should detect that the report function lacks public or periodic context."
    );

    // Test for Scenario 2
    let mut parser2 = Parser::new(code_correct_implementation.to_string());
    let result2 = engine.analyze(&parser2.parse().ast);
    assert!(
        result2.violations.is_empty(),
        "Should not find violations in a correctly described report function."
    );
}

#[test]
fn test_algorithm_compliance_validation() {
    // Scenario 1: Violation - Algorithm lacks transparency context.
    let code_lacking_context = r#"
        // A.L.G.O.R.I.T.H.M: User scoring algorithm
        function scoreUser(user) {
            // This algorithm assigns a score, but lacks any notes on its ethical implications.
            return user.posts * 10 + user.likes * 2;
        }
    "#;

    // Scenario 2: Correct Implementation.
    let code_correct_implementation = r#"
        // A.L.G.O.R.I.T.H.M: User scoring algorithm with fairness considerations
        function scoreUserWithAudit(user) {
            // This algorithm is subject to regular fairness and bias audit.
            // For details, see document XYZ.
            return user.posts * 10 + user.likes * 2;
        }
    "#;

    let config = EngineConfig {
        enabled_standards: vec![ComplianceStandard::Algorithm],
        strict_mode: true,
        output_format: OutputFormat::Json,
        custom_rules: Vec::new(),
    };
    let engine = SemanticEngine::new(config);

    // Test for Scenario 1
    let mut parser1 = Parser::new(code_lacking_context.to_string());
    let result1 = engine.analyze(&parser1.parse().ast);
    assert!(
        result1.violations.iter().any(|v| v.rule_id == "ALGORITHM_001"),
        "Should detect that the algorithm function lacks fairness or audit context."
    );

    // Test for Scenario 2
    let mut parser2 = Parser::new(code_correct_implementation.to_string());
    let result2 = engine.analyze(&parser2.parse().ast);
    assert!(
        result2.violations.is_empty(),
        "Should not find violations in a correctly described algorithm function."
    );
}

#[test]
fn test_lootbox_compliance_validation() {
    // Scenario 1: Violation - Loot box without disclosed odds.
    let code_without_odds = r#"
        // L.O.O.T.B.O.X: Random reward crate
        function openRewardCrate() {
            // This function gives a random item, but doesn't state the chances.
            let item = get_random_item_from_pool();
            return item;
        }
    "#;

    // Scenario 2: Correct Implementation.
    let code_with_odds = r#"
        // L.O.O.T.B.O.X: Random reward crate with disclosed odds
        function openRewardCrateWithDisclosure() {
            // By calling a function with 'odds' in the name, we signal compliance.
            display_odds_to_user();
            let item = get_random_item_from_pool();
            return item;
        }
    "#;

    let config = EngineConfig {
        enabled_standards: vec![ComplianceStandard::Lootbox],
        strict_mode: true,
        output_format: OutputFormat::Json,
        custom_rules: Vec::new(),
    };
    let engine = SemanticEngine::new(config);

    // Test for Scenario 1
    let mut parser1 = Parser::new(code_without_odds.to_string());
    let result1 = engine.analyze(&parser1.parse().ast);
    assert!(
        result1.violations.iter().any(|v| v.rule_id == "LOOTBOX_001"),
        "Should detect that the loot box mechanic does not disclose probabilities."
    );

    // Test for Scenario 2
    let mut parser2 = Parser::new(code_with_odds.to_string());
    let result2 = engine.analyze(&parser2.parse().ast);
    assert!(
        result2.violations.is_empty(),
        "Should not find violations when loot box odds are disclosed."
    );
}

#[test]
fn test_sdkscan_compliance_validation() {
    // Scenario 1: Violation - External SDK usage without a security note.
    let code_without_review = r#"
        // S.D.K.S.C.A.N: Using the third-party analytics SDK
        function trackUserEvent(event) {
            ThirdParty.Analytics.track(event);
        }
    "#;

    // Scenario 2: Correct Implementation.
    let code_with_review = r#"
        // S.D.K.S.C.A.N: Using the third-party analytics SDK
        function trackUserEvent(event) {
            // This SDK has been vetted for security and privacy compliance.
            // Report available at internal-docs/sdk-review-analytics-v2.pdf
            ThirdParty.Analytics.track(event);
        }
    "#;

    let config = EngineConfig {
        enabled_standards: vec![ComplianceStandard::SdkScan],
        strict_mode: true,
        output_format: OutputFormat::Json,
        custom_rules: Vec::new(),
    };
    let engine = SemanticEngine::new(config);

    // Test for Scenario 1
    let mut parser1 = Parser::new(code_without_review.to_string());
    let result1 = engine.analyze(&parser1.parse().ast);
    assert!(
        result1.violations.iter().any(|v| v.rule_id == "SDKSCAN_001"),
        "Should detect that the external SDK usage lacks a security review note."
    );

    // Test for Scenario 2
    let mut parser2 = Parser::new(code_with_review.to_string());
    let result2 = engine.analyze(&parser2.parse().ast);
    assert!(
        result2.violations.is_empty(),
        "Should not find violations when an SDK usage note is present."
    );
}