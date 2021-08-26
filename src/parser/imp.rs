mod marker;

use super::event::Event;
use super::grammar;
use super::source::Source;
use crate::lexer::{SyntaxKind, Token};
use crate::parser::parse_error::ParseError;
pub(super) use marker::Marker;
use std::collections::BTreeSet;
use std::mem;

const DEFAULT_RECOVERY_SET: [SyntaxKind; 6] = [
    SyntaxKind::Character,
    SyntaxKind::Period,
    SyntaxKind::Exclamation,
    SyntaxKind::Question,
    SyntaxKind::LBracket,
    SyntaxKind::RBracket,
];

pub(super) struct Parser<'tokens, 'input> {
    source: Source<'tokens, 'input>,
    events: Vec<Event>,
    expected_kinds: BTreeSet<SyntaxKind>,
}

impl<'tokens, 'input> Parser<'tokens, 'input> {
    pub(super) fn new(tokens: &'tokens [Token<'input>]) -> Self {
        Self {
            source: Source::new(tokens),
            events: Vec::new(),
            expected_kinds: BTreeSet::new(),
        }
    }

    pub(super) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::MarkerPlaceholder);

        Marker::new(pos)
    }

    pub(super) fn parse(mut self) -> Vec<Event> {
        grammar::root(&mut self);

        self.events
    }

    pub(super) fn error(&mut self) {
        self.error_with_recovery_set(DEFAULT_RECOVERY_SET);
    }

    pub(super) fn error_with_recovery_set<const N: usize>(
        &mut self,
        recovery_set: [SyntaxKind; N],
    ) {
        let current_token = self.source.peek_token();

        let expected = mem::take(&mut self.expected_kinds);

        if self.at_set(recovery_set) || self.at_eof() {
            self.events.push(Event::Error(ParseError {
                expected,
                found: None,
                range: self.source.previous_token_range(),
            }));
        } else {
            let (found, range) = current_token.map_or_else(
                || (None, self.source.final_token_range().unwrap()),
                |Token { kind, range, .. }| (Some(kind), range),
            );

            self.events.push(Event::Error(ParseError {
                expected,
                found,
                range,
            }));
            self.skip_error();
        }
    }

    fn at_set<const N: usize>(&self, set: [SyntaxKind; N]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }

    pub(super) fn expect(&mut self, syntax_kind: SyntaxKind) {
        if self.at(syntax_kind) {
            self.bump();
            return;
        }

        self.error();
    }

    pub(super) fn at(&mut self, syntax_kind: SyntaxKind) -> bool {
        self.expected_kinds.insert(syntax_kind);
        self.peek() == Some(syntax_kind)
    }

    pub(super) fn at_eof(&self) -> bool {
        self.peek().is_none()
    }

    pub(super) fn bump(&mut self) {
        self.expected_kinds.clear();
        let Token { kind, .. } = self.source.next_token().unwrap();
        self.events.push(Event::AddToken { kind });
    }

    pub(super) fn skip_error(&mut self) {
        let m = self.start();
        self.bump();
        m.complete(self, SyntaxKind::Error);
    }

    pub(super) fn skip(&mut self) {
        self.expected_kinds.clear();
        self.source.next_token().unwrap();

        self.events.push(Event::AddToken {
            kind: SyntaxKind::Skip,
        });
    }

    fn peek(&self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }
}
