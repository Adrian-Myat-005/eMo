use sadsmile::{execute_script, run_repl, interface};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // If arguments are provided, just run the script (Script Mode)
    if args.len() > 1 {
        execute_script(&args[1]);
    } else {
        // Interactive Mode: Enter the Environment
        interface::boot_sequence();
        run_repl();
    }
}