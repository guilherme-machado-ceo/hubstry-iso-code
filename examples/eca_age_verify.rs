//! Exemplo de análise de conformidade com o ECA Digital para verificação de idade.

use hubstry_iso_code::{
    models::{AnalysisResult, EngineConfig, Jurisdiction},
    semantic_engine::SemanticEngine,
};

fn main() {
    println!("🚀 Exemplo de Análise de Conformidade com ECA.AGE.VERIFY\n");

    // Cenário 1: Código que viola a regra (sem chamada de verificação de idade)
    // Nota: O código agora é Rust válido para que o parser `syn` funcione.
    let code_without_verification = r#"
        /// ECA.AGE.VERIFY: Acesso a conteúdo restrito
        fn access_restricted_content(user_id: &str) {
            // ERRO: Nenhuma função de verificação de idade é chamada aqui.
            grant_access(user_id);
        }
    "#;

    // Cenário 2: Código em conformidade (com chamada de verificação de idade)
    let code_with_verification = r#"
        /// ECA.AGE.VERIFY: Acesso a conteúdo restrito
        fn access_restricted_content(user_id: &str) {
            if check_age(user_id) >= 18 {
                grant_access(user_id);
            }
        }
    "#;

    // Configuração do motor para focar na jurisdição do ECA
    let config = EngineConfig {
        enabled_jurisdictions: vec![Jurisdiction::Eca],
        ..Default::default()
    };

    let engine = SemanticEngine::new(config);

    println!("--- Analisando Cenário 1: Violação ---");
    // O parsing agora é feito com `syn`
    match syn::parse_file(code_without_verification) {
        Ok(ast) => {
            match engine.analyze(&ast) {
                Ok(results) => print_analysis_results(&results),
                Err(e) => {
                    eprintln!("Erro durante a análise: {}", e);
                    eprintln!("Verifique se 'prefixes.yml' está presente e correto.");
                }
            }
        }
        Err(e) => {
            println!("Falha ao parsear o código de exemplo 1: {}", e);
        }
    }


    println!("\n--- Analisando Cenário 2: Conformidade ---");
    match syn::parse_file(code_with_verification) {
        Ok(ast) => {
            match engine.analyze(&ast) {
                Ok(results) => print_analysis_results(&results),
                Err(e) => {
                    eprintln!("Erro durante a análise: {}", e);
                    eprintln!("Verifique se 'prefixes.yml' está presente e correto.");
                }
            }
        }
        Err(e) => {
            println!("Falha ao parsear o código de exemplo 2: {}", e);
        }
    }
}

// A assinatura da função foi corrigida para usar o tipo público `AnalysisResult`
fn print_analysis_results(result: &AnalysisResult) {
    println!("  Score de Conformidade: {:.1}%", result.compliance_score);
    if result.violations.is_empty() {
        println!("  Nenhuma violação encontrada. ✅");
    } else {
        println!("  Violações Encontradas ({}):", result.violations.len());
        for violation in &result.violations {
            println!(
                "    - [{} @ L{}:C{}]: {}",
                violation.rule_id,
                violation.line.unwrap_or(0),
                violation.column.unwrap_or(0),
                violation.message
            );
            if let Some(suggestion) = &violation.suggestion {
                println!("      Sugestão: {}", suggestion);
            }
        }
    }
}