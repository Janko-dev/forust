use std::str;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    OpenParen,
    CloseParen,
    Id(&'a str),
    Num(f64),

    Plus,
    Minus,
    Star,
    Slash,
    Pow,

    Equal,
}

pub struct Scanner<'a> {
    input: &'a [u8],
    start: usize,
    current: usize,
    pub tokens: Vec<Token<'a>>
}

impl<'a> Scanner<'a>{

    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            start: 0,
            current: 0,
            tokens: Vec::new()
        }
    }

    fn peek(&self, n: usize) -> Option<&u8> {
        self.input.get(self.current+n)
    }

    fn add_token(&mut self, token: Token<'a>){
        self.tokens.push(token);
        self.next();
    }

    fn add_token_or(&mut self, token: Token<'a>, alt_char: u8, alt_token: Token<'a>){
        if self.peek(1) == Some(&alt_char) {
            self.tokens.push(alt_token);
            self.next();
            self.next();
        } else {
            self.tokens.push(token);
            self.next();
        }
    }

    fn add_number(&mut self){
        
        while let Some(b'0'..=b'9') = self.peek(0) {
            self.next();
        }

        if let Some(b'.') = self.peek(0) {
            self.next();
            while let Some(b'0'..=b'9') = self.peek(0) {
                self.next();
            }
        }

        let result = str::from_utf8(&self.input[self.start..self.current])
            .unwrap_or_else(|err| {
                println!("{:?}", err);
                &""
            })
            .parse::<f64>()
            .unwrap_or_else(|err| {
                println!("{:?}", err);
                0.0
            });
        self.tokens.push(Token::Num(result));

    }

    fn add_identifier(&mut self){
        while let Some(b'a'..=b'z') | 
                  Some(b'A'..=b'Z') |
                  Some(b'_') = self.peek(0) {
            self.next();
        }

        let result = str::from_utf8(&self.input[self.start..self.current])
            .unwrap_or_else(|err| {
                println!("{:?}", err);
                &"ERROR"
            });
        self.tokens.push(Token::Id(result));
    }

    pub fn tokenize(&mut self) {
        loop {
            self.start = self.current;
            match self.peek(0){
                Some(b'+') => self.add_token(Token::Plus),
                Some(b'-') => self.add_token(Token::Minus),
                Some(b'/') => self.add_token(Token::Slash),
                Some(b'*') => self.add_token_or(Token::Star, b'*', Token::Pow),

                Some(b'=') => self.add_token(Token::Equal),
                Some(b'(') => self.add_token(Token::OpenParen),
                Some(b')') => self.add_token(Token::CloseParen),

                
                Some(b' ') | 
                Some(b'\t') | 
                Some(b'\n') => { self.next(); },
                
                Some(b'0'..=b'9') => self.add_number(),
                Some(b'a'..=b'z') |
                Some(b'A'..=b'Z') |
                Some(b'_') => self.add_identifier(),

                Some(c) => { 
                    println!("Unexpected character: '{}'", *c as char);
                    self.next(); 
                },
                None => break,
            }
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        self.input.get(self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_simple_expr(){
        let input = "10 = x + y";
        let mut scanner = Scanner::new(input);
        scanner.tokenize();
        assert_eq!(scanner.tokens, vec![
            Token::Num(10.0), Token::Equal, Token::Id("x"), Token::Plus, Token::Id("y") 
        ])
    }

    #[test]
    fn scan_complex_expr(){
        let input = "x + (10-3.333 * 27**(y+2)) = x                 +\t23";
        let mut scanner = Scanner::new(input);
        scanner.tokenize();
        assert_eq!(scanner.tokens, vec![
            Token::Id("x"), Token::Plus, Token::OpenParen, Token::Num(10.0), Token::Minus, Token::Num(3.333), 
            Token::Star, Token::Num(27.0), Token::Pow, Token::OpenParen, Token::Id("y"), Token::Plus, Token::Num(2.0), Token::CloseParen, 
            Token::CloseParen, Token::Equal, Token::Id("x"), Token::Plus, Token::Num(23.0)
        ])
    }
}