use scraper::{Html, Selector};

#[derive(Debug)]
pub struct PrivacyPolicyResult {
    pub has_policy_link: bool,
    pub mentions_minors: bool,
    pub link_url: Option<String>,
}

pub fn check_privacy_policy(html: &str) -> PrivacyPolicyResult {
    let document = Html::parse_document(html);
    let mut has_policy_link = false;
    let mut mentions_minors = false;
    let mut link_url = None;

    let link_selector = Selector::parse("a").unwrap();
    for el in document.select(&link_selector) {
        let text = el.text().collect::<Vec<_>>().join(" ").to_lowercase();
        if text.contains("política de privacidade") || text.contains("privacy policy") {
            has_policy_link = true;
            if let Some(href) = el.value().attr("href") {
                link_url = Some(href.to_string());
            }
        }
    }

    // Simplificação para MVP: Buscar menções diretas na página atual (ou no footer)
    // Para análise profunda, idealmente faríamos um fetch() na URL da política encontrada.
    let body_text = document
        .root_element()
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    if body_text.contains("menores")
        || body_text.contains("crianças")
        || body_text.contains("lgpd")
        || body_text.contains("eca")
    {
        mentions_minors = true;
    }

    PrivacyPolicyResult {
        has_policy_link,
        mentions_minors,
        link_url,
    }
}
