//! Data models representing ISO codes, rules, and AST nodes.
//! This module defines the core data structures used throughout the Hubstry-ISO_Code engine.

use std::collections::HashMap;
use std::fmt;

/// Represents different types of compliance standards supported by the framework
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComplianceStandard {
    /// Security-related standards (S.O.S prefix)
    Security,
    /// GDPR/LGPD privacy standards (G.D.P.R prefix)
    Privacy,
    /// Quality Management System standards (Q.M.S prefix)
    Quality,
    /// Accessibility standards (A.C.C prefix)
    Accessibility,
    /// Sustainability/Green coding standards (S.U.S prefix)
    Sustainability,
    /// Diversity and inclusion standards (D.I.V prefix)
    Diversity,
    // --- Novos padrões legais ---
    /// Painel Parental (P.A.I.N.E.L prefix)
    Painel,
    /// Relatórios Semestrais (R.E.L.A.T.O prefix)
    Relato,
    /// Auditoria de Algoritmos (A.L.G.O.R.I.T.H.M prefix)
    Algorithm,
    /// Bloqueio de Loot Boxes (L.O.O.T.B.O.X prefix)
    Lootbox,
    /// Scanner de SDKs (S.D.K.S.C.A.N prefix)
    SdkScan,
    /// Custom standard with name
    Custom(String),
}

/// Represents a compliance rule with its metadata
#[derive(Debug, Clone)]
pub struct ComplianceRule {
    pub id: String,
    pub standard: ComplianceStandard,
    pub severity: RuleSeverity,
    pub description: String,
    pub validation_pattern: Option<String>,
    pub remediation_hint: Option<String>,
}

/// Severity levels for compliance rules
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Represents a node in the Abstract Syntax Tree
#[derive(Debug, Clone)]
pub struct AstNode {
    pub node_type: NodeType,
    pub content: String,
    pub raw_body: Option<String>,
    pub compliance_context: Vec<ComplianceContext>,
    pub children: Vec<AstNode>,
    pub metadata: HashMap<String, String>,
}

/// Types of AST nodes
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    /// Root node of the AST
    Root,
    /// Function or method declaration
    Function,
    /// Class declaration
    Class,
    /// Variable declaration
    Variable,
    /// A function call, like `do_something()`
    CallExpression,
    /// An assignment, like `x = y`
    AssignmentExpression,
    /// A generic expression (to be refined)
    Expression,
    /// A generic statement (to be refined)
    Statement,
    /// An 'if' statement
    IfStatement,
    /// A 'return' statement
    ReturnStatement,
    /// Comment with compliance annotation
    ComplianceComment,
    /// Block of code
    Block,
    /// Import statement
    Import,
    /// Export statement
    Export,
    /// Application node
    Application,
    /// Module node
    Module,
    /// Component node
    Component,
}

/// Represents compliance context attached to code elements
#[derive(Debug, Clone)]
pub struct ComplianceContext {
    pub prefix: String,
    pub standard: ComplianceStandard,
    pub rules: Vec<String>,
    pub parameters: HashMap<String, String>,
}

/// Represents the result of parsing ISO-Code
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub ast: AstNode,
    pub compliance_violations: Vec<ComplianceViolation>,
    pub warnings: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Represents a compliance violation found during analysis
#[derive(Debug, Clone)]
pub struct ComplianceViolation {
    pub rule_id: String,
    pub severity: RuleSeverity,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub suggestion: Option<String>,
}

/// Configuration for the semantic engine
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub enabled_standards: Vec<ComplianceStandard>,
    pub strict_mode: bool,
    pub output_format: OutputFormat,
    pub custom_rules: Vec<ComplianceRule>,
}

/// Output formats supported by the engine
#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Json,
    Yaml,
    Xml,
    PlainText,
    Markdown,
}

impl fmt::Display for ComplianceStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComplianceStandard::Security => write!(f, "Security (S.O.S)"),
            ComplianceStandard::Privacy => write!(f, "Privacy (G.D.P.R)"),
            ComplianceStandard::Quality => write!(f, "Quality (Q.M.S)"),
            ComplianceStandard::Accessibility => write!(f, "Accessibility (A.C.C)"),
            ComplianceStandard::Sustainability => write!(f, "Sustainability (S.U.S)"),
            ComplianceStandard::Diversity => write!(f, "Diversity (D.I.V)"),
            ComplianceStandard::Painel => write!(f, "Painel Parental (P.A.I.N.E.L)"),
            ComplianceStandard::Relato => write!(f, "Relatórios (R.E.L.A.T.O)"),
            ComplianceStandard::Algorithm => write!(f, "Algoritmos (A.L.G.O.R.I.T.H.M)"),
            ComplianceStandard::Lootbox => write!(f, "Loot Boxes (L.O.O.T.B.O.X)"),
            ComplianceStandard::SdkScan => write!(f, "SDK Scan (S.D.K.S.C.A.N)"),
            ComplianceStandard::Custom(name) => write!(f, "Custom ({})", name),
        }
    }
}

impl fmt::Display for RuleSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleSeverity::Critical => write!(f, "CRITICAL"),
            RuleSeverity::High => write!(f, "HIGH"),
            RuleSeverity::Medium => write!(f, "MEDIUM"),
            RuleSeverity::Low => write!(f, "LOW"),
            RuleSeverity::Info => write!(f, "INFO"),
        }
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            enabled_standards: vec![ComplianceStandard::Security, ComplianceStandard::Privacy],
            strict_mode: false,
            output_format: OutputFormat::Json,
            custom_rules: Vec::new(),
        }
    }
}

/// Helper functions for working with compliance contexts
impl ComplianceContext {
    /// Creates a new compliance context from a prefix string
    pub fn from_prefix(prefix: &str) -> Option<Self> {
        let standard = match prefix {
            "S.O.S" => ComplianceStandard::Security,
            "G.D.P.R" => ComplianceStandard::Privacy,
            "Q.M.S" => ComplianceStandard::Quality,
            "A.C.C" => ComplianceStandard::Accessibility,
            "S.U.S" => ComplianceStandard::Sustainability,
            "D.I.V" => ComplianceStandard::Diversity,
            "P.A.I.N.E.L" => ComplianceStandard::Painel,
            "R.E.L.A.T.O" => ComplianceStandard::Relato,
            "A.L.G.O.R.I.T.H.M" => ComplianceStandard::Algorithm,
            "L.O.O.T.B.O.X" => ComplianceStandard::Lootbox,
            "S.D.K.S.C.A.N" => ComplianceStandard::SdkScan,
            _ => return None,
        };

        Some(Self {
            prefix: prefix.to_string(),
            standard,
            rules: Vec::new(),
            parameters: HashMap::new(),
        })
    }
}

/// Helper functions for AST nodes
impl AstNode {
    /// Creates a new AST node
    pub fn new(node_type: NodeType, content: String) -> Self {
        Self {
            node_type,
            content,
            raw_body: None,
            compliance_context: Vec::new(),
            children: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Adds a compliance context to this node
    pub fn add_compliance_context(&mut self, context: ComplianceContext) {
        self.compliance_context.push(context);
    }

    /// Adds a child node
    pub fn add_child(&mut self, child: AstNode) {
        self.children.push(child);
    }
}


