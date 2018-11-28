#![feature(nll)]
use std::io::Write;

mod ast;
mod lexer;
mod parser;

fn prompt_and_read_line() -> std::io::Result<String> {
  let mut line: String = String::new();
  print!("> ");
  std::io::stdout().flush()?;
  std::io::stdin().read_line(&mut line)?;
  Ok(line)
}

fn main() {
  while let Ok(line) = prompt_and_read_line() {
    let mut parser = parser::Parser::new(lexer::Lexer::new(&line));
    match parser.parse().and_then(|node| ast::eval(node)) {
      Ok(result) => println!("result={:?}", result),
      Err(reason) => println!("Error: {}", reason),
    }
  }
}
