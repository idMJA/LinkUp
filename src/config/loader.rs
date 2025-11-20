use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::models::Config;

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path).context("Failed to read config file")?;
        let config: Self = toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    }
}
