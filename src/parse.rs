use crate::error::Error;
use crate::error::Error::PeatError;
use crate::expression::{Expression, Declaration};
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

fn parse_expression(expr_str: &str) -> Result<Expression, Error> {
    let expr_str_trim = expr_str.trim();
    if let Ok(parsed_bool) = expr_str_trim.parse::<bool>() {
        return Ok(BoolLiteral(parsed_bool));
    }
    if let Ok(parsed_uint) = expr_str_trim.parse::<u64>() {
        return Ok(UIntLiteral(parsed_uint));
    }
    if is_valid_id(expr_str_trim) {
        return Ok(Variable(String::from(expr_str_trim)));
    }
    Err(PeatError(format!("Could not parse {} as an expression.", expr_str_trim)))
}

fn parse_declaration(decl_str: &str) -> Result<Declaration, Error> {
    let decl_str_trim = decl_str.trim();
    let eq_pos = decl_str_trim.find('=').ok_or(PeatError(format!("Missing '='")))?;
    let (name_str_raw, post_id_str) = decl_str_trim.split_at(eq_pos);
    let name = String::from(name_str_raw.trim());
    let expression = parse_expression(&post_id_str[1..])?;
    Ok(Declaration { name, expression })
}