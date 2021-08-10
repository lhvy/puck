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

        if self.at(SyntaxKind::Character) {
            self.parse_character_def();
        }

        if self.at(SyntaxKind::LBracket) {
            self.parse_stage_direction();
        }

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
        assert!(self.at(SyntaxKind::Character));
        self.start_node(SyntaxKind::CharacterDef);
        self.bump();

        self.start_node(SyntaxKind::Comment);
        self.expect(SyntaxKind::Comma);
        while !self.at(SyntaxKind::Newline) && !self.at_eof() {
            self.skip();
        }
        self.bump_newline();
        self.finish_node();

        self.finish_node();
    }

    fn parse_stage_direction(&mut self) {
        assert!(self.at(SyntaxKind::LBracket));
        self.start_node(SyntaxKind::StageDirection);
        self.bump();

        self.expect(SyntaxKind::Enter);

        self.skip_ws();

        loop {
            self.expect(SyntaxKind::Character);
            self.skip_ws();

            if self.at(SyntaxKind::RBracket) {
                self.bump();
                break;
            }

            self.expect(SyntaxKind::And);
            self.skip_ws();
        }

        self.bump_newline();
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

    fn at_eof(&mut self) -> bool {
        self.peek().is_none()
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();
        self.builder
            .token(ShakespeareProgrammingLanguage::kind_to_raw(kind), text);
    }

    fn skip(&mut self) {
        let (_, text) = self.lexer.next().unwrap();
        self.builder.token(
            ShakespeareProgrammingLanguage::kind_to_raw(SyntaxKind::Skip),
            text,
        );
    }

    fn skip_ws(&mut self) {
        while self.at(SyntaxKind::Whitespace) {
            self.bump();
        }
    }

    fn bump_newline(&mut self) {
        if self.at_eof() {
            return;
        }

        self.expect(SyntaxKind::Newline)
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
    fn parse_character_def() {
        check(
            "Romeo, a test",
            expect![[r#"
Root@0..13
  CharacterDef@0..13
    Character@0..5 "Romeo"
    Comment@5..13
      Comma@5..6 ","
      Skip@6..7 " "
      Skip@7..8 "a"
      Skip@8..9 " "
      Skip@9..10 "t"
      Skip@10..11 "e"
      Skip@11..12 "s"
      Skip@12..13 "t""#]],
        )
    }

    #[test]
    fn parse_character_def_with_newline() {
        check(
            "Juliet, act\n",
            expect![[r#"
Root@0..12
  CharacterDef@0..12
    Character@0..6 "Juliet"
    Comment@6..12
      Comma@6..7 ","
      Skip@7..8 " "
      Skip@8..11 "act"
      Newline@11..12 "\n""#]],
        )
    }

    #[test]
    fn parse_enter_characters() {
        check(
            "[Enter Hamlet and Romeo]",
            expect![[r#"
Root@0..24
  StageDirection@0..24
    LBracket@0..1 "["
    Enter@1..6 "Enter"
    Whitespace@6..7 " "
    Character@7..13 "Hamlet"
    Whitespace@13..14 " "
    And@14..17 "and"
    Whitespace@17..18 " "
    Character@18..23 "Romeo"
    RBracket@23..24 "]""#]],
        )
    }

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]])
    }
}
