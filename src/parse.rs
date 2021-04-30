use crate::{
    expression::{Expression, Declaration},
    error::Error::PeatError,
    error::Error,
    expression::Expression::{BoolLiteral, UIntLiteral, Variable},
    peatcode::PeatCode,
    version::Version,
};
use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

fn parse_version_line(line: &str) -> Result<Version, Error> {
    if line.starts_with("Peat") {
        let version_str = line[4..].trim();
        Version::parse(version_str)
    } else {
        Err(PeatError(String::from("Version line needs to start with \"Peat\"")))
    }
}

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

const HEADER_END_LINE: &str = "===";

fn is_header_end_line(line: &str) -> bool {
    line == HEADER_END_LINE
}

type FileLines = Lines<BufReader<File>>;

fn read_next_line(lines: &mut FileLines) -> Result<String, Error> {
    Ok(lines.next().ok_or(Error::from("File is incomplete."))??)
}

fn parse_declarations(lines: &mut FileLines)
                      -> Result<Vec<Declaration>, Error> {
    let mut declarations = Vec::<Declaration>::new();
    loop {
        let line = read_next_line(lines)?;
        if is_header_end_line(&line) {
            break Ok(declarations);
        }
        let declaration = parse_declaration(&line)?;
        declarations.push(declaration);
    }
}

fn parse_body(lines: &mut FileLines) -> Result<String, Error> {
    let mut body = String::new();
    for line in lines {
        let line = line?;
        body.push_str(&line)
    }
    Ok(body)
}

pub(crate) fn parse_file(file_path: &str) -> Result<PeatCode, Error> {
    let reader = BufReader::new(File::open(file_path)?);
    let mut lines = reader.lines();
    let version = parse_version_line(&read_next_line(&mut lines)?)?;
    let declarations = parse_declarations(&mut lines)?;
    let body = parse_body(&mut lines)?;
    Ok(PeatCode { version, declarations, body })
}