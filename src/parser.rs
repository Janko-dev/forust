use std::{iter::Peekable, slice::Iter, fmt::Display};

use crate::scanner::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, &'a Token<'a>, Box<Expr<'a>>),
    Unary(&'a Token<'a>, Box<Expr<'a>>),
    Group(Box<Expr<'a>>),
    Id(&'a str),
    Num(f64)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Equation<'a> {
    pub left: Expr<'a>,
    pub right: Expr<'a>,
}

impl<'a> Display for Equation<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", expr_to_string(&self.left), expr_to_string(&self.right))
    }
}

pub fn expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::Binary(l, op, r) => {
            match op {
                Token::Plus => format!("{} + {}",  expr_to_string(&*l), expr_to_string(&*r)),
                Token::Minus => format!("{} - {}", expr_to_string(&*l), expr_to_string(&*r)),
                Token::Slash => format!("{} / {}", expr_to_string(&*l), expr_to_string(&*r)),
                Token::Star => format!("{} * {}",  expr_to_string(&*l), expr_to_string(&*r)),
                Token::Pow => format!("{}**{}",    expr_to_string(&*l), expr_to_string(&*r)),
                _ => unreachable!(),
            }
        },
        Expr::Group(e) => format!("({})", expr_to_string(&*e)),
        Expr::Unary(_, r) => format!("-{}", expr_to_string(&*r)),
        Expr::Num(n) => n.to_string(),
        Expr::Id(s) => s.to_string(),
    }
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

fn eval(expr: Expr, x: f64, y: f64) -> Result<f64, String> {
    match expr {
        Expr::Binary(l, op, r) => {
            let left = eval(*l, x, y)?;
            let right = eval(*r, x, y)?;
            match op {
                Token::Plus => Ok(left + right),
                Token::Minus => Ok(left - right),
                Token::Slash => Ok(left / right),
                Token::Star => Ok(left * right),
                Token::Pow => Ok(left.powf(right)),
                other => Err(format!("Unknown operator: {:?}", other))
            }
        },
        Expr::Unary(op, e) => {
            let right = eval(*e, x, y)?;
            match op {
                Token::Minus => Ok(-right),
                other => Err(format!("Unknown operator: {:?}", other))
            }
        },
        Expr::Group(e) => eval(*e, x, y),
        Expr::Num(n) => Ok(n),
        Expr::Id(s) => {
            match s {
                "x" | "X" => Ok(x as f64),
                "y" | "Y" => Ok(y as f64),
                other => Err(format!("Unknown identifier: '{}'", other))
            }
        }
    }
}

pub fn equal(eq: Equation, x: f64, y: f64) -> Result<bool, String> {
    let left = eval(eq.left, x, y)?;
    let right = eval(eq.right, x, y)?;
    Ok(left == right)
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