// src/main.rs

use clap::{Parser, Subcommand};
use hubstry_iso_code::{models::EngineConfig, semantic_engine::SemanticEngine};
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
    },
    /// Auditoria combinada (código + web)
    Audit {
        #[arg(short, long)]
        dir: Option<String>,
        #[arg(short, long)]
        url: Option<String>,
        #[arg(short, long, default_value = "rules/eca_digital.yml")]
        rules: String,
        #[arg(long, default_value = "html")]
        format: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Iniciar servidor API REST
    Serve {
        #[arg(short, long, default_value = "0.0.0.0")]
        host: String,
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    /// Gerar badge de compliance
    Badge {
        #[arg(short, long)]
        file: String,
        #[arg(short, long, default_value = "badge.svg")]
        output: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Analyze { file, dir: _, lang: _, rules: _, format: _, output: _, threshold } => {
            // For now, retaining backward compatibility logic inside Analyze
            let file_path = file.unwrap_or_else(|| "src/main.rs".to_string());
            let path = PathBuf::from(file_path);

            println!("🔎 Analisando o arquivo: {}", path.display());

            let content = fs::read_to_string(&path)?;
            let ast = syn::parse_file(&content)?;

            let engine = SemanticEngine::new(EngineConfig::default());
            let results = engine.analyze(&ast)?;

            let report = engine.generate_report(&results);
            println!("\n{}", report);

            let json_report = engine.generate_json_report(&results);
            let _ = fs::write("compliance_report.json", json_report);

            let html_report = engine.generate_html_report(&results);
            let _ = fs::write("compliance_report.html", html_report);

            if results.compliance_score < threshold {
                eprintln!(
                    "❌ Falha de Compliance: O score de {:.1}% está abaixo do limite mínimo de {:.1}%!",
                    results.compliance_score, threshold
                );
                std::process::exit(1);
            }
        }
        Commands::Scan { .. } => {
            println!("Web scan functionality is in development.");
        }
        Commands::Audit { .. } => {
            println!("Audit functionality is in development.");
        }
        Commands::Serve { host, port } => {
            hubstry_iso_code::api::server::start(host, port).await?;
        }
        Commands::Badge { .. } => {
            println!("Badge functionality is in development.");
        }
    }

    Ok(())
}
