use std::io::{BufRead, BufReader, Lines, Read};

use crate::{
    declaration::{Assignment, Declaration},
    error::Error,
    error::Error::PeatError,
    expression::Expression,
    peatcode::PeatCode,
    tokenize::{Token, Tokenizer},
    tree,
    version::Version,
};
use crate::declaration::Iteration;

fn parse_version_line(line: &str) -> Result<Version, Error> {
    if let Some(stripped) = line.strip_prefix("Peat") {
        let version_str = stripped.trim();
        Version::parse(version_str)
    } else {
        Err(PeatError(String::from("Version line needs to start with \"Peat\"")))
    }
}

fn parse_expression(tokenizer: Tokenizer) -> Result<Box<dyn Expression>, Error> {
    tree::reduce(tokenizer.write_to_vec()?)
}

fn parse_declaration(decl_str: &str) -> Result<Declaration, Error> {
    let mut tokenizer = Tokenizer::new(String::from(decl_str));
    let token1 =
        tokenizer.strip_token()?.ok_or_else(|| Error::from("Empty declaration"))?;
    let id =
        if let Token::Id(id) = token1 {
            id
        } else {
            return Err(Error::from("Declaration needs to start with an identifier"));
        };
    let token2 =
        tokenizer.strip_token()?.ok_or_else(|| Error::from("Missing '=' or '<-'."))?;
    match token2 {
        Token::Assign => {
            let expression = parse_expression(tokenizer)?;
            Ok(Declaration::Assign(Assignment::new(id, expression)))
        }
        Token::Iterate => {
            let expression =
                parse_expression(tokenizer)?.as_typed().as_range_expr()?.clone_range_expr();
            Ok(Declaration::Iterate(Iteration::new(id, expression)))
        }
        _ => {
            return Err(PeatError(
                format!("Expected {} or {}, but got {}.", Token::Assign, Token::Iterate, token2)
            ));
        }
    }
}

const HEADER_END_LINE: &str = "===";

fn is_header_end_line(line: &str) -> bool {
    line == HEADER_END_LINE
}

type InputLines = Lines<BufReader<Box<dyn Read>>>;

fn read_next_line(lines: &mut InputLines) -> Result<String, Error> {
    Ok(lines.next().ok_or_else(|| Error::from("File is incomplete."))??)
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