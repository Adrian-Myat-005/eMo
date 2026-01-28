use clap::{Parser as ClapParser, Subcommand};
use colored::*;
use std::path::Path;

#[derive(ClapParser)]
#[command(name = "eMo")]
#[command(version = "4.0")]
#[command(about = "The Unified eMo Ecosystem CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// File to run or build directly
    file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a project or file
    Build {
        file: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Run a script or enter REPL
    Run {
        file: Option<String>,
    },
    /// Enter the SadSmile shell
    Shell,
    /// (AI) Vibe code with ThinkingVirus
    Vibe {
        prompt: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Build { file, output } => {
                handle_build(&file, output);
            }
            Commands::Run { file } => {
                handle_run(file);
            }
            Commands::Shell => {
                sadsmile::run_repl(false);
            }
            Commands::Vibe { prompt } => {
                println!("{} ThinkingVirus is processing your vibe: '{}'", "🧠".magenta(), prompt);
                println!("(AI functionality coming soon in v4.1)");
            }
        }
    } else if let Some(file) = cli.file {
        handle_auto(&file);
    } else {
        // Default to shell if no args
        sadsmile::run_repl(false);
    }
}

fn handle_build(file: &str, output: Option<String>) {
    let path = Path::new(file);
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    match ext {
        "hpy" => {
            happy_cry_lang::build(file);
        }
        "emo" => {
            emo_compiler::compile(file, output);
        }
        "ss" => {
            println!("{} .ss files are usually interpreted. Use 'emo run {}' instead.", "Info:".yellow(), file);
        }
        _ => {
            println!("{} Unknown file extension: .{}", "Error:".red(), ext);
        }
    }
}

fn handle_run(file: Option<String>) {
    if let Some(f) = file {
        let path = Path::new(&f);
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        match ext {
            "emo" => {
                emo_compiler::run(&f);
            }
            "ss" => {
                sadsmile::execute_script(&f);
            }
            "hpy" => {
                println!("{} .hpy files must be built first. Use 'emo build {}'", "Info:".yellow(), f);
            }
            _ => {
                println!("{} Cannot run file with extension: .{}", "Error:".red(), ext);
            }
        }
    } else {
        sadsmile::run_repl(false);
    }
}

fn handle_auto(file: &str) {
    let path = Path::new(file);
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    match ext {
        "hpy" => happy_cry_lang::build(file),
        "emo" => emo_compiler::run(file),
        "ss" => sadsmile::execute_script(file),
        _ => println!("{} Unrecognized file: {}", "Error:".red(), file),
    }
}
