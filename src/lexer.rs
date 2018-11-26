#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Identifier,
    Number,
    Plus,
    Minus,
    Times,
    Divide,
    Exponent,
    LParen,
    RParen,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub start: usize,
    pub end: usize,
}
impl Token {
    pub fn new(kind: TokenKind, value: String, start: usize, end: usize) -> Token {
        Token {
            kind,
            value,
            start,
            end,
        }
    }
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
            Some((_, c)) => Some(c),
            None => None,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        match self.chars.next() {
            Some((_, c)) => {
                self.position += 1;
                Some(c)
            }
            None => None,
        }
    }

    fn eat_digits(&mut self) -> Option<String> {
        let n = self.next_char()?;
        let mut number = n.to_string();
        let mut peek = self.peek_char().cloned();
        while let Some(c) = peek {
            if !c.is_digit(10) {
                break;
            }
            self.next_char();
            number = format!("{}{}", number, c);

            peek = self.peek_char().cloned();
        }
        Some(number)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Box<Token>;
    fn next(&mut self) -> Option<Box<Token>> {
        // Eat whitespace
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }

        let start = self.position;
        let token = match self.peek_char() {
            Some(c) => match c {
                n if n.is_digit(10) => {
                    let mut number = self.eat_digits().unwrap();
                    // check for decimal portion:
                    match self.peek_char().cloned() {
                        Some(c) if c == '.' => {
                            self.next_char();
                            number = format!("{}.{}", number, self.eat_digits().unwrap());
                        }
                        _ => {}
                    }
                    Some(Token::new(TokenKind::Number, number, start, self.position))
                }
                '+' => {
                    self.next_char();
                    Some(Token::new(
                        TokenKind::Plus,
                        "+".to_owned(),
                        start,
                        self.position,
                    ))
                }
                '-' => {
                    self.next_char();
                    Some(Token::new(
                        TokenKind::Minus,
                        "-".to_owned(),
                        start,
                        self.position,
                    ))
                }
                '*' => {
                    self.next_char();
                    Some(Token::new(
                        TokenKind::Times,
                        "*".to_owned(),
                        start,
                        self.position,
                    ))
                }
                '/' => {
                    self.next_char();
                    Some(Token::new(
                        TokenKind::Divide,
                        "/".to_owned(),
                        start,
                        self.position,
                    ))
                }
                '^' => {
                    self.next_char();
                    Some(Token::new(
                        TokenKind::Exponent,
                        "^".to_owned(),
                        start,
                        self.position,
                    ))
                }
                '(' => {
                    self.next_char();
                    Some(Token::new(
                        TokenKind::LParen,
                        "(".to_owned(),
                        start,
                        self.position,
                    ))
                }
                ')' => {
                    self.next_char();
                    Some(Token::new(
                        TokenKind::RParen,
                        ")".to_owned(),
                        start,
                        self.position,
                    ))
                }
                c if c.is_alphabetic() => {
                    let mut id = String::from(self.next_char().unwrap().to_string());
                    while let Some(c) = self.peek_char() {
                        if !c.is_alphabetic() {
                            break;
                        }
                        id += self.next_char().unwrap().to_string().as_str();
                    }
                    Some(Token::new(TokenKind::Identifier, id, start, self.position))
                }
                _ => panic!("Unexpected input: {}", c),
            },
            None => None,
        };
        let _end = self.position;

        // Box it up:
        match token {
            Some(t) => Some(Box::new(t)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use lexer::{Lexer, Token, TokenKind};

    fn lex(s: &str) -> Vec<Box<Token>> {
        let mut tkns: Vec<Box<Token>> = vec![];
        for tkn in Lexer::new(s) {
            tkns.push(tkn)
        }
        tkns
    }

    #[test]
    fn numbers() {
        let tkns = lex("2 3.14");
        assert_eq!(tkns.len(), 2);
        assert_eq!(
            *tkns[0],
            Token::new(TokenKind::Number, "2".to_owned(), 0, 1)
        );
        assert_eq!(
            *tkns[1],
            Token::new(TokenKind::Number, "3.14".to_owned(), 2, 6)
        );
    }

    #[test]
    fn operators() {
        let tkns = lex("+-*/^()");
        assert_eq!(tkns.len(), 7);
        assert_eq!(*tkns[0], Token::new(TokenKind::Plus, "+".to_owned(), 0, 1));
        assert_eq!(*tkns[1], Token::new(TokenKind::Minus, "-".to_owned(), 1, 2));
        assert_eq!(*tkns[2], Token::new(TokenKind::Times, "*".to_owned(), 2, 3));
        assert_eq!(
            *tkns[3],
            Token::new(TokenKind::Divide, "/".to_owned(), 3, 4)
        );
        assert_eq!(
            *tkns[4],
            Token::new(TokenKind::Exponent, "^".to_owned(), 4, 5)
        );
        assert_eq!(
            *tkns[5],
            Token::new(TokenKind::LParen, "(".to_owned(), 5, 6)
        );
        assert_eq!(
            *tkns[6],
            Token::new(TokenKind::RParen, ")".to_owned(), 6, 7)
        );
    }

    #[test]
    fn identifier() {
        let tkns = lex("abc");
        assert_eq!(tkns.len(), 1);
        assert_eq!(
            *tkns[0],
            Token::new(TokenKind::Identifier, "abc".to_owned(), 0, 3)
        );
    }

    #[test]
    fn eof() {
        let tkns = lex("");
        assert_eq!(tkns.len(), 0);
    }
}
