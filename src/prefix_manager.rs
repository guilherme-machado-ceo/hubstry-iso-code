//! Módulo para gerenciar o carregamento de prefixos de conformidade a partir de arquivos de configuração.

use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Clone)]
pub struct PrefixInfo {
    pub prefix: String,
    pub description: String,
    pub standard: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jurisdictions {
    pub jurisdictions: HashMap<String, Vec<PrefixInfo>>,
}

/// Enum para erros personalizados no gerenciamento de prefixos.
#[derive(Debug)]
pub enum PrefixError {
    Io(std::io::Error),
    Parse(serde_yaml::Error),
}

impl fmt::Display for PrefixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrefixError::Io(e) => write!(f, "Falha de I/O ao ler 'prefixes.yml': {}", e),
            PrefixError::Parse(e) => write!(f, "Falha ao analisar o conteúdo de 'prefixes.yml': {}", e),
        }
    }
}

impl Error for PrefixError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PrefixError::Io(e) => Some(e),
            PrefixError::Parse(e) => Some(e),
        }
    }
}

/// Carrega os prefixos do arquivo de configuração `prefixes.yml`.
/// Usa um OnceLock para garantir que o arquivo seja lido e analisado apenas uma vez.
/// Retorna um Result, pois a operação pode falhar.
pub fn try_get_prefix_map() -> Result<&'static HashMap<String, PrefixInfo>, PrefixError> {
    static PREFIX_MAP: OnceLock<HashMap<String, PrefixInfo>> = OnceLock::new();

    // Tenta obter o valor. Se já estiver inicializado, retorna.
    if let Some(map) = PREFIX_MAP.get() {
        return Ok(map);
    }

    // Se não estiver inicializado, executa a lógica de inicialização.
    let file_content = fs::read_to_string("prefixes.yml").map_err(PrefixError::Io)?;
    let config: Jurisdictions =
        serde_yaml::from_str(&file_content).map_err(PrefixError::Parse)?;

    let mut map = HashMap::new();
    for (_, prefixes) in config.jurisdictions {
        for prefix_info in prefixes {
            map.insert(prefix_info.prefix.clone(), prefix_info);
        }
    }

    // Tenta inserir o mapa no OnceLock. Se falhar, significa que outra thread
    // inicializou o valor enquanto estávamos trabalhando. Isso é seguro.
    // Ignoramos o resultado de `set` porque, em qualquer caso, o valor estará presente.
    let _ = PREFIX_MAP.set(map);

    // Agora, o valor com certeza está inicializado, então podemos usar `get().unwrap()`.
    Ok(PREFIX_MAP.get().unwrap())
}

// A função `is_compliance_prefix` foi removida, pois sua lógica foi centralizada
// no `semantic_engine` para evitar chamadas redundantes.