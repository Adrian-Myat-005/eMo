use colored::*
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn boot_sequence() {
    print!("\x1B[2J\x1B[1;1H"); // Clear Screen
    print!("\x1B[?25l"); // Hide cursor
    io::stdout().flush().unwrap();

    println!("\n  {}\n", "eMo UNIFIED NEXUS".cyan().bold());
    println!("  {}\n", "────────────────────────────────────────────".dimmed());

    let steps = vec![
        "INITIALIZING KERNEL",
        "MAPPING MEMORY ARENA",
        "ESTABLISHING NEURAL LINK",
        "SYNCING AI MODELS",
        "LOADING NEXUS INTERFACE",
    ];

    for step in steps {
        print!("  {} %-30s ", "◈".blue(), step); 
        io::stdout().flush().unwrap();
        
        // Static delay for stability
        thread::sleep(Duration::from_millis(200));
        
        println!("{}", "DONE".green().bold());
        thread::sleep(Duration::from_millis(50));
    }

    print!("\x1B[?25h"); // Show cursor
    io::stdout().flush().unwrap();

    thread::sleep(Duration::from_millis(500));
    draw_dashboard();
}

pub fn draw_dashboard() {
    print!("\x1B[2J\x1B[1;1H"); // Clear
    
    let width = 64;
    let border = "━".repeat(width);
    
    println!("\n{}\
", border.blue());
    println!("  {}            {}", "eMo UNIFIED NEXUS ENVIRONMENT".cyan().bold(), "v4.0.0".green());
    println!("{}\
", border.blue());

    println!("  {} {}    {} {}    {} {}", 
             "STATUS:".white().dimmed(), "STABLE".green().bold(), 
             "KERNEL:".white().dimmed(), "LOCKED".yellow(), 
             "UPLINK:".white().dimmed(), "SYNCED".green());
    
    println!("  {} [SS] {}    [HPY] {}    [TVR] {}", 
             "MODULE:".white().dimmed(),
             "READY".blue(), 
             "READY".blue(), 
             "READY".blue());

    println!("{}\
", border.blue());
    println!("  {} | {} | {}", 
             "status: refresh".black().on_cyan(),
             "help: guide".black().on_cyan(),
             "exit: logout".black().on_red());
    println!("{}\
", border.blue());
}

pub fn draw_nexus_help() {
    println!("\n  {}\
", "◈ NEXUS COMMAND GUIDE".cyan().bold());
    println!("  {}\
", "────────────────────────────────────────────".dimmed());
    println!("  {:<10} Redraw the dashboard", "status".green());
    println!("  {:<10} Refresh environment", "clear".green());
    println!("  {:<10} Build HappyCry script", "happy".blue());
    println!("  {:<10} Run eMo script", "emo".blue());
    println!("  {:<10} Logout", "exit".red());
    println!("  {}\
", "────────────────────────────────────────────".dimmed());
}

pub fn prompt(pwd: &str) -> String {
    let clean_pwd = pwd.replace("/home/adrian", "~");
    format!("\n{} {} {} ", "eMo".blue().bold(), "◈".white(), clean_pwd.purple()) 
}