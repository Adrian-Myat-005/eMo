use crate::ast::{Statement, Expression, Op};

pub fn generate(statements: Vec<Statement>) -> String {
    let mut output = String::new();
    let mut has_server = false;
    let mut server_port = "8080".to_string();
    let mut routes = Vec::new();

    output.push_str("use std::process::Command;\n");
    output.push_str("use std::sync::Mutex;\n\n");

    for stmt in &statements {
        if let Statement::ServerNew(port) = stmt {
            has_server = true;
            server_port = port.clone();
        }
        if let Statement::Route { path } = stmt {
            routes.push(path.clone());
        }
    }

    if has_server {
        output.push_str("use actix_web::{web, App, HttpServer, HttpResponse, Responder};\n");
    }

    if has_server {
        for (i, _) in routes.iter().enumerate() {
            output.push_str(&format!("async fn handler_{}() -> impl Responder {{ \n", i));
            output.push_str("    HttpResponse::Ok().body(\"HappyCry v1.0 Online\")\n");
            output.push_str("}\n\n");
        }
    }

    if has_server {
        output.push_str("#[actix_web::main]\nasync fn main() -> std::io::Result<()> {\n");
    } else {
        output.push_str("fn main() {\n");
    }

    for stmt in statements {
        output.push_str(&generate_stmt(stmt, 1));
    }

    if has_server {
        output.push_str(&format!("    println!(\"🚀 HappyCry Server starting on port {}\" );\n", server_port));
        output.push_str("    HttpServer::new(|| {\n");
        output.push_str("        App::new()\n");
        for (i, path) in routes.iter().enumerate() {
            output.push_str(&format!("            .route(\"{}\", web::get().to(handler_{}))\n", path, i));
        }
        output.push_str("    })\n");
        output.push_str(&format!("    .bind((\"127.0.0.1\", {}.to_string().parse::<u16>().unwrap_or(8080)))?\n", server_port));
        output.push_str("    .run()\n");
        output.push_str("    .await\n");
        output.push_str("}\n");
    } else {
        output.push_str("}\n");
    }

    output
}

