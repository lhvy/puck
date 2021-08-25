use super::event::Event;
use crate::syntax::ShakespeareProgrammingLanguage;
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub(super) struct Sink<'a> {
    builder: GreenNodeBuilder<'static>,
    events: Vec<Event<'a>>,
}

impl<'a> Sink<'a> {
    pub(super) fn new(events: Vec<Event<'a>>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
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
