use crate::error::Error;
use crate::error::Error::PeatError;
use std::fmt::{Display, Formatter};
use std::fmt;

pub(crate) mod strings {
    pub(crate) const ASSIGN: &str = "=";
    pub(crate) const ITERATE: &str = "<-";
    pub(crate) const RANGE: &str = "..";
    pub(crate) const DIVIDE: &str = "/";
    pub(crate) const PICK: &str = "$";
}

#[derive(PartialEq, Eq)]
pub(crate) enum Token {
    Assign,
    Iterate,
    Range,
    Divide,
    Pick,
    Id(String),
    UInt(u64),
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::Assign => { Token::Assign }
            Token::Iterate => { Token::Iterate }
            Token::Range => { Token::Range }
            Token::Divide => { Token::Divide }
            Token::Pick => { Token::Pick }
            Token::Id(id) => { Token::Id(id.clone()) }
            Token::UInt(ui) => { Token::UInt(*ui) }
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Assign => { f.write_str(strings::ASSIGN) }
            Token::Iterate => { f.write_str(strings::ITERATE) }
            Token::Range => { f.write_str(strings::RANGE) }
            Token::Divide => { f.write_str(strings::DIVIDE) }
            Token::Pick => { f.write_str(strings::PICK) }
            Token::Id(id) => { Display::fmt(id, f) }
            Token::UInt(ui) => { Display::fmt(ui, f) }
        }
    }
}

pub(crate) struct Tokenizer {
    string: String,
}

fn is_valid_id_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_valid_id_part(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

impl Tokenizer {
    pub(crate) fn new(string: String) -> Tokenizer {
        Tokenizer { string }
    }

    fn next_token_and_remainder(&mut self) -> Result<Option<(Token, String)>, Error> {
        let trimmed = self.string.trim();
        if trimmed.is_empty() {
            Ok(None)
        } else if let Some(stripped) = trimmed.strip_prefix(strings::ASSIGN) {
            Ok(Some((Token::Assign, String::from(stripped))))
        } else if let Some(stripped) = trimmed.strip_prefix(strings::ITERATE) {
            Ok(Some((Token::Iterate, String::from(stripped))))
        } else if let Some(stripped) = trimmed.strip_prefix(strings::RANGE) {
            Ok(Some((Token::Range, String::from(stripped))))
        } else if let Some(stripped) = trimmed.strip_prefix(strings::DIVIDE) {
            Ok(Some((Token::Divide, String::from(stripped))))
        } else if let Some(stripped) = trimmed.strip_prefix(strings::PICK) {
            Ok(Some((Token::Pick, String::from(stripped))))
        } else if trimmed.starts_with(is_valid_id_start) {
            let pos =
                trimmed.find(|ch| { !is_valid_id_part(ch) }).unwrap_or_else(|| trimmed.len());
            let (id_str, str_new) = trimmed.split_at(pos);
            let id_string = String::from(id_str);
            let remainder = String::from(str_new);
            Ok(Some((Token::Id(id_string), remainder)))
        } else if trimmed.starts_with(|ch: char| ch.is_digit(10)) {
            let pos =
                trimmed
                    .find(|ch: char| { !ch.is_digit(10) }).unwrap_or_else(|| trimmed.len());
            let (num_str, str_new) = trimmed.split_at(pos);
            let number = num_str.parse::<u64>().unwrap();
            let remainder = String::from(str_new);
            Ok(Some((Token::UInt(number), remainder)))
        } else {
            Err(PeatError(format!("Unexpected token {}", trimmed)))
        }
    }

    pub(crate) fn strip_token(&mut self) -> Result<Option<Token>, Error> {
        match self.next_token_and_remainder()? {
            None => Ok(None),
            Some((token, remainder)) => {
                self.string = remainder;
                Ok(Some(token))
            }
        }
    }

    pub(crate) fn write_to_vec(mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        loop {
            match self.strip_token()? {
                None => { break Ok(tokens); }
                Some(token) => { tokens.push(token) }
            }
        }
    }
}


