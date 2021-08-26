mod event;
mod grammar;
mod imp;
mod parse_error;
mod sink;
mod source;

use crate::lexer::Lexer;
use crate::syntax::SyntaxNode;
use imp::Parser;
use parse_error::ParseError;
use rowan::GreenNode;
use sink::Sink;

pub(crate) fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let parser = Parser::new(&tokens);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    sink.finish()
}
pub(crate) struct Parse {
    green_node: GreenNode,
    pub(crate) errors: Vec<ParseError>,
}

impl Parse {
    pub(crate) fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    #[cfg(test)]
    pub(crate) fn debug_tree(&self) -> String {
        let mut s = String::new();

        let tree = format!("{:#?}", self.syntax_node());

        s.push_str(&tree[0..tree.len() - 1]);

        for error in &self.errors {
            s.push_str(&format!("\n{}", error));
        }

        s
    }
}
