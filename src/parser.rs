mod event;
mod grammar;
mod imp;
mod sink;
mod source;

use crate::lexer::Lexer;
use crate::syntax::SyntaxNode;
use imp::Parser;
use rowan::GreenNode;
use sink::Sink;

pub(crate) fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let parser = Parser::new(&tokens);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    Parse {
        green_node: sink.finish(),
    }
}
pub(crate) struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub(crate) fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    #[cfg(test)]
    pub(crate) fn debug_tree(&self) -> String {
        let tree = format!("{:#?}", self.syntax_node());

        tree[0..tree.len() - 1].to_string()
    }
}
