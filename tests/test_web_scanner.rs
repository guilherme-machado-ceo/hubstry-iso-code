#[cfg(test)]
mod tests {
    use hubstry_iso_code::scanner::{age_gate_detector, dark_pattern_detector, ad_tracker_detector, privacy_policy_checker, lootbox_detector};

    #[test]
    fn test_non_compliant_site() {
        let html = std::fs::read_to_string("examples/web/non_compliant_site.html").expect("Failed to read example");

        let age_gate_result = age_gate_detector::detect_age_gate(&html);
        assert_eq!(age_gate_result.method, age_gate_detector::AgeVerificationMethod::SelfDeclarationOnly);
        assert!(!age_gate_result.elements_found.is_empty());

        let dark_patterns_result = dark_pattern_detector::detect_dark_patterns(&html);
        assert!(dark_patterns_result.has_dark_patterns);

        let ad_tracker_result = ad_tracker_detector::detect_ad_trackers(&html);
        assert!(ad_tracker_result.has_trackers);

        let privacy_result = privacy_policy_checker::check_privacy_policy(&html);
        assert!(!privacy_result.has_policy_link);

        let lootbox_result = lootbox_detector::detect_lootbox(&html);
        assert!(lootbox_result.has_lootbox);
    }

    #[test]
    fn test_compliant_site() {
        let html = std::fs::read_to_string("examples/web/compliant_site.html").expect("Failed to read example");

        let age_gate_result = age_gate_detector::detect_age_gate(&html);
        assert_eq!(age_gate_result.method, age_gate_detector::AgeVerificationMethod::GovernmentApi);

        let dark_patterns_result = dark_pattern_detector::detect_dark_patterns(&html);
        assert!(!dark_patterns_result.has_dark_patterns);

        let ad_tracker_result = ad_tracker_detector::detect_ad_trackers(&html);
        assert!(!ad_tracker_result.has_trackers);

        let privacy_result = privacy_policy_checker::check_privacy_policy(&html);
        assert!(privacy_result.has_policy_link);

        let lootbox_result = lootbox_detector::detect_lootbox(&html);
        assert!(!lootbox_result.has_lootbox);
    }
}
