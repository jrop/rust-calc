#![feature(nll)]

#[derive(Clone, Debug)]
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
    Unary(&'a Token, Box<Node<'a>>),
    Binary(Box<Node<'a>>, Token, Box<Node<'a>>),
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
            Token::Plus => 10,
            Token::Minus => 10,
            Token::Times => 20,
            Token::Divide => 20,
            Token::Exponent => 30,
            Token::LParen => 40,
            Token::Number(_) => 100,
        }
    }

    fn nud(&mut self, t: &'a Token, bp: usize) -> Result<Node<'a>, String> {
        match t {
            Token::Number(n) => Ok(Node::Number(*n)),
            Token::Plus | Token::Minus => {
                let right = self.expr(0)?;
                Ok(Node::Unary(t, Box::new(right)))
            }
            Token::LParen => {
                let right = self.expr(0)?;
                match self.tokens.next() {
                    Some(Token::RParen) => Ok(right),
                    _ => Err("Expected ')'".to_owned()),
                }
            }
            _ => Err(format!("Unexpected token in NUD context: {:?}", t).to_owned()),
        }
    }

    fn led(&mut self, left: Node<'a>, op: &Token, bp: usize) -> Result<Node<'a>, String> {
        match op {
            Token::Plus | Token::Minus | Token::Times | Token::Divide => {
                let right = self.expr(bp)?;
                Ok(Node::Binary(Box::new(left), op.clone(), Box::new(right)))
            }
            Token::Exponent => {
                let right = self.expr(bp - 1)?;
                Ok(Node::Binary(Box::new(left), op.clone(), Box::new(right)))
            }
            _ => Err(format!(
                "Unexpected token in LED context: {:?} (left={:?})",
                op, left
            )
            .to_owned()),
        }
    }

    fn next_is_eof(&mut self) -> bool {
        let peeked = self.tokens.peek();
        peeked.is_none()
    }
    fn expr(&mut self, rbp: usize) -> Result<Node<'a>, String> {
        let err = "Undexpected EOF";
        let mut t = self.tokens.next().ok_or(err)?;

        let mut left = self.nud(t, self.bp(t))?;
        if self.next_is_eof() {
            return Ok(left);
        }
        t = self.tokens.peek().unwrap();
        while !self.next_is_eof() && rbp < self.bp(t) {
            let op = self.tokens.next().ok_or(err)?;
            left = self.led(left, op, self.bp(&op))?;

            if self.next_is_eof() {
                break;
            }
            t = self.tokens.peek().unwrap();
        }
        Ok(left)
    }
}

fn eval(node: Node) -> f64 {
    match node {
        Node::Number(n) => n,
        Node::Unary(op, node) => match op {
            Token::Minus => -eval(*node),
            Token::Plus => eval(*node),
            _ => 0_f64,
        },
        Node::Binary(left, op, right) => match op {
            Token::Plus => eval(*left) + eval(*right),
            Token::Minus => eval(*left) - eval(*right),
            Token::Times => eval(*left) * eval(*right),
            Token::Divide => eval(*left) / eval(*right),
            Token::Exponent => eval(*left).powf(eval(*right)),
            _ => 0_f64,
        },
    }
}

fn main() {
    let source = "1+2*3^2^1".to_owned();
    let tkns = lex(&source);

    let mut tkns_iter = tkns.iter().peekable();
    let mut parser = Pratt::new(&mut tkns_iter);
    let parse_result = parser.expr(0);
    if let Ok(node) = parse_result {
        println!("result={:?}", eval(node));
    }
}
