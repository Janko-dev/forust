use std::{iter::Peekable, slice::Iter};

use crate::scanner::Token;

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, &'a Token<'a>, Box<Expr<'a>>),
    Unary(&'a Token<'a>, Box<Expr<'a>>),
    Group(Box<Expr<'a>>),
    Id(&'a str),
    Num(f64)
}

#[derive(Debug, PartialEq)]
pub struct Equation<'a> {
    pub left: Expr<'a>,
    pub right: Expr<'a>,
}

pub fn parse<'a>(tokens: &'a mut Vec<Token<'a>>) -> Result<Equation<'a>, String> {
    let mut tokens = tokens.iter().peekable();
    let left = expression(&mut tokens)?;
    expect(&mut tokens, Token::Equal)?;
    let right = expression(&mut tokens)?;
    Ok(Equation { left, right})
}

fn expect<'a>(tokens: &mut Peekable<Iter<Token<'a>>>, expected: Token) -> Result<(), String> {
    if tokens.peek() != Some(&&expected) {
        return Err(format!("Expected {:?}, but got {:?}", expected, tokens));
    }
    tokens.next();
    Ok(())
}

fn expression<'a>(tokens: &mut Peekable<Iter<'a, Token<'a>>>) -> Result<Expr<'a>, String> {
    let mut left = factor(tokens)?;
    while let Some(Token::Plus) | 
              Some(Token::Minus) = tokens.peek() {
        let op = tokens.peek().unwrap().clone();
        tokens.next();
        let right = factor(tokens)?;
        left = Expr::Binary(Box::new(left), op, Box::new(right));
    } 
    Ok(left)
}

fn factor<'a>(tokens: &mut Peekable<Iter<'a, Token<'a>>>) -> Result<Expr<'a>, String> {
    let mut left = pow(tokens)?;
    while let Some(Token::Star) | 
              Some(Token::Slash) = tokens.peek() {
        let op = tokens.peek().unwrap().clone();
        tokens.next();
        let right = pow(tokens)?;
        left = Expr::Binary(Box::new(left), op, Box::new(right));
    } 
    Ok(left)
}

fn pow<'a>(tokens: &mut Peekable<Iter<'a, Token<'a>>>) -> Result<Expr<'a>, String> {
    let mut left = unary(tokens)?;
    while let Some(Token::Pow) = tokens.peek() {
        let op = tokens.peek().unwrap().clone();
        tokens.next();
        let right = unary(tokens)?;
        left = Expr::Binary(Box::new(left), op, Box::new(right));
    } 
    Ok(left)
}

fn unary<'a>(tokens: &mut Peekable<Iter<'a, Token<'a>>>) -> Result<Expr<'a>, String> {
    
    if let Some(Token::Minus) = tokens.peek() {
        let op = tokens.peek().unwrap().clone();
        tokens.next();
        return Ok(Expr::Unary(op, Box::new(unary(tokens)?)));
    } 
    primary(tokens)
}

fn primary<'a>(tokens: &mut Peekable<Iter<'a, Token<'a>>>) -> Result<Expr<'a>, String> {
    match tokens.peek() {
        Some(Token::Id(s)) => {
            tokens.next();
            Ok(Expr::Id(*s))
        },
        Some(Token::Num(n)) => {
            tokens.next();
            Ok(Expr::Num(*n))
        },
        Some(Token::OpenParen) => {
            tokens.next();
            let expr = expression(tokens)?;
            expect(tokens, Token::CloseParen)?;
            Ok(Expr::Group(Box::new(expr)))
        },
        Some(other) => Err(format!("Unexpected primary: {:?}", other)),
        None => Err(format!("Unexpected primary: None")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_equation(){
        // x = y
        let mut tokens = vec![Token::Id("x"), Token::Equal, Token::Id("y")];
        let result = parse(&mut tokens);
        assert_eq!(result, Ok(Equation {
            left: Expr::Id("x"),
            right: Expr::Id("y"),
        })) 
    }

    #[test]
    fn parse_complex_linear_equation(){
        // 100 + x = x/y + 2
        let mut tokens = vec![
            Token::Num(100.0), Token::Plus, Token::Id("x"), 
            Token::Equal, 
            Token::Id("x"), Token::Slash, Token::Id("y"), Token::Plus, Token::Num(2.0)
        ];
        let result = parse(&mut tokens);
        assert_eq!(result, Ok(Equation {
            left: Expr::Binary(Box::new(Expr::Num(100.0)), &Token::Plus, Box::new(Expr::Id("x"))),
            right: Expr::Binary(
                Box::new(Expr::Binary(Box::new(Expr::Id("x")), &Token::Slash, Box::new(Expr::Id("y")))), 
                &Token::Plus, 
                Box::new(Expr::Num(2.0))),
        }))
    }
}