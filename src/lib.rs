mod scanner;
mod parser;

pub use crate::forust::evaluate;

pub mod forust {
    use crate::scanner::{Scanner};
    use crate::parser::{parse, equal};
    
    pub fn evaluate(input: &str, minx: i32, maxx: i32, miny: i32, maxy: i32) -> Vec<[f64; 2]>{
        let mut scanner = Scanner::new(input);
        scanner.tokenize();
        let result = match parse(&mut scanner.tokens) {
            Ok(e) => e,
            Err(msg) => panic!("{}", msg),
        };
        
        
        let mut range: Vec<[f64; 2]> = Vec::new();
        for x in minx..maxx {
            for y in miny..maxy {
                let clone = result.clone();
                if Ok(true) == equal(clone, x, y) {
                    range.push([x as f64, y as f64])
                }
            }    
        }
        range
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

