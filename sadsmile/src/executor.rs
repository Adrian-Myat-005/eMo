use std::process::{Command as SysCommand, Stdio};
use std::fs::File;
use std::path::PathBuf;

use crate::types::{Command, SimpleCommand, RedirectionType, ListOp, Value};
use crate::memory::Backpack;
use crate::safe_io::Guardian;

pub fn execute(cmd: Command, memory: &mut Backpack, guardian: &Guardian) -> bool {
    match cmd {
        Command::Simple(sc) => execute_simple(sc, memory, guardian).unwrap_or(false),
        Command::Pipeline(cmds) => execute_pipeline(cmds, memory, guardian).unwrap_or(false),
        Command::List(left, op, right) => {
            let success = execute(*left, memory, guardian);
            match op {
                ListOp::Semi => execute(*right, memory, guardian),
                ListOp::And => {
                    if success {
                        execute(*right, memory, guardian)
                    } else {
                        false
                    }
                }
                ListOp::Or => {
                    if !success {
                        execute(*right, memory, guardian)
                    } else {
                        true
                    }
                }
                ListOp::Background => {
                     // TODO: Background support
                     println!("Background execution not fully supported yet.");
                     execute(*right, memory, guardian)
                }
            }
        }
        Command::If { cond, then_branch, else_branch } => {
            if execute(*cond, memory, guardian) {
                execute(*then_branch, memory, guardian)
            } else if let Some(else_cmd) = else_branch {
                execute(*else_cmd, memory, guardian)
            } else {
                true
            }
        }
    }
}

fn execute_simple(cmd: SimpleCommand, memory: &mut Backpack, _guardian: &Guardian) -> Result<bool, String> {
    if cmd.words.is_empty() {
        return Ok(true);
    }

    let args: Vec<String> = cmd.words.iter().map(|w| expand_vars(w, memory)).collect();
    let command_name = &args[0];

    // Builtins
    match command_name.as_str() {
        "cd" => {
            let target = if args.len() > 1 {
                PathBuf::from(&args[1])
            } else {
                dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
            };
            match std::env::set_current_dir(&target) {
                Ok(_) => Ok(true),
                Err(e) => {
                    eprintln!("cd: {}: {}", target.display(), e);
                    Ok(false)
                }
            }
        }
        "exit" => {
            std::process::exit(0);
        }
        "export" => {
             // TODO: Real export. For now, store in Backpack.
             if args.len() > 1 {
                 let parts: Vec<&str> = args[1].splitn(2, '=').collect();
                 if parts.len() == 2 {
                     memory.hold(parts[0].to_string(), Value::String(parts[1].to_string()));
                     unsafe { std::env::set_var(parts[0], parts[1]); }
                 }
             }
             Ok(true)
        }
        _ => {
            // Check for HappyCry suite tools locally
            let mut cmd_path = PathBuf::from(command_name);
            if ["happy", "shadow", "tvrus"].contains(&command_name.as_str()) {
                let local_path = PathBuf::from("happycry/target/debug").join(command_name);
                if local_path.exists() {
                    cmd_path = local_path;
                }
            }

            // External Command
            let mut sys_cmd = SysCommand::new(cmd_path);
            sys_cmd.args(&args[1..]);

            // Handle Redirections
            for redirect in &cmd.redirects {
                let filename = expand_vars(&redirect.target, memory);
                match redirect.rtype {
                    RedirectionType::Output => {
                        let file = File::create(filename).map_err(|e| e.to_string())?;
                        sys_cmd.stdout(Stdio::from(file));
                    }
                    RedirectionType::Append => {
                         let file = File::options().append(true).create(true).open(filename).map_err(|e| e.to_string())?;
                         sys_cmd.stdout(Stdio::from(file));
                    }
                    RedirectionType::Input => {
                         let file = File::open(filename).map_err(|e| e.to_string())?;
                         sys_cmd.stdin(Stdio::from(file));
                    }
                    RedirectionType::HereDoc => {
                         // TODO: HereDoc
                    }
                }
            }

            match sys_cmd.status() {
                Ok(status) => Ok(status.success()),
                Err(_) => {
                    eprintln!("{}: command not found", command_name);
                    Ok(false)
                }
            }
        }
    }
}

fn execute_pipeline(cmds: Vec<Command>, memory: &mut Backpack, _guardian: &Guardian) -> Result<bool, String> {
    // We need to chain commands. 
    // Only SimpleCommands work easily in pipeline for now.
    // If we have List inside Pipeline, logic gets complex (subshells).
    // Assuming flat pipeline of simple commands for MVP.
    
    let mut procs = Vec::new();
    let mut previous_stdout = None;

    for (i, cmd) in cmds.iter().enumerate() {
        if let Command::Simple(simple_cmd) = cmd {
            let args: Vec<String> = simple_cmd.words.iter().map(|w| expand_vars(w, memory)).collect();
             if args.is_empty() { continue; }
             
            let mut sys_cmd = SysCommand::new(&args[0]);
            sys_cmd.args(&args[1..]);
            
            if let Some(stdout) = previous_stdout.take() {
                sys_cmd.stdin(Stdio::from(stdout));
            }
            
            if i < cmds.len() - 1 {
                sys_cmd.stdout(Stdio::piped());
            }

            match sys_cmd.spawn() {
                Ok(child) => {
                    procs.push(child);
                    if i < cmds.len() - 1 {
                        previous_stdout = procs.last_mut().unwrap().stdout.take();
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", args[0], e);
                    return Ok(false);
                }
            }
        } else {
             return Err("Complex commands in pipeline not supported yet".to_string());
        }
    }

    let mut success = true;
    for mut child in procs {
        if let Ok(status) = child.wait() {
            if !status.success() {
                success = false;
            }
        }
    }
    
    Ok(success)
}

fn expand_vars(s: &str, memory: &Backpack) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '$' {
            let mut var_name = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphanumeric() || next_c == '_' {
                    var_name.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }
            if !var_name.is_empty() {
                 if let Ok(val) = std::env::var(&var_name) {
                     result.push_str(&val);
                 } else if let Some(val) = memory.release(&var_name) {
                     result.push_str(&val.to_string());
                 }
            } else {
                result.push('$');
            }
        } else {
            result.push(c);
        }
    }
    if result.is_empty() && !s.is_empty() { s.to_string() } else { result }
}