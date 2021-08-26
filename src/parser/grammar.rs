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

        p.bump_newline();
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
    while !p.at(SyntaxKind::Newline) && !p.at_eof() {
        p.skip();
    }
    m_comment.complete(p, SyntaxKind::Comment);

    m_def.complete(p, SyntaxKind::CharacterDef);
}

fn parse_dialog(p: &mut Parser<'_, '_>) {
    assert!(p.at(SyntaxKind::Character));
    let m = p.start();
    p.bump();

    p.expect(SyntaxKind::Colon);

    while !p.at(SyntaxKind::Newline) && !p.at_eof() {
        parse_sentence(p);
    }

    m.complete(p, SyntaxKind::Dialog);
}

fn parse_sentence(p: &mut Parser<'_, '_>) {
    match p.peek() {
        Some(SyntaxKind::SecondPerson) => parse_statement(p),
        Some(SyntaxKind::Open) => parse_int_output(p),
        Some(SyntaxKind::Speak) => parse_char_output(p),
        _ => panic!(),
    }
}

fn parse_statement(p: &mut Parser<'_, '_>) {
    assert!(p.at(SyntaxKind::SecondPerson));
    let m = p.start();
    p.bump();

    p.expect(SyntaxKind::Be);

    parse_expr(p);
    p.expect(SyntaxKind::Period);

    m.complete(p, SyntaxKind::Statement);
}

fn parse_expr(p: &mut Parser<'_, '_>) {
    if p.at(SyntaxKind::Nothing) {
        let m = p.start();
        p.bump();
        m.complete(p, SyntaxKind::NothingExpr);
        return;
    }

    if p.at(SyntaxKind::Period) {
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

    p.expect(SyntaxKind::Period);
    m.complete(p, SyntaxKind::IntOutput);
}

fn parse_char_output(p: &mut Parser<'_, '_>) {
    assert!(p.at(SyntaxKind::Speak));
    let m = p.start();
    p.bump();

    p.expect(SyntaxKind::SecondPersonPossessive);

    p.expect(SyntaxKind::Mind);

    p.expect(SyntaxKind::Period);
    m.complete(p, SyntaxKind::CharOutput);
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
