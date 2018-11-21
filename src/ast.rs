use lexer::Token;

#[derive(Debug)]
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
