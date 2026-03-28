// src/rules/loader.rs
use super::models::RuleConfig;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub struct RuleLoader;

impl RuleLoader {
    /// Loads a rule configuration from a YAML file.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<RuleConfig> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read rule file at {:?}", path.as_ref()))?;
        let config: RuleConfig = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse YAML in {:?}", path.as_ref()))?;
        Ok(config)
    }

    /// Loads multiple rule configurations from a directory.
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<Vec<RuleConfig>> {
        let mut configs = Vec::new();
        let path = dir.as_ref();

        if !path.exists() || !path.is_dir() {
            return Ok(configs);
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yml") {
                if let Ok(config) = Self::load_from_file(&path) {
                    configs.push(config);
                }
            }
        }
        Ok(configs)
    }
}
