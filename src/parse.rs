use crate::error::Error;
use crate::error::Error::PeatError;
use crate::expression::Expression;
use crate::expression::Expression::{BoolLiteral, UIntLiteral, Variable};

fn is_valid_id_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_valid_id_part(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

fn is_valid_id(st: &str) -> bool {
    let mut chars = st.chars();
    match chars.next() {
        None => return false,
        Some(char1) =>
            is_valid_id_start(char1) && chars.all(|ch| { is_valid_id_part(ch) })
    }
}

fn parse_expression(string: &str) -> Result<Expression, Error> {
    let trimmed = string.trim();
    if let Ok(parsed_bool) = trimmed.parse::<bool>() {
        return Ok(BoolLiteral(parsed_bool));
    }
    if let Ok(parsed_uint) = trimmed.parse::<u64>() {
        return Ok(UIntLiteral(parsed_uint));
    }
    if is_valid_id(trimmed) {
        return Ok(Variable(String::from(trimmed)));
    }
    Err(PeatError(format!("Could not parse {} as an expression.", trimmed)))
}