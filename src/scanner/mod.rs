use anyhow::Result;
use async_trait::async_trait;

pub mod age_gate_detector;
pub mod dark_pattern_detector;
pub mod ad_tracker_detector;
pub mod privacy_policy_checker;
pub mod lootbox_detector;

#[derive(Debug, Clone, serde::Serialize)]
pub struct WebScanResult {
    pub url: String,
    pub scan_date: String,
    pub violations: Vec<WebViolation>,
    pub score: f64,
    pub pages_scanned: usize,
    pub elements_analyzed: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct WebViolation {
    pub rule_id: String,
    pub severity: String,
    pub business_description: String,
    pub technical_description: String,
    pub element_selector: Option<String>,
    pub element_html: Option<String>,
    pub page_url: String,
    pub remediation_business: String,
    pub remediation_technical: String,
}

pub struct ScanConfig {
    pub max_pages: usize,
    pub follow_links: bool,
    pub check_subpages: Vec<String>,
    pub rules_path: String,
}

#[async_trait]
pub trait WebScanner: Send + Sync {
    async fn scan(&self, url: &str, config: &ScanConfig) -> Result<WebScanResult>;
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct QuickScanResult {
    pub url: String,
    pub has_age_verification: bool,
    pub verification_method: String,
    pub risk_level: String, // "CRITICAL" | "WARNING" | "OK"
    pub summary: String,
    pub recommendation: String,
}

pub async fn quick_scan(url: &str) -> Result<QuickScanResult> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let response = client.get(url).send().await?;
    let html = response.text().await?;

    let age_gate_result = age_gate_detector::detect_age_gate(&html);

    let has_age_verification = age_gate_result.method != age_gate_detector::AgeVerificationMethod::None;

    let verification_method = format!("{:?}", age_gate_result.method);

    let risk_level = match age_gate_result.method {
        age_gate_detector::AgeVerificationMethod::SelfDeclarationOnly => "CRITICAL",
        age_gate_detector::AgeVerificationMethod::None => "CRITICAL",
        _ => "OK",
    };

    let (summary, recommendation) = match age_gate_result.method {
        age_gate_detector::AgeVerificationMethod::SelfDeclarationOnly => (
            "Detectada autodeclaração de idade na página.".to_string(),
            "A autodeclaração é proibida pelo ECA Digital. Substitua por verificação oficial via API (Serpro/Gov.br).".to_string()
        ),
        age_gate_detector::AgeVerificationMethod::None => (
            "Nenhum bloqueio ou verificação de idade foi encontrado.".to_string(),
            "Implemente um Age-Gate na entrada da aplicação ou em fluxos de acesso sensíveis.".to_string()
        ),
        _ => (
            "Métodos de verificação oficiais detectados (API/ZKP).".to_string(),
            "A verificação está em conformidade. Continue garantindo a proteção aos menores.".to_string()
        ),
    };

    Ok(QuickScanResult {
        url: url.to_string(),
        has_age_verification,
        verification_method,
        risk_level: risk_level.to_string(),
        summary,
        recommendation,
    })
}

/// Implementação do DOM Scanner baseada em estática (fetch e extração de regras MVP)
pub struct StaticDomScanner;

impl StaticDomScanner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StaticDomScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WebScanner for StaticDomScanner {
    async fn scan(&self, url: &str, _config: &ScanConfig) -> Result<WebScanResult> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let response = client.get(url).send().await?;
        let html = response.text().await?;

        let mut violations = Vec::new();
        let mut score: f64 = 100.0;

        // 1. Age Gate Detect
        let age_gate_result = age_gate_detector::detect_age_gate(&html);
        if age_gate_result.method == age_gate_detector::AgeVerificationMethod::SelfDeclarationOnly {
             violations.push(WebViolation {
                  rule_id: "ECA.AGE.SELF_DECLARATION_BAN".to_string(),
                  severity: "CRITICAL".to_string(),
                  business_description: "Autodeclaração de idade localizada na página.".to_string(),
                  technical_description: format!("Elementos restritos encontrados: {:?}", age_gate_result.elements_found),
                  element_selector: None,
                  element_html: None,
                  page_url: url.to_string(),
                  remediation_business: "Remova a autodeclaração de idade.".to_string(),
                  remediation_technical: "Substitua checkboxes/inputs simples por integrações em backend usando as APIs oficiais como o Serpro DataValid.".to_string(),
             });
             score -= 20.0;
        } else if age_gate_result.method == age_gate_detector::AgeVerificationMethod::None {
             violations.push(WebViolation {
                  rule_id: "ECA.AGE.VERIFY".to_string(),
                  severity: "CRITICAL".to_string(),
                  business_description: "Nenhum sistema de verificação de idade foi encontrado para a plataforma.".to_string(),
                  technical_description: "A página e a aplicação atual falharam em apresentar bloqueios baseados em idade no frontend.".to_string(),
                  element_selector: None,
                  element_html: None,
                  page_url: url.to_string(),
                  remediation_business: "Adicione verificação de idade segura ao fluxo inicial da aplicação.".to_string(),
                  remediation_technical: "Crie um interceptador de requisições de página para realizar o Age-Gate com chamadas de integridade backend.".to_string(),
             });
             score -= 20.0;
        }

