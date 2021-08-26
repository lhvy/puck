use super::Parser;
use crate::lexer::SyntaxKind;
use crate::parser::event::Event;
use drop_bomb::DropBomb;

pub(in super::super) struct Marker {
    pos: usize,
    bomb: DropBomb,
}

impl Marker {
    pub(super) fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DropBomb::new("Markers need to be completed"),
        }
    }

    pub(in super::super) fn complete(mut self, p: &mut Parser<'_, '_>, kind: SyntaxKind) {
        self.bomb.defuse();

        let event = &mut p.events[self.pos];
        assert_eq!(*event, Event::MarkerPlaceholder);

        *event = Event::StartNode { kind };

        p.events.push(Event::FinishNode);
    }
}
