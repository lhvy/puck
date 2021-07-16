use crate::{
    lexer::{Lexer, SyntaxKind},
    syntax::{ShakespeareProgrammingLanguage, SyntaxNode},
};
use rowan::{GreenNode, GreenNodeBuilder, Language};
use std::iter::Peekable;

pub(crate) struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub(crate) fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);

        self.parse_character_def();

        self.finish_node();

        // loop {
        //     let syntax_kind = match self.lexer.next() {
        //         None => break,
        //         Some(syntax_kind) => syntax_kind,
        //     };
        //     dbg!(syntax_kind, self.lexer.slice());
        // }

        Parse {
            green_node: self.builder.finish(),
        }
    }

    fn parse_character_def(&mut self) {
        assert!(self.at(SyntaxKind::CharacterDef));
        self.start_node(SyntaxKind::CharacterDef);

        self.expect(SyntaxKind::Character);

        self.start_node(SyntaxKind::Comment);
        self.expect(SyntaxKind::Comma);
        while !self.at(SyntaxKind::Newline) {
            self.bump();
        }
        self.finish_node();

        self.finish_node();
    }

    fn expect(&mut self, syntax_kind: SyntaxKind) {
        if self.at(syntax_kind) {
            self.bump();
        } else {
            panic!("Expected {:?} but got {:?}", syntax_kind, self.peek());
        }
    }

    fn at(&mut self, syntax_kind: SyntaxKind) -> bool {
        self.peek() == Some(syntax_kind)
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();
        self.builder
            .token(ShakespeareProgrammingLanguage::kind_to_raw(kind), text);
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
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
