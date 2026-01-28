#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Plus,
    Is,
    Not,
    Gt,
    Lt,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    StringLit(String),
    Number(i32),
    Variable(String),
    BinaryOp(Box<Expression>, Op, Box<Expression>),
    Array(Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
    InputArg(String),
    HttpGet(Box<Expression>),
    Command(Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Set { key: String, value: Expression },
    Say(Expression),
    MathAdd { key: String, value: Expression },
    ServerNew(String),
    Route { path: String },
    Serve(Expression),
    Page { path: String, body: Vec<Statement> },
    Title(Expression),
    Header(Expression),
    Button { label: Expression, target: String },
    StyleThemeDark,
    SwarmJoin(String),
    SwarmBroadcast(Expression),
    OnSwarmMessage { trigger: String, body: Vec<Statement> },
    DbOpen(String),
    DbRun(String),
    DbInsert { table: String, values: String },
    LinkLibrary(String),
    ForeignFn { name: String, args: String, ret: String },
    CallFn { name: String, args: String },
    AsyncTask { name: String, body: Vec<Statement> },
    Await(String),
    TryConnect { host: Expression, port: Expression, body: Vec<Statement> },
    RawRust(String),
    Loop { count: Expression, body: Vec<Statement> },
    If { condition: Expression, then_branch: Vec<Statement>, else_branch: Option<Vec<Statement>> },
    While { condition: Expression, body: Vec<Statement> },
    FuncDef { name: String, params: Vec<String>, body: Vec<Statement> },
    Call { name: String, args: Vec<Expression> },
    Return(Expression),
    // New Gate
    Gate { lang: String, code: Expression },
    // AI
    AiThink { input: Expression, prompt: Expression },
}
