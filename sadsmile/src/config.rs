use crate::memory::Backpack;
use crate::safe_io::Guardian;
use crate::executor;
use crate::lexer::Tokenizer;
use crate::parser::Parser;
use colored::*;
use std::fs;

pub fn load_config(memory: &mut Backpack, guardian: &Guardian) {
    // 1. Find Home Directory
    let home = match dirs::home_dir() {
        Some(path) => path,
        None => return,
    };

    // 2. Check for .ssrc
    let config_path = home.join(".ssrc");
    if !config_path.exists() {
        return;
    }

    println!("{}", "⚙️  Loading configuration...".dimmed());

    // 3. Read & Execute
    if let Ok(contents) = fs::read_to_string(&config_path) {
        // Treat each line as a command (simple config)
        // Or read whole file. Bash reads .bashrc line by line? No, it sources it.
        // Our parser handles lists. Let's try parsing the whole thing if we can, or line by line.
        // Since our parser parses a "List", we can just feed it the whole file content if we want.
        // But the loop here iterates lines.
        
        for line in contents.lines() {
            let input = line.trim();
            if input.is_empty() || input.starts_with('#') { continue; }

            match Tokenizer::tokenize(input) {
                Ok(tokens) => {
                    match Parser::parse(tokens) {
                        Ok(cmd) => {
                             executor::execute(cmd, memory, guardian);
                        },
                        Err(e) => eprintln!("{} Config Error: {}", "⚠️".yellow(), e),
                    }
                }
                Err(e) => eprintln!("{} Config Syntax Error: {}", "⚠️".yellow(), e),
            }
        }
    }
}
