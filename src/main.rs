#![feature(nll)]

#[derive(Debug)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Times,
    Divide,
    Exponent,
    LParen,
    RParen,
}

#[derive(Debug)]
enum Node<'a> {
    Number(f64),
    Unary(&'a Token, &'a Node<'a>),
    Binary(&'a Node<'a>, Token, &'a Node<'a>),
}

fn eat_digits_as_f64(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<f64> {
    let n = chars.next()?;
    let mut number = n.to_string().parse::<f64>().unwrap();
    let mut peek = chars.peek().cloned();
    while let Some(c) = peek {
        if !c.is_digit(10) {
            break;
        }
        chars.next();
        let digit_value = c.to_string().parse::<f64>().unwrap();
        number = number * 10f64 + digit_value;

        peek = chars.peek().cloned();
    }
    Some(number)
}

fn lex(s: &String) -> Vec<Token> {
    let mut tkns = Vec::<Token>::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.peek() {
        match c {
            _ws if c.is_whitespace() => {
                chars.next();
            }
            n if n.is_digit(10) => {
                let mut number = eat_digits_as_f64(&mut chars).unwrap();
                // check for decimal portion:
                match chars.peek().cloned() {
                    Some(c) if c == '.' => {
                        chars.next();
                        let fraction = eat_digits_as_f64(&mut chars).unwrap();
                        let places = fraction.log10().ceil();
                        number += fraction / 10f64.powf(places);
                    }
                    _ => {}
                }
                tkns.push(Token::Number(number));
            }
            '+' => {
                chars.next();
                tkns.push(Token::Plus)
            }
            '-' => {
                chars.next();
                tkns.push(Token::Minus)
            }
            '*' => {
                chars.next();
                tkns.push(Token::Times)
            }
            '/' => {
                chars.next();
                tkns.push(Token::Divide)
            }
            '^' => {
                chars.next();
                tkns.push(Token::Exponent)
            }
            '(' => {
                chars.next();
                tkns.push(Token::LParen)
            }
            ')' => {
                chars.next();
                tkns.push(Token::RParen)
            }
            _ => panic!("Unexpected input: {}", c),
        }
    }
    tkns
}

struct Pratt<'a> {
    tokens: &'a mut std::iter::Peekable<std::slice::Iter<'a, Token>>,
}
impl<'a> Pratt<'a> {
    fn new(tokens: &'a mut std::iter::Peekable<std::slice::Iter<'a, Token>>) -> Pratt<'a> {
        Pratt { tokens }
    }

    fn bp(&self, t: &Token) -> usize {
        match t {
            Token::RParen => 0,
            Token::Number(_) => 5,
            Token::Plus => 10,
            Token::Minus => 10,
            Token::Times => 20,
            Token::Divide => 20,
            Token::Exponent => 30,
            Token::LParen => 40,
        }
    }

    fn nud(&mut self, t: &'a Token, bp: usize) -> Result<Node<'a>, &'a str> {
        match t {
            Token::Number(n) => Ok(Node::Number(*n)),
            Token::Plus | Token::Minus => {
                let right = self.expr(bp)?;
                Ok(Node::Unary(t, &right))
            }
            Token::LParen => Err("TODO"),
            _ => Err(format!("Unexpected token {:?}", t).as_str()),
        }
    }

    fn led(&mut self, left: Node<'a>, op: &Token, bp: usize) -> Result<Node<'a>, &'a str> {
        Err("TODO")
    }

    fn expr(&mut self, rbp: usize) -> Result<Node<'a>, &'a str> {
        let err = "Undexpected EOF";
        let mut t = self.tokens.next().ok_or(err)?;
        let mut left = self.nud(t, self.bp(t))?;
        println!("left = {:?}", left);
        while rbp < self.bp(t) {
            let op = self.tokens.next().ok_or(err)?;
            left = self.led(left, &op, self.bp(op))?;
            t = self.tokens.next().ok_or(err)?;
        }
        Ok(left)
    }
}

fn main() {
    let source = "12.34 + 2*3 - 1^2^3".to_owned();
    let tkns = lex(&source);

    let mut tkns_iter = tkns.iter().peekable();
    let mut parser = Pratt::new(&mut tkns_iter);
    let node = parser.expr(0);
    println!(
        "Hello, world! source={}; tkns={:?}; node={:?}",
        source, tkns, node
    );
}
