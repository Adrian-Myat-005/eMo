use std::fmt;

// --- Shell AST ---

#[derive(Debug, Clone, PartialEq)]
pub enum RedirectionType {
    Input,  // <
    Output, // >
    Append, // >>
    HereDoc,// <<
}

#[derive(Debug, Clone, PartialEq)]
pub struct Redirect {
    pub rtype: RedirectionType,
    pub target: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleCommand {
    pub words: Vec<String>,
    pub redirects: Vec<Redirect>,
    pub assignments: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Simple(SimpleCommand),
    Pipeline(Vec<Command>),
    List(Box<Command>, ListOp, Box<Command>),
    If {
        cond: Box<Command>,
        then_branch: Box<Command>,
        else_branch: Option<Box<Command>>,
    },
    // We can add While, For, Case, Subshell later
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListOp {
    And, // &&
    Or,  // ||
    Semi, // ;
    Background, // &
}

// --- Legacy Types (Keep for now to avoid immediate compile breaks during migration) ---

#[derive(Debug, Clone, PartialEq)]
pub struct FileObj {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    #[allow(dead_code)]
    Bool(bool),
    File(FileObj),
    List(Vec<Value>),
    Nothing,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::File(file) => {
                let icon = if file.is_dir { "📁" } else { "📄" };
                // Format: 📄 filename.txt (1024b)
                write!(f, "{} {} ({}b)", icon, file.name, file.size)
            }
            Value::List(l) => {
                if l.is_empty() {
                    write!(f, "[Empty List]")
                } else {
                    write!(f, "[")?;
                    for (i, val) in l.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", val)?;
                    }
                    write!(f, "]")
                }
            }
            Value::Nothing => write!(f, "Nothing"),
        }
    }
}

impl Value {
    pub fn to_list(&self) -> Vec<Value> {
        match self {
            Value::List(l) => l.clone(),
            Value::String(s) => s.lines().map(|line| Value::String(line.to_string())).collect(),
            v => vec![v.clone()],
        }
    }
}