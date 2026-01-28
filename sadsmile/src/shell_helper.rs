use std::borrow::Cow;
use colored::*;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::{Highlighter, CmdKind};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Context, Helper, Result};

pub struct SadSmileHelper {
    pub completer: FilenameCompleter,
    pub hinter: HistoryHinter,
}

impl SadSmileHelper {
    pub fn new() -> Self {
        SadSmileHelper {
            completer: FilenameCompleter::new(),
            hinter: HistoryHinter::new(),
        }
    }
}

impl Completer for SadSmileHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for SadSmileHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for SadSmileHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        let mut result = String::new();
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
             match c {
                 '"' | '\'' => {
                     result.push_str(&format!("{}", c.to_string().magenta()));
                 }
                 '|' | '&' | ';' | '>' | '<' => {
                     result.push_str(&format!("{}", c.to_string().yellow().bold()));
                 }
                 _ => {
                     result.push(c);
                 }
             }
        }

        Cow::Owned(result)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _kind: CmdKind) -> bool {
        true 
    }
}

impl Validator for SadSmileHelper {
    fn validate(&self, _ctx: &mut ValidationContext) -> Result<ValidationResult> {
        // Simplified validation: Always valid for now
        Ok(ValidationResult::Valid(None))
    }
}

impl Helper for SadSmileHelper {}