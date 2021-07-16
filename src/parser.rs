use crate::{
    lexer::SyntaxKind,
    syntax::{ShakespeareProgrammingLanguage, SyntaxNode},
};
use logos::Logos;
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub(crate) struct Parser<'a> {
    lexer: logos::Lexer<'a, SyntaxKind>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            lexer: SyntaxKind::lexer(input),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub(crate) fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);
        self.finish_node();

        Parse {
            green_node: self.builder.finish(),
        }
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder
            .start_node(ShakespeareProgrammingLanguage::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }
}

pub(crate) struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub(crate) fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let tree = format!("{:#?}", syntax_node);

        tree[0..tree.len() - 1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = Parser::new(input).parse();

        expected_tree.assert_eq(&parse.debug_tree());
    }

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]])
    }
}
