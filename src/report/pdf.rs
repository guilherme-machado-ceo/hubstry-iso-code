use anyhow::{anyhow, Result};
use std::process::Command;

pub struct PdfReportGenerator;

impl Default for PdfReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfReportGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Converts an HTML report into a PDF file using wkhtmltopdf.
    pub fn generate_from_html(&self, html_path: &str, output_pdf_path: &str) -> Result<()> {
        let output = Command::new("wkhtmltopdf")
            .args(["--enable-local-file-access", html_path, output_pdf_path])
            .output();

        match output {
            Ok(out) => {
                if out.status.success() {
                    Ok(())
                } else {
                    let err_msg = String::from_utf8_lossy(&out.stderr);
                    Err(anyhow!("Failed to generate PDF: {}", err_msg))
                }
            }
            Err(e) => {
                tracing::warn!(
                    "wkhtmltopdf not found. Skipping PDF generation. Error: {}",
                    e
                );
                Err(anyhow!("wkhtmltopdf missing or inaccessible"))
            }
        }
    }
}