        // 2. Dark Patterns
        let dark_patterns_result = dark_pattern_detector::detect_dark_patterns(&html);
        if dark_patterns_result.has_dark_patterns {
             violations.push(WebViolation {
                  rule_id: "ECA.DESIGN.DARK_PATTERNS".to_string(),
                  severity: "HIGH".to_string(),
                  business_description: "Padrão de design para engajamento e hiperuso contínuo detectado.".to_string(),
                  technical_description: format!("Elementos problemáticos: {:?}", dark_patterns_result.elements_found),
                  element_selector: None,
                  element_html: None,
                  page_url: url.to_string(),
                  remediation_business: "Desative autoplay de mídias e scrolls infinitos.".to_string(),
                  remediation_technical: "Remova a tag 'autoplay' do video e substitua 'infinite-scroll' por paginação controlada.".to_string(),
             });
             score -= 10.0;
        }

        // 3. Ad Trackers
        let trackers_result = ad_tracker_detector::detect_ad_trackers(&html);
        if trackers_result.has_trackers {
             violations.push(WebViolation {
                  rule_id: "ECA.DATA.RETENTION_BAN".to_string(),
                  severity: "HIGH".to_string(),
                  business_description: "Scripts de rastreamento de anúncios (Trackers/Analytics) estão injetados na página desprotegida.".to_string(),
                  technical_description: format!("Trackers encontrados: {:?}", trackers_result.scripts_found),
                  element_selector: None,
                  element_html: None,
                  page_url: url.to_string(),
                  remediation_business: "Solicite gerenciamento de consentimento ou remova o tracking por padrão para perfis de risco (menores de 18).".to_string(),
                  remediation_technical: "Envolva os scripts identificados com regras de validação para injetá-los apenas após aprovação explícita e verificação de idade do visitante.".to_string(),
             });
             score -= 15.0;
        }

        // 4. Privacy Policy
        let privacy_result = privacy_policy_checker::check_privacy_policy(&html);
        if !privacy_result.has_policy_link {
             violations.push(WebViolation {
                  rule_id: "ECA.PRIVACY.MAX_DEFAULT".to_string(),
                  severity: "HIGH".to_string(),
                  business_description: "O portal parece não conter um link de acesso ou referência explícita a uma política de privacidade.".to_string(),
                  technical_description: "Nenhuma tag com href referenciando 'Privacy Policy' ou 'Política de Privacidade' encontrada.".to_string(),
                  element_selector: None,
                  element_html: None,
                  page_url: url.to_string(),
                  remediation_business: "Inclua a política detalhando LGPD e adequações para menores de idade de modo vísivel no rodapé/header.".to_string(),
                  remediation_technical: "Adicione <a href='/politica-de-privacidade'>Política de Privacidade</a> globalmente no DOM e atualize os menús.".to_string(),
             });
             score -= 10.0;
        }

        // 5. Lootboxes
        let lootbox_result = lootbox_detector::detect_lootbox(&html);
        if lootbox_result.has_lootbox {
             violations.push(WebViolation {
                  rule_id: "ECA.DESIGN.LOOTBOX_BAN".to_string(),
                  severity: "CRITICAL".to_string(),
                  business_description: "Termos ligados à mecânica gacha/lootboxes identificados nas rotinas sem restrições explícitas acionadas.".to_string(),
                  technical_description: format!("Identificadores listados no script: {:?}", lootbox_result.terms_found),
                  element_selector: None,
                  element_html: None,
                  page_url: url.to_string(),
                  remediation_business: "A compra ou acesso sem controle a lootboxes é proibida para menores. Requer Gate Bloqueante imediato.".to_string(),
                  remediation_technical: "Restringir execução dessa mecânica. Implementar `verify_age_wall` e certificar-se da validade do token de acesso do usuário de forma rigorosa.".to_string(),
             });
             score -= 25.0;
        }

        Ok(WebScanResult {
            url: url.to_string(),
            scan_date: chrono::Utc::now().to_rfc3339(),
            violations,
            score: score.max(0.0),
            pages_scanned: 1,
            elements_analyzed: html.len(), // Just an approximation for the MVP
        })
    }
}
