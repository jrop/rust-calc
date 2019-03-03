mod ast;
mod lexer;
mod parser;

fn eval(s: &str) -> Result<f64, String> {
  let l = lexer::Lexer::new(s);
  let mut p = parser::Parser::new(l);
  let tree = p.parse()?;
  ast::eval(tree)
}

fn main() {
  let matches = clap::App::new("rust-calc")
    .version(clap::crate_version!())
    .author(clap::crate_authors!())
    .about("A simple calculator, written in Rust.")
    .arg(
      clap::Arg::with_name("expr")
        .short("e")
        .long("expression")
        .help("An expression to evaluate")
        .takes_value(true),
    )
    .get_matches();

  match matches.value_of("expr") {
    Some(expr) => match eval(expr) {
      // print result and exit:
      Ok(x) => println!("{}", x),
      Err(msg) => eprintln!("error: {}", msg),
    },
    None => {
      // fire up REPL:
      let mut rl = rustyline::Editor::<()>::new();
      loop {
        let readline = rl.readline(">> ");
        match readline {
          Ok(line) => match eval(line.as_str()) {
            Ok(x) => println!("result={}", x),
            Err(msg) => eprintln!("{}", msg),
          },
          Err(rustyline::error::ReadlineError::Interrupted) => {
            break;
          }
          Err(rustyline::error::ReadlineError::Eof) => {
            break;
          }
          Err(err) => {
            println!("error: {:?}", err);
            break;
          }
        }
      }
    }
  }
}
