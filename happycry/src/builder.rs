use std::fs;
use std::process::Command;
use std::path::Path;
use anyhow::{Result, Context};
use colored::*;

pub struct Builder;

impl Builder {
    pub fn build(project_name: &str, rust_code: &str, target: &str, features: &[String]) -> Result<()> {
        let cache_dir = ".happy_cache";
        
        // 1. Initialize Cache Directory (if missing)
        if !Path::new(cache_dir).exists() {
            fs::create_dir_all(format!("{}/src", cache_dir))?;
        }

        // 2. Define Dependencies
        let mut deps = String::from(r#"
colored = "2.0"
human-panic = "1.0"
"#);

        if features.contains(&"async".to_string()) || features.contains(&"web".to_string()) || features.contains(&"p2p".to_string()) {
            deps.push_str("tokio = { version = \"1\", features = [\"full\"] }\n");
            deps.push_str("reqwest = { version = \"0.11\", default-features = false, features = [\"json\", \"blocking\", \"rustls-tls\"] }\n");
        }

        if features.contains(&"web".to_string()) {
             deps.push_str("actix-web = \"4\"\n");
             deps.push_str("maud = \"0.25\"\n");
        }
        
        if features.contains(&"db".to_string()) {
             deps.push_str("rusqlite = { version = \"0.29\", features = [\"bundled\"] }\n");
        }

        if features.contains(&"p2p".to_string()) {
            deps.push_str("libp2p = { version = \"0.50\", features = [\"tcp\", \"tokio\", \"gossipsub\", \"mdns\", \"noise\", \"yamux\", \"macros\"] }\n");
            deps.push_str("futures = \"0.3\"\n");
        }
        
        // Needed for FFI
        deps.push_str("libc = \"0.2\"\n");

        let cargo_toml_content = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
{}
"#, project_name, deps);

        // 3. Smart Cargo.toml Update (Avoid touching file if identical to preserve mtime)
        let cargo_toml_path = format!("{}/Cargo.toml", cache_dir);
        let should_write_toml = if Path::new(&cargo_toml_path).exists() {
            match fs::read_to_string(&cargo_toml_path) {
                Ok(existing) => existing != cargo_toml_content,
                Err(_) => true,
            }
        } else {
            true
        };

        if should_write_toml {
            fs::write(&cargo_toml_path, cargo_toml_content)?;
        }

        // 4. Always update the source code
        // Ensure src directory exists in case it was deleted manually but .happy_cache remained
        fs::create_dir_all(format!("{}/src", cache_dir))?;
        fs::write(format!("{}/src/main.rs", cache_dir), rust_code)?;

        println!("{} {} [{}]", "🚀".cyan(), "Compiling Ultimate Binary...".bold().white(), target.yellow());
        if !features.is_empty() {
             println!("   {} {:?}", "Features:".green(), features);
        }

        // 5. Build Command
        let mut cmd = Command::new("cargo");
        cmd.arg("build").arg("--release").current_dir(cache_dir);

        let binary_ext = if target == "windows" { ".exe" } else { "" };
        let mut binary_path = format!("{}/target/release/{}", cache_dir, project_name);

        if target == "windows" {
            cmd.arg("--target").arg("x86_64-pc-windows-gnu");
            binary_path = format!("{}/target/x86_64-pc-windows-gnu/release/{}.exe", cache_dir, project_name);
        }

        let status = cmd.status().context("Cargo build failed")?;

        if status.success() {
            let dest_path = format!("./{}{}", project_name, binary_ext);
            fs::copy(&binary_path, &dest_path)?;
            println!("{} {} {}", "✨".green(), "Success!".green().bold(), format!("Binary generated at {}", dest_path).white());
        } else {
             println!("{}", "Build failed. Cache directory preserved for debugging.".red());
        }

        Ok(())
    }
}
