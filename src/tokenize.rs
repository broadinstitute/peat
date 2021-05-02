use crate::error::Error;
use crate::error::Error::PeatError;
use std::fmt::{Display, Formatter, Debug};
use std::fmt;

mod strings {
    pub(crate) const ASSIGN: &str = "=";
}

pub(crate) enum Token {
    Assign,
    Id(String),
    UInt(u64),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Assign => { f.write_str("=") }
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
        } else if trimmed.starts_with(is_valid_id_start) {
            let pos =
                trimmed.find(|ch| { !is_valid_id_part(ch) }).unwrap_or(trimmed.len());
            let (id_str, str_new) = trimmed.split_at(pos);
            let id_string = String::from(id_str);
            let remainder = String::from(str_new);
            Ok(Some((Token::Id(id_string), remainder)))
        } else if trimmed.starts_with(|ch: char| { ch.is_digit(10) }) {
            let pos =
                trimmed.find(|ch: char| { !ch.is_digit(10) }).unwrap_or(trimmed.len());
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
}


