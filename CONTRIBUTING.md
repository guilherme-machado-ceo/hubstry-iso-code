# Como Contribuir para o Hubstry-ISO_Code

Bem-vindo ao projeto Hubstry-ISO_Code! Agradecemos o seu interesse em contribuir para este framework inovador. Suas contribuições são essenciais para o sucesso e a evolução contínua do projeto.

Este documento descreve as diretrizes e o processo para contribuir com o código, documentação, relatórios de bugs e sugestões de recursos.

## Sumário

1.  [Código de Conduta](#1-código-de-conduta)
2.  [Como Fazer Perguntas](#2-como-fazer-perguntas)
3.  [Como Reportar um Bug](#3-como-reportar-um-bug)
4.  [Como Sugerir um Novo Recurso](#4-como-sugerir-um-novo-recurso)
5.  [Seu Primeiro Pull Request](#5-seu-primeiro-pull-request)
6.  [Processo de Desenvolvimento](#6-processo-de-desenvolvimento)
    *   [Configuração do Ambiente](#configuração-do-ambiente)
    *   [Testes](#testes)
    *   [Estilo de Código](#estilo-de-código)
7.  [Assinatura do Contrato de Licença de Contribuidor (CLA)](#7-assinatura-do-contrato-de-licença-de-contribuidor-cla)

---

### 1. Código de Conduta

Este projeto e todos que participam dele são regidos pelo [Código de Conduta do Contribuidor](CODE_OF_CONDUCT.md). Ao participar, você espera defender este código. Por favor, reporte comportamento inaceitável para [guilherme.ceo@hubstry.com](mailto:guilherme.ceo@hubstry.com).

### 2. Como Fazer Perguntas

Se você tiver dúvidas sobre como usar o Hubstry-ISO_Code ou quiser discutir ideias, por favor, abra uma [Issue no GitHub](https://github.com/Hubstry/hubstry-iso-code/issues) com a tag `question` ou `discussion`.

### 3. Como Reportar um Bug

Se você encontrar um bug no Hubstry-ISO_Code, por favor, siga estas diretrizes para reportá-lo:

*   **Verifique as Issues Existentes:** Antes de abrir uma nova issue, verifique se o bug já foi reportado. Se sim, adicione seus comentários e informações adicionais à issue existente.
*   **Abra uma Nova Issue:** Se o bug não foi reportado, abra uma nova [Issue no GitHub](https://github.com/Hubstry/hubstry-iso-code/issues/new?assignees=&labels=bug&projects=&template=bug_report.md&title=) e use o template de `Bug Report`.
*   **Seja Descritivo:** Forneça o máximo de detalhes possível, incluindo:
    *   Uma descrição clara e concisa do bug.
    *   Passos para reproduzir o comportamento.
    *   O comportamento esperado.
    *   O comportamento real.
    *   Sua versão do Hubstry-ISO_Code e do Rust (se aplicável).
    *   Qualquer mensagem de erro relevante.
    *   Capturas de tela ou logs (se apropriado).

### 4. Como Sugerir um Novo Recurso

Ideias para novos recursos são sempre bem-vindas! Para sugerir um novo recurso:

*   **Verifique as Issues Existentes:** Verifique se o recurso já foi sugerido.
*   **Abra uma Nova Issue:** Se não, abra uma nova [Issue no GitHub](https://github.com/Hubstry/hubstry-iso-code/issues/new?assignees=&labels=enhancement&projects=&template=feature_request.md&title=) e use o template de `Feature Request`.
*   **Descreva o Recurso:** Explique a funcionalidade, o caso de uso e por que você acredita que seria uma adição valiosa ao projeto.

### 5. Seu Primeiro Pull Request

Se você é novo em contribuições de código aberto, aqui estão os passos básicos para enviar seu primeiro Pull Request (PR):

1.  **Faça um Fork do Repositório:** Clique no botão "Fork" no canto superior direito da página do GitHub do Hubstry-ISO_Code.
2.  **Clone o Fork:** Clone seu fork para sua máquina local:
    ```bash
    git clone https://github.com/SEU_USUARIO/hubstry-iso-code.git
    cd hubstry-iso-code
    ```
3.  **Crie uma Nova Branch:** Crie uma branch para suas alterações. Use um nome descritivo, como `feature/nome-do-recurso` ou `bugfix/correcao-do-bug`.
    ```bash
    git checkout -b feature/meu-novo-recurso
    ```
4.  **Faça Suas Alterações:** Implemente suas alterações no código ou na documentação.
5.  **Commit Suas Alterações:** Faça commits atômicos e com mensagens claras e descritivas.
    ```bash
    git add .
    git commit -m "feat: Adiciona novo recurso X"
    ```
6.  **Envie para o GitHub:** Envie suas alterações para o seu fork no GitHub.
    ```bash
    git push origin feature/meu-novo-recurso
    ```
7.  **Abra um Pull Request:** Vá para a página do seu fork no GitHub e você verá um botão para "Compare & pull request". Clique nele, preencha os detalhes e abra o PR. Certifique-se de referenciar qualquer issue relevante.

### 6. Processo de Desenvolvimento

#### Configuração do Ambiente

Para configurar seu ambiente de desenvolvimento para o Hubstry-ISO_Code (Rust engine):

1.  **Instale o Rust:** Se você ainda não tem o Rust instalado, siga as instruções em [rustup.rs](https://rustup.rs/).
2.  **Clone o Repositório:**
    ```bash
    git clone https://github.com/Hubstry/hubstry-iso-code.git
    cd hubstry-iso-code
    ```
3.  **Construa o Projeto:**
    ```bash
    cargo build
    ```

#### Testes

Para executar os testes do projeto:

```bash
cargo test
```

Certifique-se de que todos os testes passem antes de enviar um Pull Request.

#### Estilo de Código

Utilizamos `rustfmt` para formatar o código Rust. Certifique-se de que seu código esteja formatado corretamente antes de enviar um PR:

```bash
cargo fmt
```

### 7. Assinatura do Contrato de Licença de Contribuidor (CLA)

Para que suas contribuições sejam aceitas, a Hubstry exige que todos os contribuidores assinem um Contrato de Licença de Contribuidor (CLA). Isso garante que a Hubstry tenha os direitos necessários para usar e distribuir suas contribuições como parte do projeto. O processo é simples e será fornecido quando você abrir seu primeiro Pull Request.

---

Obrigado por contribuir para o Hubstry-ISO_Code!


