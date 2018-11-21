#![feature(nll)]

mod ast;
mod lexer;
mod pratt;

use lexer::lex;
use pratt::Pratt;

fn main() {
    let source = "1+2*3^2^1".to_owned();
    let tkns = lex(&source);

    let mut tkns_iter = tkns.iter().peekable();
    let mut parser = Pratt::new(&mut tkns_iter);
    let parse_result = parser.expr(0);
    if let Ok(node) = parse_result {
        println!("result={:?}", ast::eval(node));
    }
}
