use crate::tokenize::Token;
use crate::expression::{Expression, Type, UIntRangeExpression, UIntVariable, UIntLiteral};
use crate::error::Error;
use std::iter;

enum Tree {
    TokenNode(Token),
    ExpressionNode(Box<dyn Expression>),
}

impl Tree {
    fn from_token(token: Token) -> Tree {
        match token {
            Token::UInt(ui) =>
                Tree::from_expression(Box::new(UIntLiteral::new(ui))),
            Token::Id(id) =>
                Tree::from_expression(Box::new(UIntVariable::new(id))),
            _ => Tree::TokenNode(token)
        }
    }
    fn from_expression(expression: Box<dyn Expression>) -> Tree {
        Tree::ExpressionNode(expression)
    }
}

pub(crate) fn reduce(tokens: Vec<Token>) -> Result<Box<dyn Expression>, Error> {
    let mut trees: Vec<Tree> =
        tokens.iter().map(|token| { Tree::from_token(token.clone()) }).collect();
    loop {
        if let Some(bin_expr_parts) = get_bin_expr_parts(&trees, Token::Range)? {
            println!("Range");
            let range_expr = build_range_expression(&bin_expr_parts)?;
            let op_pos = bin_expr_parts.op_pos;
            replace_with_bin_expr(&mut trees, range_expr, op_pos);
            continue;
        }
        if let [Tree::ExpressionNode(expression)] = &trees[..] {
            println!("Got a single expression!");
            break Ok(expression.clone_expr());
        } else {
            println!("Reduction failed!");
            break Err(Error::from("Cannot parse expression."));
        }
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
            println!("op_pos == {}", op_pos);
            println!("trees.len() == {}", trees.len());
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
                            format!("An expression cannot end with {}.", op))
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

fn build_range_expression(bin_expr_parts: &BinExprParts)
                          -> Result<UIntRangeExpression, Error> {
    let from = bin_expr_parts.lhs.as_typed().as_int_expr()?.clone_int_expr();
    let until = bin_expr_parts.rhs.as_typed().as_int_expr()?.clone_int_expr();
    Ok(UIntRangeExpression::new(from, until))
}

fn replace_with_bin_expr<E: Expression + 'static>(trees: &mut Vec<Tree>,
                                                  bin_expr: E,
                                                  op_pos: usize)
                                                  -> Vec<Tree> {
    let range = (op_pos - 1)..=(op_pos + 1);
    let tree_new_iter =
        iter::once(Tree::ExpressionNode(Box::new(bin_expr)));
    trees.splice(range, tree_new_iter).collect()
}