// tests/test_eca_validation.rs

use hubstry_iso_code::semantic_engine::SemanticEngine;
use hubstry_iso_code::models::ComplianceViolation;
use syn::{self, File};

/// Helper function to parse a string of code and run validation.
fn run_validation_on_code(code: &str) -> Vec<ComplianceViolation> {
    let ast: File = syn::parse_file(code).expect("Failed to parse code");
    let engine = SemanticEngine::default(); // Use default config for testing
    // In a test context, we expect the analysis to succeed. If it fails
    // (e.g., prefixes.yml is missing), the test should panic.
    let result = engine.analyze(&ast).expect("Analysis should succeed in test environment");
    result.violations
}

#[test]
fn test_age_verify_success() {
    let code = r#"
        /// ECA.AGE.VERIFY: This function must check the user's age.
        fn check_user_age() {
            let user_age = get_age_from_id();
            if user_age < 18 {
                block_access();
            }
        }
    "#;
    let violations = run_validation_on_code(code);
    assert!(violations.is_empty(), "Should be no violations for correct age verification");
}

#[test]
fn test_age_verify_failure() {
    let code = r#"
        /// ECA.AGE.VERIFY: This function must check the user's age.
        fn check_user_age() {
            // Missing actual age check logic
            println!("Proceeding without age check.");
        }
    "#;
    let violations = run_validation_on_code(code);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].rule_id, "ECA.AGE.VERIFY.1");
    assert!(violations[0].message.contains("does not appear to call a relevant verification function"));
}

#[test]
fn test_parental_consent_success() {
    let code = r#"
        /// ECA.PARENT.CONSENT: This function collects data and must have consent.
        fn save_user_profile() {
            if get_parental_consent() {
                save_data_to_database();
            }
        }
    "#;
    let violations = run_validation_on_code(code);
    assert!(violations.is_empty(), "Should be no violations when consent is present");
}

#[test]
fn test_parental_consent_failure() {
    let code = r#"
        /// ECA.PARENT.CONSENT: This function collects data and must have consent.
        fn save_user_profile() {
            // Missing consent check
            save_data_to_database();
        }
    "#;
    let violations = run_validation_on_code(code);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].rule_id, "ECA.PARENT.CONSENT.1");
    assert!(violations[0].message.contains("lacks a call to a parental consent function"));
}

#[test]
fn test_lootbox_block_success() {
    let code = r#"
        /// ECA.LOOTBOX.BLOCK: This function must be age-gated.
        fn open_loot_box() {
            if verify_age_for_purchase() {
                grant_random_reward();
            }
        }
    "#;
    let violations = run_validation_on_code(code);
    assert!(violations.is_empty(), "Should be no violations when loot box is age-gated");
}

#[test]
fn test_lootbox_block_failure() {
    let code = r#"
        /// ECA.LOOTBOX.BLOCK: This function must be age-gated.
        fn open_loot_box() {
            // Missing age check
            grant_random_reward();
        }
    "#;
    let violations = run_validation_on_code(code);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].rule_id, "ECA.LOOTBOX.BLOCK.1");
    assert!(violations[0].message.contains("without an age verification check"));
}

#[test]
fn test_no_relevant_prefix() {
    let code = r#"
        /// This is a regular function with no compliance requirements.
        fn calculate_sum(a: i32, b: i32) -> i32 {
            a + b
        }
    "#;
    let violations = run_validation_on_code(code);
    assert!(violations.is_empty(), "Should be no violations for functions without compliance prefixes");
}

#[test]
fn test_parental_consent_failure_with_method_call() {
    let code = r#"
        /// ECA.PARENT.CONSENT: This function collects data and must have consent.
        fn update_user_data(user: &mut User) {
            // This method call should be detected as data collection.
            user.save_data();
        }
    "#;
    let violations = run_validation_on_code(code);
    assert_eq!(violations.len(), 1, "Should detect missing consent for method call");
    assert_eq!(violations[0].rule_id, "ECA.PARENT.CONSENT.1");
}