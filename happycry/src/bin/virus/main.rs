mod config;

use clap::{Parser, Subcommand};
use colored::*;
use walkdir::WalkDir;
use std::fs;
use config::VirusConfig;
use anyhow::{Result, Context, anyhow};
use serde_json::json;

#[derive(Parser)]
#[command(name = "tvrus")]
#[command(about = "ThinkingVirus: AI-Powered API Editor & Generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new configuration file
    Init,
    /// Generate code or APIs based on natural language instructions
    Vibe {
        #[arg(index = 1)]
        instruction: String,
    },
    /// Edit an existing file using natural language
    Edit {
        #[arg(index = 1)]
        file: String,
        #[arg(index = 2)]
        instruction: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            VirusConfig::init_default()?;
            println!("{} {}", "🦠".green(), "Initialized 'happy.tvrus' config.".bold());
        }
        Commands::Vibe { instruction } => {
            handle_ai_request(None, instruction)?;
        }
        Commands::Edit { file, instruction } => {
            handle_ai_request(Some(file), instruction)?;
        }
    }

    Ok(())
}

fn handle_ai_request(target_file: Option<String>, instruction: String) -> Result<()> {
    // 1. Load Config
    let config = VirusConfig::load()?;
    println!("{} {}", "🧠".purple(), format!("ThinkingVirus ({}) active...", config.ai.model).bold());

    // 2. Gather Context
    let mut context_buffer = String::new();
    
    if let Some(ref f) = target_file {
        let content = fs::read_to_string(f).context("Failed to read target file")?;
        context_buffer.push_str(&format!("--- TARGET FILE: {} ---\n{}\n\n", f, content));
        println!("   loaded target file: {}", f.cyan());
    } else {
        // Scan for context if generating from scratch
        let walker = WalkDir::new(".").into_iter().filter_map(|e| e.ok());
        let mut file_count = 0;
        context_buffer.push_str("Project Context:\n");
        for entry in walker {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "hpy") {
                if let Ok(content) = fs::read_to_string(path) {
                    context_buffer.push_str(&format!("\n--- FILE: {} ---\n{}\n", path.display(), content));
                    file_count += 1;
                }
            }
        }
        if file_count > 0 { println!("   Scanned {} .hpy files for context.", file_count); }
    }

    // 3. Call AI
    let client = reqwest::blocking::Client::new();
    
    let sys_prompt = if target_file.is_some() {
        "You are an expert code editor. Output ONLY the modified code. Do not wrap in markdown blocks if possible, or plain text."
    } else {
        &config.ai.persona
    };

    let payload = json!({
        "model": config.ai.model,
        "messages": [
            { "role": "system", "content": sys_prompt },
            { "role": "user", "content": format!("{}\n\nInstruction: {}", context_buffer, instruction) }
        ]
    });

    println!("   Connecting to {}...", config.ai.endpoint);

    let res = client.post(&config.ai.endpoint)
        .header("Authorization", format!("Bearer {}", config.ai.api_key))
        .json(&payload)
        .send()
        .context("Failed to contact AI API")?;

    if !res.status().is_success() {
        let status = res.status();
        let error_msg = res.text().unwrap_or_default();
        return Err(anyhow!("API Error {}: {}", status, error_msg));
    }

    let response_json: serde_json::Value = res.json().context("Failed to parse API JSON response")?;

    // 4. Output
    if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
        println!("\n{}", "--- RESULT ---".green().bold());
        println!("{}", content);
        println!("{}", "--------------".green().bold());
        
        // If editing, maybe suggest overwriting? For now, just print.
    } else {
        return Err(anyhow!("Invalid response format."));
    }
    
    Ok(())
}
