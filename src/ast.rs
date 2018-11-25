use lexer::{Token, TokenKind};

#[derive(Debug, PartialEq)]
pub enum Node {
    Number(f64),
    Unary(Box<Token>, Box<Node>),
    Binary(Box<Node>, Box<Token>, Box<Node>),
}

pub fn eval(node: Node) -> f64 {
    match node {
        Node::Number(n) => n,
        Node::Unary(op, node) => match op.kind {
            TokenKind::Minus => -eval(*node),
            TokenKind::Plus => eval(*node),
            _ => 0_f64,
        },
        Node::Binary(left, op, right) => match op.kind {
            TokenKind::Plus => eval(*left) + eval(*right),
            TokenKind::Minus => eval(*left) - eval(*right),
            TokenKind::Times => eval(*left) * eval(*right),
            TokenKind::Divide => eval(*left) / eval(*right),
            TokenKind::Exponent => eval(*left).powf(eval(*right)),
            _ => 0_f64,
        },
    }
}

#[cfg(test)]
mod tests {
    use ast;
    use ast::Node;
    use lexer::{Token, TokenKind};

    #[test]
    fn number() {
        assert_eq!(ast::eval(ast::Node::Number(3.14_f64)), 3.14_f64);
    }

    #[test]
    fn unary() {
        let num = Box::new(Node::Number(3.14_f64));
        assert_eq!(
            ast::eval(Node::Unary(
                Box::new(Token::new(TokenKind::Minus, "-".to_owned(), 0, 0)),
                num
            )),
            -3.14_f64
        );
    }

    #[test]
    fn binary() {
        let _3 = Box::new(Node::Number(3_f64));
        let _4 = Box::new(Node::Number(4_f64));
        let times = Box::new(Token::new(TokenKind::Times, "*".to_owned(), 0, 0));
        assert_eq!(ast::eval(Node::Binary(_3, times, _4)), 12_f64);
    }
}
