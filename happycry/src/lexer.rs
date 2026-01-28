#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Old Keywords (keeping some for compatibility if needed, or deprecating)
    Set,
    To,
    Say,
    Server,
    New,
    Port,
    Route,
    Serve,
    If,
    Else,
    End,
    While,
    Func,
    Call,
    Exec,
    Return,

    // Automation & System
    Math,
    Add,
    Async,
    Task,
    Await,
    Loop,
    Times,
    Try,
    Connect,
    On,
    Raw,

    // HappyWeb
    Page,
    Title,
    Header,
    Button,
    Triggers,
    Style,
    Theme,
    Dark,

    // HappyBase & Swarm
    Db,
    Open,
    Run,
    Insert,
    Values,
    Swarm,
    Join,
    Broadcast,
    Message,

    // Bridge
    Link,
    Library,
    Foreign,

    // AI / Brain
    Ai,
    Brain,
    Think,

    // New English Keywords
    Create,
    Variable,
    With,
    Value,
    Gate,
    Execute,
    Python,
    Rust,

    // Logic
    Is,
    Not,
    Gt,
    Lt,
    Then, // "if x is 5 then"
    Do,   // "if x is 5 do"

    // Structure
    LBracket,
    RBracket,
    Comma,

    // Literals
    StringLit(String),
    Number(i32),

    // Identifiers
    Identifier(String),

    // Symbols
    Plus,
    Equals,
    Slash,

    // End of File
    EOF,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
            }
            '#' => {
                while let Some(&ch) = chars.peek() {
                    if ch == '\n' { break; }
                    chars.next();
                }
            }
            '[' => {
                tokens.push(Token::LBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::RBracket);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '=' => {
                tokens.push(Token::Equals);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '>' => {
                tokens.push(Token::Gt);
                chars.next();
            }
            '<' => {
                tokens.push(Token::Lt);
                chars.next();
            }
            '!' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    tokens.push(Token::Not);
                    chars.next();
                } else {
                    panic!("Lexer Error: Expected '=' after '!'");
                }
            }
            '"' => {
                chars.next();
                let mut s = String::new();
                loop {
                    match chars.peek() {
                        Some('"') => {
                            chars.next();
                            break;
                        }
                        Some(&ch) => {
                            s.push(ch);
                            chars.next();
                        }
                        None => panic!("Lexer Error: Unterminated string literal"),
                    }
                }
                tokens.push(Token::StringLit(s));
            }
            c if c.is_ascii_digit() => {
                let mut num_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() {
                        num_str.push(ch);
                        chars.next();
                    } else { break; }
                }
                let num = num_str.parse::<i32>().unwrap();
                tokens.push(Token::Number(num));
            }
            c if c.is_ascii_alphabetic() => {
                let mut ident_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        ident_str.push(ch);
                        chars.next();
                    } else { break; }
                }
                let token = match ident_str.to_lowercase().as_str() {
                    "set" => Token::Set,
                    "to" => Token::To,
                    "say" => Token::Say,
                    "server" => Token::Server,
                    "new" => Token::New,
                    "port" => Token::Port,
                    "route" => Token::Route,
                    "serve" => Token::Serve,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "end" => Token::End,
                    "while" => Token::While,
                    "func" => Token::Func,
                    "call" => Token::Call,
                    "exec" => Token::Exec,
                    "return" => Token::Return,
                    "is" => Token::Is,
                    "not" => Token::Not,

                    // Automation & System
                    "math" => Token::Math,
                    "add" => Token::Add,
                    "async" => Token::Async,
                    "task" => Token::Task,
                    "await" => Token::Await,
                    "loop" => Token::Loop,
                    "times" => Token::Times,
                    "try" => Token::Try,
                    "connect" => Token::Connect,
                    "on" => Token::On,
                    "raw" => Token::Raw,

                    // Web
                    "page" => Token::Page,
                    "title" => Token::Title,
                    "header" => Token::Header,
                    "button" => Token::Button,
                    "triggers" => Token::Triggers,
                    "style" => Token::Style,
                    "theme" => Token::Theme,
                    "dark" => Token::Dark,

                    // Db & Swarm
                    "db" => Token::Db,
                    "open" => Token::Open,
                    "run" => Token::Run,
                    "insert" => Token::Insert,
                    "values" => Token::Values,
                    "swarm" => Token::Swarm,
                    "join" => Token::Join,
                    "broadcast" => Token::Broadcast,
                    "message" => Token::Message,

                    // Bridge
                    "link" => Token::Link,
                    "library" => Token::Library,
                    "foreign" => Token::Foreign,

                    // AI
                    "ai" => Token::Ai,
                    "brain" => Token::Brain,
                    "think" => Token::Think,
                    
                    // New Keywords
                    "create" => Token::Create,
                    "variable" => Token::Variable,
                    "with" => Token::With,
                    "value" => Token::Value,
                    "gate" => Token::Gate,
                    "execute" => Token::Execute,
                    "python" => Token::Python,
                    "rust" => Token::Rust,
                    "then" => Token::Then,
                    "do" => Token::Do,

                    _ => Token::Identifier(ident_str),
                };
                tokens.push(token);
            }
            _ => panic!("Lexer Error: Unknown character '{}'", c),
        }
    }
    tokens.push(Token::EOF);
    tokens
}
