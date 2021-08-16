mod ast; // Abstract syntax tree
mod eval;
mod hir;
mod lexer;
mod parser;
mod syntax;

use crate::eval::Evaluator;
use crate::parser::Parser;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut input = String::new();

    let mut evaluator = Evaluator::default();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = Parser::new(&input).parse();
        let root = ast::Root::cast(parse.syntax_node()).unwrap();
        let (items, db) = hir::lower(root);

        evaluator.eval(&items, db);

        input.clear();
    }
}
