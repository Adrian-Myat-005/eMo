use crate::lexer::Token;
use crate::types::{Command, SimpleCommand, Redirect, RedirectionType, ListOp};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Command, String> {
        let mut parser = Parser { tokens, pos: 0 };
        if parser.tokens.is_empty() {
            return Err("Empty input".to_string());
        }
        parser.parse_list()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    // List: Pipeline (Op Pipeline)*
    fn parse_list(&mut self) -> Result<Command, String> {
        let mut left = self.parse_pipeline()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Semi => ListOp::Semi,
                Token::And => ListOp::And,
                Token::Or => ListOp::Or,
                Token::Amp => ListOp::Background,
                _ => break,
            };
            self.advance();
            
            // Handle trailing operator (e.g., "ls &" or "ls ;")
            if self.peek().is_none() {
                 // For now, we return the command as is, but we might want to attach the "Background" property to it.
                 // In our AST, ListOp is between two commands. 
                 // We'll append an empty command as the right side for now.
                 let empty = Command::Simple(SimpleCommand {
                    words: vec![],
                    redirects: vec![],
                    assignments: vec![],
                 });
                 return Ok(Command::List(Box::new(left), op, Box::new(empty)));
            }
            
            let right = self.parse_pipeline()?;
            left = Command::List(Box::new(left), op, Box::new(right));
        }

        Ok(left)
    }

    // Pipeline: Simple (| Simple)*
    fn parse_pipeline(&mut self) -> Result<Command, String> {
        let mut commands = Vec::new();
        commands.push(self.parse_simple()?);

        while let Some(Token::Pipe) = self.peek() {
            self.advance();
            commands.push(self.parse_simple()?);
        }

        if commands.len() == 1 {
            Ok(commands.pop().unwrap())
        } else {
            Ok(Command::Pipeline(commands))
        }
    }

    fn parse_simple(&mut self) -> Result<Command, String> {
        let mut words = Vec::new();
        let mut redirects = Vec::new();

        loop {
            match self.peek() {
                Some(Token::Word(w)) => {
                    words.push(w.clone());
                    self.advance();
                }
                Some(Token::Gt) => { self.advance(); redirects.push(self.parse_redirect(RedirectionType::Output)?); }
                Some(Token::Lt) => { self.advance(); redirects.push(self.parse_redirect(RedirectionType::Input)?); }
                Some(Token::GtGt) => { self.advance(); redirects.push(self.parse_redirect(RedirectionType::Append)?); }
                Some(Token::LtLt) => { self.advance(); redirects.push(self.parse_redirect(RedirectionType::HereDoc)?); }
                _ => break,
            }
        }
        
        // Return empty command if nothing parsed, but only if we are in a context where that's okay.
        // For now, if words and redirects are empty, it's an empty command (e.g. inside a loop or extra ;).
        
        Ok(Command::Simple(SimpleCommand {
            words,
            redirects,
            assignments: Vec::new(),
        }))
    }

    fn parse_redirect(&mut self, rtype: RedirectionType) -> Result<Redirect, String> {
        match self.peek() {
            Some(Token::Word(w)) => {
                let target = w.clone();
                self.advance();
                Ok(Redirect { rtype, target })
            }
            _ => Err("Expected filename after redirection".to_string()),
        }
    }
}