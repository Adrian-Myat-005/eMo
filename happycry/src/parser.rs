use crate::lexer::Token;
use crate::ast::{Statement, Expression, Op};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn consume(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        token
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.peek() != &Token::EOF {
            statements.push(self.parse_stmt());
        }
        statements
    }

    fn parse_stmt(&mut self) -> Statement {
        match self.peek() {
            // Old Syntax: set x to 5
            Token::Set => {
                self.consume(); // eat set
                let name = match self.consume() {
                    Token::Identifier(s) => s,
                    t => panic!("Expected Identifier after 'set', found {:?}", t),
                };
                if self.consume() != Token::To {
                    panic!("Expected 'to' after variable name");
                }
                let val = self.parse_expr();
                Statement::Set { key: name, value: val }
            }
            // New Syntax: create variable x with value 5
            Token::Create => {
                self.consume(); // eat create
                if self.consume() != Token::Variable {
                     panic!("Expected 'variable' after 'create'");
                }
                let name = match self.consume() {
                    Token::Identifier(s) => s,
                    t => panic!("Expected Identifier after 'create variable', found {:?}", t),
                };
                if self.consume() != Token::With {
                    panic!("Expected 'with' after variable name");
                }
                if self.consume() != Token::Value {
                    panic!("Expected 'value' after 'with'");
                }
                let val = self.parse_expr();
                Statement::Set { key: name, value: val }
            }
            // Math: math x add 5
            Token::Math => {
                self.consume(); // math
                let name = match self.consume() {
                    Token::Identifier(s) => s,
                    t => panic!("Expected variable name after 'math', found {:?}", t),
                };
                if self.consume() != Token::Add {
                    panic!("Expected 'add' after variable name in math statement");
                }
                let val = self.parse_expr();
                Statement::MathAdd { key: name, value: val }
            }
            // Loop: loop 5 times do
            Token::Loop => {
                self.consume(); // loop
                let count = self.parse_expr();
                if self.peek() == &Token::Times { self.consume(); }
                if self.peek() == &Token::Do { self.consume(); } // Optional do
                
                let mut body = Vec::new();
                while self.peek() != &Token::End {
                    body.push(self.parse_stmt());
                }
                self.consume(); // end
                Statement::Loop { count, body }
            }
            // Page: page "/" do ... end
            Token::Page => {
                self.consume(); // page
                let path = match self.consume() {
                    Token::StringLit(s) => s,
                    Token::Slash => "/".to_string(),
                    t => panic!("Expected path after 'page', found {:?}", t),
                };
                if self.peek() == &Token::Do { self.consume(); } // Optional do
                
                let mut body = Vec::new();
                while self.peek() != &Token::End {
                    body.push(self.parse_stmt());
                }
                self.consume(); // end
                Statement::Page { path, body }
            }
            Token::Title => {
                self.consume();
                Statement::Title(self.parse_expr())
            }
            Token::Header => {
                self.consume();
                Statement::Header(self.parse_expr())
            }
            // Db: db open "..."
            Token::Db => {
                self.consume(); // db
                match self.consume() {
                    Token::Open => Statement::DbOpen(match self.consume() {
                        Token::StringLit(s) => s,
                        t => panic!("Expected filename after 'db open', found {:?}", t),
                    }),
                    Token::Run => Statement::DbRun(match self.consume() {
                        Token::StringLit(s) => s,
                        t => panic!("Expected query after 'db run', found {:?}", t),
                    }),
                    Token::Insert => {
                         // Check for 'into'
                         match self.consume() {
                             Token::Identifier(s) if s == "into" => {},
                             t => panic!("Expected 'into' after 'db insert', found {:?}", t),
                         }
                         let table = match self.consume() {
                             Token::StringLit(s) => s,
                             t => panic!("Expected table name string, found {:?}", t),
                         };
                         if self.consume() != Token::Values {
                             panic!("Expected 'values' after table name");
                         }
                         let values = match self.consume() {
                             Token::StringLit(s) => s,
                             t => panic!("Expected values string, found {:?}", t),
                         };
                         Statement::DbInsert { table, values }
                    }
                    t => panic!("Expected 'open', 'run', or 'insert' after 'db', found {:?}", t),
                }
            }
            // Add: add header "..." or add button "..."
            Token::Add => {
                self.consume(); // eat add
                match self.peek() {
                    Token::Header => {
                        self.consume();
                        Statement::Header(self.parse_expr())
                    }
                    Token::Button => {
                        self.consume();
                        let label = self.parse_expr();
                        if self.consume() != Token::Triggers {
                             panic!("Expected 'triggers' after button label");
                        }
                        let target = match self.consume() {
                            Token::StringLit(s) => s,
                            t => panic!("Expected trigger target string, found {:?}", t),
                        };
                        Statement::Button { label, target }
                    }
                    t => panic!("Expected 'header' or 'button' after 'add', found {:?}", t),
                }
            }
            // Swarm: swarm join "..."
            Token::Swarm => {
                self.consume(); // swarm
                match self.consume() {
                    Token::Join => Statement::SwarmJoin(match self.consume() {
                        Token::StringLit(s) => s,
                        t => panic!("Expected topic after 'swarm join', found {:?}", t),
                    }),
                    Token::Broadcast => Statement::SwarmBroadcast(self.parse_expr()),
                    _ => panic!("Expected 'join' or 'broadcast' after 'swarm'"),
                }
            }
            // AI: ai think "prompt" about input
            Token::Ai => {
                self.consume(); // ai
                if self.consume() != Token::Think {
                    panic!("Expected 'think' after 'ai'");
                }
                let prompt = self.parse_expr();
                let mut input = Expression::StringLit("".to_string());
                
                if let Token::Identifier(ref s) = self.peek() {
                    if s == "about" {
                        self.consume();
                        input = self.parse_expr();
                    }
                }
                Statement::AiThink { input, prompt }
            }
            // Gate Syntax: gate to python execute "code"
            Token::Gate => {
                self.consume(); // eat gate
                if self.consume() != Token::To {
                    panic!("Expected 'to' after 'gate'");
                }
                let lang = match self.consume() {
                    Token::Python => "python".to_string(),
                    Token::Rust => "rust".to_string(),
                    Token::Identifier(s) => s,
                    t => panic!("Expected language identifier, found {:?}", t),
                };
                if self.consume() != Token::Execute {
                    panic!("Expected 'execute' after language");
                }
                let code = self.parse_expr();
                Statement::Gate { lang, code }
            }
            Token::Say => {
                self.consume(); // eat say
                Statement::Say(self.parse_expr())
            }
            Token::Server => {
                self.consume(); // server
                if self.consume() != Token::New { panic!("Expected 'new' after 'server'"); }
                if self.consume() != Token::Port { panic!("Expected 'port' after 'new'"); }
                let port = match self.consume() {
                    Token::Number(n) => n.to_string(),
                    Token::Identifier(s) => s,
                    t => panic!("Expected port number or variable, found {:?}", t),
                };
                Statement::ServerNew(port)
            }
            Token::Route => {
                self.consume(); // route
                let path = match self.consume() {
                    Token::Slash => "/".to_string(),
                    Token::StringLit(s) => s,
                    t => panic!("Expected route path, found {:?}", t),
                };
                Statement::Route { path }
            }
            Token::Serve => {
                self.consume(); // serve
                Statement::Serve(self.parse_expr())
            }
            Token::If => {
                self.consume(); // eat if
                let condition = self.parse_expr();
                
                // Allow optional 'then' or 'do'
                if self.peek() == &Token::Then || self.peek() == &Token::Do {
                    self.consume();
                }

                let mut then_branch = Vec::new();
                let mut else_branch = None;

                while self.peek() != &Token::End && self.peek() != &Token::Else {
                    then_branch.push(self.parse_stmt());
                }

                if self.peek() == &Token::Else {
                    self.consume(); // eat else
                    let mut e_branch = Vec::new();
                    while self.peek() != &Token::End {
                        e_branch.push(self.parse_stmt());
                    }
                    else_branch = Some(e_branch);
                }

                if self.consume() != Token::End {
                    panic!("Expected 'end' after 'if' block");
                }

                Statement::If { condition, then_branch, else_branch }
            }
            Token::While => {
                self.consume(); // eat while
                let condition = self.parse_expr();
                let mut body = Vec::new();

                while self.peek() != &Token::End {
                    body.push(self.parse_stmt());
                }

                if self.consume() != Token::End {
                    panic!("Expected 'end' after 'while' block");
                }

                Statement::While { condition, body }
            }
            Token::Func => {
                self.consume(); // eat func
                let name = match self.consume() {
                    Token::Identifier(s) => s,
                    t => panic!("Expected function name, found {:?}", t),
                };
                
                if self.peek() == &Token::Do { self.consume(); } // Optional do

                let params = Vec::new();
                let mut body = Vec::new();
                while self.peek() != &Token::End {
                    body.push(self.parse_stmt());
                }
                if self.consume() != Token::End {
                    panic!("Expected 'end' after 'func' block");
                }
                Statement::FuncDef { name, params, body }
            }
            // Async Task: async task name do ... end
            Token::Async => {
                self.consume(); // eat async
                if self.consume() != Token::Task {
                    panic!("Expected 'task' after 'async'");
                }
                let name = match self.consume() {
                    Token::Identifier(s) => s,
                    t => panic!("Expected task name, found {:?}", t),
                };
                
                if self.peek() == &Token::Do { self.consume(); } // Optional do

                let mut body = Vec::new();
                while self.peek() != &Token::End {
                    body.push(self.parse_stmt());
                }
                if self.consume() != Token::End {
                    panic!("Expected 'end' after 'async task' block");
                }
                Statement::AsyncTask { name, body }
            }
            // Await: await name
            Token::Await => {
                self.consume(); // eat await
                let name = match self.consume() {
                    Token::Identifier(s) => s,
                    t => panic!("Expected task name after 'await', found {:?}", t),
                };
                Statement::Await(name)
            }
            Token::Call => {
                self.consume(); // eat call
                let name = match self.consume() {
                    Token::Identifier(s) => s,
                    t => panic!("Expected function name after 'call', found {:?}", t),
                };
                let args = Vec::new();
                Statement::Call { name, args }
            }
            Token::Return => {
                self.consume();
                Statement::Return(self.parse_expr())
            }
            t => panic!("Parser Error: Unexpected token {:?}", t),
        }
    }

    fn parse_expr(&mut self) -> Expression {
        let mut left = match self.peek() {
            Token::LBracket => {
                self.consume(); // eat [
                let mut elements = Vec::new();
                if self.peek() != &Token::RBracket {
                    elements.push(self.parse_expr());
                    while self.peek() == &Token::Comma {
                        self.consume(); // eat ,
                        elements.push(self.parse_expr());
                    }
                }
                if self.consume() != Token::RBracket {
                    panic!("Expected ']' after array elements");
                }
                Expression::Array(elements)
            }
            Token::Exec => {
                self.consume(); // eat exec
                Expression::Command(Box::new(self.parse_expr()))
            }
            _ => {
                match self.consume() {
                    Token::StringLit(s) => Expression::StringLit(s),
                    Token::Number(n) => Expression::Number(n),
                    Token::Identifier(s) => Expression::Variable(s),
                    t => panic!("Expected expression value, found {:?}", t),
                }
            }
        };

        // Handle Indexing
        if self.peek() == &Token::LBracket {
            self.consume(); // eat [
            let index = self.parse_expr();
            if self.consume() != Token::RBracket {
                panic!("Expected ']' after index");
            }
            left = Expression::Index(Box::new(left), Box::new(index));
        }

        // Handle Binary Ops
        match self.peek() {
            Token::Plus | Token::Is | Token::Not | Token::Gt | Token::Lt => {
                let op = match self.consume() {
                    Token::Plus => Op::Plus,
                    Token::Is => Op::Is,
                    Token::Not => Op::Not,
                    Token::Gt => Op::Gt,
                    Token::Lt => Op::Lt,
                    _ => unreachable!(),
                };
                let right = self.parse_expr();
                left = Expression::BinaryOp(Box::new(left), op, Box::new(right));
            }
            _ => {}
        }

        left
    }
}
