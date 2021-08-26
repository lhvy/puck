mod event;
mod sink;
mod source;

use crate::{
    lexer::{Lexer, SyntaxKind, Token},
    syntax::SyntaxNode,
};
use event::Event;
use rowan::GreenNode;
use sink::Sink;
use source::Source;

pub(crate) fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let parser = Parser::new(&tokens);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    Parse {
        green_node: sink.finish(),
    }
}

struct Parser<'tokens, 'input> {
    source: Source<'tokens, 'input>,
    events: Vec<Event<'input>>,
}

impl<'tokens, 'input> Parser<'tokens, 'input> {
    pub(crate) fn new(tokens: &'tokens [Token<'input>]) -> Self {
        Self {
            source: Source::new(tokens),
            events: Vec::new(),
        }
    }

    pub(crate) fn parse(mut self) -> Vec<Event<'input>> {
        self.start_node(SyntaxKind::Root);

        loop {
            if self.at(SyntaxKind::Character) {
                match self.lookahead(1) {
                    Some(SyntaxKind::Colon) => self.parse_dialog(),
                    Some(SyntaxKind::Comma) => self.parse_character_def(),
                    _ => panic!(),
                }
            }

            if self.at(SyntaxKind::LBracket) {
                self.parse_stage_direction();
            }

            self.bump_newline();
            if self.at_eof() {
                break;
            }
        }

        self.finish_node();

        self.events
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
        self.finish_node();

        self.finish_node();
    }

    fn parse_dialog(&mut self) {
        assert!(self.at(SyntaxKind::Character));
        self.start_node(SyntaxKind::Dialog);
        self.bump();

        self.expect(SyntaxKind::Colon);

        while !self.at(SyntaxKind::Newline) && !self.at_eof() {
            self.parse_sentence();
        }

        self.finish_node();
    }

    fn parse_sentence(&mut self) {
        match self.peek() {
            Some(SyntaxKind::SecondPerson) => self.parse_statement(),
            Some(SyntaxKind::Open) => self.parse_int_output(),
            Some(SyntaxKind::Speak) => self.parse_char_output(),
            _ => panic!(),
        }
    }

    fn parse_statement(&mut self) {
        assert!(self.at(SyntaxKind::SecondPerson));
        self.start_node(SyntaxKind::Statement);
        self.bump();

        self.expect(SyntaxKind::Be);

        self.parse_expr();
        self.expect(SyntaxKind::Period);

        self.finish_node();
    }

    fn parse_expr(&mut self) {
        if self.at(SyntaxKind::Nothing) {
            self.start_node(SyntaxKind::NothingExpr);
            self.bump();
            self.finish_node();
            return;
        }

        if self.at(SyntaxKind::Period) {
            return;
        }

        let checkpoint = self.checkpoint();

        self.expect(SyntaxKind::Article);

        match self.peek() {
            Some(
                SyntaxKind::PositiveAdjective
                | SyntaxKind::NegativeAdjective
                | SyntaxKind::NeutralAdjective
                | SyntaxKind::PositiveNoun
                | SyntaxKind::NegativeNoun
                | SyntaxKind::NeutralNoun,
            ) => self.parse_noun_expr(checkpoint),
            Some(
                SyntaxKind::Difference
                | SyntaxKind::Product
                | SyntaxKind::Quotient
                | SyntaxKind::Remainder
                | SyntaxKind::Sum,
            ) => self.parse_bin_expr(checkpoint),
            _ => panic!(),
        }
    }

    fn parse_noun_expr(&mut self, checkpoint: usize) {
        self.start_node_at(checkpoint, SyntaxKind::NounExpr);

        loop {
            if matches!(
                self.peek(),
                Some(SyntaxKind::PositiveNoun | SyntaxKind::NegativeNoun | SyntaxKind::NeutralNoun)
            ) {
                self.bump();
                break;
            }

            match self.peek() {
                Some(
                    SyntaxKind::PositiveAdjective
                    | SyntaxKind::NegativeAdjective
                    | SyntaxKind::NeutralAdjective,
                ) => self.bump(),
                _ => panic!(),
            }
        }

        self.finish_node();
    }

    fn parse_bin_expr(&mut self, checkpoint: usize) {
        self.start_node_at(checkpoint, SyntaxKind::BinExpr);

        match self.peek() {
            Some(SyntaxKind::Difference | SyntaxKind::Quotient) => {
                self.bump();
                self.expect(SyntaxKind::Between);
            }
            Some(SyntaxKind::Product | SyntaxKind::Sum) => {
                self.bump();
                self.expect(SyntaxKind::Of);
            }
            Some(SyntaxKind::Remainder) => {
                self.bump();
                self.expect(SyntaxKind::Of);
                self.expect(SyntaxKind::Article);
                self.expect(SyntaxKind::Quotient);
                self.expect(SyntaxKind::Between);
            }
            _ => panic!(),
        }

        self.parse_expr();

        self.expect(SyntaxKind::And);

        self.parse_expr();

        self.finish_node();
    }

    fn parse_int_output(&mut self) {
        assert!(self.at(SyntaxKind::Open));
        self.start_node(SyntaxKind::IntOutput);
        self.bump();

        self.expect(SyntaxKind::SecondPersonPossessive);

        self.expect(SyntaxKind::Heart);

        self.expect(SyntaxKind::Period);
        self.finish_node();
    }

    fn parse_char_output(&mut self) {
        assert!(self.at(SyntaxKind::Speak));
        self.start_node(SyntaxKind::CharOutput);
        self.bump();

        self.expect(SyntaxKind::SecondPersonPossessive);

        self.expect(SyntaxKind::Mind);

        self.expect(SyntaxKind::Period);
        self.finish_node();
    }

    fn parse_stage_direction(&mut self) {
        assert!(self.at(SyntaxKind::LBracket));
        self.start_node(SyntaxKind::StageDirection);
        self.bump();

        match self.peek() {
            Some(SyntaxKind::Enter | SyntaxKind::Exit) => {
                self.bump();

                loop {
                    self.expect(SyntaxKind::Character);

                    if self.at(SyntaxKind::RBracket) {
                        self.bump();
                        break;
                    }

                    self.expect(SyntaxKind::And);
                }
            }
            Some(SyntaxKind::Exeunt) => {
                self.bump();
                self.expect(SyntaxKind::RBracket);
            }
            _ => panic!(),
        }

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
        let Token { kind, text } = self.source.next_token().unwrap();

        self.events.push(Event::AddToken { kind, text });
    }

    fn skip(&mut self) {
        let Token { text, .. } = self.source.next_token().unwrap();

        self.events.push(Event::AddToken {
            kind: SyntaxKind::Skip,
            text,
        });
    }

    fn bump_newline(&mut self) {
        if self.at_eof() {
            return;
        }

        self.expect(SyntaxKind::Newline)
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek()
    }

    fn lookahead(&mut self, amount: usize) -> Option<SyntaxKind> {
        assert!(amount > 0, "Use peek instead for amount 0");
        self.source.lookahead(amount)
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.events.push(Event::StartNode { kind });
    }

    fn start_node_at(&mut self, checkpoint: usize, kind: SyntaxKind) {
        self.events.push(Event::StartNodeAt { kind, checkpoint });
    }

    fn finish_node(&mut self) {
        self.events.push(Event::FinishNode);
    }

    fn checkpoint(&self) -> usize {
        self.events.len()
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

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = parse(input);

        expected_tree.assert_eq(&parse.debug_tree());
    }

    #[test]
    fn parse_character_def() {
        check(
            "Romeo, a test.",
            expect![[r#"
Root@0..14
  CharacterDef@0..14
    Character@0..5 "Romeo"
    Comment@5..14
      Comma@5..6 ","
      Whitespace@6..7 " "
      Skip@7..8 "a"
      Whitespace@8..9 " "
      Skip@9..10 "t"
      Skip@10..11 "e"
      Skip@11..12 "s"
      Skip@12..13 "t"
      Skip@13..14 ".""#]],
        )
    }

    #[test]
    fn parse_character_def_with_newline() {
        check(
            "Juliet, act\n",
            expect![[r#"
Root@0..12
  CharacterDef@0..11
    Character@0..6 "Juliet"
    Comment@6..11
      Comma@6..7 ","
      Whitespace@7..8 " "
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
    fn parse_exit_characters() {
        check(
            "[Exit Juliet]",
            expect![[r#"
Root@0..13
  StageDirection@0..13
    LBracket@0..1 "["
    Exit@1..5 "Exit"
    Whitespace@5..6 " "
    Character@6..12 "Juliet"
    RBracket@12..13 "]""#]],
        )
    }
    #[test]
    fn parse_exeunt() {
        check(
            "[Exeunt]",
            expect![[r#"
Root@0..8
  StageDirection@0..8
    LBracket@0..1 "["
    Exeunt@1..7 "Exeunt"
    RBracket@7..8 "]""#]],
        )
    }

    #[test]
    fn parse_dialog_0() {
        check(
            "Juliet: You are nothing.",
            expect![[r#"
Root@0..24
  Dialog@0..24
    Character@0..6 "Juliet"
    Colon@6..7 ":"
    Whitespace@7..8 " "
    Statement@8..24
      SecondPerson@8..11 "You"
      Whitespace@11..12 " "
      Be@12..15 "are"
      Whitespace@15..16 " "
      NothingExpr@16..23
        Nothing@16..23 "nothing"
      Period@23..24 ".""#]],
        )
    }

    #[test]
    fn parse_multiple_sentence() {
        check(
            "Juliet: Thou art a lord. Thou art a lord.",
            expect![[r#"
Root@0..41
  Dialog@0..41
    Character@0..6 "Juliet"
    Colon@6..7 ":"
    Whitespace@7..8 " "
    Statement@8..25
      SecondPerson@8..12 "Thou"
      Whitespace@12..13 " "
      Be@13..16 "art"
      Whitespace@16..17 " "
      NounExpr@17..23
        Article@17..18 "a"
        Whitespace@18..19 " "
        PositiveNoun@19..23 "lord"
      Period@23..24 "."
      Whitespace@24..25 " "
    Statement@25..41
      SecondPerson@25..29 "Thou"
      Whitespace@29..30 " "
      Be@30..33 "art"
      Whitespace@33..34 " "
      NounExpr@34..40
        Article@34..35 "a"
        Whitespace@35..36 " "
        PositiveNoun@36..40 "lord"
      Period@40..41 ".""#]],
        )
    }

    #[test]
    fn parse_dialog_1() {
        check(
            "Juliet: Thou art a lord.",
            expect![[r#"
Root@0..24
  Dialog@0..24
    Character@0..6 "Juliet"
    Colon@6..7 ":"
    Whitespace@7..8 " "
    Statement@8..24
      SecondPerson@8..12 "Thou"
      Whitespace@12..13 " "
      Be@13..16 "art"
      Whitespace@16..17 " "
      NounExpr@17..23
        Article@17..18 "a"
        Whitespace@18..19 " "
        PositiveNoun@19..23 "lord"
      Period@23..24 ".""#]],
        )
    }

    #[test]
    fn parse_dialog_2() {
        check(
            "Juliet: Thou art a fine lord.",
            expect![[r#"
Root@0..29
  Dialog@0..29
    Character@0..6 "Juliet"
    Colon@6..7 ":"
    Whitespace@7..8 " "
    Statement@8..29
      SecondPerson@8..12 "Thou"
      Whitespace@12..13 " "
      Be@13..16 "art"
      Whitespace@16..17 " "
      NounExpr@17..28
        Article@17..18 "a"
        Whitespace@18..19 " "
        PositiveAdjective@19..23 "fine"
        Whitespace@23..24 " "
        PositiveNoun@24..28 "lord"
      Period@28..29 ".""#]],
        )
    }

    #[test]
    fn parse_dialoge_3() {
        check(
            "Juliet: Thou art the sum of a fellow and a fine lord.",
            expect![[r#"
Root@0..53
  Dialog@0..53
    Character@0..6 "Juliet"
    Colon@6..7 ":"
    Whitespace@7..8 " "
    Statement@8..53
      SecondPerson@8..12 "Thou"
      Whitespace@12..13 " "
      Be@13..16 "art"
      Whitespace@16..17 " "
      BinExpr@17..52
        Article@17..20 "the"
        Whitespace@20..21 " "
        Sum@21..24 "sum"
        Whitespace@24..25 " "
        Of@25..27 "of"
        Whitespace@27..28 " "
        NounExpr@28..37
          Article@28..29 "a"
          Whitespace@29..30 " "
          NeutralNoun@30..36 "fellow"
          Whitespace@36..37 " "
        And@37..40 "and"
        Whitespace@40..41 " "
        NounExpr@41..52
          Article@41..42 "a"
          Whitespace@42..43 " "
          PositiveAdjective@43..47 "fine"
          Whitespace@47..48 " "
          PositiveNoun@48..52 "lord"
      Period@52..53 ".""#]],
        )
    }

    // #[test]
    // fn parse_dialog_4() {
    //     check(
    //         "Juliet: Thou art the square of a fine lord.",
    //         expect![[r#""#]],
    //     )
    // }

    #[test]
    fn parse_empty_input() {
        check("", expect![[r#"Root@0..0"#]])
    }

    #[test]
    fn parse_whitespace() {
        check(
            " \t  ",
            expect![[r#"
Root@0..4
  Whitespace@0..4 " \t  ""#]],
        )
    }

    #[test]
    fn parse_num_output() {
        check(
            "Juliet: Open your heart.",
            expect![[r#"
Root@0..24
  Dialog@0..24
    Character@0..6 "Juliet"
    Colon@6..7 ":"
    Whitespace@7..8 " "
    IntOutput@8..24
      Open@8..12 "Open"
      Whitespace@12..13 " "
      SecondPersonPossessive@13..17 "your"
      Whitespace@17..18 " "
      Heart@18..23 "heart"
      Period@23..24 ".""#]],
        )
    }

    #[test]
    fn parse_char_output() {
        check(
            "Juliet: Speak your mind.",
            expect![[r#"
Root@0..24
  Dialog@0..24
    Character@0..6 "Juliet"
    Colon@6..7 ":"
    Whitespace@7..8 " "
    CharOutput@8..24
      Speak@8..13 "Speak"
      Whitespace@13..14 " "
      SecondPersonPossessive@14..18 "your"
      Whitespace@18..19 " "
      Mind@19..23 "mind"
      Period@23..24 ".""#]],
        )
    }
}
