use std::ops::Range;

pub type Span = Range<usize>;

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Import {
        module: String,
        source: Option<String>,
    },
    FunctionDef {
        name: String,
        params: Vec<(String, String)>,
        body: Vec<Spanned<Statement>>,
    },
    Let {
        name: String,
        value: Spanned<Expression>,
    },
    Set {
        name: String,
        value: Spanned<Expression>,
    },
    Return(Spanned<Expression>),
    Break,
    Expression(Spanned<Expression>),
    If {
        cond: Spanned<Expression>,
        then_block: Vec<Spanned<Statement>>,
        else_block: Option<Vec<Spanned<Statement>>>,
    },
    Loop {
        count: Option<Spanned<Expression>>,
        body: Vec<Spanned<Statement>>,
    },
    While {
        cond: Spanned<Expression>,
        body: Vec<Spanned<Statement>>,
    },
    StructDef {
        name: String,
        fields: Vec<(String, String)>, // name, type
    },
    EnumDef {
        name: String,
        variants: Vec<String>, // simplified for now
    },
    UnsafeBlock(Vec<Spanned<Statement>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(i64),
    StringLit(String),
    Bool(bool),
    Null,
    Identifier(String),
    StructLiteral {
        name: String,
        fields: Vec<(String, Spanned<Expression>)>,
    },
    BinaryOp(Box<Spanned<Expression>>, Op, Box<Spanned<Expression>>),
    Call {
        func: Box<Spanned<Expression>>,
        args: Vec<Spanned<Expression>>,
    },
    MemberAccess {
        object: Box<Spanned<Expression>>,
        member: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Mul,
    Div,
    Eq,
    NotEq,
    Gt,
    Lt,
    Gte,
    Lte,
}