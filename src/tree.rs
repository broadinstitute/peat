use crate::tokenize::Token;
use crate::expression::Expression;
use crate::error::Error;

enum Tree {
    TokenNode(Token),
    ExpressionNode(Box<dyn Expression>),
}

impl Tree {
    fn from_token(token: Token) -> Tree { Tree::TokenNode(token) }
    fn from_expression(expression: Box<dyn Expression>) -> Tree {
        Tree::ExpressionNode(expression)
    }
}

pub(crate) fn reduce(tokens: Vec<Token>) -> Result<Box<dyn Expression>, Error> {
    let mut trees: Vec<Tree> =
        tokens.iter().map(|token| { Tree::from_token(token.clone()) }).collect();
    loop {
        todo!();
        break Err(Error::from("Cannot parse expression."))
    }
}

fn build_binary_expression(trees: &Vec<Tree>, op: Token)
    -> Result<Option<(usize, Box<dyn Expression>)>, Error> {
    let pos_opt = trees.iter().position(|tree| {
        match tree {
            Tree::TokenNode(token) => true,
            _ => false
        }
    });
    match pos_opt {
        Some(pos) => {
            if pos == 0 {
                Err(Error::from(format!("An expression cannot start with {}.", op)))
            } else if pos == trees.len() - 1 {
                Err(Error::from(format!("An expression cannot end with {}.", op)))
            } else {
                todo!()
            }
        },
        None => Ok(None)
    }
}