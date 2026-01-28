use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Result, Context};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct VirusConfig {
    pub package: PackageConfig,
    pub ai: AiConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AiConfig {
    pub endpoint: String,
    pub model: String,
    pub api_key: String,
    pub persona: String,
}

impl VirusConfig {
    /// Load the configuration from happy.tvrus
    pub fn load() -> Result<Self> {
        let content = fs::read_to_string("happy.tvrus")
            .context("Could not find 'happy.tvrus'. Run 'virus init' first.")?;
        let config: VirusConfig = toml::from_str(&content)
            .context("Failed to parse 'happy.tvrus'. Check TOML syntax.")?;
        Ok(config)
    }

    /// Create a default happy.tvrus file
    pub fn init_default() -> Result<()> {
        if Path::new("happy.tvrus").exists() {
            println!("happy.tvrus already exists.");
            return Ok(());
        }

        let default_config = VirusConfig {
            package: PackageConfig {
                name: "my_happy_app".to_string(),
                version: "0.1.0".to_string(),
            },
            ai: AiConfig {
                endpoint: "https://api.groq.com/openai/v1/chat/completions".to_string(),
                model: "llama3-70b-8192".to_string(),
                api_key: "gsk_...".to_string(),
                persona: "You are a HappyCry expert. Write secure SecOps code.".to_string(),
            },
        };

        let toml_string = toml::to_string_pretty(&default_config)?;
        let mut file = fs::File::create("happy.tvrus")?;
        file.write_all(toml_string.as_bytes())?;
        
        Ok(())
    }
}
