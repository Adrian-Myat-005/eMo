use crate::ast::{Statement, Expression, Op, Spanned};

pub struct Formatter {
    indent_level: usize,
}

impl Formatter {
    pub fn new() -> Self {
        Self { indent_level: 0 }
    }

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    pub fn format(&mut self, statements: &[Spanned<Statement>]) -> String {
        let mut output = String::new();
        for stmt in statements {
            output.push_str(&self.format_statement(stmt));
            output.push('\n');
        }
        output
    }

    fn format_statement(&mut self, stmt: &Spanned<Statement>) -> String {
        let mut out = String::new();
        match &stmt.node {
            Statement::Import { module, source } => {
                out.push_str(&format!("import {}", module));
                if let Some(src) = source {
                    out.push_str(&format!(" from \"{}\"", src));
                }
            }
            Statement::FunctionDef { name, params, body } => {
                out.push_str(&format!("fn {}(", name));
                for (i, (p_name, p_type)) in params.iter().enumerate() {
                    out.push_str(&format!("{}: {}", p_name, p_type));
                    if i < params.len() - 1 { out.push_str(", "); }
                }
                out.push_str(") {\n");
                self.indent_level += 1;
                for s in body {
                    out.push_str(&self.indent());
                    out.push_str(&self.format_statement(s));
                    out.push('\n');
                }
                self.indent_level -= 1;
                out.push_str(&self.indent());
                out.push('}');
            }
            Statement::Let { name, value } => {
                out.push_str(&format!("let {} = ", name));
                out.push_str(&self.format_expression(value));
            }
            Statement::Set { name, value } => {
                out.push_str(&format!("set {} to ", name));
                out.push_str(&self.format_expression(value));
            }
            Statement::Return(expr) => {
                out.push_str("return ");
                out.push_str(&self.format_expression(expr));
            }
            Statement::Break => {
                out.push_str("break");
            }
            Statement::Expression(expr) => {
                out.push_str(&self.format_expression(expr));
            }
            Statement::If { cond, then_block, else_block } => {
                out.push_str("if ");
                out.push_str(&self.format_expression(cond));
                out.push_str(" {\n");
                self.indent_level += 1;
                for s in then_block {
                    out.push_str(&self.indent());
                    out.push_str(&self.format_statement(s));
                    out.push('\n');
                }
                self.indent_level -= 1;
                out.push_str(&self.indent());
                out.push('}');
                if let Some(eb) = else_block {
                    out.push_str(" else {\n");
                    self.indent_level += 1;
                    for s in eb {
                        out.push_str(&self.indent());
                        out.push_str(&self.format_statement(s));
                        out.push('\n');
                    }
                    self.indent_level -= 1;
                    out.push_str(&self.indent());
                    out.push('}');
                }
            }
            Statement::Loop { count, body } => {
                out.push_str("loop");
                if let Some(c) = count {
                    out.push_str(" ");
                    out.push_str(&self.format_expression(c));
                    out.push_str(" times");
                }
                out.push_str(" {\n");
                self.indent_level += 1;
                for s in body {
                    out.push_str(&self.indent());
                    out.push_str(&self.format_statement(s));
                    out.push('\n');
                }
                self.indent_level -= 1;
                out.push_str(&self.indent());
                out.push('}');
            }
            Statement::While { cond, body } => {
                out.push_str("while ");
                out.push_str(&self.format_expression(cond));
                out.push_str(" {\n");
                self.indent_level += 1;
                for s in body {
                    out.push_str(&self.indent());
                    out.push_str(&self.format_statement(s));
                    out.push('\n');
                }
                self.indent_level -= 1;
                out.push_str(&self.indent());
                out.push('}');
            }
            Statement::StructDef { name, fields } => {
                out.push_str(&format!("struct {} {{ \n", name));
                self.indent_level += 1;
                for (f_name, f_type) in fields {
                    out.push_str(&self.indent());
                    out.push_str(&format!("{}: {},\n", f_name, f_type));
                }
                self.indent_level -= 1;
                out.push_str(&self.indent());
                out.push('}');
            }
            Statement::EnumDef { name, variants } => {
                out.push_str(&format!("enum {} {{ \n", name));
                self.indent_level += 1;
                for v in variants {
                    out.push_str(&self.indent());
                    out.push_str(&format!("{},\n", v));
                }
                self.indent_level -= 1;
                out.push_str(&self.indent());
                out.push('}');
            }
            Statement::UnsafeBlock(body) => {
                out.push_str("unsafe {\n");
                self.indent_level += 1;
                for s in body {
                    out.push_str(&self.indent());
                    out.push_str(&self.format_statement(s));
                    out.push('\n');
                }
                self.indent_level -= 1;
                out.push_str(&self.indent());
                out.push('}');
            }
        }
        out
    }

    fn format_expression(&mut self, expr: &Spanned<Expression>) -> String {
        match &expr.node {
            Expression::Number(n) => n.to_string(),
            Expression::StringLit(s) => format!("\"{}\"", s),
            Expression::Bool(b) => b.to_string(),
            Expression::Null => "null".to_string(),
            Expression::Identifier(name) => name.clone(),
            Expression::StructLiteral { name, fields } => {
                let mut out = format!("{} {{ ", name);
                for (i, (f_name, f_val)) in fields.iter().enumerate() {
                    out.push_str(&format!("{}: ", f_name));
                    out.push_str(&self.format_expression(f_val));
                    if i < fields.len() - 1 { out.push_str(", "); }
                }
                out.push_str(" }");
                out
            }
            Expression::BinaryOp(left, op, right) => {
                let op_str = match op {
                    Op::Plus => "+",
                    Op::Minus => "-",
                    Op::Mul => "*",
                    Op::Div => "/",
                    Op::Eq => "==",
                    Op::NotEq => "!=",
                    Op::Gt => ">",
                    Op::Lt => "<",
                    Op::Gte => ">=",
                    Op::Lte => "<=",
                };
                format!("({} {} {})", self.format_expression(left), op_str, self.format_expression(right))
            }
            Expression::Call { func, args } => {
                let mut out = self.format_expression(func);
                out.push('(');
                for (i, arg) in args.iter().enumerate() {
                    out.push_str(&self.format_expression(arg));
                    if i < args.len() - 1 { out.push_str(", "); }
                }
                out.push(')');
                out
            }
            Expression::MemberAccess { object, member } => {
                format!("{}.{}", self.format_expression(object), member)
            }
        }
    }
}
