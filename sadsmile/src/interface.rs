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

    let width = 64;
    let border = "═".repeat(width);
    
    // Header with "Glow" effect using bold cyan
    println!("{}", border.blue());
    println!("  {}            {}", "eMo UNIFIED NEXUS ENVIRONMENT".cyan().bold(), "v4.0.0-PRO".green());
    println!("{}", border.blue());

    // Status Panels
    println!("  {} {}    {} {}    {} {}", 
             "SYSTEM:".white().dimmed(), "STABLE".green().bold(), 
             "KERNEL:".white().dimmed(), "LOCKED".yellow(), 
             "UPLINK:".white().dimmed(), "SYNCED".green());
    
    // Module Grid
    println!("  {} [SS] {}    [HPY] {}    [TVR] {}", 
             "ENGINES:".white().dimmed(),
             "READY".blue(), 
             "READY".blue(), 
             "STANDBY".magenta());

    println!("{}", border.blue());
    println!("  {} | {} | {}", 
             "status: refresh hud".black().on_cyan(),
             "help: nexus guide".black().on_cyan(),
             "exit: logout".black().on_red());
    println!("{}", border.blue());
    println!(""); 
}

pub fn draw_nexus_help() {
    println!("\n{}", "◈ eMo NEXUS COMMAND GUIDE ◈".cyan().bold());
    println!("{}", "--------------------------------------------------".blue());
    println!("  {}      Redraw the immersive dashboard", "status".green());
    println!("  {}      Clear screen and refresh HUD", "clear".green());
    println!("  {}      Compile a HappyCry automation script", "happy".blue());
    println!("  {}      Run an eMo or SadSmile script", "emo".blue());
    println!("  {}      Synthesize libraries from external repos", "shadow".yellow());
    println!("  {}      Terminate the Nexus link", "exit".red());
    println!("{}", "--------------------------------------------------".blue());
    println!("");
}

pub fn prompt(pwd: &str) -> String {
    // Format: eMo ◈ ~/projects/my_app $ 
    format!("{} {} {} ", "eMo".blue().bold(), "◈".white(), pwd.replace("/home/adrian", "~").purple())
}