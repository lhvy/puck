mod ast; // Abstract Syntax Tree
mod eval;
mod hir; // High-level Intermediate Representation
mod lexer;
mod parser; // Creates a Concrete Syntax Tree
mod syntax;

use crate::eval::Evaluator;
use crate::parser::parse;
use mimalloc::MiMalloc;
use std::io::{self, Write};
use std::{env, fs};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> io::Result<()> {
    let mut args = env::args();
    match args.len() {
        1 => Repl::new().run()?,
        2 => {
            let contents = fs::read_to_string(args.nth(1).unwrap())?;

            let parse = parse(&contents, true);
            let root = ast::Root::cast(parse.syntax_node()).unwrap();
            let (items, db) = hir::lower(root);

            Evaluator::default().eval(&items, db);
        }
        _ => eprintln!("Usage: puck [filepath]"),
    }

    Ok(())
}

struct Repl {
    stdin: io::Stdin,
    stdout: io::Stdout,
    input: String,
    evaluator: Evaluator,
}

impl Repl {
    fn new() -> Self {
        Repl {
            stdin: io::stdin(),
            stdout: io::stdout(),
            input: String::new(),
            evaluator: Evaluator::default(),
        }
    }

    fn run(mut self) -> io::Result<()> {
        loop {
            write!(self.stdout, "→ ")?;
            self.stdout.flush()?;

            self.stdin.read_line(&mut self.input)?;

            let parse = parse(&self.input, false);
            for error in &parse.errors {
                println!("{}", error);
            }

            if parse.errors.is_empty() {
                let root = ast::Root::cast(parse.syntax_node()).unwrap();
                let (items, db) = hir::lower(root);

                self.evaluator.eval(&items, db);
            }

            self.input.clear();
        }
    }
}
