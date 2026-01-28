use crate::ast::{Statement, Expression, Op, Spanned};
use crate::shadow_synthesizer::ShadowSynthesizer;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;
use serde_json;

#[derive(Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Bool(bool),
    Pointer(usize),
    Library(Rc<libloading::Library>),
    Function {
        name: String,
        params: Vec<(String, String)>,
        body: Vec<Spanned<Statement>>,
    },
    NativeFn(String),
    BoundNativeFn {
        receiver: Box<Value>,
        name: String,
    },
    Object(HashMap<String, Value>),
    Model {
        model_type: String,
        focus: String,
        state: Rc<RefCell<ModelState>>,
    },
    Type {
        name: String,
        definition: TypeDef,
    },
    EnumVariant {
        enum_name: String,
        variant_name: String,
    },
    Null,
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::String(s) => write!(f, "String({:?})", s),
            Value::Bool(b) => write!(f, "Bool({})", b),
            Value::Pointer(p) => write!(f, "Pointer(0x{:x})", p),
            Value::Library(_) => write!(f, "Library(...)"),
            Value::Function { name, .. } => write!(f, "Function({})", name),
            Value::NativeFn(name) => write!(f, "NativeFn({})", name),
            Value::BoundNativeFn { name, .. } => write!(f, "BoundNativeFn({})", name),
            Value::Object(map) => write!(f, "Object({:?})", map),
            Value::Model { model_type, focus, .. } => write!(f, "Model({}, focus: {})", model_type, focus),
            Value::Type { name, .. } => write!(f, "Type({})", name),
            Value::EnumVariant { enum_name, variant_name } => write!(f, "EnumVariant({}::{})", enum_name, variant_name),
            Value::Null => write!(f, "Null"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Pointer(a), Value::Pointer(b)) => a == b,
            (Value::Library(a), Value::Library(b)) => Rc::ptr_eq(a, b),
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDef {
    Struct { fields: Vec<(String, String)> },
    Enum { variants: Vec<String> },
}

#[derive(Debug, Clone, Default)]
pub struct ModelState {
    pub trained_data: Vec<String>,
}

impl PartialEq for ModelState {
    fn eq(&self, other: &Self) -> bool {
        self.trained_data == other.trained_data
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.values.get(name) {
            Some(v.clone())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow().get(name)
        } else {
            None
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(format!("Undefined variable '{}'", name))
        }
    }
}

