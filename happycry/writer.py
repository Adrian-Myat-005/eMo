use crate::parser::{Stmt, Expr, Op};

pub fn generate(statements: Vec<Stmt>) -> String {
    let mut output = String::new();
    let mut has_server = false;
    let mut server_port = "8080".to_string();
    let mut routes = Vec::new();

    output.push_str("use actix_web::{web, App, HttpServer, HttpResponse, Responder};\n");
    output.push_str("use std::process::Command;\n");
    output.push_str("use std::sync::Mutex;\n\n");

    for stmt in &statements {
        if let Stmt::ServerNew(port) = stmt {
            has_server = true;
            server_port = port.clone();
        }
        if let Stmt::Route(path) = stmt {
            routes.push(path.clone());
        }
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
        output.push_str(&format!("    println!(\"🚀 HappyCry Server starting on port {}{}\");\n", server_port));
        output.push_str("    HttpServer::new(|| {\n");
        output.push_str("        App::new()\n");
        for (i, path) in routes.iter().enumerate() {
            output.push_str(&format!("            .route(\"{}\", web::get().to(handler_{}))\n", path, i));
        }
        output.push_str("    })\n");
        output.push_str(&format!("    .bind((\"127.0.0.1\", {}.parse::<u16>().unwrap_or(8080)))?\n", server_port));
        output.push_str("    .run()\n");
        output.push_str("    .await\n");
        output.push_str("}\n");
    } else {
        output.push_str("}\n");
    }

    output
}

fn generate_stmt(stmt: Stmt, indent: usize) -> String {
    let mut out = String::new();
    let pad = "    ".repeat(indent);

    match stmt {
        Stmt::Set(name, expr) => {
            out.push_str(&format!("{0}let mut {1} = {2};\n", pad, name, expr_to_rust(expr)));
        }
        Stmt::Say(expr) => {
            out.push_str(&format!("{0}println!(\"{{:?}}\", {1});\n", pad, expr_to_rust(expr)));
        }
        Stmt::If(cond, then_block, else_block) => {
            out.push_str(&format!("{0}if {1} {{ {2}", pad, expr_to_rust(cond)));
            for s in then_block {
                out.push_str(&generate_stmt(s, indent + 1));
            }
            if let Some(eb) = else_block {
                out.push_str(&format!("{0}}} else {{ {1}", pad));
                for s in eb {
                    out.push_str(&generate_stmt(s, indent + 1));
                }
            }
            out.push_str(&format!("{0}}}
", pad));
        }
        Stmt::While(cond, body) => {
            out.push_str(&format!("{0}while {1} {{ {2}", pad, expr_to_rust(cond)));
            for s in body {
                out.push_str(&generate_stmt(s, indent + 1));
            }
            out.push_str(&format!("{0}}}
", pad));
        }
        Stmt::FuncDef(name, args, body) => {
            let args_str = args.join(", ");
            out.push_str(&format!("{0}let mut {1} = |{2}| {{ {3}", pad, name, args_str));
            for s in body {
                out.push_str(&generate_stmt(s, indent + 1));
            }
            out.push_str(&format!("{0}}};
", pad));
        }
        Stmt::Call(name, args) => {
            let args_str: Vec<String> = args.into_iter().map(expr_to_rust).collect();
            out.push_str(&format!("{0}({1});\n", pad, name, args_str.join(", ")));
        }
        Stmt::Return(expr) => {
            out.push_str(&format!("{0}return {1};\n", pad, expr_to_rust(expr)));
        }
        _ => {{}}
    }}
    out
}

fn expr_to_rust(expr: Expr) -> String {
    match expr {
        Expr::StringLit(s) => format!("\"{0}\".to_string()", s),
        Expr::Number(n) => n.to_string(),
        Expr::Identifier(s) => s,
        Expr::BinaryOp(left, op, right) => {{
            let l = expr_to_rust(*left);
            let r = expr_to_rust(*right);
            let op_str = match op {
                Op::Plus => "+",
                Op::Is => "==",
                Op::Not => "!=",
                Op::Gt => ">",
                Op::Lt => "<",
            };
            format!("({0} {1} {2})", l, op_str, r)
        }}
        Expr::Array(elements) => {{
            let items: Vec<String> = elements.into_iter().map(expr_to_rust).collect();
            format!("vec![{{0}}]", items.join(", "))
        }}
        Expr::Index(arr, idx) => {{
            let a = expr_to_rust(*arr);
            let i = expr_to_rust(*idx);
            format!("{0}[{1} as usize].clone()", a, i)
        }}
        Expr::Command(cmd) => {{
            let c = expr_to_rust(*cmd);
            format!("String::from_utf8_lossy(&Command::new(\"sh\").arg(\" -c\").arg({}).output().unwrap().stdout).to_string()", c)
        }}
    }}
}