use super::event::Event;
use crate::lexer::{SyntaxKind, Token};
use crate::syntax::ShakespeareProgrammingLanguage;
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub(super) struct Sink<'tokens, 'input> {
    builder: GreenNodeBuilder<'static>,
    tokens: &'tokens [Token<'input>],
    cursor: usize,
    events: Vec<Event<'input>>,
}

impl<'tokens, 'input> Sink<'tokens, 'input> {
    pub(super) fn new(tokens: &'tokens [Token<'input>], events: Vec<Event<'input>>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            cursor: 0,
            events,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        let mut reordered_events = self.events.clone();

        for (idx, event) in self.events.iter().enumerate() {
            if let Event::StartNodeAt { kind, checkpoint } = event {
                reordered_events.remove(idx);
                reordered_events.insert(*checkpoint, Event::StartNode { kind: *kind });
            }
        }

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => self
                    .builder
                    .start_node(ShakespeareProgrammingLanguage::kind_to_raw(kind)),
                Event::StartNodeAt { .. } => unreachable!(),
                Event::FinishNode => self.builder.finish_node(),
                Event::AddToken { kind, text } => self.token(kind, text),
            }

            self.skip_ws();
        }

        self.builder.finish()
    }

    fn token(&mut self, kind: SyntaxKind, text: &str) {
        self.builder
            .token(ShakespeareProgrammingLanguage::kind_to_raw(kind), text);
        self.cursor += 1;
    }

    fn skip_ws(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if token.kind != SyntaxKind::Whitespace {
                break;
            }

            self.token(token.kind, token.text);
        }
    }
}
