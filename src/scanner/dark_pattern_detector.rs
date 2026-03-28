use scraper::{Html, Selector};

#[derive(Debug)]
pub struct DarkPatternResult {
    pub has_dark_patterns: bool,
    pub elements_found: Vec<String>,
}

pub fn detect_dark_patterns(html: &str) -> DarkPatternResult {
    let document = Html::parse_document(html);
    let mut elements_found = Vec::new();
    let mut has_dark_patterns = false;

    // Detect Auto-play video
    let video_selector = Selector::parse("video").unwrap();
    for el in document.select(&video_selector) {
        if el.value().attr("autoplay").is_some() || el.value().attr("data-autoplay").is_some() {
            has_dark_patterns = true;
            elements_found.push("Vídeo com Autoplay detectado".to_string());
        }
    }

    // Detect Infinite Scroll
    let all_elements_selector = Selector::parse("*").unwrap();
    for el in document.select(&all_elements_selector) {
        let classes = el.value().classes().collect::<Vec<_>>().join(" ");
        let classes_lower = classes.to_lowercase();

        let has_infinite_scroll_attr = el.value().attrs().any(|(attr_name, _)| {
            let name_lower = attr_name.to_lowercase();
            name_lower.contains("infinite-scroll") || name_lower.contains("endless-feed")
        });

        if classes_lower.contains("infinite-scroll") || classes_lower.contains("endless-feed") || has_infinite_scroll_attr {
            has_dark_patterns = true;
            elements_found.push("Rolagem Infinita (Infinite Scroll) detectada".to_string());
        }
    }

    DarkPatternResult {
        has_dark_patterns,
        elements_found,
    }
}
