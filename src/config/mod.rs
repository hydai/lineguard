use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub checks: CheckConfig,
    pub ignore_patterns: Vec<String>,
    pub file_extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckConfig {
    pub newline_ending: bool,
    pub trailing_spaces: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            checks: CheckConfig {
                newline_ending: true,
                trailing_spaces: true,
            },
            ignore_patterns: vec![],
            file_extensions: vec![],
        }
    }
}

pub fn load_config(_path: Option<&Path>) -> Result<Config, anyhow::Error> {
    Ok(Config::default())
}
