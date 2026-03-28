# Como Usar o Hubstry CaaS: Guia para Gestores e C-Levels

Bem-vindo ao guia de utilização do **Hubstry Compliance as a Service (CaaS)**.
Criamos esta ferramenta de forma a não exigir uma equipe de TI experiente ou dedicada à Segurança da Informação. O objetivo deste documento é ensiná-lo a configurar as leis que deseja auditar, como rodar a verificação de código e como interpretar o relatório gerado.

---

## 1. O que são Prefixos Semânticos?
Para que a ferramenta identifique qual parte do código deve atender a qual regra, nossos auditores pedem que os desenvolvedores coloquem "Anotações" (`///`) nos códigos criados.
Um prefixo semântico é apenas um "carimbo" que o desenvolvedor coloca antes do código indicando o objetivo dele.

**Exemplo:**
Se o desenvolvedor cria uma rotina no aplicativo que checa se um usuário é maior de 18 anos, ele deverá colocar o prefixo do ECA Digital para que nossa ferramenta audite o processo.

```rust
/// ECA.AGE.VERIFY: Checagem oficial de identidade
fn checar_identidade_usuario() {
    let idade = serpro_datavalid.verify_age(id_usuario);
    // ...
}
```
Nosso motor lerá o arquivo, e caso o desenvolvedor não tenha implementado as validações oficiais exigidas pelas políticas de sua empresa, ele o alertará!

---

## 2. Como Configurar as Regras da Sua Empresa (YAML)
Sua equipe não precisa saber programar para editar as regras que devem ser analisadas! Tudo está configurado em um arquivo de texto simples chamado **`prefixes.yml`**.

Se você abrir este arquivo, você verá configurações da seguinte forma:

```yaml
  ECA:
    - prefix: "ECA.AGE.VERIFY"
      description: "Verificação de idade para acesso a conteúdo restrito."
      standard: "Eca"
      expected_calls:
        - "verify_age"
        - "check_age"
        - "serpro_datavalid.verify_age" # Exigência da sua empresa: Chamar API Oficial do Serpro
```

### O que você pode alterar como gestor?
No campo `expected_calls`, você lista quais *funções oficiais* a sua aplicação deve invocar obrigatoriamente quando esse prefixo for encontrado.
Se a sua equipe jurídica recomenda a validação governamental brasileira, basta adicionar ali a chamada de código utilizada (`serpro_datavalid.verify_age` por exemplo).

Você pode adicionar restrições de **Publicidade Direcionada** `ECA.AD.NO_TARGETING` exigindo chamadas ou procurando palavras na configuração `data_collection_keywords` que denunciam práticas ilegais, e a ferramenta fará o trabalho de rastrear falhas no fluxo!

---

## 3. Como Rodar a Ferramenta e o Pipeline Automático (CI/CD)

Você ou a sua equipe de tecnologia podem rodar a ferramenta localmente, ou melhor ainda, integrar em *processos de checagem automática (CI/CD)* para que a verificação ocorra sempre que o desenvolvedor tentar publicar uma nova funcionalidade.

### Via Terminal Local
```bash
cargo run -- --file src/meu_arquivo.rs --threshold 90.0
```
- A variável `--threshold 90.0` define que o **Score Mínimo de Aprovação de Conformidade é 90%**. Se o código testado ficar abaixo desse score (devido a violações legais), o programa avisará e emitirá um erro de segurança.

### Automatização e Scripts
Fornecemos um script chamado `ci.sh`. Ele pode ser utilizado nos ambientes automáticos de integração da sua empresa (Ex: GitHub Actions).
Basta que a sua equipe técnica insira a seguinte linha no pipeline de publicação:

```bash
./ci.sh 90.0 src/
```
Ele irá auditar toda a pasta do código e reprovar qualquer tentativa de alteração insegura!

---

## 4. Interpretando os Relatórios de Conformidade

Após a execução (seja automática ou manual), a ferramenta vai criar (ou atualizar) dois arquivos:
1. `compliance_report.html` (Relatório Visual para C-Levels e Gestores)
2. `compliance_report.json` (Relatório para integrações de software)

### Abrindo o HTML
Se você clicar no arquivo `compliance_report.html`, o navegador irá exibir um **Relatório Executivo de Conformidade**.

**Nele você encontrará:**
* **O Score Geral (Nota):** Em grande destaque. Ex: `95.0%`. Fica Verde, Amarelo ou Vermelho dependendo do quão perto de 100% ele se aproxima.
* **Violações (Erros Legais):** Uma lista de onde o código feriu a sua diretriz.
* **Linguagem de Negócios e Sugestão:** "A função controla anúncios mas não desativa a coleta de dados e retenção para publicidade" -> *Sugestão de Mitigação: Garanta que você está usando um serviço que explicitamente limite o rastreio (Tracking).*
* **Local Exato:** "Linha 12, Coluna 5". (Para você informar ao seu desenvolvedor focar direto no problema, economizando horas de análise).

Com este relatório na sua tela, você tem uma forma mensurável de gerenciar a segurança do que está sendo publicado para seus usuários (especialmente menores de idade), garantindo que sua PME esteja preparada para o ECA Digital.
