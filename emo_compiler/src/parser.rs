use crate::lexer::{Token, SpannedToken, Span};
use crate::ast::{Statement, Expression, Op, Spanned};
use logos::Logos;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

pub struct Parser<'a> {
    source: &'a str,
    tokens: Vec<SpannedToken>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Token::lexer(source);
        let mut tokens = Vec::new();
        
        while let Some(res) = lexer.next() {
            let span = lexer.span();
            match res {
                Ok(token) => tokens.push(SpannedToken { token, span }),
                Err(_) => {
                    // We could handle lexer errors better here
                }
            }
        }

        Self { source, tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        if self.pos >= self.tokens.len() {
             static EOF: Token = Token::Identifier(String::new()); // Pseudo-EOF
             &EOF
        } else {
             &self.tokens[self.pos].token
        }
    }

    fn peek_is(&self, expected: Token) -> bool {
        if self.is_at_end() { return false; }
        std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected)
    }

    fn peek_span(&self) -> Span {
        if self.pos >= self.tokens.len() {
            let last_pos = self.source.len();
            last_pos..last_pos
        } else {
            self.tokens[self.pos].span.clone()
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn advance(&mut self) -> SpannedToken {
        if self.is_at_end() {
             let last_pos = self.source.len();
             return SpannedToken { token: Token::Identifier("EOF".into()), span: last_pos..last_pos };
        }
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        token
    }

    fn match_token(&mut self, expected: Token) -> bool {
        if self.is_at_end() { return false; }
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected) {
            self.advance();
            return true;
        }
        false
    }

    fn consume(&mut self, expected: Token, msg: &str) -> ParseResult<SpannedToken> {
        if self.is_at_end() {
            return Err(ParseError { message: format!("{} but found EOF", msg), span: self.peek_span() });
        }
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else {
            Err(ParseError { 
                message: format!("{} but found {:?}", msg, self.peek()), 
                span: self.peek_span() 
            })
        }
    }
    
    fn consume_identifier(&mut self, msg: &str) -> ParseResult<(String, Span)> {
        if self.is_at_end() { 
            return Err(ParseError { message: format!("{} but found EOF", msg), span: self.peek_span() }); 
        }
        match self.peek() {
            Token::Identifier(_) => {
                let st = self.advance();
                if let Token::Identifier(s) = st.token {
                    Ok((s, st.span))
                } else {
                    unreachable!()
                }
            },
            _ => Err(ParseError { 
                message: format!("{} but found {:?}", msg, self.peek()), 
                span: self.peek_span() 
            }),
        }
    }

    pub fn parse(&mut self) -> ParseResult<Vec<Spanned<Statement>>> {
        let mut stmts = Vec::new();
        while !self.is_at_end() {
            stmts.push(self.parse_statement()?);
        }
        Ok(stmts)
    }

    fn spanned<T>(&self, start: usize, node: T) -> Spanned<T> {
        let end = if self.pos > 0 {
            self.tokens[self.pos - 1].span.end
        } else {
            start
        };
        Spanned { node, span: start..end }
    }

    fn parse_statement(&mut self) -> ParseResult<Spanned<Statement>> {
        let start = self.peek_span().start;
        match self.peek() {
            Token::Import => self.parse_import(),
            Token::Fn => self.parse_fn(),
            Token::Let => self.parse_let(),
            Token::Set => self.parse_set(),
            Token::If => self.parse_if(),
            Token::Loop => self.parse_loop(),
            Token::While => self.parse_while(),
            Token::Struct => self.parse_struct_def(),
            Token::Enum => self.parse_enum_def(),
            Token::Break => {
                Ok(self.spanned(start, Statement::Break))
            }
            Token::Return => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(self.spanned(start, Statement::Return(expr)))
            }
            Token::Unsafe => {
                self.advance();
                let body = self.parse_block()?;
                Ok(self.spanned(start, Statement::UnsafeBlock(body)))
            }
            Token::Identifier(_) => {
                if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1].token == Token::Assign {
                    let (name, _) = self.consume_identifier("Expected identifier")?;
                    self.advance(); // consume =
                    let value = self.parse_expr()?;
                    Ok(self.spanned(start, Statement::Set { name, value }))
                } else {
                    let expr = self.parse_expr()?;
                    Ok(self.spanned(start, Statement::Expression(expr)))
                }
            }
            _ => {
                let expr = self.parse_expr()?;
                Ok(self.spanned(start, Statement::Expression(expr)))
            }
        }
    }

    fn parse_import(&mut self) -> ParseResult<Spanned<Statement>> {
        let start = self.advance().span.start; // import
        let (module, _) = self.consume_identifier("Expected identifier after import")?;
        
        let mut source = None;
        if self.match_token(Token::From) {
            match self.advance().token {
                Token::StringLit(s) => source = Some(s),
                _ => return Err(ParseError { message: "Expected string literal after from".into(), span: self.peek_span() }),
            }
        }
        
        Ok(self.spanned(start, Statement::Import { module, source }))
    }

    fn parse_fn(&mut self) -> ParseResult<Spanned<Statement>> {
        let start = self.advance().span.start; // fn
        let (name, _) = self.consume_identifier("Expected function name")?;

        self.consume(Token::LParen, "Expected ( after function name")?;

        let mut params = Vec::new();
        while !self.match_token(Token::RParen) {
            let (p_name, _) = self.consume_identifier("Expected param name")?;
            if self.match_token(Token::Colon) {
                self.consume_identifier("Expected type identifier")?;
            }
            params.push((p_name, "Any".to_string()));
            self.match_token(Token::Comma);
        }

        let body = self.parse_block()?;
        Ok(self.spanned(start, Statement::FunctionDef { name, params, body }))
    }

    fn parse_block(&mut self) -> ParseResult<Vec<Spanned<Statement>>> {
        self.consume(Token::LBrace, "Expected { start of block")?;
        let mut stmts = Vec::new();
        while !self.is_at_end() && self.peek() != &Token::RBrace {
            stmts.push(self.parse_statement()?);
        }
        self.consume(Token::RBrace, "Expected } end of block")?;
        Ok(stmts)
    }

    fn parse_let(&mut self) -> ParseResult<Spanned<Statement>> {
        let start = self.advance().span.start; // let
        let (name, _) = self.consume_identifier("Expected var name")?;
        
        if self.match_token(Token::Colon) {
             self.consume_identifier("Expected type name")?;
        }

        self.consume(Token::Assign, "Expected =")?;
        let value = self.parse_expr()?;
        Ok(self.spanned(start, Statement::Let { name, value }))
    }

    fn parse_set(&mut self) -> ParseResult<Spanned<Statement>> {
        let start = self.advance().span.start; // set
        let (name, _) = self.consume_identifier("Expected var name")?;
        if !self.match_token(Token::To) && !self.match_token(Token::Assign) {
             return Err(ParseError { message: "Expected 'to' or '='".into(), span: self.peek_span() });
        }
        let value = self.parse_expr()?;
        Ok(self.spanned(start, Statement::Set { name, value }))
    }

    fn parse_if(&mut self) -> ParseResult<Spanned<Statement>> {
        let start = self.advance().span.start; // if
        let cond = self.parse_expr()?;
        let then_block = self.parse_block()?;
        let mut else_block = None;
        if self.match_token(Token::Else) {
            else_block = Some(self.parse_block()?);
        }
        Ok(self.spanned(start, Statement::If { cond, then_block, else_block }))
    }

    fn parse_loop(&mut self) -> ParseResult<Spanned<Statement>> {
        let start = self.advance().span.start; // loop
        let mut count = None;
        if let Token::Integer(_) = self.peek() {
            count = Some(self.parse_expr()?);
        }
        let body = self.parse_block()?;
        Ok(self.spanned(start, Statement::Loop { count, body }))
    }

    fn parse_while(&mut self) -> ParseResult<Spanned<Statement>> {
        let start = self.advance().span.start; // while
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(self.spanned(start, Statement::While { cond, body }))
    }

    fn parse_struct_def(&mut self) -> ParseResult<Spanned<Statement>> {
        let start = self.advance().span.start; // struct
        let (name, _) = self.consume_identifier("Expected struct name")?;
        self.consume(Token::LBrace, "Expected '{' after struct name")?;
        
        let mut fields = Vec::new();
        while !self.match_token(Token::RBrace) {
            let (f_name, _) = self.consume_identifier("Expected field name")?;
            self.consume(Token::Colon, "Expected ':' after field name")?;
            let (f_type, _) = self.consume_identifier("Expected field type")?;
            fields.push((f_name, f_type));
            self.match_token(Token::Comma);
        }
        Ok(self.spanned(start, Statement::StructDef { name, fields }))
    }

    fn parse_enum_def(&mut self) -> ParseResult<Spanned<Statement>> {
        let start = self.advance().span.start; // enum
        let (name, _) = self.consume_identifier("Expected enum name")?;
        self.consume(Token::LBrace, "Expected '{' after enum name")?;
        
        let mut variants = Vec::new();
        while !self.match_token(Token::RBrace) {
            let (v_name, _) = self.consume_identifier("Expected variant name")?;
            variants.push(v_name);
            self.match_token(Token::Comma);
        }
        Ok(self.spanned(start, Statement::EnumDef { name, variants }))
    }

    fn parse_expr(&mut self) -> ParseResult<Spanned<Expression>> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> ParseResult<Spanned<Expression>> {
        let start = self.peek_span().start;
        let mut expr = self.parse_comparison()?;
        while !self.is_at_end() {
             match self.peek() {
                 Token::Eq | Token::NotEq => {
                     let op = match self.advance().token {
                         Token::Eq => Op::Eq,
                         Token::NotEq => Op::NotEq,
                         _ => unreachable!(),
                     };
                     let right = self.parse_comparison()?;
                     let span = start..right.span.end;
                     expr = Spanned {
                         node: Expression::BinaryOp(Box::new(expr), op, Box::new(right)),
                         span
                     };
                 },
                 _ => break,
             }
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> ParseResult<Spanned<Expression>> {
        let start = self.peek_span().start;
        let mut expr = self.parse_term()?;
        while !self.is_at_end() {
             match self.peek() {
                 Token::Gt | Token::Lt | Token::Gte | Token::Lte => {
                     let op = match self.advance().token {
                         Token::Gt => Op::Gt,
                         Token::Lt => Op::Lt,
                         Token::Gte => Op::Gte,
                         Token::Lte => Op::Lte,
                         _ => unreachable!(),
                     };
                     let right = self.parse_term()?;
                     let span = start..right.span.end;
                     expr = Spanned {
                         node: Expression::BinaryOp(Box::new(expr), op, Box::new(right)),
                         span
                     };
                 },
                 _ => break,
             }
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> ParseResult<Spanned<Expression>> {
        let start = self.peek_span().start;
        let mut expr = self.parse_factor()?;
        while !self.is_at_end() {
             match self.peek() {
                 Token::Plus | Token::Minus => {
                     let op = match self.advance().token {
                         Token::Plus => Op::Plus,
                         Token::Minus => Op::Minus,
                         _ => unreachable!(),
                     };
                     let right = self.parse_factor()?;
                     let span = start..right.span.end;
                     expr = Spanned {
                         node: Expression::BinaryOp(Box::new(expr), op, Box::new(right)),
                         span
                     };
                 },
                 _ => break,
             }
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> ParseResult<Spanned<Expression>> {
        let start = self.peek_span().start;
        let mut expr = self.parse_primary()?;
        while !self.is_at_end() {
             match self.peek() {
                 Token::Star | Token::Slash => {
                     let op = match self.advance().token {
                         Token::Star => Op::Mul,
                         Token::Slash => Op::Div,
                         _ => unreachable!(),
                     };
                     let right = self.parse_primary()?;
                     let span = start..right.span.end;
                     expr = Spanned {
                         node: Expression::BinaryOp(Box::new(expr), op, Box::new(right)),
                         span
                     };
                 },
                 _ => break,
             }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> ParseResult<Spanned<Expression>> {
        let start = self.peek_span().start;
        let mut expr = match self.peek() {
            Token::Integer(n) => {
                let v = *n;
                self.advance();
                self.spanned(start, Expression::Number(v))
            },
            Token::StringLit(s) => {
                let v = s.clone();
                self.advance();
                self.spanned(start, Expression::StringLit(v))
            },
            Token::True => {
                self.advance();
                self.spanned(start, Expression::Bool(true))
            },
            Token::False => {
                self.advance();
                self.spanned(start, Expression::Bool(false))
            },
            Token::Null => {
                self.advance();
                self.spanned(start, Expression::Null)
            },
            Token::Minus => {
                self.advance();
                let right = self.parse_primary()?;
                self.spanned(start, Expression::BinaryOp(
                    Box::new(self.spanned(start, Expression::Number(0))),
                    Op::Minus,
                    Box::new(right)
                ))
            },
            Token::Identifier(s) => {
                let name = s.clone();
                
                // Peek ahead to see if it's a struct literal: Name { field: ... }
                let is_struct_lit = if self.pos + 1 < self.tokens.len() {
                    self.tokens[self.pos + 1].token == Token::LBrace && 
                    self.pos + 2 < self.tokens.len() && 
                    matches!(self.tokens[self.pos + 2].token, Token::Identifier(_)) &&
                    self.pos + 3 < self.tokens.len() &&
                    self.tokens[self.pos + 3].token == Token::Colon
                } else {
                    false
                };

                self.advance();
                
                if is_struct_lit {
                    self.advance(); // {
                    let mut fields = Vec::new();
                    while !self.match_token(Token::RBrace) {
                        let (f_name, _) = self.consume_identifier("Expected field name")?;
                        self.consume(Token::Colon, "Expected ':' after field name")?;
                        let f_val = self.parse_expr()?;
                        fields.push((f_name, f_val));
                        self.match_token(Token::Comma);
                    }
                    self.spanned(start, Expression::StructLiteral { name, fields })
                } else {
                    self.spanned(start, Expression::Identifier(name))
                }
            },
            Token::LParen => {
                self.advance(); // (
                let inner = self.parse_expr()?;
                self.consume(Token::RParen, "Expected ')' after expression")?;
                self.spanned(start, inner.node) // Re-wrap with new span
            },
            _ => return Err(ParseError { message: format!("Unexpected token in expression: {:?}", self.peek()), span: self.peek_span() }),
        };

        loop {
            if self.is_at_end() { break; }
            match self.peek() {
                Token::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    while !self.match_token(Token::RParen) {
                        args.push(self.parse_expr()?);
                        if !self.peek_is(Token::RParen) && !self.match_token(Token::Comma) {
                            return Err(ParseError { 
                                message: "Expected ',' or ')' in argument list".into(), 
                                span: self.peek_span() 
                            });
                        }
                    }
                    let span = start..self.tokens[self.pos - 1].span.end;
                    expr = Spanned {
                        node: Expression::Call { func: Box::new(expr), args },
                        span
                    };
                },
                Token::Dot => {
                    self.advance();
                    let (member, m_span) = self.consume_identifier("Expected member name")?;
                    let span = start..m_span.end;
                    expr = Spanned {
                        node: Expression::MemberAccess { object: Box::new(expr), member },
                        span
                    };
                },
                _ => break,
            }
        }

        Ok(expr)
    }
}