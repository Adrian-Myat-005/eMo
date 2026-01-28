use clap::{Parser, Subcommand};
use colored::*; // Assuming colored is used for styling output
use std::process::Command;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "shadow")]
#[command(about = "The Shadow System: Learn and Clone from Repositories", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Learn a repository and create a .shw blueprint
    Learn {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Clone a .shw blueprint into HappyCry code
    Clone {
        #[arg(index = 1)]
        file: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct ShadowFile {
    source_url: String,
    project_type: String,
    files: Vec<String>,
    readme_summary: Option<String>,
    learned_patterns: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Learn { url, output } => {
            println!("{} {} {}", "🌑".black().on_white(), "Shadow System engaged on:", url);
            
            let temp_dir = ".shadow_cache";
            if Path::new(temp_dir).exists() {
                fs::remove_dir_all(temp_dir).ok();
            }

            println!("{} Cloning repository...", "⬇️".cyan());
            let status = Command::new("git")
                .arg("clone")
                .arg("--depth")
                .arg("1") // Shallow clone for speed
                .arg(&url)
                .arg(temp_dir)
                .status();

            if status.is_err() || !status.unwrap().success() {
                eprintln!("{} Failed to clone repository. Is git installed?", "❌".red());
                return;
            }

            println!("{} Analyzing structure...", "🧠".purple());
            let mut file_list = Vec::new();
            let mut project_type = "Unknown".to_string();
            let mut readme_content = None;
            let mut learned_patterns = Vec::new(); // Initialize learned_patterns here

            for entry in WalkDir::new(temp_dir) {
                if let Ok(entry) = entry {
                    if entry.file_type().is_file() {
                        if let Some(name) = entry.file_name().to_str() {
                            if !name.starts_with(".git") {
                                file_list.push(name.to_string());

                                // Detect Type
                                if name == "Cargo.toml" { project_type = "Rust".to_string(); }
                                if name == "package.json" { project_type = "NodeJS/Web".to_string(); }
                                if name == "requirements.txt" || name == "pyproject.toml" { project_type = "Python".to_string(); }
                                if name == "go.mod" { project_type = "Go".to_string(); }
                                if name == "index.html" && project_type == "Unknown" { project_type = "Static Web".to_string(); }

                                // Read Readme
                                if name.to_lowercase().starts_with("readme") {
                                    if let Ok(c) = fs::read_to_string(entry.path()) {
                                        readme_content = Some(c.chars().take(500).collect::<String>() + "...");
                                    }
                                }

                                // Absorb Logic (Python)
                                if name.ends_with(".py") {
                                    if let Ok(content) = fs::read_to_string(entry.path()) {
                                        for line in content.lines() {
                                            if line.trim().starts_with("def ") {
                                                let sig = line.trim().trim_start_matches("def ").trim_end_matches(':');
                                                learned_patterns.push(format!("Python Fn: {}", sig));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            println!("   Detected Project Type: {}", project_type.yellow());
            println!("   Absorbed {} patterns.", learned_patterns.len().to_string().cyan());

            let shadow_data = ShadowFile {
                source_url: url.clone(),
                project_type,
                files: file_list.clone(),
                readme_summary: readme_content,
                learned_patterns: learned_patterns,
            };

            let json = serde_json::to_string_pretty(&shadow_data).unwrap();
            let out_file = output.unwrap_or_else(|| "blueprint.shw".to_string());
            
            fs::write(&out_file, json).expect("Failed to write .shw file");
            
            println!("{} Blueprint saved to {}", "💾".green(), out_file);
            println!("{} You can now use 'shadow clone {}' to scaffold this in HappyCry.", "💡".yellow(), out_file);
            
            fs::remove_dir_all(temp_dir).ok();
        }
        Commands::Clone { file } => {
            println!("{} Reading blueprint {}...", "📖".cyan(), file);
            if let Ok(content) = fs::read_to_string(&file) {
                if let Ok(data) = serde_json::from_str::<ShadowFile>(&content) {
                    println!("{} Source: {} ({})", "🔗".blue(), data.source_url, data.project_type);
                    println!("{} Generating HappyCry scaffold...", "🔨".green());
                    
                    let mut hpy_code = format!(
                        "#happy\n# Generated by Shadow from {}\n# Type: {}\n\n",
                        data.source_url, data.project_type
                    );

                    if let Some(readme) = data.readme_summary {
                        hpy_code.push_str("#_ Source README:\n");
                        hpy_code.push_str(&readme.replace("_#", "")); // Prevent closing comment injection
                        hpy_code.push_str("\n_#\n\n");
                    }
                    
                    if !data.learned_patterns.is_empty() {
                         hpy_code.push_str("# Learned Capabilities:\n");
                         for pattern in &data.learned_patterns {
                             hpy_code.push_str(&format!("# - {}\n", pattern));
                         }
                         hpy_code.push_str("\n");
                    }

                    match data.project_type.as_str() {
                        "NodeJS/Web" | "Static Web" => {
                            hpy_code.push_str("server new port 3000\n");
                            hpy_code.push_str("page \"/\" do\n");
                            hpy_code.push_str("    title \"Shadow Clone\"\n");
                            hpy_code.push_str("    add header \"Cloned from Web Project\"\n");
                            hpy_code.push_str("end\n\n");
                            hpy_code.push_str("serve \"Web Service Online\"\n");
                        }
                        "Rust" | "Go" | "Python" => {
                             hpy_code.push_str("async task main_logic do\n");
                             hpy_code.push_str("    say \"Running logic cloned from backend project...\"\n");
                             hpy_code.push_str("end\n\n");
                             hpy_code.push_str("await main_logic\n");
                        }
                        _ => {
                            hpy_code.push_str("say \"Shadow clone active.\"\n");
                        }
                    }
                    
                    fs::write("main.hpy", hpy_code).ok();
                    println!("{} Generated 'main.hpy'", "✅".green());
                } else {
                    eprintln!("Invalid .shw file format.");
                }
            } else {
                eprintln!("File not found.");
            }
        }
    }
}