pub enum ExecResult {
    Ok,
    Return(Value),
    Break,
}

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub backpack: Rc<RefCell<HashMap<String, Value>>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        let backpack = Rc::new(RefCell::new(HashMap::new()));
        
        let mut sys_methods = HashMap::new();
        // ... (existing sys methods)
        
        // Add Backpack methods to sys
        let _bp_capture = backpack.clone();
        sys_methods.insert("hold".to_string(), Value::NativeFn("sys.hold".to_string()));
        sys_methods.insert("release".to_string(), Value::NativeFn("sys.release".to_string()));
        
        sys_methods.insert("log".to_string(), Value::NativeFn("sys.log".to_string()));
        sys_methods.insert("poll".to_string(), Value::NativeFn("sys.poll".to_string()));
        sys_methods.insert("read_file".to_string(), Value::NativeFn("sys.read_file".to_string()));
        sys_methods.insert("write_file".to_string(), Value::NativeFn("sys.write_file".to_string()));
        sys_methods.insert("append_file".to_string(), Value::NativeFn("sys.append_file".to_string()));
        sys_methods.insert("alloc".to_string(), Value::NativeFn("sys.alloc".to_string()));
        sys_methods.insert("free".to_string(), Value::NativeFn("sys.free".to_string()));
        sys_methods.insert("peek".to_string(), Value::NativeFn("sys.peek".to_string()));
        sys_methods.insert("poke".to_string(), Value::NativeFn("sys.poke".to_string()));
        sys_methods.insert("load_lib".to_string(), Value::NativeFn("sys.load_lib".to_string()));
        sys_methods.insert("call_ffi".to_string(), Value::NativeFn("sys.call_ffi".to_string()));
        globals.borrow_mut().define("sys".to_string(), Value::Object(sys_methods));

        let mut joy_methods = HashMap::new();
        joy_methods.insert("say".to_string(), Value::NativeFn("joy.say".to_string()));
        globals.borrow_mut().define("joy".to_string(), Value::Object(joy_methods));

        let mut math_methods = HashMap::new();
        math_methods.insert("sqrt".to_string(), Value::NativeFn("math.sqrt".to_string()));
        math_methods.insert("pow".to_string(), Value::NativeFn("math.pow".to_string()));
        math_methods.insert("sin".to_string(), Value::NativeFn("math.sin".to_string()));
        math_methods.insert("cos".to_string(), Value::NativeFn("math.cos".to_string()));
        math_methods.insert("abs".to_string(), Value::NativeFn("math.abs".to_string()));
        math_methods.insert("min".to_string(), Value::NativeFn("math.min".to_string()));
        math_methods.insert("max".to_string(), Value::NativeFn("math.max".to_string()));
        globals.borrow_mut().define("math".to_string(), Value::Object(math_methods));

        let mut json_methods = HashMap::new();
        json_methods.insert("parse".to_string(), Value::NativeFn("json.parse".to_string()));
        json_methods.insert("stringify".to_string(), Value::NativeFn("json.stringify".to_string()));
        globals.borrow_mut().define("json".to_string(), Value::Object(json_methods));

        let mut mind_methods = HashMap::new();
        mind_methods.insert("think".to_string(), Value::NativeFn("mind.think".to_string()));
        mind_methods.insert("spawn_model".to_string(), Value::NativeFn("mind.spawn_model".to_string()));
        globals.borrow_mut().define("mind".to_string(), Value::Object(mind_methods));

        let mut net_methods = HashMap::new();
        net_methods.insert("fetch".to_string(), Value::NativeFn("net.fetch".to_string()));
        globals.borrow_mut().define("net".to_string(), Value::Object(net_methods));

        let mut void_methods = HashMap::new();
        void_methods.insert("absorb".to_string(), Value::NativeFn("void.absorb".to_string()));
        void_methods.insert("synthesize_lib".to_string(), Value::NativeFn("void.synthesize_lib".to_string()));
        globals.borrow_mut().define("void".to_string(), Value::Object(void_methods));

        let mut time_methods = HashMap::new();
        time_methods.insert("now".to_string(), Value::NativeFn("time.now".to_string()));
        time_methods.insert("sleep".to_string(), Value::NativeFn("time.sleep".to_string()));
        globals.borrow_mut().define("time".to_string(), Value::Object(time_methods));

        let mut os_methods = HashMap::new();
        os_methods.insert("exit".to_string(), Value::NativeFn("os.exit".to_string()));
        os_methods.insert("env".to_string(), Value::NativeFn("os.env".to_string()));
        os_methods.insert("args".to_string(), Value::NativeFn("os.args".to_string()));
        os_methods.insert("name".to_string(), Value::NativeFn("os.name".to_string()));
        globals.borrow_mut().define("os".to_string(), Value::Object(os_methods));

        let mut random_methods = HashMap::new();
        random_methods.insert("int".to_string(), Value::NativeFn("random.int".to_string()));
        globals.borrow_mut().define("random".to_string(), Value::Object(random_methods));

        globals.borrow_mut().define("log".to_string(), Value::NativeFn("sys.log".to_string()));

        Self { globals, backpack }
    }

    pub fn interpret(&mut self, statements: Vec<Spanned<Statement>>) -> Result<(), String> {
        for stmt in &statements {
            match &stmt.node {
                Statement::FunctionDef { name, params, body } => {
                     let func = Value::Function { name: name.clone(), params: params.clone(), body: body.clone() };
                     self.globals.borrow_mut().define(name.clone(), func);
                }
                Statement::StructDef { name, fields } => {
                    let def = Value::Type { 
                        name: name.clone(), 
                        definition: TypeDef::Struct { fields: fields.clone() } 
                    };
                    self.globals.borrow_mut().define(name.clone(), def);
                }
                Statement::EnumDef { name, variants } => {
                    let mut variant_map = HashMap::new();
                    for v in variants {
                        variant_map.insert(v.clone(), Value::EnumVariant { enum_name: name.clone(), variant_name: v.clone() });
                    }
                    self.globals.borrow_mut().define(name.clone(), Value::Object(variant_map));
                }
                _ => {}
            }
        }

        for stmt in &statements {
            if !matches!(stmt.node, Statement::FunctionDef { .. }) {
                self.execute(stmt, self.globals.clone())?;
            }
        }

        let main = self.globals.borrow().get("main");
        if let Some(Value::Function { body, .. }) = main {
             println!("--- Running main ---");
             match self.execute_block(&body, self.globals.clone()) {
                 Ok(_) => Ok(()),
                 Err(e) => Err(e),
             }
        } else {
            Ok(())
        }
    }

    fn execute(&mut self, stmt: &Spanned<Statement>, env: Rc<RefCell<Environment>>) -> Result<ExecResult, String> {
        match &stmt.node {
            Statement::Import { .. } => Ok(ExecResult::Ok),
            Statement::FunctionDef { .. } => Ok(ExecResult::Ok),
            Statement::StructDef { .. } => Ok(ExecResult::Ok),
            Statement::EnumDef { .. } => Ok(ExecResult::Ok),
            Statement::Let { name, value } => {
                let val = self.evaluate(value, env.clone())?;
                env.borrow_mut().define(name.clone(), val);
                Ok(ExecResult::Ok)
            },
            Statement::Set { name, value } => {
                let val = self.evaluate(value, env.clone())?;
                env.borrow_mut().assign(name, val)?;
                Ok(ExecResult::Ok)
            },
            Statement::If { cond, then_block, else_block } => {
                let cond_val = self.evaluate(cond, env.clone())?;
                if self.is_truthy(&cond_val) {
                    self.execute_block(then_block, env)
                } else if let Some(else_stmt) = else_block {
                    self.execute_block(else_stmt, env)
                } else {
                    Ok(ExecResult::Ok)
                }
            },
            Statement::Loop { count: _, body } => {
                 loop {
                     let res = self.execute_block(body, env.clone())?;
                     match res {
                         ExecResult::Break => return Ok(ExecResult::Ok),
                         ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                         ExecResult::Ok => {},
                     }
                 }
            },
            Statement::While { cond, body } => {
                loop {
                    let cond_val = self.evaluate(cond, env.clone())?;
                    if !self.is_truthy(&cond_val) {
                        break;
                    }
                    let res = self.execute_block(body, env.clone())?;
                    match res {
                        ExecResult::Break => break,
                        ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                        ExecResult::Ok => {},
                    }
                }
                Ok(ExecResult::Ok)
            },
            Statement::Break => Ok(ExecResult::Break),
            Statement::Return(expr) => {
                let val = self.evaluate(expr, env)?;
                Ok(ExecResult::Return(val))
            },
            Statement::Expression(expr) => {
                self.evaluate(expr, env)?;
                Ok(ExecResult::Ok)
            },
            Statement::UnsafeBlock(body) => {
                self.execute_block(body, env)
            }
        }
    }

    fn execute_block(&mut self, statements: &[Spanned<Statement>], env: Rc<RefCell<Environment>>) -> Result<ExecResult, String> {
        let block_env = Rc::new(RefCell::new(Environment::with_enclosing(env)));
        for stmt in statements {
            let res = self.execute(stmt, block_env.clone())?;
            if !matches!(res, ExecResult::Ok) {
                return Ok(res);
            }
        }
        Ok(ExecResult::Ok)
    }

    fn evaluate(&mut self, expr: &Spanned<Expression>, env: Rc<RefCell<Environment>>) -> Result<Value, String> {
        match &expr.node {
            Expression::Number(n) => Ok(Value::Number(*n)),
            Expression::StringLit(s) => Ok(Value::String(s.clone())),
            Expression::Bool(b) => Ok(Value::Bool(*b)),
            Expression::Null => Ok(Value::Null),
            Expression::Identifier(name) => {
                env.borrow().get(name).ok_or_else(|| format!("Undefined variable '{}'", name))
            },
            Expression::StructLiteral { name: _, fields } => {
                let mut vals = HashMap::new();
                for (f_name, f_expr) in fields {
                    vals.insert(f_name.clone(), self.evaluate(f_expr, env.clone())?);
                }
                Ok(Value::Object(vals))
            },
            Expression::BinaryOp(left, op, right) => {
                let l = self.evaluate(left, env.clone())?;
                let r = self.evaluate(right, env.clone())?;
                self.apply_op(l, op, r)
            },
            Expression::Call { func, args } => {
                let callee = self.evaluate(func, env.clone())?;
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.evaluate(arg, env.clone())?);
                }
                
                match callee {
                    Value::NativeFn(name) => self.call_native(&name, arg_vals),
                    Value::BoundNativeFn { receiver, name } => self.call_bound_native(*receiver, &name, arg_vals),
                    Value::Function { body, .. } => {
                        let fn_env = Rc::new(RefCell::new(Environment::with_enclosing(self.globals.clone())));
                        match self.execute_block(&body, fn_env)? {
                            ExecResult::Return(v) => Ok(v),
                            _ => Ok(Value::Null),
                        }
                    },
                    _ => Err("Trying to call non-function".to_string()),
                }
            },
            Expression::MemberAccess { object, member } => {
                let obj = self.evaluate(object, env)?;
                if let Value::Object(map) = obj {
                    map.get(member).cloned().ok_or_else(|| format!("Member '{}' not found", member))
                } else if let Value::Model { .. } = obj {
                    match member.as_str() {
                        "train" => Ok(Value::BoundNativeFn { receiver: Box::new(obj.clone()), name: "model.train".to_string() }),
                        "save" => Ok(Value::BoundNativeFn { receiver: Box::new(obj.clone()), name: "model.save".to_string() }),
                        "think" => Ok(Value::BoundNativeFn { receiver: Box::new(obj.clone()), name: "model.think".to_string() }),
                        _ => Err(format!("Model has no member '{}'", member)),
                    }
                } else {
                    Err("Only objects and models have members".to_string())
                }
            },
        }
    }

    fn apply_op(&self, left: Value, op: &Op, right: Value) -> Result<Value, String> {
        match (left, op, right) {
            (Value::Number(l), Op::Plus, Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::Number(l), Op::Minus, Value::Number(r)) => Ok(Value::Number(l - r)),
            (Value::Number(l), Op::Mul, Value::Number(r)) => Ok(Value::Number(l * r)),
            (Value::Number(l), Op::Div, Value::Number(r)) => Ok(Value::Number(l / r)),
            (Value::Number(l), Op::Gt, Value::Number(r)) => Ok(Value::Bool(l > r)),
            (Value::Number(l), Op::Lt, Value::Number(r)) => Ok(Value::Bool(l < r)),
            (Value::Number(l), Op::Gte, Value::Number(r)) => Ok(Value::Bool(l >= r)),
            (Value::Number(l), Op::Lte, Value::Number(r)) => Ok(Value::Bool(l <= r)),
            (Value::Number(l), Op::Eq, Value::Number(r)) => Ok(Value::Bool(l == r)),
            (Value::Number(l), Op::NotEq, Value::Number(r)) => Ok(Value::Bool(l != r)),
            (Value::EnumVariant { enum_name: e1, variant_name: v1 }, Op::Eq, Value::EnumVariant { enum_name: e2, variant_name: v2 }) => {
                Ok(Value::Bool(e1 == e2 && v1 == v2))
            },
            _ => Err("Invalid operation".to_string()),
        }
    }

    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0,
            Value::Null => false,
            _ => true,
        }
    }

    fn json_to_value(&self, j: serde_json::Value) -> Value {
        match j {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Number(i)
                } else if let Some(f) = n.as_f64() {
                    Value::Number(f as i64) // eMo currently only supports i64 numbers? I should check.
                } else {
                    Value::Null
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(a) => {
                let mut map = HashMap::new();
                for (i, v) in a.into_iter().enumerate() {
                    map.insert(i.to_string(), self.json_to_value(v));
                }
                Value::Object(map)
            }
            serde_json::Value::Object(o) => {
                let mut map = HashMap::new();
                for (k, v) in o {
                    map.insert(k, self.json_to_value(v));
                }
                Value::Object(map)
            }
        }
    }

    fn value_to_json(&self, v: Value) -> serde_json::Value {
        match v {
            Value::Null => serde_json::Value::Null,
            Value::Bool(b) => serde_json::Value::Bool(b),
            Value::Number(n) => serde_json::Value::Number(n.into()),
            Value::String(s) => serde_json::Value::String(s),
            Value::Object(map) => {
                let mut obj = serde_json::Map::new();
                for (k, val) in map {
                    obj.insert(k, self.value_to_json(val));
                }
                serde_json::Value::Object(obj)
            }
            _ => serde_json::Value::Null,
        }
    }

    fn call_native(&self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            "mind.spawn_model" => {
                let model_type = if let Some(Value::String(s)) = args.get(0) { s.clone() } else { "Generic".to_string() };
                let focus = if let Some(Value::String(s)) = args.get(1) { s.clone() } else { "Everything".to_string() };
                println!("[MIND] Spawning new model: {} (focus: {})", model_type, focus);
                Ok(Value::Model { 
                    model_type, 
                    focus, 
                    state: Rc::new(RefCell::new(ModelState::default())) 
                })
            },
            "void.absorb" => {
                if let Some(Value::String(url)) = args.get(0) {
                    println!("[VOID] Absorbing knowledge from {}...", url);
                    let absorbed = ShadowSynthesizer::absorb(url);
                    Ok(Value::String(absorbed))
                } else {
                    Err("void.absorb expects a string URL".to_string())
                }
            },
            "void.synthesize_lib" => {
                if let (Some(Value::String(name)), Some(Value::String(source))) = (args.get(0), args.get(1)) {
                    println!("[VOID] Synthesizing native library: {}...", name);
                    let synthesized = ShadowSynthesizer::synthesize(source, name);
                    let file_path = format!("{}.shw", name.replace("::", "_"));
                    fs::write(&file_path, &synthesized).map_err(|e| format!("IO Error: {}", e))?;
                    println!("[VOID] Synthesis complete: {}", file_path);
                    Ok(Value::String(file_path))
                } else {
                    Err("void.synthesize_lib expects name and source strings".to_string())
                }
            },
            "sys.log" | "joy.say" => {
                let output = args.iter().map(|v| {
                    match v {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Pointer(p) => format!("0x{:x}", p),
                        _ => format!("{:?}", v),
                    }
                }).collect::<Vec<_>>().join(" ");
                println!("[STDOUT] {}", output);
                Ok(Value::Null)
            },
            "math.sqrt" => {
                if let Some(Value::Number(n)) = args.get(0) {
                    Ok(Value::Number((*n as f64).sqrt() as i64))
                } else {
                    Err("math.sqrt expects a number".to_string())
                }
            },
            "math.pow" => {
                if let (Some(Value::Number(base)), Some(Value::Number(exp))) = (args.get(0), args.get(1)) {
                    Ok(Value::Number((*base as f64).powf(*exp as f64) as i64))
                } else {
                    Err("math.pow expects two numbers".to_string())
                }
            },
            "math.sin" => {
                if let Some(Value::Number(n)) = args.get(0) {
                    Ok(Value::Number((*n as f64).to_radians().sin() as i64))
                } else {
                    Err("math.sin expects a number".to_string())
                }
            },
            "math.cos" => {
                if let Some(Value::Number(n)) = args.get(0) {
                    Ok(Value::Number((*n as f64).to_radians().cos() as i64))
                } else {
                    Err("math.cos expects a number".to_string())
                }
            },
            "math.abs" => {
                if let Some(Value::Number(n)) = args.get(0) {
                    Ok(Value::Number(n.abs()))
                } else {
                    Err("math.abs expects a number".to_string())
                }
            },
            "math.min" => {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    Ok(Value::Number(*a.min(b)))
                } else {
                    Err("math.min expects two numbers".to_string())
                }
            },
            "math.max" => {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    Ok(Value::Number(*a.max(b)))
                } else {
                    Err("math.max expects two numbers".to_string())
                }
            },
            "json.parse" => {
                if let Some(Value::String(s)) = args.get(0) {
                    let j: serde_json::Value = serde_json::from_str(s).map_err(|e| format!("JSON Error: {}", e))?;
                    Ok(self.json_to_value(j))
                } else {
                    Err("json.parse expects a string".to_string())
                }
            },
            "json.stringify" => {
                if let Some(val) = args.get(0) {
                    let j = self.value_to_json(val.clone());
                    Ok(Value::String(serde_json::to_string(&j).map_err(|e| format!("JSON Error: {}", e))?))
                } else {
                    Err("json.stringify expects one argument".to_string())
                }
            },
            "mind.think" => {
                if let Some(Value::String(prompt)) = args.get(0) {
                    if let Ok(api_key) = std::env::var("GEMINI_API_KEY") {
                        // Real AI Call
                        let client = reqwest::blocking::Client::new();
                        let payload = serde_json::json!({
                            "contents": [{
                                "parts": [{"text": prompt}]
                            }]
                        });
                        
                        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key);
                        
                        match client.post(&url).json(&payload).send() {
                            Ok(res) => {
                                if res.status().is_success() {
                                    if let Ok(json) = res.json::<serde_json::Value>() {
                                        if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                                            Ok(Value::String(text.to_string()))
                                        } else {
                                            Ok(Value::String("AI Response Error: Invalid JSON structure".to_string()))
                                        }
                                    } else {
                                        Ok(Value::String("AI Response Error: Could not parse JSON".to_string()))
                                    }
                                } else {
                                     Ok(Value::String(format!("AI Error: HTTP {}", res.status())))
                                }
                            },
                            Err(e) => Ok(Value::String(format!("AI Connection Error: {}", e)))
                        }
                    } else {
                        // Simulation / Fallback
                        println!("[MIND] No API Key found. Using Neural Simulation.");
                        let response = format!("(Simulated) I have contemplated '{}'. The answer lies within the code.", prompt);
                        Ok(Value::String(response))
                    }
                } else {
                    Err("mind.think expects a string prompt".to_string())
                }
            },
            "net.fetch" => {
                if let Some(Value::String(url)) = args.get(0) {
                    match reqwest::blocking::get(url) {
                        Ok(response) => {
                            let content = response.text().map_err(|e| format!("Net Error: {}", e))?;
                            Ok(Value::String(content))
                        },
                        Err(_) => {
                            println!("[NET] Warning: Could not reach {}. Using cached/placeholder response.", url);
                            Ok(Value::String("<html>eMo Offline Mode</html>".to_string()))
                        }
                    }
                } else {
                    Err("net.fetch expects a string URL".to_string())
                }
            },
            "time.now" => {
                let start = SystemTime::now();
                let since_the_epoch = start.duration_since(UNIX_EPOCH)
                    .map_err(|e| format!("Time Error: {}", e))?;
                Ok(Value::Number(since_the_epoch.as_secs() as i64))
            },
            "time.sleep" => {
                if let Some(Value::Number(ms)) = args.get(0) {
                    thread::sleep(Duration::from_millis(*ms as u64));
                    Ok(Value::Null)
                } else {
                    Err("time.sleep expects a number (milliseconds)".to_string())
                }
            },
            "os.exit" => {
                if let Some(Value::Number(code)) = args.get(0) {
                    std::process::exit(*code as i32);
                } else {
                    std::process::exit(0);
                }
            },
            "os.env" => {
                if let Some(Value::String(name)) = args.get(0) {
                    match std::env::var(name) {
                        Ok(val) => Ok(Value::String(val)),
                        Err(_) => Ok(Value::Null),
                    }
                } else {
                    Err("os.env expects a string argument".to_string())
                }
            },
            "os.args" => {
                let mut map = HashMap::new();
                for (i, arg) in std::env::args().enumerate() {
                    map.insert(i.to_string(), Value::String(arg));
                }
                Ok(Value::Object(map))
            },
            "os.name" => {
                Ok(Value::String(std::env::consts::OS.to_string()))
            },
            "random.int" => {
                let min = if let Some(Value::Number(n)) = args.get(0) { *n } else { 0 };
                let max = if let Some(Value::Number(n)) = args.get(1) { *n } else { 100 };
                unsafe {
                    let r = libc::rand() as i64;
                    let range = max - min + 1;
                    if range <= 0 {
                        Ok(Value::Number(min))
                    } else {
                        Ok(Value::Number(min + (r % range)))
                    }
                }
            },
            "sys.poll" => Ok(Value::Number(0)),
            "sys.hold" => {
                if let (Some(Value::String(name)), Some(val)) = (args.get(0), args.get(1)) {
                    self.backpack.borrow_mut().insert(name.clone(), val.clone());
                    Ok(Value::Null)
                } else {
                    Err("sys.hold expects a name (str) and a value".to_string())
                }
            },
            "sys.release" => {
                if let Some(Value::String(name)) = args.get(0) {
                    Ok(self.backpack.borrow().get(name).cloned().unwrap_or(Value::Null))
                } else {
                    Err("sys.release expects a name (str)".to_string())
                }
            },
            "sys.read_file" => {
                if let Some(Value::String(path)) = args.get(0) {
                    let content = fs::read_to_string(path).map_err(|e| format!("IO Error: {}", e))?;
                    Ok(Value::String(content))
                } else {
                    Err("sys.read_file expects a string argument".to_string())
                }
            },
            "sys.write_file" => {
                if let (Some(Value::String(path)), Some(Value::String(content))) = (args.get(0), args.get(1)) {
                    fs::write(path, content).map_err(|e| format!("IO Error: {}", e))?;
                    Ok(Value::Null)
                } else {
                    Err("sys.write_file expects two string arguments (path, content)".to_string())
                }
            },
            "sys.append_file" => {
                if let (Some(Value::String(path)), Some(Value::String(content))) = (args.get(0), args.get(1)) {
                    let mut file = fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(path)
                        .map_err(|e| format!("IO Error: {}", e))?;
                    file.write_all(content.as_bytes()).map_err(|e| format!("IO Error: {}", e))?;
                    Ok(Value::Null)
                } else {
                    Err("sys.append_file expects two string arguments (path, content)".to_string())
                }
            },
            "sys.alloc" => {
                if let Some(Value::Number(size)) = args.get(0) {
                    unsafe {
                        let ptr = libc::malloc(*size as usize);
                        if ptr.is_null() {
                            Ok(Value::Null)
                        } else {
                            Ok(Value::Pointer(ptr as usize))
                        }
                    }
                } else {
                    Err("sys.alloc expects a number argument (size)".to_string())
                }
            },
            "sys.free" => {
                if let Some(Value::Pointer(ptr)) = args.get(0) {
                    unsafe {
                        libc::free(*ptr as *mut libc::c_void);
                    }
                    Ok(Value::Null)
                } else {
                    Err("sys.free expects a pointer argument".to_string())
                }
            },
            "sys.peek" => {
                if let Some(Value::Pointer(ptr)) = args.get(0) {
                    unsafe {
                        let val = *(*ptr as *const u8);
                        Ok(Value::Number(val as i64))
                    }
                } else {
                    Err("sys.peek expects a pointer argument".to_string())
                }
            },
            "sys.poke" => {
                if let (Some(Value::Pointer(ptr)), Some(Value::Number(val))) = (args.get(0), args.get(1)) {
                    unsafe {
                        *(*ptr as *mut u8) = *val as u8;
                    }
                    Ok(Value::Null)
                } else {
                    Err("sys.poke expects a pointer and a number (value)".to_string())
                }
            },
            "sys.load_lib" => {
                if let Some(Value::String(path)) = args.get(0) {
                    unsafe {
                        let lib = libloading::Library::new(path).map_err(|e| format!("FFI Error: {}", e))?;
                        Ok(Value::Library(Rc::new(lib)))
                    }
                } else {
                    Err("sys.load_lib expects a string argument (path)".to_string())
                }
            },
            "sys.call_ffi" => {
                if let (Some(Value::Library(lib)), Some(Value::String(func_name))) = (args.get(0), args.get(1)) {
                    unsafe {
                        let func: libloading::Symbol<unsafe extern "C" fn(i64, i64, i64, i64) -> i64> = 
                            lib.get(func_name.as_bytes()).map_err(|e| format!("FFI Error: {}", e))?;
                        
                        let mut ffi_args = [0i64; 4];
                        for i in 0..4 {
                            if let Some(val) = args.get(i + 2) {
                                ffi_args[i] = match val {
                                    Value::Number(n) => *n,
                                    Value::Pointer(p) => *p as i64,
                                    Value::String(s) => s.as_ptr() as i64,
                                    _ => 0,
                                };
                            }
                        }
                        
                        let res = func(ffi_args[0], ffi_args[1], ffi_args[2], ffi_args[3]);
                        Ok(Value::Number(res))
                    }
                } else {
                    Err("sys.call_ffi expects a library and a function name".to_string())
                }
            },
            _ => Err(format!("Unknown native function {}", name)),
        }
    }

    fn call_bound_native(&self, receiver: Value, name: &str, args: Vec<Value>) -> Result<Value, String> {
        match (receiver, name) {
            (Value::Model { model_type, state, .. }, "model.train") => {
                if let Some(Value::String(path)) = args.get(0) {
                    println!("[MIND] Training {} on {}...", model_type, path);
                    state.borrow_mut().trained_data.push(path.clone());
                    Ok(Value::Null)
                } else {
                    Err("model.train expects a string path".to_string())
                }
            },
            (Value::Model { model_type, .. }, "model.save") => {
                if let Some(Value::String(path)) = args.get(0) {
                    println!("[MIND] Saving model to {}...", path);
                    Ok(Value::Null)
                } else {
                    Err("model.save expects a string path".to_string())
                }
            },
            (Value::Model { model_type, .. }, "model.think") => {
                if let Some(Value::String(prompt)) = args.get(0) {
                    Ok(Value::String(format!("[{}] After processing your request '{}', I conclude that eMo is the future.", model_type, prompt)))
                } else {
                    Err("model.think expects a string prompt".to_string())
                }
            },
            _ => Err(format!("Unknown bound native function {}", name)),
        }
    }
}