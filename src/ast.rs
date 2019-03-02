use crate::lexer::{Token, TokenKind};

#[derive(Debug, PartialEq)]
pub enum Node {
  Number(f64),
  Unary(Box<Token>, Box<Node>),
  Binary(Box<Node>, Box<Token>, Box<Node>),
  Application(String, Box<Node>),
}

pub fn eval(node: Node) -> Result<f64, String> {
  match node {
    Node::Number(n) => Ok(n),
    Node::Unary(op, node) => match op.kind {
      TokenKind::Minus => Ok(-eval(*node)?),
      TokenKind::Plus => Ok(eval(*node)?),
      _ => Err("Not a unary operator".to_owned()),
    },
    Node::Binary(left, op, right) => match op.kind {
      TokenKind::Plus => Ok(eval(*left)? + eval(*right)?),
      TokenKind::Minus => Ok(eval(*left)? - eval(*right)?),
      TokenKind::Times => Ok(eval(*left)? * eval(*right)?),
      TokenKind::Divide => Ok(eval(*left)? / eval(*right)?),
      TokenKind::Exponent => Ok(eval(*left)?.powf(eval(*right)?)),
      _ => Err("Not a binary operator".to_owned()),
    },
    Node::Application(func, arg) => match func.as_str() {
      "abs" => Ok(eval(*arg)?.abs()),
      "acos" => Ok(eval(*arg)?.acos()),
      "asin" => Ok(eval(*arg)?.asin()),
      "atan" => Ok(eval(*arg)?.atan()),
      "ceil" => Ok(eval(*arg)?.ceil()),
      "cos" => Ok(eval(*arg)?.cos()),
      "floor" => Ok(eval(*arg)?.floor()),
      "ln" => Ok(eval(*arg)?.ln()),
      "log" => Ok(eval(*arg)?.log10()),
      "log2" => Ok(eval(*arg)?.log2()),
      "sin" => Ok(eval(*arg)?.sin()),
      "tan" => Ok(eval(*arg)?.tan()),
      _ => Err(format!("Unknown function {:?}", func).to_owned()),
    },
  }
}

#[cfg(test)]
mod tests {
  use crate::ast;
  use crate::ast::Node;
  use crate::lexer::{Token, TokenKind};

  #[test]
  fn number() {
    assert_eq!(ast::eval(ast::Node::Number(3.14_f64)).unwrap(), 3.14_f64);
  }

  #[test]
  fn unary() {
    let num = Box::new(Node::Number(3.14_f64));
    assert_eq!(
      ast::eval(Node::Unary(
        Box::new(Token::new(TokenKind::Minus, "-".to_owned(), 0, 0, 0, 0)),
        num
      ))
      .unwrap(),
      -3.14_f64
    );
  }

  #[test]
  fn binary() {
    let _3 = Box::new(Node::Number(3_f64));
    let _4 = Box::new(Node::Number(4_f64));
    let times = Box::new(Token::new(TokenKind::Times, "*".to_owned(), 0, 0, 0, 0));
    assert_eq!(ast::eval(Node::Binary(_3, times, _4)).unwrap(), 12_f64);
  }

  #[test]
  fn application() {
    let pi = Box::new(Node::Number(std::f64::consts::PI));
    assert_eq!(
      ast::eval(Node::Application("cos".to_owned(), pi)).unwrap(),
      -1_f64
    );
  }
}
