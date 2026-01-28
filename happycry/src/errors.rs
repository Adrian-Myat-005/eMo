use thiserror::Error;
use colored::*;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum HappyError {
    #[error("😢 Confusion Error: {0}")]
    SyntaxError(String),
    
    #[error("🚫 Missing Power Error: {0}")]
    LibraryNotFound(String),
    
    #[error("❓ Variable Not Found: {0}")]
    VariableNotFound(String),

    #[error("💥 Runtime Error: {0}")]
    RuntimeError(String),
}

pub fn report_error(err: HappyError) {
    eprintln!("{}", err.to_string().red().bold());
}
