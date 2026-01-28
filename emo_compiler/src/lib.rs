pub mod lexer;
pub mod ast;
pub mod parser;
pub mod interpreter;
pub mod codegen_c;
pub mod formatter;
pub mod shadow_synthesizer;
pub mod type_checker;

use std::fs;
use std::process::Command;
use colored::*;
use codegen_c::Dimension;

pub fn compile(file: &str, output: Option<String>) {
    println!("{} Building {}...", "   Building".green().bold(), file);
    
    let content = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} Could not read file: {}", "Error:".red().bold(), e);
            return;
        }
    };

    let mut parser = parser::Parser::new(&content);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            // report_error(&content, file, &e.message, e.span);
            eprintln!("{} {}: {}", "Error:".red().bold(), file, e.message);
            return;
        }
    };

    println!("{} Checking types...", "   Checking".magenta().bold());
    let mut tc = type_checker::TypeChecker::new();
    if let Err(e) = tc.check_program(&ast) {
        eprintln!("{} {}: {}", "Type Error:".red().bold(), file, e.message);
        // In a real compiler, we might stop here.
        // For now, we log it and continue to see what else we catch.
        // return; 
    }

    let dimension = if file.ends_with(".ss") {
        Dimension::SadSmile
    } else if file.ends_with(".hpy") {
        Dimension::HappyCry
    } else if file.ends_with(".tvrus") {
        Dimension::ThinkingVirus
    } else if file.ends_with(".shw") {
        Dimension::Shadow
    } else {
        Dimension::Default
    };

    let mut codegen = codegen_c::CodegenC::new(dimension);
    let c_code = codegen.generate(&ast);

    let c_file = format!("{}.c", file);
    fs::write(&c_file, c_code).expect("Failed to write C file");

    let runtime_h = include_str!("emo_runtime.h");
    fs::write("emo_runtime.h", runtime_h).expect("Failed to write runtime header");

    let out_file = output.unwrap_or_else(|| file.replace(".emo", ""));
    
    println!("{} Compiling C code with GCC...", "   Compiling".blue().bold());
    let status = Command::new("gcc")
        .arg("-std=c11")
        .arg(&c_file)
        .arg("-o")
        .arg(&out_file)
        .status()
        .expect("Failed to execute GCC");

    if status.success() {
        println!("{} Build successful: {}", "   Finished".green().bold(), out_file);
        let _ = fs::remove_file(c_file);
        let _ = fs::remove_file("emo_runtime.h");

        // Merge Feature: If HappyCry dimension, and it has a UI/Web component, 
        // we might want to suggest starting the server.
        if dimension == Dimension::HappyCry {
            println!("{} HappyCry project detected. Run with './{}' to start the fluid interface.", "   Hint:".cyan().bold(), out_file);
        }
    } else {
        eprintln!("{} Native compilation failed", "Error:".red().bold());
    }
}

pub fn run(file: &str) {
    println!("{} Interpreting {}...", "   Running".cyan().bold(), file);
    let content = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} Could not read file: {}", "Error:".red().bold(), e);
            return;
        }
    };
    let mut parser = parser::Parser::new(&content);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{} {}: {}", "Error:".red().bold(), file, e.message);
            return;
        }
    };
    let mut interpreter = interpreter::Interpreter::new();
    if let Err(e) = interpreter.interpret(ast) {
        eprintln!("{} Interpretation error: {}", "Error:".red().bold(), e);
    }
}

pub fn format_file(file: &str) {
    let content = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} Could not read file: {}", "Error:".red().bold(), e);
            return;
        }
    };
    let mut parser = parser::Parser::new(&content);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{} {}: {}", "Error:".red().bold(), file, e.message);
            return;
        }
    };
    let mut formatter = formatter::Formatter::new();
    let formatted = formatter.format(&ast);
    if let Err(e) = fs::write(file, formatted) {
        eprintln!("{} Could not write file: {}", "Error:".red().bold(), e);
    } else {
        println!("{} Formatted {}", "   Formatted".green().bold(), file);
    }
}
