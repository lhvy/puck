mod lexer;
mod parser;
mod syntax;

use crate::parser::Parser;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = Parser::new(&input).parse();

        writeln!(stdout, "{}", parse.debug_tree())?;

        input.clear();
    }
}
