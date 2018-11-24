#[derive(Clone, Copy, Debug, PartialEq)]
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

pub struct Lexer<'a> {
    position: usize,
    chars: std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'a>>>,
}
impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            position: 0,
            chars: source.chars().enumerate().peekable(),
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        match self.chars.peek() {
            Some((i, c)) => {
                self.position = i.to_owned();
                Some(c)
            }
            None => None,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        match self.chars.next() {
            Some((i, c)) => {
                self.position = i;
                Some(c)
            }
            None => None,
        }
    }

    fn eat_digits_as_f64(&mut self) -> Option<f64> {
        let n = self.next_char()?;
        let mut number = n.to_string().parse::<f64>().unwrap();
        let mut peek = self.peek_char().cloned();
        while let Some(c) = peek {
            if !c.is_digit(10) {
                break;
            }
            self.next_char();
            let digit_value = c.to_string().parse::<f64>().unwrap();
            number = number * 10f64 + digit_value;

            peek = self.peek_char().cloned();
        }
        Some(number)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        // Eat whitespace
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }

        let _start = self.position;
        let token = match self.peek_char() {
            Some(c) => match c {
                n if n.is_digit(10) => {
                    let mut number = self.eat_digits_as_f64().unwrap();
                    // check for decimal portion:
                    match self.peek_char().cloned() {
                        Some(c) if c == '.' => {
                            self.next_char();
                            let fraction = self.eat_digits_as_f64().unwrap();
                            let places = fraction.log10().ceil();
                            number += fraction / 10f64.powf(places);
                        }
                        _ => {}
                    }
                    Some(Token::Number(number))
                }
                '+' => {
                    self.next_char();
                    Some(Token::Plus)
                }
                '-' => {
                    self.next_char();
                    Some(Token::Minus)
                }
                '*' => {
                    self.next_char();
                    Some(Token::Times)
                }
                '/' => {
                    self.next_char();
                    Some(Token::Divide)
                }
                '^' => {
                    self.next_char();
                    Some(Token::Exponent)
                }
                '(' => {
                    self.next_char();
                    Some(Token::LParen)
                }
                ')' => {
                    self.next_char();
                    Some(Token::RParen)
                }
                _ => panic!("Unexpected input: {}", c),
            },
            None => None,
        };
        let _end = self.position;
        token
    }
}

#[cfg(test)]
mod tests {
    use lexer::{Lexer, Token};

    fn lex(s: &str) -> Vec<Token> {
        let mut tkns: Vec<Token> = vec![];
        for tkn in Lexer::new(s) {
            tkns.push(tkn)
        }
        tkns
    }

    #[test]
    fn numbers() {
        let tkns = lex("2 3.14");
        assert_eq!(tkns.len(), 2);
        assert_eq!(tkns[0], Token::Number(2_f64));
        assert_eq!(tkns[1], Token::Number(3.14_f64));
    }

    #[test]
    fn operators() {
        let tkns = lex("+-*/^()");
        assert_eq!(tkns.len(), 7);
        assert_eq!(tkns[0], Token::Plus);
        assert_eq!(tkns[1], Token::Minus);
        assert_eq!(tkns[2], Token::Times);
        assert_eq!(tkns[3], Token::Divide);
        assert_eq!(tkns[4], Token::Exponent);
        assert_eq!(tkns[5], Token::LParen);
        assert_eq!(tkns[6], Token::RParen);
    }

    #[test]
    fn eof() {
        let tkns = lex(&"".to_owned());
        assert_eq!(tkns.len(), 0);
    }
}
