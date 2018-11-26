use ast::Node;
use lexer::{Lexer, Token, TokenKind};

pub struct Parser<'a> {
    lexer: std::iter::Peekable<Lexer<'a>>,
}
impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    pub fn bp(&self, t: &Token) -> usize {
        match t.kind {
            TokenKind::RParen => 0,
            TokenKind::Plus => 10,
            TokenKind::Minus => 10,
            TokenKind::Times => 20,
            TokenKind::Divide => 20,
            TokenKind::Exponent => 30,
            TokenKind::LParen => 40,
            _ => 100,
        }
    }

    pub fn nud(&mut self, t: Box<Token>, _bp: usize) -> Result<Node, String> {
        match t.kind {
            TokenKind::Number => Ok(Node::Number(t.value.parse::<f64>().unwrap())),
            TokenKind::Plus | TokenKind::Minus => {
                let right = self.expr(0)?;
                Ok(Node::Unary(t, Box::new(right)))
            }
            TokenKind::LParen => {
                let right = self.expr(0)?;
                match self.lexer.next() {
                    Some(ref t) if t.kind == TokenKind::RParen => Ok(right),
                    _ => Err("Expected ')'".to_owned()),
                }
            }
            _ => Err(format!("Unexpected token in NUD context: {:?}", t).to_owned()),
        }
    }

    pub fn led(&mut self, left: Node, op: Box<Token>, bp: usize) -> Result<Node, String> {
        match op.kind {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Times | TokenKind::Divide => {
                let right = self.expr(bp)?;
                Ok(Node::Binary(Box::new(left), op, Box::new(right)))
            }
            TokenKind::Exponent => {
                let right = self.expr(bp - 1)?;
                Ok(Node::Binary(Box::new(left), op, Box::new(right)))
            },
            _ => Err(format!(
                "Unexpected token in LED context: {:?} (left={:?})",
                op, left
            )
            .to_owned()),
        }
    }

    pub fn expr(&mut self, rbp: usize) -> Result<Node, String> {
        let err = "Undexpected EOF";
        let first_t = self.lexer.next().ok_or(err)?;
        let first_t_bp = self.bp(&first_t);

        let mut left = self.nud(first_t, first_t_bp)?;
        if self.lexer.peek().is_none() {
            return Ok(left);
        }

        let mut peeked = self.lexer.peek().cloned();
        while !peeked.is_none() && rbp < self.bp(&peeked.unwrap()) {
            let op = self.lexer.next().ok_or(err)?;
            let op_bp = self.bp(&op);
            left = self.led(left, op, op_bp)?;

            if self.lexer.peek().is_none() {
                break;
            }
            peeked = self.lexer.peek().cloned();
        }
        Ok(left)
    }
}

#[cfg(test)]
mod tests {
    use ast::Node;
    use lexer::{Lexer, Token, TokenKind};
    use parser::Parser;

    #[test]
    fn number() {
        let ast = Parser::new(Lexer::new("1")).expr(0).unwrap();
        assert_eq!(ast, Node::Number(1_f64));
    }

    #[test]
    fn plus_times() {
        let ast = Parser::new(Lexer::new("1+2*3")).expr(0).unwrap();
        assert_eq!(
            ast,
            Node::Binary(
                Box::new(Node::Number(1_f64)),
                Box::new(Token::new(TokenKind::Plus, "+".to_owned(), 1, 2)),
                Box::new(Node::Binary(
                    Box::new(Node::Number(2_f64)),
                    Box::new(Token::new(TokenKind::Times, "*".to_owned(), 3, 4)),
                    Box::new(Node::Number(3_f64))
                ))
            )
        );
    }

    #[test]
    fn times_plus() {
        let ast = Parser::new(Lexer::new("1*2+3")).expr(0).unwrap();
        assert_eq!(
            ast,
            Node::Binary(
                Box::new(Node::Binary(
                    Box::new(Node::Number(1_f64)),
                    Box::new(Token::new(TokenKind::Times, "*".to_owned(), 1, 2)),
                    Box::new(Node::Number(2_f64))
                )),
                Box::new(Token::new(TokenKind::Plus, "+".to_owned(), 3, 4)),
                Box::new(Node::Number(3_f64)),
            )
        );
    }

    #[test]
    fn parens() {
        let ast = Parser::new(Lexer::new("1*(2+3)")).expr(0).unwrap();
        assert_eq!(
            ast,
            Node::Binary(
                Box::new(Node::Number(1_f64)),
                Box::new(Token::new(TokenKind::Times, "*".to_owned(), 1, 2)),
                Box::new(Node::Binary(
                    Box::new(Node::Number(2_f64)),
                    Box::new(Token::new(TokenKind::Plus, "+".to_owned(), 4, 5)),
                    Box::new(Node::Number(3_f64)),
                )),
            )
        );
    }

    #[test]
    fn rassoc() {
        let ast = Parser::new(Lexer::new("1^2^3")).expr(0).unwrap();
        assert_eq!(
            ast,
            Node::Binary(
                Box::new(Node::Number(1_f64)),
                Box::new(Token::new(TokenKind::Exponent, "^".to_owned(), 1, 2)),
                Box::new(Node::Binary(
                    Box::new(Node::Number(2_f64)),
                    Box::new(Token::new(TokenKind::Exponent, "^".to_owned(), 3, 4)),
                    Box::new(Node::Number(3_f64)),
                )),
            )
        );
    }
}
