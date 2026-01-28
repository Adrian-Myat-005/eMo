use sadsmile::{execute_script, run_repl, interface};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Detect Identity
    let bin_path = std::env::current_exe().unwrap_or_default();
    let bin_name = bin_path.file_name().unwrap_or_default().to_string_lossy();
    
    let is_nexus = bin_name.contains("nexus") || args.contains(&"--visual".to_string());

    if args.len() > 1 && !args[1].starts_with("-") {
        // Script Mode
        execute_script(&args[1]);
    } else {
        // Interactive Mode
        if is_nexus {
            interface::boot_sequence();
        }
        run_repl(is_nexus);
    }
}
