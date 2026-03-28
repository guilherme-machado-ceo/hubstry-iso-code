use scraper::{Html, Selector};

#[derive(Debug)]
pub struct LootboxResult {
    pub has_lootbox: bool,
    pub terms_found: Vec<String>,
}

pub fn detect_lootbox(html: &str) -> LootboxResult {
    let document = Html::parse_document(html);
    let mut terms_found = Vec::new();
    let mut has_lootbox = false;

    let script_selector = Selector::parse("script").unwrap();
    for el in document.select(&script_selector) {
        let script_content = el.text().collect::<Vec<_>>().join(" ").to_lowercase();

        let target_terms = ["lootbox", "loot_box", "gacha", "mystery_box", "random_reward"];
        for term in target_terms {
            if script_content.contains(term) {
                has_lootbox = true;
                terms_found.push(format!("Termo detectado em script: {:?}", term));
            }
        }
    }

    LootboxResult {
        has_lootbox,
        terms_found,
    }
}
