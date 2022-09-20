mod scanner;
mod parser;

pub use crate::equarust::run;

pub mod equarust {
    use crate::scanner::{Scanner};
    use crate::parser::{parse};

    
    pub fn run(input: &str) {
        let mut scanner = Scanner::new(input);
        scanner.tokenize();
        let result = parse(&mut scanner.tokens);
        println!("{:?}", result);
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::scanner::{Scanner, Token};
    use crate::parser::{parse, Equation, Expr};

    #[test]
    fn scanner_and_parser_integration_simple(){
        // input = "-x = y + 20"
        let input = "-x = y + 20";
        let mut scanner = Scanner::new(input);
        scanner.tokenize();
        let result = parse(&mut scanner.tokens);
        assert_eq!(result, Ok(Equation {
            left: Expr::Unary(&Token::Minus, Box::new(Expr::Id("x"))),
            right: Expr::Binary(Box::new(Expr::Id("y")), &Token::Plus, Box::new(Expr::Num(20.0))),
        })) 
    }
}

