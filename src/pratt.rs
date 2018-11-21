use ast::Node;
use lexer::Token;

pub struct Pratt<'a> {
    tokens: &'a mut std::iter::Peekable<std::slice::Iter<'a, Token>>,
}
impl<'a> Pratt<'a> {
    pub fn new(tokens: &'a mut std::iter::Peekable<std::slice::Iter<'a, Token>>) -> Pratt<'a> {
        Pratt { tokens }
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

    pub fn nud(&mut self, t: &'a Token, _bp: usize) -> Result<Node<'a>, String> {
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

    pub fn led(&mut self, left: Node<'a>, op: &Token, bp: usize) -> Result<Node<'a>, String> {
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
    pub fn expr(&mut self, rbp: usize) -> Result<Node<'a>, String> {
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
