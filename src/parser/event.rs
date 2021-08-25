use crate::lexer::SyntaxKind;

#[derive(Debug, Clone)]
pub(super) enum Event<'a> {
    StartNode { kind: SyntaxKind },
    StartNodeAt { kind: SyntaxKind, checkpoint: usize },
    FinishNode,
    AddToken { kind: SyntaxKind, text: &'a str },
}
