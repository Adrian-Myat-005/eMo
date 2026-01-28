pub mod lexer;
pub mod memory;
pub mod parser;
pub mod types;
pub mod safe_io;
pub mod executor;
pub mod shell_helper;
pub mod config;
pub mod tvrus_client;

use colored::*;
use crate::memory::Backpack;
use crate::lexer::Tokenizer;
use crate::parser::Parser;
use crate::safe_io::Guardian;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Config, CompletionType};
use crate::shell_helper::SadSmileHelper;

pub fn execute_script(filename: &str) {
    let mut backpack = Backpack::new();
    let guardian = Guardian::new(false);

    let contents = match std::fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} Failed to read script '{}': {}", "❌".red(), filename, e);
            return;
        }
    };

    match Tokenizer::tokenize(&contents) {
        Ok(tokens) => {
            match Parser::parse(tokens) {
                Ok(cmd) => {
                    executor::execute(cmd, &mut backpack, &guardian);
                }
                Err(e) => {
                     eprintln!("{} {}", "Parser Error:".red(), e);
                }
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Lexer Error:".red(), e);
        }
    }
}

pub fn run_repl() {
    let mut backpack = Backpack::new();
    let guardian = Guardian::new(false);

    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    let mut editor = Editor::<SadSmileHelper, rustyline::history::DefaultHistory>::with_config(config).expect("Failed to init editor");
    editor.set_helper(Some(SadSmileHelper::new()));

    let history_path = std::env::var("HOME").ok().map(|p| std::path::PathBuf::from(p).join(".ss_history"));
    if let Some(ref path) = history_path {
        let _ = editor.load_history(path);
    }

    println!("{}", "Sadsmile Shell (Bash-like Mode)".green().bold());
    println!("Type 'exit' to quit.");

    loop {
        let readline = editor.readline("ss$ ");

        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                
                let _ = editor.add_history_entry(input);

                if input == "exit" {
                    break;
                }

                match Tokenizer::tokenize(input) {
                    Ok(tokens) => {
                        match Parser::parse(tokens) {
                            Ok(cmd) => {
                                executor::execute(cmd, &mut backpack, &guardian);
                            }
                            Err(e) => {
                                eprintln!("{} {}", "Parser Error:".red(), e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{} {}", "Lexer Error:".red(), e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    if let Some(ref path) = history_path {
        let _ = editor.save_history(path);
    }
}
