# Hubstry ISO Code Framework

Hubstry ISO Code is a proof-of-concept framework designed to integrate international compliance, security, quality, and ethical standards directly into the software development workflow. It uses a semantic analysis engine to parse source code, identify special compliance prefixes, and validate the code against a defined set of rules.

This approach allows developers to embed compliance requirements directly into their code, enabling automated checks and generating reports to ensure adherence to legal and ethical standards.

## Current Capabilities

This prototype is focused on a single jurisdiction and a limited set of rules to demonstrate the core functionality.

*   **Language Support:** Rust
*   **Analysis Target:** Functions in Rust source code.
*   **Supported Jurisdiction:** **ECA Digital (Estatuto da Criança e do Adolescente - Brazil)**
*   **Implemented Rules:**
    *   `ECA.AGE.VERIFY`: Ensures a function annotated for age verification contains a call to an age-checking mechanism.
    *   `ECA.PARENT.CONSENT`: Ensures a function that collects user data also contains a call to a parental consent mechanism.
    *   `ECA.LOOTBOX.BLOCK`: Ensures a function implementing loot box mechanics is protected by an age verification check.

## How It Works

The engine works by parsing Rust source files into an Abstract Syntax Tree (AST). It then traverses the AST, looking for functions that have special "doc comments" containing a compliance prefix.

**Example:**
```rust
/// ECA.AGE.VERIFY: This function must check the user's age.
fn access_feature() {
    // The engine will check if this function body
    // contains a call to a function like `check_age()`.
}
```

If a function is annotated with a known prefix, the engine applies the corresponding validation logic to its body. Violations are collected and presented in a final report.

## Getting Started

### Prerequisites

*   [Rust programming language and Cargo](https://www.rust-lang.org/tools/install)

### Build

To build the command-line analysis tool, clone the repository and run the following command from the project root:

```bash
cargo build --release
```
The executable will be available at `target/release/hubstry_iso_code`.

### Usage

The primary way to use the tool is via the command-line interface (CLI).

1.  **Run Analysis on a File:**
    Use the `--file` argument to specify the path to the Rust file you want to analyze.

    ```bash
    cargo run -- --file <path/to/your/file.rs>
    ```

2.  **Example with Violations:**
    An example file with known violations is provided in `examples/simple_violation.rs`. You can run the analyzer on it to see the report generation in action:

    ```bash
    cargo run -- --file examples/simple_violation.rs
    ```

    **Expected Output:**
    ```
    🔎 Analisando o arquivo: examples/simple_violation.rs

    # Hubstry-ISO_Code Compliance Report

    **Compliance Score:** 80.0%

    ## Violations (2)

    - **HIGH** [ECA.PARENT.CONSENT.1]: Function appears to collect user data but lacks a call to a parental consent function.
      *Suggestion: Ensure that any data collection from minors is preceded by a call to a verifiable parental consent mechanism (e.g., 'get_parental_consent()').*

    - **HIGH** [ECA.AGE.VERIFY.1]: Function is annotated for age verification, but does not appear to call a relevant verification function.
      *Suggestion: Ensure the function calls a service or helper for age verification (e.g., 'verify_age_with_id()').*
    ```

## Future Development

This prototype serves as the foundation for a more comprehensive compliance framework. Future work could include:
*   Implementing the remaining jurisdictions (LGPD, GDPR, etc.).
*   Adding support for more programming languages.
*   Building adapters for popular frameworks (React, Django, Unity).
*   Developing a web-based dashboard for viewing reports.