fn generate_stmt(stmt: Statement, indent: usize) -> String {
    let pad = "    ".repeat(indent);
    let mut out = String::new();

    match stmt {
        Statement::Set { key, value } => {
            let expr_rust = expr_to_rust(value);
            out.push_str(&pad);
            out.push_str(&format!("let mut {} = {};\n", key, expr_rust));
        }
        Statement::Say(expr) => {
            let expr_rust = expr_to_rust(expr);
            out.push_str(&pad);
            out.push_str("println!(\"{}\", "); // literal "{}" for generated code
            out.push_str(&expr_rust);
            out.push_str(");\n");
        }
        Statement::Gate { lang, code } => {
            let code_str = expr_to_rust(code); 
            if lang == "python" {
                 out.push_str(&pad);
                 out.push_str(&format!("let _ = Command::new(\"python3\").arg(\"-c\").arg({}).status();\n", code_str));
            } else if lang == "rust" {
                 out.push_str(&pad);
                 out.push_str("println!(\"Gate to Rust not fully supported at runtime, use #happy-raw\");\n");
            } else {
                 out.push_str(&pad);
                 out.push_str(&format!("println!(\"Unknown gate: {}\");\n", lang));
            }
        }
        Statement::If { condition, then_branch, else_branch } => {
            let cond_rust = expr_to_rust(condition);
            out.push_str(&format!("{}if {} {{ \n", pad, cond_rust));
            for s in then_branch {
                out.push_str(&generate_stmt(s, indent + 1));
            }
            if let Some(eb) = else_branch {
                out.push_str(&format!("{}}} else {{ \n", pad));
                for s in eb {
                    out.push_str(&generate_stmt(s, indent + 1));
                }
            }
            out.push_str(&format!("{}}}
", pad));
        }
        Statement::While { condition, body } => {
            let cond_rust = expr_to_rust(condition);
            out.push_str(&format!("{}while {} {{ \n", pad, cond_rust));
            for s in body {
                out.push_str(&generate_stmt(s, indent + 1));
            }
            out.push_str(&format!("{}}}
", pad));
        }
        Statement::FuncDef { name, params: _, body } => {
            out.push_str(&format!("{}let mut {} = || {{ \n", pad, name));
            for s in body {
                out.push_str(&generate_stmt(s, indent + 1));
            }
            out.push_str(&format!("{}}};
", pad));
        }
        Statement::Call { name, args: _ } => {
            out.push_str(&format!("{}{}();\n", pad, name)); 
        }
        Statement::Return(expr) => {
            let expr_rust = expr_to_rust(expr);
            out.push_str(&format!("{}return {};\n", pad, expr_rust));
        }
        Statement::MathAdd { key, value } => {
            let expr_rust = expr_to_rust(value);
            out.push_str(&pad);
            out.push_str(&format!("{} += {};\n", key, expr_rust));
        }
        Statement::Loop { count, body } => {
            let count_rust = expr_to_rust(count);
            out.push_str(&pad);
            out.push_str(&format!("for _ in 0..{} {{ \n", count_rust));
            for s in body {
                out.push_str(&generate_stmt(s, indent + 1));
            }
            out.push_str(&format!("{}}} \n", pad));
        }
        Statement::AiThink { input, prompt } => {
            let p_rust = expr_to_rust(prompt);
            let i_rust = expr_to_rust(input);
            out.push_str(&pad);
            out.push_str(&format!("println!(\"🤖 [AI Thinking] Prompt: {{}} | Input: {{}}\", {}, {});\n", p_rust, i_rust));
            out.push_str(&pad);
            out.push_str("println!(\"   (In a real deployment, this would call the Tvrus API gateway)\");\n");
        }
        Statement::Page { path, body } => {
            out.push_str(&pad);
            out.push_str(&format!("println!(\"Rendering page: {}\");\n", path));
            for s in body {
                out.push_str(&generate_stmt(s, indent + 1));
            }
        }
        Statement::AsyncTask { name, body } => {
            out.push_str(&pad);
            // Generating a tokio::spawn block assigned to a handle? 
            // For simplicity in this prototype, we'll define an async function and spawn it.
            // But we are inside main. Let's define it as a closure or async block.
            out.push_str(&format!("let {} = actix_web::rt::spawn(async move {{ \n", name));
            for s in body {
                out.push_str(&generate_stmt(s, indent + 1));
            }
            out.push_str(&format!("{}}});\n", pad));
        }
        Statement::Await(name) => {
            out.push_str(&pad);
            out.push_str(&format!("let _ = {}.await;\n", name));
        }
        Statement::DbOpen(path) => {
            out.push_str(&pad);
            out.push_str(&format!("println!(\"💾 [DB] Opening database: {}\");\n", path));
        }
        Statement::DbRun(query) => {
            out.push_str(&pad);
            out.push_str(&format!("println!(\"💾 [DB] Running SQL: {}\");\n", query));
        }
        Statement::DbInsert { table, values } => {
            out.push_str(&pad);
            out.push_str(&format!("println!(\"💾 [DB] Inserting into '{}': {}\");\n", table, values));
        }
        Statement::SwarmJoin(topic) => {
            out.push_str(&pad);
            out.push_str(&format!("println!(\"🐝 [Swarm] Joining topic: {}\");\n", topic));
        }
        Statement::SwarmBroadcast(expr) => {
            let msg = expr_to_rust(expr);
            out.push_str(&pad);
            out.push_str(&format!("println!(\"🐝 [Swarm] Broadcasting: {{}}\", {});\n", msg));
        }
        Statement::Title(expr) => {
            let val = expr_to_rust(expr);
            out.push_str(&pad);
            out.push_str(&format!("println!(\"   [HTML Title]: {{}}\", {});\n", val));
        }
        Statement::Header(expr) => {
            let val = expr_to_rust(expr);
            out.push_str(&pad);
            out.push_str(&format!("println!(\"   [HTML Header]: {{}}\", {});\n", val));
        }
        _ => {{}}
    }
    out
}

fn expr_to_rust(expr: Expression) -> String {
    match expr {
        Expression::StringLit(s) => {
            let mut res = String::from("\"");
            res.push_str(&s);
            res.push_str("\".to_string()");
            res
        }
        Expression::Number(n) => n.to_string(),
        Expression::Variable(s) => s,
        Expression::BinaryOp(left, op, right) => {
            let l = expr_to_rust(*left);
            let r = expr_to_rust(*right);
            let op_str = match op {
                Op::Plus => "+",
                Op::Is => "==", 
                Op::Not => "!=",
                Op::Gt => ">",
                Op::Lt => "<",
            };
            format!("({} {} {})", l, op_str, r)
        }
        Expression::Array(elements) => {
            let items: Vec<String> = elements.into_iter().map(expr_to_rust).collect();
            format!("vec![{}]", items.join(", "))
        }
        Expression::Index(arr, idx) => {
            let a = expr_to_rust(*arr);
            let i = expr_to_rust(*idx);
            format!("{}[{} as usize].clone()", a, i)
        }
        Expression::Command(cmd) => {
            let c = expr_to_rust(*cmd);
            format!("String::from_utf8_lossy(&Command::new(\"sh\").arg(\"-\").arg({}).output().unwrap().stdout).to_string()", c)
        }
        _ => "()".to_string(), 
    }
}
