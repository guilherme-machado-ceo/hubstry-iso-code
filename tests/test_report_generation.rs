#[cfg(test)]
mod report_tests {
    use hubstry_iso_code::report::html::{HtmlReportGenerator, TemplateContext, TemplateViolation};

    #[test]
    fn generate_sample_report() {
        let generator = HtmlReportGenerator::new().unwrap();

        let ctx = TemplateContext {
            client_name: "Projeto Exemplo PME",
            analysis_date: "10/05/2026",
            rules_version: "ECA Digital 1.0",
            valid_until: "08/08/2026",
            score: 65.5,
            violations: vec![
                TemplateViolation {
                    severity: "CRITICAL".to_string(),
                    business_description: "Autodeclaração de idade encontrada. Proibido pela ANPD.".to_string(),
                    law_article: Some("Art. 12, §1º".to_string()),
                    business_remediation: "Substitua por um sistema backend validado (Serpro/Gov.br).".to_string(),
                    rule_id: "ECA.AGE.SELF_DECLARATION_BAN".to_string(),
                    technical_description: "Input do tipo checkbox com atributos restritos encontrado e sem rotina backend de validação.".to_string(),
                    file: Some("src/views/Login.jsx".to_string()),
                    line: Some(42),
                    column: Some(15),
                    url: None,
                    element_selector: None,
                    code_snippet: Some("<input type=\"checkbox\" name=\"age\" /> Tenho 18 anos".to_string()),
                    technical_remediation: "Remover tag e vincular chamada para a API DataValid no submit.".to_string(),
                },
                TemplateViolation {
                    severity: "HIGH".to_string(),
                    business_description: "Tracker publicítario de Analytics detectado.".to_string(),
                    law_article: Some("Art. 14".to_string()),
                    business_remediation: "Bloquear disparo de pixels se usuário for menor de idade.".to_string(),
                    rule_id: "ECA.DATA.RETENTION_BAN".to_string(),
                    technical_description: "Script injetando GTAG detectado sem envoltório de consentimento.".to_string(),
                    file: None,
                    line: None,
                    column: None,
                    url: Some("https://exemplo.com.br".to_string()),
                    element_selector: Some("script[src*='gtag']".to_string()),
                    code_snippet: None,
                    technical_remediation: "Use o consent_manager() antes de carregar.".to_string(),
                }
            ],
        };

        let html = generator.generate(&ctx).unwrap();
        std::fs::write("examples/reports/sample_report.html", html).unwrap();
    }
}
