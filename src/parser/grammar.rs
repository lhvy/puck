use super::imp::{Marker, Parser};
use crate::lexer::SyntaxKind;

pub(super) fn root(p: &mut Parser<'_, '_>) {
    let m = p.start();

    loop {
        if p.at(SyntaxKind::Character) {
            match p.lookahead(1) {
                Some(SyntaxKind::Colon) => parse_dialog(p),
                Some(SyntaxKind::Comma) => parse_character_def(p),
                _ => panic!(),
            }
        }

        if p.at(SyntaxKind::LBracket) {
            parse_stage_direction(p);
        }

        if p.at_eof() {
            break;
        }
    }

    m.complete(p, SyntaxKind::Root);
}

fn parse_character_def(p: &mut Parser<'_, '_>) {
    assert!(p.at(SyntaxKind::Character));
    let m_def = p.start();
    p.bump();

    let m_comment = p.start();
    p.expect(SyntaxKind::Comma);
    while !p.at(SyntaxKind::Period) && !p.at(SyntaxKind::Exclamation) && !p.at_eof() {
        p.skip();
    }
    parse_terminator(p);

    m_comment.complete(p, SyntaxKind::Comment);

    m_def.complete(p, SyntaxKind::CharacterDef);
}

fn parse_dialog(p: &mut Parser<'_, '_>) {
    assert!(p.at(SyntaxKind::Character));
    let m = p.start();
    p.bump();

    p.expect(SyntaxKind::Colon);

    while parse_sentence(p) {}

    m.complete(p, SyntaxKind::Dialog);
}

fn parse_sentence(p: &mut Parser<'_, '_>) -> bool {
    match p.peek() {
        Some(SyntaxKind::SecondPerson) => parse_statement(p),
        Some(SyntaxKind::Open) => parse_int_output(p),
        Some(SyntaxKind::Speak) => parse_char_output(p),
        _ => return false,
    }

    true
}

fn parse_statement(p: &mut Parser<'_, '_>) {
    assert!(p.at(SyntaxKind::SecondPerson));
    let m = p.start();
    p.bump();

    p.expect(SyntaxKind::Be);

    parse_expr(p);
    parse_terminator(p);

    m.complete(p, SyntaxKind::Statement);
}

fn parse_expr(p: &mut Parser<'_, '_>) {
    if p.at(SyntaxKind::Nothing) {
        let m = p.start();
        p.bump();
        m.complete(p, SyntaxKind::NothingExpr);
        return;
    }

    if p.at(SyntaxKind::Period) | p.at(SyntaxKind::Exclamation) {
        return;
    }

    let m = p.start();

    p.expect(SyntaxKind::Article);

    match p.peek() {
        Some(
            SyntaxKind::PositiveAdjective
            | SyntaxKind::NegativeAdjective
            | SyntaxKind::NeutralAdjective
            | SyntaxKind::PositiveNoun
            | SyntaxKind::NegativeNoun
            | SyntaxKind::NeutralNoun,
        ) => parse_noun_expr(p, m),
        Some(
            SyntaxKind::Difference
            | SyntaxKind::Product
            | SyntaxKind::Quotient
            | SyntaxKind::Remainder
            | SyntaxKind::Sum,
        ) => parse_bin_expr(p, m),
        _ => panic!(),
    }
}

fn parse_noun_expr(p: &mut Parser<'_, '_>, m: Marker) {
    loop {
        if matches!(
            p.peek(),
            Some(SyntaxKind::PositiveNoun | SyntaxKind::NegativeNoun | SyntaxKind::NeutralNoun)
        ) {
            p.bump();
            break;
        }

        match p.peek() {
            Some(
                SyntaxKind::PositiveAdjective
                | SyntaxKind::NegativeAdjective
                | SyntaxKind::NeutralAdjective,
            ) => p.bump(),
            _ => panic!(),
        }
    }

    m.complete(p, SyntaxKind::NounExpr);
}

fn parse_bin_expr(p: &mut Parser<'_, '_>, m: Marker) {
    match p.peek() {
        Some(SyntaxKind::Difference | SyntaxKind::Quotient) => {
            p.bump();
            p.expect(SyntaxKind::Between);
        }
        Some(SyntaxKind::Product | SyntaxKind::Sum) => {
            p.bump();
            p.expect(SyntaxKind::Of);
        }
        Some(SyntaxKind::Remainder) => {
            p.bump();
            p.expect(SyntaxKind::Of);
            p.expect(SyntaxKind::Article);
            p.expect(SyntaxKind::Quotient);
            p.expect(SyntaxKind::Between);
        }
        _ => panic!(),
    }

    parse_expr(p);

    p.expect(SyntaxKind::And);

    parse_expr(p);

    m.complete(p, SyntaxKind::BinExpr);
}

