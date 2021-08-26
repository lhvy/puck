mod marker;

use super::event::Event;
use super::grammar;
use super::source::Source;
use crate::lexer::{SyntaxKind, Token};
pub(super) use marker::Marker;

pub(super) struct Parser<'tokens, 'input> {
    source: Source<'tokens, 'input>,
    events: Vec<Event>,
}

impl<'tokens, 'input> Parser<'tokens, 'input> {
    pub(super) fn new(tokens: &'tokens [Token<'input>]) -> Self {
        Self {
            source: Source::new(tokens),
            events: Vec::new(),
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

    pub(super) fn expect(&mut self, syntax_kind: SyntaxKind) {
        if self.at(syntax_kind) {
            self.bump();
        } else {
            panic!("Expected {:?} but got {:?}", syntax_kind, self.peek());
        }
    }

    pub(super) fn at(&mut self, syntax_kind: SyntaxKind) -> bool {
        self.peek() == Some(syntax_kind)
    }

    pub(super) fn at_eof(&mut self) -> bool {
        self.peek().is_none()
    }

    pub(super) fn bump(&mut self) {
        let Token { kind, .. } = self.source.next_token().unwrap();

        self.events.push(Event::AddToken { kind });
    }

    pub(super) fn skip(&mut self) {
        self.source.next_token().unwrap();

        self.events.push(Event::AddToken {
            kind: SyntaxKind::Skip,
        });
    }

    pub(super) fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek()
    }

    pub(super) fn lookahead(&mut self, amount: usize) -> Option<SyntaxKind> {
        assert!(amount > 0, "Use peek instead for amount 0");
        self.source.lookahead(amount)
    }
}
