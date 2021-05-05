use crate::tokenize::Token;
use crate::expression::{Expression, Type};
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
        if let Some(bin_expr_parts) = get_bin_expr_parts(&trees, Token::Range)? {
            todo!()
        }
        break Err(Error::from("Cannot parse expression."));
    }
}

struct BinExprParts<'a> {
    op_pos: usize,
    lhs: &'a Box<dyn Expression>,
    op: Token,
    rhs: &'a Box<dyn Expression>,
}

fn get_bin_expr_parts(trees: &Vec<Tree>, op: Token) -> Result<Option<BinExprParts>, Error> {
    let pos_opt = trees.iter().position(|tree| {
        match tree {
            Tree::TokenNode(token) => token == &op,
            _ => false
        }
    });
    match pos_opt {
        Some(op_pos) => {
            if op_pos == 0 {
                Err(Error::from(format!("An expression cannot start with {}.", op)))
            } else if op_pos == trees.len() - 1 {
                Err(Error::from(format!("An expression cannot end with {}.", op)))
            } else {
                let lhs_tree =
                    trees.get(op_pos - 1)
                        .ok_or(Error::from(
                            format!("An expression cannot start with {}.", op))
                        )?;
                let lhs = match lhs_tree {
                    Tree::TokenNode(token) => {
                        return Err(Error::from(
                            format!("Expected expression before {}, but got {}", op, token))
                        );
                    }
                    Tree::ExpressionNode(expression) => { expression }
                };
                let rhs_tree =
                    trees.get(op_pos + 1)
                        .ok_or(Error::from(
                            format!("An expression cannot start with {}.", op))
                        )?;
                let rhs = match rhs_tree {
                    Tree::TokenNode(token) => {
                        return Err(Error::from(
                            format!("Expected expression after {}, but got {}", op, token))
                        );
                    }
                    Tree::ExpressionNode(expression) => { expression }
                };
                Ok(Some(BinExprParts { op_pos, lhs, op, rhs }))
            }
        }
        None => Ok(None)
    }
}

// fn build_range_expression() -> Result<Box<dyn >>