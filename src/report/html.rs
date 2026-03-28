use anyhow::Result;
use serde::Serialize;
use tera::{Context, Tera};

#[derive(Serialize)]
pub struct TemplateContext<'a> {
    pub client_name: &'a str,
    pub analysis_date: &'a str,
    pub rules_version: &'a str,
    pub valid_until: &'a str,
    pub score: f64,
    pub violations: Vec<TemplateViolation>,
}

#[derive(Serialize, Clone)]
pub struct TemplateViolation {
    pub severity: String,
    pub business_description: String,
    pub law_article: Option<String>,
    pub business_remediation: String,
    pub rule_id: String,
    pub technical_description: String,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub url: Option<String>,
    pub element_selector: Option<String>,
    pub code_snippet: Option<String>,
    pub technical_remediation: String,
}

pub struct HtmlReportGenerator {
    tera: Tera,
}

impl HtmlReportGenerator {
    pub fn new() -> Result<Self> {
        let mut tera = Tera::default();
        // Load the template dynamically. During build, this could be included via include_str!
        // But for development, we will read from the file system.
        let template_str = include_str!("templates/report_base.html");
        tera.add_raw_template("report_base.html", template_str)?;
        Ok(Self { tera })
    }

    pub fn generate(&self, ctx: &TemplateContext) -> Result<String> {
        let context = Context::from_serialize(ctx)?;
        let rendered = self.tera.render("report_base.html", &context)?;
        Ok(rendered)
    }
}
