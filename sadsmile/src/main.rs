use sadsmile::{execute_script, run_repl};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        execute_script(&args[1]);
    } else {
        run_repl();
    }
}
