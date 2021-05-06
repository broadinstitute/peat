use crate::tokenize::Token;
use crate::expression::{Expression, UIntRangeRangeExpression, UIntSimpleRangeExpression,
                        UIntPickRangeExpression, UIntVariable, UIntLiteral};
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
            let range_expr = build_range_expression(&bin_expr_parts)?;
            let op_pos = bin_expr_parts.op_pos;
            replace_with_bin_expr(&mut trees, range_expr, op_pos);
            continue;
        } else if let Some(bin_expr_parts) = get_bin_expr_parts(&trees, Token::Divide)? {
            let divide_expr = build_divide_expression(&bin_expr_parts)?;
            let op_pos = bin_expr_parts.op_pos;
            replace_with_bin_expr(&mut trees, divide_expr, op_pos);
            continue;
        } else if let Some(bin_expr_parts) = get_bin_expr_parts(&trees, Token::Pick)? {
            let pick_expr = build_pick_expression(&bin_expr_parts)?;
            let op_pos = bin_expr_parts.op_pos;
            replace_with_bin_expr(&mut trees, pick_expr, op_pos);
            continue;
        }
        if let [Tree::ExpressionNode(expression)] = &trees[..] {
            break Ok(expression.clone_expr());
        } else {
            break Err(Error::from("Cannot parse expression."));
        }
    }
}

struct BinExprParts<'a> {
    op_pos: usize,
    lhs: &'a Box<dyn Expression>,
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
                Ok(Some(BinExprParts { op_pos, lhs, rhs }))
            }
        }
        None => Ok(None)
    }
}

fn build_range_expression(bin_expr_parts: &BinExprParts)
                          -> Result<UIntSimpleRangeExpression, Error> {
    let from = bin_expr_parts.lhs.as_typed().as_int_expr()?.clone_int_expr();
    let until = bin_expr_parts.rhs.as_typed().as_int_expr()?.clone_int_expr();
    Ok(UIntSimpleRangeExpression::new(from, until))
}

fn build_divide_expression(bin_expr_parts: &BinExprParts)
                           -> Result<UIntRangeRangeExpression, Error> {
    let dividend =
        bin_expr_parts.lhs.as_typed().as_range_expr()?.clone_range_expr();
    let divisor =
        bin_expr_parts.rhs.as_typed().as_range_expr()?.clone_range_expr();
    Ok(UIntRangeRangeExpression::new(dividend, divisor))
}

fn build_pick_expression(bin_expr_parts: &BinExprParts)
                         -> Result<UIntPickRangeExpression, Error> {
    let groups =
        bin_expr_parts.lhs.as_typed().as_range_range_expr()?.clone_range_range_expr();
    let pick =
        bin_expr_parts.rhs.as_typed().as_int_expr()?.clone_int_expr();
    Ok(UIntPickRangeExpression::new(groups, pick))
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