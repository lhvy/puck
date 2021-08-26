use super::event::Event;
use crate::lexer::{SyntaxKind, Token};
use crate::syntax::ShakespeareProgrammingLanguage;
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub(super) struct Sink<'tokens, 'input> {
    builder: GreenNodeBuilder<'static>,
    tokens: &'tokens [Token<'input>],
    cursor: usize,
    events: Vec<Event>,
}

impl<'tokens, 'input> Sink<'tokens, 'input> {
    pub(super) fn new(tokens: &'tokens [Token<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            cursor: 0,
            events,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        for idx in 0..self.events.len() {
            match self.events[idx] {
                Event::StartNode { kind } => self
                    .builder
                    .start_node(ShakespeareProgrammingLanguage::kind_to_raw(kind)),
                Event::FinishNode => self.builder.finish_node(),
                Event::AddToken { kind } => self.token(kind),
                Event::MarkerPlaceholder => unreachable!(),
            }

            self.skip_ws();
        }

        self.builder.finish()
    }

    fn token(&mut self, kind: SyntaxKind) {
        self.builder.token(
            ShakespeareProgrammingLanguage::kind_to_raw(kind),
            self.tokens[self.cursor].text,
        );
        self.cursor += 1;
    }

    fn skip_ws(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if token.kind != SyntaxKind::Whitespace {
                break;
            }

            self.token(token.kind);
        }
    }
}
