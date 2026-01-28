#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Pipe,       // |
    Semi,       // ;
    And,        // &&
    Or,         // ||
    Amp,        // &
    Gt,         // >
    Lt,         // <
    GtGt,       // >>
    LtLt,       // <<
    LParen,     // (
    RParen,     // )
}

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            if c == '\n' {
                chars.next();
                tokens.push(Token::Semi);
                continue;
            }
            if c.is_whitespace() {
                chars.next();
                continue;
            }

            match c {
                '|' => {
                    chars.next();
                    if let Some(&'|') = chars.peek() {
                        chars.next();
                        tokens.push(Token::Or);
                    } else {
                        tokens.push(Token::Pipe);
                    }
                }
                '&' => {
                    chars.next();
                    if let Some(&'&') = chars.peek() {
                        chars.next();
                        tokens.push(Token::And);
                    } else {
                        tokens.push(Token::Amp);
                    }
                }
                ';' => {
                    chars.next();
                    tokens.push(Token::Semi);
                }
                '>' => {
                    chars.next();
                    if let Some(&'>') = chars.peek() {
                        chars.next();
                        tokens.push(Token::GtGt);
                    } else {
                        tokens.push(Token::Gt);
                    }
                }
                '<' => {
                    chars.next();
                    if let Some(&'<') = chars.peek() {
                        chars.next();
                        tokens.push(Token::LtLt);
                    } else {
                        tokens.push(Token::Lt);
                    }
                }
                '(' => {
                    chars.next();
                    tokens.push(Token::LParen);
                }
                ')' => {
                    chars.next();
                    tokens.push(Token::RParen);
                }
                '#' => {
                    // Comment, skip rest of line
                    while let Some(&k) = chars.peek() {
                        if k == '\n' { break; }
                        chars.next();
                    }
                }
                _ => {
                    // Word start
                    let word = Self::read_word(&mut chars)?;
                    tokens.push(Token::Word(word));
                }
            }
        }

        Ok(tokens)
    }

    fn read_word(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<String, String> {
        let mut word = String::new();
        let mut in_quote = None; // None, Some('"'), Some('\'')

        while let Some(&c) = chars.peek() {
            if let Some(q) = in_quote {
                if c == q {
                    chars.next();
                    in_quote = None;
                    // We keep the quote char in the word? 
                    // Bash removes quotes unless escaped. 
                    // For simplicity, let's just parse the content.
                    // Actually, usually we want to keep structure or expand later.
                    // But standard tokenization usually treats "foo bar" as a single Word token.
                } else {
                    if q == '"' && c == '\\' {
                        chars.next();
                        if let Some(&next_c) = chars.peek() {
                            word.push(next_c);
                            chars.next();
                        } else {
                            word.push('\\');
                        }
                    } else {
                        word.push(c);
                        chars.next();
                    }
                }
            } else {
                if c.is_whitespace() {
                    break;
                }
                match c {
                    '|' | '&' | ';' | '>' | '<' | '(' | ')' => break,
                    '"' | '\'' => {
                        chars.next();
                        in_quote = Some(c);
                    }
                    '\\' => {
                        chars.next();
                        if let Some(&next_c) = chars.peek() {
                            word.push(next_c);
                            chars.next();
                        } else {
                            word.push('\\');
                        }
                    }
                    _ => {
                        word.push(c);
                        chars.next();
                    }
                }
            }
        }
        
        if in_quote.is_some() {
             return Err("Unclosed quote".to_string());
        }

        Ok(word)
    }
}