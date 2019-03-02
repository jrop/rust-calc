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
  pub line: usize,
  pub column: usize,
}
impl Token {
  pub fn new(
    kind: TokenKind,
    value: String,
    start: usize,
    end: usize,
    line: usize,
    column: usize,
  ) -> Token {
    Token {
      kind,
      value,
      start,
      end,
      line,
      column,
    }
  }

  pub fn error(&self) -> String {
    format!(
      "Unexpected token \"{}\" ({:?}) ({}:{})",
      self.value, self.kind, self.line, self.column
    )
    .to_owned()
  }
}

pub struct Lexer<'a> {
  position: usize,
  peeked: Option<Box<Token>>,
  chars: std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'a>>>,
  current_line: usize,
  current_column: usize,
}
impl<'a> Lexer<'a> {
  pub fn new(source: &'a str) -> Lexer<'a> {
    Lexer {
      position: 0,
      peeked: None,
      chars: source.chars().enumerate().peekable(),
      current_line: 1,
      current_column: 1,
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
        match c {
          '\n' => {
            self.current_line += 1;
            self.current_column = 1;
          }
          _ => {
            self.current_column += 1;
          }
        }
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

  pub fn expect(&mut self, kind: TokenKind) -> Result<Box<Token>, String> {
    let tkn = self.next();
    match tkn {
      Some(t) => {
        if t.kind == kind {
          Ok(t)
        } else {
          Err(t.error())
        }
      }
      _ => Err("Unexpected EOF".to_owned()),
    }
  }

  pub fn peek(&mut self) -> &Option<Box<Token>> {
    if self.peeked.is_some() {
      return &self.peeked;
    }

    // Eat whitespace
    while let Some(c) = self.peek_char() {
      if c.is_whitespace() {
        self.next_char();
      } else {
        break;
      }
    }

    let start = self.position;
    let line = self.current_line;
    let column = self.current_column;
    let peeked_char = self.peek_char().cloned();
    let token = match peeked_char {
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
          Some(Token::new(
            TokenKind::Number,
            number,
            start,
            self.position,
            line,
            column,
          ))
        }
        '+' => {
          self.next_char();
          Some(Token::new(
            TokenKind::Plus,
            "+".to_owned(),
            start,
            self.position,
            line,
            column,
          ))
        }
        '-' => {
          self.next_char();
          Some(Token::new(
            TokenKind::Minus,
            "-".to_owned(),
            start,
            self.position,
            line,
            column,
          ))
        }
        '*' => {
          self.next_char();
          Some(Token::new(
            TokenKind::Times,
            "*".to_owned(),
            start,
            self.position,
            line,
            column,
          ))
        }
        '/' => {
          self.next_char();
          Some(Token::new(
            TokenKind::Divide,
            "/".to_owned(),
            start,
            self.position,
            line,
            column,
          ))
        }
        '^' => {
          self.next_char();
          Some(Token::new(
            TokenKind::Exponent,
            "^".to_owned(),
            start,
            self.position,
            line,
            column,
          ))
        }
        '(' => {
          self.next_char();
          Some(Token::new(
            TokenKind::LParen,
            "(".to_owned(),
            start,
            self.position,
            line,
            column,
          ))
        }
        ')' => {
          self.next_char();
          Some(Token::new(
            TokenKind::RParen,
            ")".to_owned(),
            start,
            self.position,
            line,
            column,
          ))
        }
        c if c.is_alphabetic() => {
          let mut id = self.next_char().unwrap().to_string();
          while let Some(c) = self.peek_char() {
            if !c.is_alphabetic() {
              break;
            }
            id += self.next_char().unwrap().to_string().as_str();
          }
          Some(Token::new(
            TokenKind::Identifier,
            id,
            start,
            self.position,
            line,
            column,
          ))
        }
        _ => panic!(
          "Unexpected input: {} ({}:{})",
          c, self.current_line, self.current_column
        ),
      },
      None => None,
    };
    let _end = self.position;

    // Box it up:
    self.peeked = match token {
      Some(t) => Some(Box::new(t)),
      None => None,
    };
    &self.peeked
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Box<Token>;
  fn next(&mut self) -> Option<Box<Token>> {
    if self.peeked.is_none() {
      self.peek();
    }
    let tkn = self.peeked.as_ref().cloned();
    self.peeked = None;
    tkn
  }
}

#[cfg(test)]
mod tests {
  use crate::lexer::{Lexer, Token, TokenKind};

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
      Token::new(TokenKind::Number, "2".to_owned(), 0, 1, 1, 1)
    );
    assert_eq!(
      *tkns[1],
      Token::new(TokenKind::Number, "3.14".to_owned(), 2, 6, 1, 3)
    );
  }

  #[test]
  fn operators() {
    let tkns = lex("+-*/^()");
    assert_eq!(tkns.len(), 7);
    assert_eq!(
      *tkns[0],
      Token::new(TokenKind::Plus, "+".to_owned(), 0, 1, 1, 1)
    );
    assert_eq!(
      *tkns[1],
      Token::new(TokenKind::Minus, "-".to_owned(), 1, 2, 1, 2)
    );
    assert_eq!(
      *tkns[2],
      Token::new(TokenKind::Times, "*".to_owned(), 2, 3, 1, 3)
    );
    assert_eq!(
      *tkns[3],
      Token::new(TokenKind::Divide, "/".to_owned(), 3, 4, 1, 4)
    );
    assert_eq!(
      *tkns[4],
      Token::new(TokenKind::Exponent, "^".to_owned(), 4, 5, 1, 5)
    );
    assert_eq!(
      *tkns[5],
      Token::new(TokenKind::LParen, "(".to_owned(), 5, 6, 1, 6)
    );
    assert_eq!(
      *tkns[6],
      Token::new(TokenKind::RParen, ")".to_owned(), 6, 7, 1, 7)
    );
  }

  #[test]
  fn identifier() {
    let tkns = lex("abc");
    assert_eq!(tkns.len(), 1);
    assert_eq!(
      *tkns[0],
      Token::new(TokenKind::Identifier, "abc".to_owned(), 0, 3, 1, 1)
    );
  }

  #[test]
  fn eof() {
    let tkns = lex("");
    assert_eq!(tkns.len(), 0);
  }
}
