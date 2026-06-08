use super::types::Config;
use anyhow::Result;
use std::path::Path;

pub fn load_config(path: Option<&str>) -> Result<Config> {
    let config_path = path.unwrap_or("user-config.json");

    if Path::new(config_path).exists() {
        let content = std::fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&content)?;
        println!("✅ Loaded config from {}", config_path);
        Ok(config)
    } else {
        println!("⚠️  No config file found. Using defaults.");
        Ok(Config::default())
    }
}