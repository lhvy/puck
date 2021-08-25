use super::event::Event;
use crate::lexer::Token;
use crate::syntax::ShakespeareProgrammingLanguage;
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub(super) struct Sink<'tokens, 'input> {
    builder: GreenNodeBuilder<'static>,
    tokens: &'tokens [Token<'input>],
    events: Vec<Event<'input>>,
}

impl<'tokens, 'input> Sink<'tokens, 'input> {
    pub(super) fn new(tokens: &'tokens [Token<'input>], events: Vec<Event<'input>>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            events,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        let mut reordered_events = self.events.clone();

        for (idx, event) in self.events.into_iter().enumerate() {
            if let Event::StartNodeAt { kind, checkpoint } = event {
                reordered_events.remove(idx);
                reordered_events.insert(checkpoint, Event::StartNode { kind });
            }
        }

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => self
                    .builder
                    .start_node(ShakespeareProgrammingLanguage::kind_to_raw(kind)),
                Event::StartNodeAt { .. } => unreachable!(),
                Event::FinishNode => self.builder.finish_node(),
                Event::AddToken { kind, text } => self
                    .builder
                    .token(ShakespeareProgrammingLanguage::kind_to_raw(kind), text),
            }
        }

        self.builder.finish()
    }
}
