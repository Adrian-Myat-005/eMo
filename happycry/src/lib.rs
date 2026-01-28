pub mod lexer;
pub mod parser;
pub mod codegen_fixed;
pub mod ast;
pub mod builder;
pub mod errors;

use std::fs;
use std::process::Command;

pub fn build(filename: &str) {
    println!("[*] Reading {}...", filename);
    let source_code = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to read file: {}", e);
            return;
        }
    };

    // --- PHASE 1: LEXER ---
    println!("[1/3] Tokenizing...");
    let tokens = lexer::tokenize(&source_code);

    // --- PHASE 2: PARSER ---
    println!("[2/3] Parsing...");
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();

    // --- PHASE 3: GENERATOR ---
    println!("[3/3] Generating Rust...");
    let rust_code = codegen_fixed::generate(ast);

    // --- PHASE 4: COMPILATION (Cargo) ---
    println!("[*] Compiling Binary (via Cargo)...");
    
    // Create temp project structure
    let path = std::path::Path::new(filename);
    let project_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("happy_app").replace(".", "_");
    let build_dir = format!(".build_{}", project_name);
    let _ = fs::create_dir_all(format!("{}/src", build_dir));

    // Write Cargo.toml
    let cargo_toml = format!(r#"
[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
tokio = {{ version = "1", features = ["full"] }}
serde = {{ version = "1.0", features = ["derive"] }}

[workspace]
"#, project_name);

    fs::write(format!("{}/Cargo.toml", build_dir), cargo_toml).expect("Failed to write Cargo.toml");
    fs::write(format!("{}/src/main.rs", build_dir), rust_code).expect("Failed to write main.rs");

    // Build
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(&build_dir)
        .status()
        .expect("Failed to run cargo");

    if status.success() {
        // Move binary out
        let binary_name = if cfg!(windows) { format!("{}.exe", project_name) } else { project_name.clone() };
        let _ = fs::rename(
            format!("{}/target/release/{}", build_dir, binary_name),
            &binary_name
        );
        println!("[+] Build Complete! Run with ./{}", binary_name);
         let _ = fs::remove_dir_all(build_dir);
    } else {
        println!("[-] Compilation Failed.");
    }
}
