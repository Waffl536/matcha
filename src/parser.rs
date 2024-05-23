use crate::lexer::*;
use std::iter::Peekable;
use std::vec::IntoIter;

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut node_vec = Vec::new();
    let mut tokens_iter = tokens.into_iter().peekable();

    while tokens_iter.peek().is_some() {
        if let Some(node) = parse_statement(&mut tokens_iter) {
            if let Node::Expr(e) = node { continue; }
            node_vec.push(node);
        } 
        // else {
        //     panic!("Unexpected token while parsing: ");
        // }
    }

    node_vec
}

fn parse_statement(tokens: &mut Peekable<IntoIter<Token>>) -> Option<Node> {
    if let Some(token) = tokens.peek().cloned() {
        match token {
            Token::Keyword(ref kw) if kw == "if" => {
                tokens.next();
                if let Some(condition) = parse_expression(tokens) {
                    if let Some(body) = parse_statement(tokens) {
                        return Some(Node::IfStatement(condition, Box::new(body)));
                    }
                }
            },
            Token::Keyword(ref kw) if kw == "out" => {
                tokens.next();
                if let Some(e) = parse_expression(tokens) {
                    return Some(Node::Out(e));
                }
            },
            Token::DTypeToken(ref dt) => {
                tokens.next();
                if let Some(Token::Id(var_name)) = tokens.next() {
                    if let Some(Token::Op(op)) = tokens.peek().cloned() {
                        if op == "=" {
                            tokens.next();
                            if let Some(init_value) = parse_expression(tokens) {
                                return Some(Node::VarDec(dt.clone(), VarName(var_name), init_value));
                            }
                        }
                    }
                }
            },
            _ => return parse_expression(tokens).map(Node::Expr),
        }
    }
    None
}

fn parse_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Option<Expr> {
    parse_term(tokens)
}

fn parse_term(tokens: &mut Peekable<IntoIter<Token>>) -> Option<Expr> {
    let mut left = parse_factor(tokens)?;

    while let Some(token) = tokens.peek().cloned() {
        match token {
            Token::Op(ref op) if op == "+" || op == "-"|| op == "==" => {
                let op = tokens.next().unwrap();
                let right = parse_factor(tokens)?;
                left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
            },
            _ => break,
        }
    }

    Some(left)
}

fn parse_factor(tokens: &mut Peekable<IntoIter<Token>>) -> Option<Expr> {
    let token = tokens.next()?;
    match token {
        Token::Literal(dt, lv) => Some(Expr::Literal(dt, lv)),
        Token::Id(id) => Some(Expr::Var(VarName(id))),
        Token::Op(ref op) if op == "(" => {
            let expr = parse_expression(tokens)?;
            if let Some(Token::Op(ref op)) = tokens.next() {
                if op == ")" {
                    return Some(expr);
                }
            }
            None
        },
        _ => None,
    }
}

#[derive(Debug)]
pub struct VarName(String);

#[derive(Debug)]
pub enum Value {
    LitValue(DataType, LitValue),
    VarValue(VarName),
}

#[derive(Debug)]
pub enum Node {
    VarDec(DataType, VarName, Expr),
    IfStatement(Expr, Box<Node>),
    ElseStatement(Box<Node>),
    Out(Expr),
    Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Literal(DataType, LitValue),
    Var(VarName),
    BinaryOp(Box<Expr>, Token, Box<Expr>),
}
