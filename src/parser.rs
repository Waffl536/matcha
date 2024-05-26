use crate::lexer::*;
use std::iter::Peekable;
use std::vec::IntoIter;

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut node_vec = Vec::new();
    let mut tokens_iter = tokens.into_iter().peekable();

    while tokens_iter.peek().is_some() {
        if let Some(node) = parse_statement(&mut tokens_iter) {
            node_vec.push(node);
        } 
        else {
            panic!("Unexpected token: {:?}", tokens_iter.peek().cloned());
        }
    }

    node_vec
}

fn parse_statement(tokens_iter: &mut Peekable<IntoIter<Token>>) -> Option<Node> {
    if let Some(token) = tokens_iter.peek().cloned() {
        match token {
            Token::Keyword(ref kw) if kw == "if" => {
                tokens_iter.next();
                if let Some(condition) = parse_expression(tokens_iter) {
                    let mut node_vec = Vec::new();
                    while tokens_iter.peek().is_some() {
                        if let Some(node) = parse_statement(tokens_iter) {
                            if let Node::Expr(e) = node { continue; }
                            if let Node::Fi = node { break; }
                            // if let Node::ElseStatement(v) = node { 
                            //     break; 
                            // }
                            node_vec.push(node);
                            
                        } 
                        else {
                            panic!("Unexpected token: {:?}", tokens_iter.peek().cloned());
                        }
                    }
                    
                    return Some(Node::IfStatement(condition, node_vec));
                }
            },
            // Token::Keyword(ref kw) if kw == "else" => {
            //     tokens_iter.next();
            //     let mut node_vec = Vec::new();
            //     while tokens_iter.peek().is_some() {
            //         if let Some(node) = parse_statement(tokens_iter) {
            //             if let Node::Expr(e) = node { continue; }
            //             if let Node::Fi = node { break; }
            //             node_vec.push(node);
            //         } 
            //         // else {
            //         //     panic!("Unexpected token: {:?}", tokens_iter.peek().cloned());
            //         // }
            //     }
                
            //     return Some(Node::ElseStatement(node_vec));
            // },
            Token::Keyword(ref kw) if kw == "fi" => {
                tokens_iter.next();
                return Some(Node::Fi);
            },
            Token::Keyword(ref kw) if kw == "out" => {
                tokens_iter.next();
                if let Some(e) = parse_expression(tokens_iter) {
                    return Some(Node::Out(e));
                }
            },
            Token::Keyword(ref kw) if kw == "read" => {
                tokens_iter.next();
                if let Some(e) = parse_expression(tokens_iter) {
                    return Some(Node::Read(e));
                }
            },
            Token::DTypeToken(ref dt) => {
                tokens_iter.next();
                if let Some(Token::Id(var_name)) = tokens_iter.next() {
                    if let Some(Token::Op(op)) = tokens_iter.peek().cloned() {
                        if op == "=" {
                            tokens_iter.next();
                            if let Some(init_value) = parse_expression(tokens_iter) {
                                return Some(Node::VarDec(dt.clone(), VarName(var_name), init_value));
                            }
                        }
                    }
                }
            },
            _ => return parse_expression(tokens_iter).map(Node::Expr),
        }
    }
    None
}

fn parse_expression(tokens_iter: &mut Peekable<IntoIter<Token>>) -> Option<Expr> {
    let mut left = parse_factor(tokens_iter)?;
    let binary = ["=", "+", "-", "*", "/", ">", "<", "==", "||", "&&"];
    let unary = ["!", "++", "--"];
    while let Some(token) = tokens_iter.peek().cloned() {
        match token {
            Token::Op(ref op) if binary.into_iter().any(|i| i==op) => {
                let op = tokens_iter.next().unwrap();
                let right = parse_factor(tokens_iter)?;
                left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
            },
            _ => break,
        }
    }

    Some(left)
}

fn parse_factor(tokens_iter: &mut Peekable<IntoIter<Token>>) -> Option<Expr> {
    let token = tokens_iter.next()?;
    match token {
        Token::Literal(lv) => Some(Expr::Literal(lv)),
        Token::Id(id) => Some(Expr::Var(VarName(id))),
        // Token::Op(ref op) if op == "(" => {
        //     let expr = parse_expression(tokens_iter)?;
        //     if let Some(Token::Op(ref op)) = tokens_iter.next() {
        //         if op == ")" {
        //             return Some(expr);
        //         }
        //     }
        //     None
        // },
        _ => None,
    }
}

#[derive(Debug)]
pub struct VarName(String);

#[derive(Debug)]
pub enum Node {
    Block(Vec<Node>),
    VarDec(DataType, VarName, Expr),
    IfStatement(Expr, Vec<Node>),
    ElifStatement(Expr, Vec<Node>),
    ElseStatement(Vec<Node>),
    Fi,
    Out(Expr),
    Read(Expr),
    Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Literal(LitValue),
    Var(VarName),
    BinaryOp(Box<Expr>, Token, Box<Expr>),
}
