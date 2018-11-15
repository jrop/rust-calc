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

fn lex(s: &String) -> Vec<Token> {
    let mut tkns = Vec::<Token>::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            _ws if c.is_whitespace() => {}
            n if n.is_digit(10) => {
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
                // check for '.1234...':
                match chars.peek().cloned() {
                    Some(c) if c == '.' => {
                        let mut fraction = 0f64;
                        let mut places = 0f64;
                        chars.next(); // skip '.'
                        peek = chars.peek().cloned();
                        while let Some(c) = peek {
                            if !c.is_digit(10) {
                                break;
                            }
                            chars.next();
                            let digit_value = c.to_string().parse::<f64>().unwrap();
                            fraction = fraction * 10f64 + digit_value;
                            places += 1f64;
                            peek = chars.peek().cloned();
                        }
                        number += fraction / 10f64.powf(places);
                    }
                    _ => {}
                }
                tkns.push(Token::Number(number));
            }
            '+' => tkns.push(Token::Plus),
            '-' => tkns.push(Token::Minus),
            '*' => tkns.push(Token::Times),
            '/' => tkns.push(Token::Divide),
            '^' => tkns.push(Token::Exponent),
            '(' => tkns.push(Token::LParen),
            ')' => tkns.push(Token::RParen),
            _ => panic!("Unexpected input: {}", c),
        }
    }
    tkns
}

fn bp(t: Token) -> usize {
    // TODO: check these binding powers:
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

fn expr(v: Vec<Token>) {

}

fn main() {
    let source = "12.34 + 2*3 - 1^2^3".to_owned();
    let tkns = lex(&source);
    println!("Hello, world! source={}; tkns={:?}", source, tkns);
}
