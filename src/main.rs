// src/main.rs

use clap::Parser;
use hubstry_iso_code::{models::EngineConfig, semantic_engine::SemanticEngine};
use std::fs;
use std::path::PathBuf;

/// Hubstry-ISO-Code: Um framework para análise de conformidade de código.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// O caminho para o arquivo Rust a ser analisado.
    #[arg(short, long)]
    file: PathBuf,

    /// O limiar mínimo de score de compliance (0 a 100) para sucesso do processo.
    #[arg(short, long, default_value_t = 90.0)]
    threshold: f64,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    println!("🔎 Analisando o arquivo: {}", cli.file.display());

    // 1. Ler o conteúdo do arquivo
    let content = match fs::read_to_string(&cli.file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "Erro: Não foi possível ler o arquivo '{}'.",
                cli.file.display()
            );
            eprintln!("Detalhes: {}", e);
            return Err(e);
        }
    };

    // 2. Parsear o conteúdo para uma AST `syn`
    let ast = match syn::parse_file(&content) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!(
                "Erro: Falha ao parsear o código Rust no arquivo '{}'.",
                cli.file.display()
            );
            eprintln!("Certifique-se de que o arquivo contém código Rust válido.");
            eprintln!("Detalhes do parser: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Falha no parsing do código",
            ));
        }
    };

    // 3. Executar o motor semântico
    let engine = SemanticEngine::new(EngineConfig::default());
    let results = match engine.analyze(&ast) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("\nErro Crítico: Falha ao inicializar o motor de análise.");
            eprintln!("Causa: {}", e);
            eprintln!("Por favor, verifique se o arquivo 'prefixes.yml' existe no diretório raiz e está formatado corretamente.");
            return Err(std::io::Error::other(
                "Falha na configuração do motor semântico",
            ));
        }
    };

    // 4. Gerar e imprimir o relatório
    let report = engine.generate_report(&results);
    println!("\n{}", report);

    // Output JSON and HTML as well to files
    let json_report = engine.generate_json_report(&results);
    if let Err(e) = fs::write("compliance_report.json", json_report) {
        eprintln!("Warning: Failed to write JSON report: {}", e);
    }

    let html_report = engine.generate_html_report(&results);
    if let Err(e) = fs::write("compliance_report.html", html_report) {
        eprintln!("Warning: Failed to write HTML report: {}", e);
    }

    // 5. Exit with an error if the compliance score is below the threshold or critical violations are found
    if results.compliance_score < cli.threshold {
        eprintln!(
            "❌ Falha de Compliance: O score de {:.1}% está abaixo do limite mínimo de {:.1}%!",
            results.compliance_score, cli.threshold
        );
        std::process::exit(1);
    }

    Ok(())
}
