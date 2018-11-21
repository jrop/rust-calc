#[derive(Clone, Debug)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Times,
    Divide,
    Exponent,
    LParen,
    RParen,
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

pub fn lex(s: &String) -> Vec<Token> {
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