fn parse_int_output(p: &mut Parser<'_, '_>) {
    assert!(p.at(SyntaxKind::Open));
    let m = p.start();
    p.bump();

    p.expect(SyntaxKind::SecondPersonPossessive);

    p.expect(SyntaxKind::Heart);

    parse_terminator(p);
    m.complete(p, SyntaxKind::IntOutput);
}

fn parse_char_output(p: &mut Parser<'_, '_>) {
    assert!(p.at(SyntaxKind::Speak));
    let m = p.start();
    p.bump();

    p.expect(SyntaxKind::SecondPersonPossessive);

    p.expect(SyntaxKind::Mind);

    parse_terminator(p);
    m.complete(p, SyntaxKind::CharOutput);
}

fn parse_terminator(p: &mut Parser<'_, '_>) {
    match p.peek() {
        Some(SyntaxKind::Period | SyntaxKind::Exclamation) => p.bump(),
        _ => panic!(),
    }
}

fn parse_stage_direction(p: &mut Parser<'_, '_>) {
    assert!(p.at(SyntaxKind::LBracket));
    let m = p.start();
    p.bump();

    match p.peek() {
        Some(SyntaxKind::Enter | SyntaxKind::Exit) => {
            p.bump();

            loop {
                p.expect(SyntaxKind::Character);

                if p.at(SyntaxKind::RBracket) {
                    p.bump();
                    break;
                }

                p.expect(SyntaxKind::And);
            }
        }
        Some(SyntaxKind::Exeunt) => {
            p.bump();
            p.expect(SyntaxKind::RBracket);
        }
        _ => panic!(),
    }

    m.complete(p, SyntaxKind::StageDirection);
}

#[cfg(test)]
mod tests {
    use super::super::parse;
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
      Period@13..14 ".""#]],
        )
    }

    #[test]
    fn parse_character_def_exclamation() {
        check(
            "Romeo, a test!",
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
      Exclamation@13..14 "!""#]],
        )
    }

    #[test]
    fn parse_character_def_whitespace() {
        check(
            "Juliet, act.\n",
            expect![[r#"
Root@0..13
  CharacterDef@0..13
    Character@0..6 "Juliet"
    Comment@6..13
      Comma@6..7 ","
      Whitespace@7..8 " "
      Skip@8..11 "act"
      Period@11..12 "."
      Whitespace@12..13 "\n""#]],
        )
    }

    #[test]
    fn parse_character_def_newline_comment() {
        check(
            "Romeo, a\ntest.",
            expect![[r#"
Root@0..14
  CharacterDef@0..14
    Character@0..5 "Romeo"
    Comment@5..14
      Comma@5..6 ","
      Whitespace@6..7 " "
      Skip@7..8 "a"
      Whitespace@8..9 "\n"
      Skip@9..10 "t"
      Skip@10..11 "e"
      Skip@11..12 "s"
      Skip@12..13 "t"
      Period@13..14 ".""#]],
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
            "Juliet: Open your heart!",
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
      Exclamation@23..24 "!""#]],
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

    #[test]
    fn parse_multiple_sentence_with_spaces() {
        check(
            "Juliet: Thou art a lord.\n Thou \t\t\n\t  art a \tlord.",
            expect![[r#"
Root@0..49
  Dialog@0..49
    Character@0..6 "Juliet"
    Colon@6..7 ":"
    Whitespace@7..8 " "
    Statement@8..26
      SecondPerson@8..12 "Thou"
      Whitespace@12..13 " "
      Be@13..16 "art"
      Whitespace@16..17 " "
      NounExpr@17..23
        Article@17..18 "a"
        Whitespace@18..19 " "
        PositiveNoun@19..23 "lord"
      Period@23..24 "."
      Whitespace@24..26 "\n "
    Statement@26..49
      SecondPerson@26..30 "Thou"
      Whitespace@30..37 " \t\t\n\t  "
      Be@37..40 "art"
      Whitespace@40..41 " "
      NounExpr@41..48
        Article@41..42 "a"
        Whitespace@42..44 " \t"
        PositiveNoun@44..48 "lord"
      Period@48..49 ".""#]],
        )
    }
}
