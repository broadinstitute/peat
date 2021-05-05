use crate::{
    expression::{Expression, UIntLiteral, UIntVariable},
    declaration::{Declaration, Assignment},
    error::Error::PeatError,
    error::Error,
    peatcode::PeatCode,
    version::Version,
    tokenize::{Token, Tokenizer},
};
use std::io::{BufReader, BufRead, Lines, Read};

fn parse_version_line(line: &str) -> Result<Version, Error> {
    if line.starts_with("Peat") {
        let version_str = line[4..].trim();
        Version::parse(version_str)
    } else {
        Err(PeatError(String::from("Version line needs to start with \"Peat\"")))
    }
}

fn parse_expression(mut tokenizer: Tokenizer) -> Result<Box<dyn Expression>, Error> {
    let token =
        tokenizer.strip_token()?.ok_or(Error::from("Missing expression."))?;
    match token {
        Token::Id(id) => { Ok(Box::new(UIntVariable::new(id))) }
        Token::UInt(ui) => { Ok(Box::new(UIntLiteral::new(ui))) }
        _ => { Err(Error::from(format!("Expected expression, but got {}.", token))) }
    }
}

fn parse_declaration(decl_str: &str) -> Result<Declaration, Error> {
    let mut tokenizer = Tokenizer::new(String::from(decl_str));
    let token1 =
        tokenizer.strip_token()?.ok_or(Error::from("Empty declaration"))?;
    let id =
        if let Token::Id(id) = token1 {
            id
        } else {
            return Err(Error::from("Declaration needs to start with an identifier"));
        };
    let token2 = tokenizer.strip_token()?.ok_or(Error::from("Missing '='"))?;
    if let Token::Assign = token2 {
        ()
    } else {
        return Err(PeatError(format!("Expected '=', but got {}.", token2)));
    }
    let expression = parse_expression(tokenizer)?;
    Ok(Declaration::Assign(Assignment::new(id, expression)))
}

const HEADER_END_LINE: &str = "===";

fn is_header_end_line(line: &str) -> bool {
    line == HEADER_END_LINE
}

type InputLines = Lines<BufReader<Box<dyn Read>>>;

fn read_next_line(lines: &mut InputLines) -> Result<String, Error> {
    Ok(lines.next().ok_or(Error::from("File is incomplete."))??)
}

fn parse_declarations(lines: &mut InputLines)
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

fn parse_body(lines: &mut InputLines) -> Result<String, Error> {
    let mut body = String::new();
    for line in lines {
        let line = line?;
        body.push_str(&line)
    }
    Ok(body)
}

pub(crate) fn parse_input(reader: BufReader<Box<dyn Read>>) -> Result<PeatCode, Error> {
    let mut lines = reader.lines();
    let version = parse_version_line(&read_next_line(&mut lines)?)?;
    let declarations = parse_declarations(&mut lines)?;
    let body = parse_body(&mut lines)?;
    Ok(PeatCode { version, declarations, body })
}

// fn print_lines(file_path_opt: Option<String>) -> Result<(), String> {
//     let buf_reader = match file_path_opt {
//         Some(file_path) => BufReader::new(File::open(file_path)?),
//         None => BufReader::new(std::io::stdin())
//     };
//     for line in buf_reader.lines() {
//         println!("{}", line?);
//     }
//     Ok(())
// }