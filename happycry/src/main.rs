use std::env;
use happy_cry_lang::build;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: happy build <file.hpy>");
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    if command == "build" {
        build(filename);
    } else {
        println!("Unknown command: {}", command);
    }
}