use crate::lexer::SyntaxKind;

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Event {
    StartNode { kind: SyntaxKind },
    FinishNode,
    AddToken { kind: SyntaxKind },
    MarkerPlaceholder,
}
