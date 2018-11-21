use lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    Number(f64),
    Unary(&'a Token, Box<Node<'a>>),
    Binary(Box<Node<'a>>, Token, Box<Node<'a>>),
}

pub fn eval(node: Node) -> f64 {
    match node {
        Node::Number(n) => n,
        Node::Unary(op, node) => match op {
            Token::Minus => -eval(*node),
            Token::Plus => eval(*node),
            _ => 0_f64,
        },
        Node::Binary(left, op, right) => match op {
            Token::Plus => eval(*left) + eval(*right),
            Token::Minus => eval(*left) - eval(*right),
            Token::Times => eval(*left) * eval(*right),
            Token::Divide => eval(*left) / eval(*right),
            Token::Exponent => eval(*left).powf(eval(*right)),
            _ => 0_f64,
        },
    }
}

#[cfg(test)]
mod tests {
    use ast;
    use ast::Node;
    use lexer::Token;

    #[test]
    fn number() {
        assert_eq!(ast::eval(ast::Node::Number(3.14_f64)), 3.14_f64);
    }

    #[test]
    fn unary() {
        let num = Box::new(Node::Number(3.14_f64));
        let minus = Token::Minus;
        assert_eq!(ast::eval(Node::Unary(&minus, num)), -3.14_f64);
    }

    #[test]
    fn binary() {
        let _3 = Box::new(Node::Number(3_f64));
        let _4 = Box::new(Node::Number(4_f64));
        let times = Token::Times;
        assert_eq!(ast::eval(Node::Binary(_3, times, _4)), 12_f64);
    }
}
