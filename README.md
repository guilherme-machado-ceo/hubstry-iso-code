# Hubstry-ISO_Code Framework

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tests](https://img.shields.io/badge/tests-passing-green.svg)](#testing)

Um framework Rust inovador para análise de conformidade de código usando a metodologia **ISO-Code** com prefixos padronizados para diferentes domínios de compliance.

## 🎯 Visão Geral

O Hubstry-ISO_Code é um framework semântico que permite:

- **Análise de Conformidade**: Verificação automática de padrões de compliance em código
- **Prefixos Padronizados**: Sistema de anotações baseado em ISO para diferentes domínios
- **Engine Semântico**: Análise inteligente com regras customizáveis
- **Relatórios Detalhados**: Geração de relatórios em múltiplos formatos
- **Extensibilidade**: Suporte a regras personalizadas e novos padrões

## 🏗️ Arquitetura

### Prefixos ISO-Code Suportados

| Prefixo | Domínio | Descrição |
|---------|---------|----------|
| **S.O.S** | Security | Segurança e proteção de dados |
| **G.D.P.R** | Privacy | Conformidade com GDPR |
| **A.C.C** | Accessibility | Acessibilidade e inclusão |
| **S.U.S** | Sustainability | Sustentabilidade e eficiência |
| **D.I.V** | Diversity | Diversidade e linguagem inclusiva |

### Componentes Principais

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│     Parser      │───▶│ Semantic Engine │───▶│    Reporter     │
│   (ISO-Code)    │    │   (Rules &      │    │  (Multi-format) │
│                 │    │   Analysis)     │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🚀 Instalação

### Pré-requisitos

- Rust 1.70 ou superior
- Cargo (incluído com Rust)

### Compilação do Código Fonte

```bash
git clone https://github.com/seu-usuario/hubstry-iso-code.git
cd hubstry-iso-code
cargo build --release
```

## 📖 Guia de Uso

### Exemplo Básico

```rust
use hubstry_iso_code::{
    SemanticEngine, EngineConfig, Parser,
    ComplianceStandard, OutputFormat
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Código com anotações ISO-Code
    let source_code = r#"
        S.O.S function authenticate_user(password: string) {
            // Função de autenticação segura
            return validate_credentials(password);
        }
        
        G.D.P.R function process_personal_data(user_data: UserData) {
            // Processamento conforme GDPR
            return anonymize_data(user_data);
        }
    "#;
    
    // Parse do código
    let mut parser = Parser::new(source_code.to_string());
    let parse_result = parser.parse();
    
    // Configuração do engine
    let config = EngineConfig {
        enabled_standards: vec![
            ComplianceStandard::Security,
            ComplianceStandard::Privacy,
        ],
        strict_mode: false,
        output_format: OutputFormat::Json,
    };
    
    // Análise semântica
    let engine = SemanticEngine::new(config);
    let result = engine.analyze(&parse_result.ast);
    
    // Geração de relatório
    let report = engine.generate_report(&result)?;
    println!("Compliance Score: {:.1}%", result.compliance_score);
    println!("Report:\n{}", report);
    
    Ok(())
}
```

### Exemplo com Regras Personalizadas

```rust
use hubstry_iso_code::{
    SemanticEngine, ComplianceRule, ComplianceStandard, RuleSeverity
};

fn main() {
    let mut engine = SemanticEngine::default();
    
    // Adicionar regra personalizada
    let custom_rule = ComplianceRule {
        id: "CUSTOM_001".to_string(),
        standard: ComplianceStandard::Security,
        severity: RuleSeverity::High,
        description: "Admin functions must have security prefix".to_string(),
        validation_pattern: Some("function.*admin.*\\{".to_string()),
        remediation_hint: Some("Add S.O.S prefix to admin functions".to_string()),
    };
    
    engine.add_rule(custom_rule);
    
    // Usar engine com regra personalizada...
}
```

## 🧪 Exemplos Práticos

### Executar Exemplos

```bash
# Exemplo básico de análise
cargo run --example basic_analysis

# Exemplo avançado com múltiplos padrões
cargo run --example advanced_compliance
```

### Estrutura dos Exemplos

- **basic_analysis**: Demonstra análise básica de conformidade
- **advanced_compliance**: Múltiplos padrões, regras personalizadas e análise em lote

## 🔧 API Reference

### Core Types

#### `SemanticEngine`

```rust
impl SemanticEngine {
    pub fn new(config: EngineConfig) -> Self
    pub fn default() -> Self
    pub fn add_rule(&mut self, rule: ComplianceRule)
    pub fn analyze(&self, ast: &AstNode) -> AnalysisResult
    pub fn generate_report(&self, result: &AnalysisResult) -> Result<String, String>
}
```

#### `Parser`

```rust
impl Parser {
    pub fn new(input: String) -> Self
    pub fn parse(&mut self) -> ParseResult
}
```

#### `EngineConfig`

```rust
pub struct EngineConfig {
    pub enabled_standards: Vec<ComplianceStandard>,
    pub strict_mode: bool,
    pub output_format: OutputFormat,
}
```

### Compliance Standards

```rust
pub enum ComplianceStandard {
    Security,      // S.O.S
    Privacy,       // G.D.P.R
    Accessibility, // A.C.C
    Sustainability,// S.U.S
    Diversity,     // D.I.V
}
```

### Output Formats

```rust
pub enum OutputFormat {
    Json,
    Yaml,
    Xml,
    PlainText,
    Markdown,
}
```

## 🧪 Testing

### Executar Testes

```bash
# Todos os testes
cargo test

# Apenas testes unitários
cargo test --lib

# Apenas testes de integração
cargo test --test integration_tests

# Testes com output detalhado
cargo test -- --nocapture
```

### Cobertura de Testes

- ✅ Testes unitários para todos os módulos principais
- ✅ Testes de integração para workflows completos
- ✅ Testes de performance para grandes bases de código
- ✅ Testes de tratamento de erros

## 📊 Métricas de Compliance

### Score de Conformidade

O framework calcula um score de 0-100% baseado em:

- **Violações Críticas**: -20 pontos cada
- **Violações Altas**: -10 pontos cada
- **Violações Médias**: -5 pontos cada
- **Violações Baixas**: -2 pontos cada
- **Cobertura de Anotações**: +pontos por cobertura adequada

### Relatórios

Os relatórios incluem:

- Score geral de conformidade
- Detalhamento por padrão (S.O.S, G.D.P.R, etc.)
- Lista de violações com sugestões
- Métricas de cobertura
- Recomendações de melhoria

## 🤝 Contribuição

### Como Contribuir

1. Fork o repositório
2. Crie uma branch para sua feature (`git checkout -b feature/nova-feature`)
3. Commit suas mudanças (`git commit -am 'Adiciona nova feature'`)
4. Push para a branch (`git push origin feature/nova-feature`)
5. Abra um Pull Request

### Diretrizes

- Mantenha o código bem documentado
- Adicione testes para novas funcionalidades
- Siga as convenções de código Rust
- Atualize a documentação quando necessário

## 📄 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## 📈 Roadmap

- [ ] Suporte a mais linguagens de programação
- [ ] Interface web para análise online
- [ ] Integração com IDEs populares
- [ ] Plugin para CI/CD pipelines
- [ ] Dashboard de métricas em tempo real
- [ ] Suporte a padrões de compliance adicionais

---

**Desenvolvido com ❤️ em Rust**



