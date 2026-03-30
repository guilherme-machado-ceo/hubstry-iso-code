# Hubstry Compliance as a Service (CaaS)

[![CI/CD Pipeline](https://github.com/guilherme-machado-ceo/hubstry-iso-code/actions/workflows/ci.yml/badge.svg)](https://github.com/guilherme-machado-ceo/hubstry-iso-code/actions/workflows/ci.yml)
![Compliance Status](https://img.shields.io/badge/Compliance-Pass-success)

O **Hubstry CaaS** é um Produto Mínimo Viável (MVP) em Rust projetado para auxiliar Pequenas e Médias Empresas (PMEs) brasileiras na conformidade com o **ECA Digital** (Estatuto da Criança e do Adolescente) e **LGPD** (Lei Geral de Proteção de Dados).

Esta ferramenta traduz as complexidades regulatórias da legislação digital para a auditoria contínua de software de forma automatizada.
Sem exigir desenvolvedores sêniores ou de um profundo conhecimento de segurança, qualquer gestor e C-Level pode validar, monitorar, exportar relatórios visuais e mitigar riscos nas aplicações desenvolvidas ou mantidas pela sua PME.

---

## 🚀 Proposta de Valor
A partir de Março de 2026, novas diretrizes do **ECA Digital** entrarão em vigor.
Sua empresa possui softwares que interagem com o público infanto-juvenil?
Evite sanções, multas pesadas e proteja a reputação do seu negócio! O Hubstry CaaS oferece:

* **Auditoria Contínua:** Analise regras relacionadas a Verificação de Idade segura (ex: APIs Serpro/Gov.br), Limitação de Coleta de Dados, e restrição em Publicidade Direcionada.
* **Fácil de Usar:** Focado na acessibilidade, com arquivos de configuração em YAML que são perfeitamente entendidos por não-programadores.
* **Relatórios Acionáveis:** Receba um score claro (ex: `95%`) de conformidade e um relatório HTML executivo contendo o local exato da violação com sugestão de negócio para mitigação.
* **Integração CI/CD Transparente:** Adicione verificações no pipeline (e.g. GitHub Actions) da sua equipe e exija conformidade sem atrito.

## 🛠 Quick Start

1. **Instale as ferramentas necessárias**
Certifique-se de que o [Rust](https://www.rust-lang.org/tools/install) está instalado.

2. **Clone e Compile o Hubstry CaaS**
```bash
git clone https://github.com/guilherme-machado-ceo/hubstry-iso-code
cd hubstry-iso-code
cargo build --release
```

3. **Verifique a Conformidade do Seu Código**
Aponte o Hubstry CaaS para um dos seus arquivos de código. A ferramenta analisará a conformidade gerando resultados no terminal, assim como arquivos `compliance_report.json` e `compliance_report.html` no seu diretório atual.

```bash
cargo run -- --file caminho/do/seu/codigo.rs --threshold 90.0
```

4. **Veja o Relatório!**
Abra o arquivo gerado: `compliance_report.html` no seu navegador!
Nele você poderá ver os detalhes para a diretoria: score final, violações identificadas, e uma dica prática focada em negócio.

---

## 📚 Documentação Completa (Para C-Levels e Gestores)
Para entender melhor como configurar suas políticas usando a nossa ferramenta de declaração simplificada e integrar dentro do seu fluxo de desenvolvimento, acesse o guia de uso clicando no link:

👉 [Como Usar e Interpretar (USAGE.md)](./USAGE.md)

## ⚖️ Licenciamento
Hubstry CaaS trabalha sobre o modelo de assinatura Open-Core (*Compliance as a Service*).
O código do motor central (PoC aberto) está sob licença Apache-2.0, no entanto o uso corporativo e extensões avançadas das APIs seguem a tabela comercial descrita detalhadamente em [LICENSE_COMMERCIAL.md](LICENSE_COMMERCIAL.md):

| Feature | Community | Starter (R$197/mês) | Pro (R$497/mês) | Enterprise |
|---|:---:|:---:|:---:|:---:|
| Análise de Código | ✅ | ✅ | ✅ | ✅ |
| Relatório no Terminal | ✅ | ✅ | ✅ | ✅ |
| Relatórios HTML / PDF | ❌ | ✅ | ✅ | ✅ |
| Web Scanning (DOM / APIs) | ❌ | ❌ | ✅ | ✅ |

*(MVP: Construído para adequação do ECA Digital e demonstração de flexibilidade com LGPD).*
