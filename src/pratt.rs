use ast::Node;
use lexer::{Lexer, Token};

pub struct Pratt<'a> {
    lexer: std::iter::Peekable<Lexer<'a>>,
}
impl<'a> Pratt<'a> {
    pub fn new(lexer: Lexer<'a>) -> Pratt<'a> {
        Pratt {
            lexer: lexer.peekable(),
        }
    }

    pub fn bp(&self, t: &Token) -> usize {
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

    pub fn nud(&mut self, t: &Token, _bp: usize) -> Result<Node, String> {
        match t {
            Token::Number(n) => Ok(Node::Number(*n)),
            Token::Plus | Token::Minus => {
                let right = self.expr(0)?;
                Ok(Node::Unary(t.clone(), Box::new(right)))
            }
            Token::LParen => {
                let right = self.expr(0)?;
                match self.lexer.next() {
                    Some(Token::RParen) => Ok(right),
                    _ => Err("Expected ')'".to_owned()),
                }
            }
            _ => Err(format!("Unexpected token in NUD context: {:?}", t).to_owned()),
        }
    }

    pub fn led(&mut self, left: Node, op: &Token, bp: usize) -> Result<Node, String> {
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
        let peeked = self.lexer.peek();
        peeked.is_none()
    }
    pub fn expr(&mut self, rbp: usize) -> Result<Node, String> {
        let err = "Undexpected EOF";
        let mut t = self.lexer.next().ok_or(err)?;

        let mut left = self.nud(&t, self.bp(&t))?;
        if self.next_is_eof() {
            return Ok(left);
        }
        t = self.lexer.peek().unwrap().to_owned();
        while !self.next_is_eof() && rbp < self.bp(&t) {
            let op = self.lexer.next().ok_or(err)?;
            left = self.led(left, &op, self.bp(&op))?;

            if self.next_is_eof() {
                break;
            }
            t = self.lexer.peek().unwrap().to_owned();
        }
        Ok(left)
    }
}

#[cfg(test)]
mod tests {
    use ast::Node;
    use lexer::{Lexer, Token};
    use pratt::Pratt;

    #[test]
    fn number() {
        let ast = Pratt::new(Lexer::new("1")).expr(0).unwrap();
        assert_eq!(ast, Node::Number(1_f64));
    }

    #[test]
    fn plus_times() {
        let ast = Pratt::new(Lexer::new("1+2*3")).expr(0).unwrap();
        assert_eq!(
            ast,
            Node::Binary(
                Box::new(Node::Number(1_f64)),
                Token::Plus,
                Box::new(Node::Binary(
                    Box::new(Node::Number(2_f64)),
                    Token::Times,
                    Box::new(Node::Number(3_f64))
                ))
            )
        );
    }

    #[test]
    fn times_plus() {
        let ast = Pratt::new(Lexer::new("1*2+3")).expr(0).unwrap();
        assert_eq!(
            ast,
            Node::Binary(
                Box::new(Node::Binary(
                    Box::new(Node::Number(1_f64)),
                    Token::Times,
                    Box::new(Node::Number(2_f64))
                )),
                Token::Plus,
                Box::new(Node::Number(3_f64)),
            )
        );
    }

    #[test]
    fn parens() {
        let ast = Pratt::new(Lexer::new("1*(2+3)")).expr(0).unwrap();
        assert_eq!(
            ast,
            Node::Binary(
                Box::new(Node::Number(1_f64)),
                Token::Times,
                Box::new(Node::Binary(
                    Box::new(Node::Number(2_f64)),
                    Token::Plus,
                    Box::new(Node::Number(3_f64)),
                )),
            )
        );
    }

    #[test]
    fn rassoc() {
        let ast = Pratt::new(Lexer::new("1^2^3")).expr(0).unwrap();
        assert_eq!(
            ast,
            Node::Binary(
                Box::new(Node::Number(1_f64)),
                Token::Exponent,
                Box::new(Node::Binary(
                    Box::new(Node::Number(2_f64)),
                    Token::Exponent,
                    Box::new(Node::Number(3_f64)),
                )),
            )
        );
    }
}
