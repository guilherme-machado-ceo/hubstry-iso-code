// src/main.rs

use clap::{Parser, Subcommand};
use hubstry_iso_code::{models::EngineConfig, scanner, semantic_engine::SemanticEngine};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "hubstry")]
#[command(about = "Hubstry CaaS — Compliance as a Service")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analisar código-fonte localmente
    Analyze {
        #[arg(short, long)]
        file: Option<String>,
        #[arg(short, long)]
        dir: Option<String>,
        #[arg(short, long, default_value = "auto")]
        lang: String,
        #[arg(short, long, default_value = "rules/eca_digital.yml")]
        rules: String,
        #[arg(long, default_value = "terminal")]
        format: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long, default_value_t = 90.0)]
        threshold: f64,
        #[arg(long)]
        license_key: Option<String>,
    },
    /// Escanear URL de website
    Scan {
        #[arg(short, long)]
        url: String,
        #[arg(short, long, default_value = "rules/eca_digital.yml")]
        rules: String,
        #[arg(long, default_value = "html")]
        format: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(long)]
        license_key: Option<String>,
    },
    /// Escanear rapidamente a URL
    QuickScan {
        #[arg(short, long)]
        url: String,
        #[arg(long)]
        license_key: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Analyze {
            file,
            dir: _,
            lang: _,
            rules: _,
            format,
            output: _,
            threshold,
            license_key,
        } => {
            use hubstry_iso_code::licensing::tier_checker::{validate_license_key, Feature};
            let tier = validate_license_key(license_key.as_deref());

            let mut effective_format = format;
            if effective_format == "html" && !tier.has_access(Feature::ReportHtml) {
                println!("👋 Ops! Relatórios HTML avançados e multicamadas estão disponíveis a partir do plano Starter (R$197/mês). Assine para ativar!");
                effective_format = "terminal".to_string();
            }

            let file_path = file.unwrap_or_else(|| "src/main.rs".to_string());
            let path = PathBuf::from(file_path);

            println!("🔎 Analisando o arquivo: {}", path.display());

            let content = fs::read_to_string(&path)?;
            let ast = syn::parse_file(&content)?;

            let engine = SemanticEngine::new(EngineConfig::default());
            let results = engine.analyze(&ast)?;

            if effective_format == "terminal" {
                let report = engine.generate_report(&results);
                println!("\n{}", report);
            }

            if tier.has_access(Feature::ReportJson) {
                let json_report = engine.generate_json_report(&results);
                let _ = fs::write("compliance_report.json", json_report);
            }

            if tier.has_access(Feature::ReportHtml) {
                let html_report = engine.generate_html_report(&results);
                let _ = fs::write("compliance_report.html", html_report);
            }

            if results.compliance_score < threshold {
                eprintln!(
                    "❌ Falha de Compliance: O score de {:.1}% está abaixo do limite mínimo de {:.1}%!",
                    results.compliance_score, threshold
                );
                std::process::exit(1);
            }
        }
        Commands::Scan {
            url,
            rules: _,
            format: _,
            output: _,
            license_key,
        } => {
            use hubstry_iso_code::licensing::tier_checker::{validate_license_key, Feature};
            let tier = validate_license_key(license_key.as_deref());
            if !tier.has_access(Feature::WebScanning) {
                eprintln!("👋 Ops! O Web Scanning é uma funcionalidade disponível a partir do plano Pro (R$497/mês).");
                std::process::exit(1);
            }

            println!("Iniciando Web Scan para URL: {}", url);
            let html = if url.starts_with("http") {
                let client = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(30))
                    .build()?;
                let res = client.get(&url).send().await?;
                res.text().await?
            } else {
                fs::read_to_string(&url)?
            };

            use scanner::WebScanner;
            let dom_scanner = scanner::StaticDomScanner::new();
            let config = scanner::ScanConfig {
                max_pages: 1,
                follow_links: false,
                check_subpages: vec![],
                rules_path: "".to_string(),
            };

            println!("Html length: {} bytes", html.len());

            let res = dom_scanner.scan(&url, &config).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);

            println!("Detailed scanner analysis complete.");
        }
        Commands::QuickScan {
            url,
            license_key: _,
        } => {
            println!("Iniciando Quick Scan para URL: {}", url);

            let res = if url.starts_with("http") {
                scanner::quick_scan(&url).await?
            } else {
                let html = fs::read_to_string(&url)?;
                let age_gate_result = scanner::age_gate_detector::detect_age_gate(&html);

                let has_age_verification = age_gate_result.method
                    != scanner::age_gate_detector::AgeVerificationMethod::None;
                let verification_method = format!("{:?}", age_gate_result.method);

                let risk_level = match age_gate_result.method {
                    scanner::age_gate_detector::AgeVerificationMethod::SelfDeclarationOnly => {
                        "CRITICAL"
                    }
                    scanner::age_gate_detector::AgeVerificationMethod::None => "CRITICAL",
                    _ => "OK",
                };

                let (summary, recommendation) = match age_gate_result.method {
                     scanner::age_gate_detector::AgeVerificationMethod::SelfDeclarationOnly => (
                         "Detectada autodeclaração de idade na página.".to_string(),
                         "A autodeclaração é proibida pelo ECA Digital. Substitua por verificação oficial via API (Serpro/Gov.br).".to_string()
                     ),
                     scanner::age_gate_detector::AgeVerificationMethod::None => (
                         "Nenhum bloqueio ou verificação de idade foi encontrado.".to_string(),
                         "Implemente um Age-Gate na entrada da aplicação ou em fluxos de acesso sensíveis.".to_string()
                     ),
                     _ => (
                         "Métodos de verificação oficiais detectados (API/ZKP).".to_string(),
                         "A verificação está em conformidade. Continue garantindo a proteção aos menores.".to_string()
                     ),
                 };
                scanner::QuickScanResult {
                    url,
                    has_age_verification,
                    verification_method,
                    risk_level: risk_level.to_string(),
                    summary,
                    recommendation,
                }
            };
            println!("Resumo da Avaliação Rápida:");
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
    }

    Ok(())
}
