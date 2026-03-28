use scraper::{Html, Selector};

#[derive(Debug, PartialEq)]
pub enum AgeVerificationMethod {
    None,
    SelfDeclarationOnly,
    GovernmentApi,
    GoogleZkp,
    ThirdParty,
}

#[derive(Debug)]
pub struct AgeGateResult {
    pub method: AgeVerificationMethod,
    pub elements_found: Vec<String>,
}

pub fn detect_age_gate(html: &str) -> AgeGateResult {
    let document = Html::parse_document(html);
    let mut method = AgeVerificationMethod::None;
    let mut elements_found = Vec::new();

    // 1. Detect Self-Declaration
    let checkbox_selector = Selector::parse("input[type='checkbox']").unwrap();
    for el in document.select(&checkbox_selector) {
        if let Some(name) = el.value().attr("name") {
            let name_lower = name.to_lowercase();
            if name_lower.contains("age") || name_lower.contains("idade") || name_lower.contains("18") {
                method = AgeVerificationMethod::SelfDeclarationOnly;
                elements_found.push(format!("Checkbox autodeclaração: {:?}", name));
            }
        }
    }

    let select_selector = Selector::parse("select").unwrap();
    for el in document.select(&select_selector) {
        if let Some(name) = el.value().attr("name") {
            let name_lower = name.to_lowercase();
            if name_lower.contains("birth") || name_lower.contains("nascimento") {
                method = AgeVerificationMethod::SelfDeclarationOnly;
                elements_found.push(format!("Select autodeclaração: {:?}", name));
            }
        }
    }

    let button_selector = Selector::parse("button").unwrap();
    for el in document.select(&button_selector) {
        let text_content: String = el.text().collect::<Vec<_>>().join(" ");
        let text_lower = text_content.to_lowercase();
        if text_lower.contains("tenho 18") || text_lower.contains("sou maior") || text_lower.contains("i am 18+") {
            method = AgeVerificationMethod::SelfDeclarationOnly;
            elements_found.push(format!("Botão autodeclaração: {:?}", text_content.trim()));
        }
    }

    // 2. Detect Robust Methods (APIs)
    let script_selector = Selector::parse("script").unwrap();
    for el in document.select(&script_selector) {
        if let Some(src) = el.value().attr("src") {
            let src_lower = src.to_lowercase();
            if src_lower.contains("gov.br") || src_lower.contains("serpro") || src_lower.contains("datavalid") {
                method = AgeVerificationMethod::GovernmentApi;
                elements_found.push(format!("API Governamental: {:?}", src));
            } else if src_lower.contains("accounts.google.com") || src_lower.contains("identity.googleapis") {
                method = AgeVerificationMethod::GoogleZkp;
                elements_found.push(format!("API Google ZKP: {:?}", src));
            }
        }
    }

    AgeGateResult {
        method,
        elements_found,
    }
}
