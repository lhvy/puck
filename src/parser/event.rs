use crate::lexer::SyntaxKind;

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Event<'a> {
    StartNode { kind: SyntaxKind },
    FinishNode,
    AddToken { kind: SyntaxKind, text: &'a str },
    MarkerPlaceholder,
}
