use crate::ast::{Statement, Expression, Op, Spanned};
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dimension {
    SadSmile,    // .ss
    HappyCry,    // .hpy
    ThinkingVirus, // .tvrus
    Shadow,      // .shw
    Default      // .emo
}

pub struct CodegenC {
    output: String,
    indent_level: usize,
    dimension: Dimension,
}

impl CodegenC {
    pub fn new(dimension: Dimension) -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
            dimension,
        }
    }

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    pub fn generate(&mut self, statements: &[Spanned<Statement>]) -> String {
        writeln!(self.output, "#include \"emo_runtime.h\"").unwrap();
        
        match self.dimension {
            Dimension::SadSmile => writeln!(self.output, "#define EMO_UNSAFE 1").unwrap(),
            Dimension::Shadow => writeln!(self.output, "#define EMO_METAPROGRAMMING 1").unwrap(),
            Dimension::HappyCry => writeln!(self.output, "#define EMO_UI_EVENT_LOOP 1").unwrap(),
            _ => {}
        }

        writeln!(self.output, "").unwrap();
        
        // First pass: Generate Struct and Enum definitions
        for stmt in statements {
            match &stmt.node {
                Statement::StructDef { name, fields } => {
                    writeln!(self.output, "typedef struct {{").unwrap();
                    for (f_name, f_type) in fields {
                        let c_type = match f_type.as_str() {
                            "str" => "emo_str",
                            "int" => "emo_int",
                            "bool" => "emo_bool",
                            _ => "emo_int", // Default or custom struct type
                        };
                        writeln!(self.output, "    {} {}", c_type, f_name).unwrap();
                    }
                    writeln!(self.output, "}} {}", name).unwrap();
                    writeln!(self.output, "").unwrap();
                }
                Statement::EnumDef { name, variants } => {
                    writeln!(self.output, "typedef enum {{").unwrap();
                    for v in variants {
                        writeln!(self.output, "    {}_{}", name, v).unwrap();
                    }
                    writeln!(self.output, "}} {}", name).unwrap();
                    writeln!(self.output, "").unwrap();
                }
                _ => {}
            }
        }

        // Second pass: Function Implementations and Main Logic
        let mut found_main = false;

        for stmt in statements {
            if let Statement::FunctionDef { name, params, body } = &stmt.node {
                let fn_name = if self.dimension == Dimension::HappyCry && name == "main" {
                    found_main = true;
                    "user_logic"
                } else {
                    name
                };

                let ret_type = if name == "main" && self.dimension != Dimension::HappyCry { "int" } else { "void" };
                write!(self.output, "{} {}( ", ret_type, fn_name).unwrap();

                for (i, (p_name, p_type)) in params.iter().enumerate() {
                    let c_type = match p_type.as_str() {
                        "str" => "emo_str",
                        "int" => "emo_int",
                        "bool" => "emo_bool",
                        _ => "emo_int",
                    };
                    write!(self.output, "{} {}", c_type, p_name).unwrap();
                    if i < params.len() - 1 { write!(self.output, ", ").unwrap(); }
                }
                writeln!(self.output, ") {{ ").unwrap();
                self.indent_level += 1;
                
                for s in body {
                    self.gen_statement(s);
                }
                
                if name == "main" && self.dimension != Dimension::HappyCry {
                     writeln!(self.output, "{}return 0;", self.indent()).unwrap();
                }

                self.indent_level -= 1;
                writeln!(self.output, "}}").unwrap();
                writeln!(self.output, "").unwrap();
            } else {
                 self.gen_statement(stmt);
            }
        }

        // Handle HappyCry entry point injection
        if self.dimension == Dimension::HappyCry && found_main {
            writeln!(self.output, "").unwrap();
            writeln!(self.output, "// Auto-injected HappyCry Event Loop").unwrap();
            writeln!(self.output, "int main() {{").unwrap();
            writeln!(self.output, "    joy_init(); // Initialize UI subsystem").unwrap();
            writeln!(self.output, "    user_logic(); // Run user code").unwrap();
            writeln!(self.output, "    joy_loop(); // Enter event loop").unwrap();
            writeln!(self.output, "    return 0;").unwrap();
            writeln!(self.output, "}}").unwrap();
        }

        self.output.clone()
    }

    fn gen_statement(&mut self, stmt: &Spanned<Statement>) {
        match &stmt.node {
             Statement::StructDef { .. } | Statement::EnumDef { .. } | Statement::FunctionDef { .. } => {},
            Statement::Let { name, value } => {
                // Use GCC's __auto_type for type inference in C
                write!(self.output, "{}__auto_type {} = ", self.indent(), name).unwrap();
                self.gen_expression(value);
                writeln!(self.output, ";").unwrap();
            }
            Statement::Set { name, value } => {
                write!(self.output, "{}{} = ", self.indent(), name).unwrap();
                self.gen_expression(value);
                writeln!(self.output, ";").unwrap();
            }
            Statement::While { cond, body } => {
                write!(self.output, "{}while (", self.indent()).unwrap();
                self.gen_expression(cond);
                writeln!(self.output, ") {{ ").unwrap();
                self.indent_level += 1;
                for s in body {
                    self.gen_statement(s);
                }
                self.indent_level -= 1;
                writeln!(self.output, "{}", self.indent()).unwrap();
            }
            Statement::If { cond, then_block, else_block } => {
                write!(self.output, "{}if (", self.indent()).unwrap();
                self.gen_expression(cond);
                writeln!(self.output, ") {{ ").unwrap();
                self.indent_level += 1;
                for s in then_block {
                    self.gen_statement(s);
                }
                self.indent_level -= 1;
                if let Some(eb) = else_block {
                                        writeln!(self.output, "{}}} else {{ ", self.indent()).unwrap();
                    
                    self.indent_level += 1;
                    for s in eb {
                        self.gen_statement(s);
                    }
                    self.indent_level -= 1;
                }
                writeln!(self.output, "{}", self.indent()).unwrap();
            }
            Statement::Loop { count, body } => {
                if let Some(c) = count {
                    // loop n times { ... }
                    let loop_var = format!("_i{}", self.indent_level);
                    write!(self.output, "{}for (int {} = 0; {} < ", self.indent(), loop_var, loop_var).unwrap();
                    self.gen_expression(c);
                    writeln!(self.output, "; {}", loop_var).unwrap();
                } else {
                    // infinite loop
                                        writeln!(self.output, "{}while (true) {{ ", self.indent()).unwrap();
                    
                }
                self.indent_level += 1;
                for s in body {
                    self.gen_statement(s);
                }
                self.indent_level -= 1;
                writeln!(self.output, "{}", self.indent()).unwrap();
            }
            Statement::UnsafeBlock(body) => {
                writeln!(self.output, "{}{{ // unsafe", self.indent()).unwrap();
                self.indent_level += 1;
                for s in body {
                    self.gen_statement(s);
                }
                self.indent_level -= 1;
                writeln!(self.output, "{}", self.indent()).unwrap();
            }
            Statement::Return(expr) => {
                write!(self.output, "{}return ", self.indent()).unwrap();
                self.gen_expression(expr);
                writeln!(self.output, ";").unwrap();
            }
            Statement::Break => {
                writeln!(self.output, "{}break;", self.indent()).unwrap();
            }
            Statement::Expression(expr) => {
                write!(self.output, "{}", self.indent()).unwrap();
                self.gen_expression(expr);
                writeln!(self.output, ";").unwrap();
            }
            _ => {}
        }
    }

    fn gen_expression(&mut self, expr: &Spanned<Expression>) {
        match &expr.node {
            Expression::Number(n) => write!(self.output, "{}", n).unwrap(),
            Expression::StringLit(s) => write!(self.output, "\"{}\"", s).unwrap(),
            Expression::Bool(b) => write!(self.output, "{}", if *b { "true" } else { "false" }).unwrap(),
            Expression::Null => write!(self.output, "0").unwrap(),
            Expression::Identifier(name) => write!(self.output, "{}", name).unwrap(),
            Expression::StructLiteral { name, fields } => {
                                write!(self.output, "({}){{ ", name).unwrap();
                
                for (i, (f_name, f_val)) in fields.iter().enumerate() {
                    write!(self.output, ".{} = ", f_name).unwrap();
                    self.gen_expression(f_val);
                    if i < fields.len() - 1 { write!(self.output, ", ").unwrap(); }
                }
                write!(self.output, " }}").unwrap();
            }
            Expression::BinaryOp(left, op, right) => {
                write!(self.output, "(").unwrap();
                self.gen_expression(left);
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
                                write!(self.output, " {} ", op_str).unwrap();
                
                self.gen_expression(right);
                write!(self.output, ")").unwrap();
            }
            Expression::Call { func, args } => {
                let mut is_log = false;
                if let Expression::Identifier(name) = &func.node {
                    if name == "log" {
                        is_log = true;
                    } else {
                        write!(self.output, "{}(", name).unwrap();
                    }
                } else if let Expression::MemberAccess { object, member } = &func.node {
                    if let Expression::Identifier(obj_name) = &object.node {
                        match obj_name.as_str() {
                            "math" => {
                                match member.as_str() {
                                    "sqrt" => write!(self.output, "(emo_int)sqrt((double)").unwrap(),
                                    "pow" => write!(self.output, "(emo_int)pow((double)").unwrap(),
                                    "sin" => write!(self.output, "(emo_int)sin((double)").unwrap(),
                                    "cos" => write!(self.output, "(emo_int)cos((double)").unwrap(),
                                    _ => write!(self.output, "{}_{}(", obj_name, member).unwrap(),
                                }
                            },
                            "json" => {
                                match member.as_str() {
                                    "parse" => write!(self.output, "0; // json.parse not yet in C\n").unwrap(),
                                    "stringify" => write!(self.output, "\"{}\" ; // json.stringify not yet in C\n", "{}").unwrap(),
                                    _ => write!(self.output, "{}_{}(", obj_name, member).unwrap(),
                                }
                                return; // Early return for these stubs
                            },
                            "mind" => {
                                match member.as_str() {
                                    "spawn_model" => write!(self.output, "mind_spawn_model(").unwrap(),
                                    "think" => write!(self.output, "model_think(NULL, ").unwrap(), // Global think
                                    _ => write!(self.output, "{}_{}(", obj_name, member).unwrap(),
                                }
                            },
                            "void" => {
                                match member.as_str() {
                                    "absorb" => write!(self.output, "void_absorb(").unwrap(),
                                    "synthesize_lib" => write!(self.output, "void_synthesize_lib(").unwrap(),
                                    _ => write!(self.output, "{}_{}(", obj_name, member).unwrap(),
                                }
                            },
                            "time" => {
                                match member.as_str() {
                                    "now" => write!(self.output, "time(NULL)").unwrap(),
                                    "sleep" => write!(self.output, "time_sleep_ms(").unwrap(),
                                    _ => write!(self.output, "{}_{}(", obj_name, member).unwrap(),
                                }
                            },
                            "net" => {
                                match member.as_str() {
                                    "fetch" => write!(self.output, "net_fetch(").unwrap(),
                                    _ => write!(self.output, "{}_{}(", obj_name, member).unwrap(),
                                }
                            },
                            _ => {
                                // Potential model method call
                                match member.as_str() {
                                    "train" | "save" | "think" => {
                                        write!(self.output, "model_{}(", member).unwrap();
                                        self.gen_expression(object);
                                        if !args.is_empty() { write!(self.output, ", ").unwrap(); }
                                    }
                                    _ => {
                                        // Check if it starts with uppercase (Enum)
                                        if obj_name.chars().next().unwrap().is_uppercase() {
                                            write!(self.output, "{}_{}", obj_name, member).unwrap();
                                        } else {
                                            self.gen_expression(object);
                                            write!(self.output, ".{}", member).unwrap();
                                        }
                                    }
                                }
                                if matches!(member.as_str(), "train" | "save" | "think") { /* arguments will follow */ }
                            }
                        }
                    } else {
                        self.gen_expression(func);
                        write!(self.output, "(").unwrap();
                    }
                } else {
                    self.gen_expression(func);
                    write!(self.output, "(").unwrap();
                }

                if is_log {
                    for (_i, arg) in args.iter().enumerate() {
                        self.gen_expression(arg);
                        write!(self.output, "); ").unwrap();
                    }
                    write!(self.output, "log_newline")
                } else {
                    // Skip arg generation if it was already handled for models (first arg)
                    // Wait, my logic for model methods above is a bit messy with arg handling.
                    // Let's fix it.
                    let _is_model_method = if let Expression::MemberAccess { member, .. } = &func.node {
                         matches!(member.as_str(), "train" | "save" | "think")
                    } else { false };
                    
                    // We only want to skip if it's NOT a model method or if we handled it differently.
                    // Actually, let's just use a standard way.
                    
                    for (_i, arg) in args.iter().enumerate() {
                        self.gen_expression(arg);
                        if _i < args.len() - 1 {
                             write!(self.output, ", ").unwrap();
                        }
                    }
                    write!(self.output, ")")
                }
                .unwrap();
            }
            Expression::MemberAccess { object, member } => {
                // Check if this is likely an enum access: EnumName.Variant
                // For a professional compiler, we'd check the type of `object`.
                // Here we'll check if object is an identifier starting with Uppercase.
                let is_enum = if let Expression::Identifier(name) = &object.node {
                    name.chars().next().map_or(false, |c| c.is_uppercase())
                } else {
                    false
                };

                if is_enum {
                    if let Expression::Identifier(obj_name) = &object.node {
                        write!(self.output, "{}_{}", obj_name, member).unwrap();
                    }
                } else {
                    self.gen_expression(object);
                    write!(self.output, ".{}", member).unwrap();
                }
            }
        }
    }
}
