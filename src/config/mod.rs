use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(default)]
    pub checks: CheckConfig,
    #[serde(default)]
    pub ignore_patterns: Vec<String>,
    #[serde(default)]
    pub file_extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CheckConfig {
    #[serde(default = "default_true")]
    pub newline_ending: bool,
    #[serde(default = "default_true")]
    pub trailing_spaces: bool,
}

fn default_true() -> bool {
    true
}

impl Default for CheckConfig {
    fn default() -> Self {
        Self {
            newline_ending: true,
            trailing_spaces: true,
        }
    }
}

pub fn load_config(explicit_path: Option<&Path>) -> Result<Config, anyhow::Error> {
    // If explicit path is provided, load from that file
    if let Some(path) = explicit_path {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let config: Config = toml::from_str(&content)?;
            return Ok(config);
        } else {
            return Err(anyhow::anyhow!(
                "Configuration file not found: {}",
                path.display()
            ));
        }
    }

    // Otherwise, search for .lineguardrc in current and parent directories
    let config_path = find_config_file()?;
    if let Some(path) = config_path {
        let content = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    } else {
        // No config file found, use defaults
        Ok(Config::default())
    }
}

fn find_config_file() -> Result<Option<PathBuf>, anyhow::Error> {
    let current_dir = std::env::current_dir()?;
    let mut dir = current_dir.as_path();

    loop {
        let config_path = dir.join(".lineguardrc");
        if config_path.exists() {
            return Ok(Some(config_path));
        }

        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
    }

    Ok(None)
}
