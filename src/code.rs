use declaration::Declaration;
use version::Version;
use crate::util::error::Error;
use std::{
    io::{Read, BufReader},
    fs::File,
    io
};

pub mod version;
pub mod value;
pub mod types;
pub mod expression;
pub mod declaration;
pub mod parse;
pub mod evaluate;
pub mod tokenize;
mod tree;

pub struct PeatCode {
    pub(crate) version: Version,
    pub(crate) declarations: Vec<Declaration>,
    pub(crate) body: String
}

pub(crate) fn get_peat_code(input_file_name: &Option<String>) -> Result<PeatCode, Error> {
    let source: Box<dyn Read> = match input_file_name {
        Some(file_path) => Box::new(File::open(file_path)?),
        None => Box::new(io::stdin())
    };
    let input_buf_reader = BufReader::new(source);
    parse::parse_input(input_buf_reader)
}

