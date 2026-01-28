use colored::*
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn boot_sequence() {
    print!("\x1B[2J\x1B[1;1H"); // Clear Screen
    
    // Hide cursor
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();

    let steps = vec![
        ("CORE", "Initializing Kernel..."),
        ("MEM", "Allocating Zero-Copy Arena..."),
        ("NET", "Scanning Neural Ports..."),
        ("AI", "Waking Up ThinkingVirus..."),
        ("UI", "Loading HappyCry Engine..."),
    ];

    println!("{}", "eMo SYSTEM v4.0 (UNIFIED)".blue().bold());
    println!("{}", "=============================".blue());
    println!("");

    for (module, desc) in steps {
        // Print initial state: [   ] MODULE : Desc
        print!("   [{{}}] {{}} : {{}} ", "   ".dimmed(), module.blue().bold(), desc);
        io::stdout().flush().unwrap();
        
        // Animate the dots inside the brackets
        let anim_chars = vec![" . ", " ..", "..."];
        for i in 0..10 {
            // Move back to start of brackets
            print!("\r   [{{}}]", anim_chars[i % 3].cyan());
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(50));
        }

        // Finalize state: [ OK ]
        print!("\r   [{{}}] {{}} : {{}} \n", "OK".green().bold(), module.blue().bold(), desc);
        thread::sleep(Duration::from_millis(100));
    }

    // Show cursor again
    print!("\x1B[?25h");
    io::stdout().flush().unwrap();

    println!("\n{}", "SYSTEM READY. WELCOME, ARCHITECT.".cyan().bold());
    thread::sleep(Duration::from_millis(800));
    
    print!("\x1B[2J\x1B[1;1H"); // Clear Screen again for clean start
    draw_dashboard();
}

pub fn draw_dashboard() {
    // Clear screen and move to top
    print!("\x1B[2J\x1B[1;1H"); 

    let width = 60;
    let border = "=".repeat(width);
    
    // Header
    println!("{}", border.blue());
    println!("  {}            {}", "eMo UNIFIED INTELLIGENCE SYSTEM".cyan().bold(), "v4.0".green());
    println!("{}", border.blue());

    // Stats Row
    println!("  STATUS: {}    MEM: {}    NET: {}", 
             "ONLINE".green().bold(), 
             "ACTIVE".yellow(), 
             "CONNECTED".green());
    
    // Modules Row
    println!("  MODULES: [SS] {}  [HPY] {}  [TV] {}", 
             "READY".green(), 
             "READY".green(), 
             "IDLE".dimmed());

    println!("{}", border.blue());
    println!("{}", "Type 'exit' to disconnect.".black().on_white());
    println!(""); // Spacer
}

pub fn prompt(pwd: &str) -> String {
    // Format: eMo ◈ ~/projects/my_app $ 
    format!("{} {} {} ", "eMo".blue().bold(), "◈".white(), pwd.replace("/home/adrian", "~").purple())
}