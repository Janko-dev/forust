mod scanner;
mod parser;

pub use crate::forust::evaluate;

pub mod forust {
    use crate::scanner::{Scanner};
    use crate::parser::{parse, equal, Equation};
    
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
                let eq = equal(clone, x, y);
                if Ok(true) == eq {
                    range.push([x as f64, y as f64])
                } else if let Err(msg) = eq {
                    println!("{}", msg);   
                }
            }    
        }
        range
    }

    pub fn solve(input: &str) {
        let mut scanner = Scanner::new(input);
        scanner.tokenize();
        let result = match parse(&mut scanner.tokens) {
            Ok(e) => e,
            Err(msg) => panic!("{}", msg),
        };
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use crate::forust::solve;
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

    #[test]
    fn solve_simple_test(){
        solve("100 + (x - 3.7) = y**3");
        assert!(false)
    }
}

