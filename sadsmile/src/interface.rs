use colored::*
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn boot_sequence() {
    print!("\x1B[2J\x1B[1;1H"); // Clear Screen
    
    let steps = vec![
        ("CORE", "Initializing Kernel..."),
        ("MEM", "Allocating Zero-Copy Arena..."),
        ("NET", "Scanning Neural Ports..."),
        ("AI", "Waking Up ThinkingVirus..."),
        ("UI", "Loading HappyCry Engine..."),
    ];

    println!("{}", "eMo SYSTEM v4.0 (UNIFIED)".blue().bold());
    println!("{}", "=============================".blue());

    for (module, desc) in steps {
        print!("[{}] {} ", module.green(), desc); 
        io::stdout().flush().unwrap();
        
        // Fake loading dots
        for _ in 0..3 {
            print!(".");
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        println!(" {}", "OK".green().bold());
        thread::sleep(Duration::from_millis(50));
    }

    println!("\n{}", "SYSTEM READY. WELCOME, ARCHITECT.".cyan().bold());
    thread::sleep(Duration::from_millis(500));
    print!("\x1B[2J\x1B[1;1H"); // Clear Screen again for clean start
    draw_banner();
}

pub fn draw_banner() {
    let banner = r#" 
   .____. .____.    
   |    | |    |    eMo Unified Interface
   |    |_|    |    ---------------------
   |    .-.    |    [S]adSmile  : Active
   |    | |    |    [H]appyCry  : Ready
   |____| |____|    [T]vrus     : Listening
    "#;
    println!("{}", banner.cyan());
    println!("{}", "Type 'exit' to disconnect.".yellow().dimmed());
    println!("");
}

pub fn prompt(pwd: &str) -> String {
    // Format: eMo ◈ ~/projects/my_app $
    format!("{} {} {} ", "eMo".blue().bold(), "◈".white(), pwd.replace("/home/adrian", "~").purple())
}
