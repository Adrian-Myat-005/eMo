use std::fs;
use colored::*;

#[derive(Clone)]
pub struct Guardian {
    pub is_real: bool,
}

impl Guardian {
    pub fn new(is_real: bool) -> Self {
        Guardian { is_real }
    }

    pub fn delete_file(&self, path: &str) -> Result<String, String> {
        // Safety: Hardcoded checks to prevent deleting root or home
        if path == "/" {
            return Err("CRITICAL: Cannot delete root directory /".red().to_string());
        }
        
        if let Ok(home) = std::env::var("HOME") {
            if path == home || path == "~" {
                return Err("CRITICAL: Cannot delete home directory".red().to_string());
            }
        }

        if !self.is_real {
            return Ok(format!("{} Would delete {}", "👻 [GHOST]".yellow(), path));
        }

        match fs::remove_file(path) {
            Ok(_) => Ok(format!("{} Deleted {}", "🗑️".red(), path)),
            Err(e) => Err(format!("Failed to delete {}: {}", path, e)),
        }
    }
}