// src/tvrus.rs
use crate::types::Value;
use colored::*;

// The Brain Function
pub fn consult_brain(input: Value, key_code: &str, prompt: &str) -> Result<Value, String> {
    println!("{}", format!("🤖 [ThinkingVirus] Analyzing intent: '{}' (Key: {})", prompt, key_code).purple().italic());

    // 1. SECURITY CHECK (The "Happy" Safety)
    // The virus refuses to do dangerous things if the "vibe" is bad.
    if prompt.contains("delete") || prompt.contains("kill") {
        return Err("⚠️  Security Alert: The ThinkingVirus refuses to destroy data without explicit authorization.".to_string());
    }

    // 2. CONTEXT AWARENESS
    // The AI looks at what data it received.
    match input {
        Value::List(mut items) => {
            // 3. INTENT MATCHING (Simulating the AI)
            
            // Scenario A: "Sort by size"
            if prompt.contains("sort") && prompt.contains("size") {
                // Sort the vector in place
                items.sort_by(|a, b| {
                    match (a, b) {
                        (Value::File(f1), Value::File(f2)) => f1.size.cmp(&f2.size),
                        _ => std::cmp::Ordering::Equal,
                    }
                });
                return Ok(Value::List(items));
            }

            // Scenario B: "Get the biggest one"
            if prompt.contains("biggest") || prompt.contains("largest") {
                // Sort and take the last one
                items.sort_by(|a, b| {
                    match (a, b) {
                        (Value::File(f1), Value::File(f2)) => f1.size.cmp(&f2.size),
                        _ => std::cmp::Ordering::Equal,
                    }
                });
                if let Some(biggest) = items.pop() {
                    return Ok(Value::List(vec![biggest]));
                }
            }

            // Scenario C: "Only show text files" (Fuzzy filtering)
            if prompt.contains("text") || prompt.contains("code") {
                let filtered: Vec<Value> = items.into_iter().filter(|item| {
                    match item {
                        Value::File(f) => f.name.ends_with(".rs") || f.name.ends_with(".txt") || f.name.ends_with(".md"),
                        _ => false,
                    }
                }).collect();
                return Ok(Value::List(filtered));
            }

            Err("🤖 I don't understand that 'vibe' yet. Try 'sort by size' or 'biggest'.".to_string())
        }
        _ => Err("🤖 I can only think about Lists right now.".to_string()),
    }
}
