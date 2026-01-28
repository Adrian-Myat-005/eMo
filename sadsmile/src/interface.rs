use colored::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn typewriter(text: &str, speed: u64) {
    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(speed));
    }
}

pub fn boot_sequence() {
    print!("\x1B[2J\x1B[1;1H"); // Clear
    print!("\x1B[?25l"); // Hide cursor
    io::stdout().flush().unwrap();
    
    println!("\n  {}", "◈ eMo UNIFIED NEXUS UPLINK".cyan().bold());
    println!("  {}", "────────────────────────────────────────────".dimmed());
    
    let steps = vec![
        "AUTHENTICATING ARCHITECT...",
        "DECRYPTING KERNEL SEGMENTS...",
        "INITIALIZING NEURAL INTERFACE...",
        "SYNCHRONIZING DIMENSIONAL GATES...",
    ];

    for step in steps {
        print!("  {} ", "»".blue());
        typewriter(step, 15);
        
        // Stabilization animation
        print!(" ");
        let frames = vec!["◐", "◓", "◑", "◒"];
        for i in 0..6 {
            print!("\r  {} {} {}", "»".blue(), step, frames[i % 4].cyan());
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(60));
        }
        
        println!("\r  {} {} {}", "»".blue(), step, "DONE".green().bold());
    }

    // Final "Flash"
    thread::sleep(Duration::from_millis(300));
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    draw_dashboard();
    print!("\x1B[?25h"); // Show cursor
    io::stdout().flush().unwrap();
}

pub fn draw_dashboard() {
    let width = 64;
    let border = "━".repeat(width);
    
    println!("\n{}", border.blue());
    println!("  {}            {}", "eMo UNIFIED NEXUS ENVIRONMENT".cyan().bold(), "v4.0.0".green());
    println!("{}", border.blue());

    println!("  {} {}    {} {}    {} {}", 
             "STATUS:".white().dimmed(), "STABLE".green().bold(), 
             "KERNEL:".white().dimmed(), "LOCKED".yellow(), 
             "UPLINK:".white().dimmed(), "SYNCED".green());
    
    println!("  {} [SS] {}    [HPY] {}    [TVR] {}", 
             "MODULE:".white().dimmed(),
             "READY".blue(), 
             "READY".blue(), 
             "READY".blue());

    println!("{}", border.blue());
    println!("  {} | {} | {}", 
             "status: refresh".black().on_cyan(),
             "help: guide".black().on_cyan(),
             "exit: logout".black().on_red());
    println!("{}\n", border.blue());
}

pub fn draw_nexus_help() {
    println!("\n  {}", "◈ NEXUS COMMAND GUIDE".cyan().bold());
    println!("  {}", "────────────────────────────────────────────".dimmed());
    println!("  {:<10} Redraw the dashboard", "status".green());
    println!("  {:<10} Refresh environment", "clear".green());
    println!("  {:<10} Build HappyCry script", "happy".blue());
    println!("  {:<10} Run eMo script", "emo".blue());
    println!("  {:<10} Logout", "exit".red());
    println!("  {}\n", "────────────────────────────────────────────".dimmed());
}

pub fn prompt(pwd: &str) -> String {
    let clean_pwd = pwd.replace("/home/adrian", "~");
    format!("\n{} {} {} ", "eMo".blue().bold(), "◈".white(), clean_pwd.purple()) 
}