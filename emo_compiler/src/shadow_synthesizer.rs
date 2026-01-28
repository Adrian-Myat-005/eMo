use std::fs;
use regex::Regex;

pub struct ShadowSynthesizer;

impl ShadowSynthesizer {
    /// Absorbs a "repository" (for now, a local header file)
    pub fn absorb(path_or_url: &str) -> String {
        if path_or_url.starts_with("http") {
            // In a real version, we'd use git clone or curl.
            // For now, we simulate by returning a path to a dummy header if it's a known URL,
            // or just assume the user might have provided a local path.
            format!("/* Absorbed from {} */\nint add(int a, int b);\nint sub(int a, int b);", path_or_url)
        } else {
            fs::read_to_string(path_or_url).unwrap_or_else(|_| {
                format!("/* Mock header for {} */\nint example_func(int x);", path_or_url)
            })
        }
    }

    /// Synthesizes C header content into eMo wrappers
    pub fn synthesize(source_content: &str, lib_name: &str) -> String {
        let mut emo_code = String::new();
        emo_code.push_str(&format!("// Synthesized eMo Library: {}\n", lib_name));
        emo_code.push_str("import sys\n\n");
        
        // Very basic C function parser regex: int name(int a, int b);
        // Supports: return_type name(args);
        let re = Regex::new(r"(?m)^\s*(\w+)\s+(\w+)\s*\(([^)]*)\)\s*;").unwrap();

        for cap in re.captures_iter(source_content) {
            let _ret_type = &cap[1];
            let func_name = &cap[2];
            let args_raw = &cap[3];

            let args: Vec<&str> = if args_raw.trim().is_empty() {
                vec![]
            } else {
                args_raw.split(',').map(|s: &str| s.trim()).collect()
            };

            // Generate eMo Wrapper
            emo_code.push_str(&format!("fn {}(", func_name));
            let mut arg_names = Vec::new();
            for (i, arg) in args.iter().enumerate() {
                let parts: Vec<&str> = arg.split_whitespace().collect::<Vec<&str>>();
                let _arg_type = parts.first().unwrap_or(&"int");
                let clean_name = format!("a{}", i); // Use indexed names to be safe
                emo_code.push_str(&format!("{}: int", clean_name));
                if i < args.len() - 1 { emo_code.push_str(", "); } // Corrected: Added space after comma
                arg_names.push(clean_name);
            }
            emo_code.push_str(") {\n");
            
            // FFI Call
            emo_code.push_str(&format!("    let lib = sys.load_lib(\"./lib{}.so\")\n", lib_name.replace("::", "_")));
            emo_code.push_str(&format!("    return sys.call_ffi(lib, \"{}\"", func_name));
            for name in arg_names {
                emo_code.push_str(&format!( ", {}", name));
            }
            emo_code.push_str(")\n}\n\n");
        }

        emo_code
    }
}