use scraper::{Html, Selector};

#[derive(Debug)]
pub struct AdTrackerResult {
    pub has_trackers: bool,
    pub scripts_found: Vec<String>,
}

pub fn detect_ad_trackers(html: &str) -> AdTrackerResult {
    let document = Html::parse_document(html);
    let mut scripts_found = Vec::new();
    let mut has_trackers = false;

    let script_selector = Selector::parse("script").unwrap();
    for el in document.select(&script_selector) {
        if let Some(src) = el.value().attr("src") {
            let src_lower = src.to_lowercase();
            if src_lower.contains("google-analytics")
                || src_lower.contains("googletagmanager")
                || src_lower.contains("gtag")
                || src_lower.contains("facebook") && src_lower.contains("pixel")
                || src_lower.contains("mixpanel")
                || src_lower.contains("hotjar")
            {
                has_trackers = true;
                scripts_found.push(format!("Ad Tracker script: {:?}", src));
            }
        }
    }

    AdTrackerResult {
        has_trackers,
        scripts_found,
    }
}
