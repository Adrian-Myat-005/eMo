use logos::Logos;
use std::ops::Range;

pub type Span = Range<usize>;

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace
#[logos(skip r"//[^\n]*")]   // Skip comments
pub enum Token {
    // Keywords
    #[token("import")]
    Import,
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("set")]
    Set,
    #[token("to")]
    To,
    #[token("if")]
    If,
    #[token("then")]
    Then,
    #[token("else")]
    Else,
    #[token("loop")]
    Loop,
    #[token("while")]
    While,
    #[token("do")]
    Do,
    #[token("const")]
    Const,
    #[token("return")]
    Return,
    #[token("break")]
    Break,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("unsafe")]
    Unsafe,
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("from")]
    From,
    #[token("null")]
    Null,

    // Brackets
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    // Operators
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("!=")]
    NotEq,
    #[token(">")]
    Gt,
    #[token("<")]
    Lt,
    #[token(">=")]
    Gte,
    #[token("<=")]
    Lte,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("->")]
    Arrow,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("exec")]
    Exec,

    // Literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        let s = &s[1..s.len()-1];
        let mut res = String::new();
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => res.push('\n'),
                    Some('t') => res.push('\t'),
                    Some('r') => res.push('\r'),
                    Some('\\') => res.push('\\'),
                    Some('"') => res.push('"'),
                    Some(other) => res.push(other),
                    None => res.push('\\'),
                }
            } else {
                res.push(c);
            }
        }
        res
    })]
    StringLit(String),

    #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),
}