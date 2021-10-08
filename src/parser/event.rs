use super::parse_error::ParseError;
use crate::lexer::SyntaxKind;

#[derive(Debug, PartialEq)]
pub(super) enum Event {
    StartNode { kind: SyntaxKind },
    FinishNode,
    AddToken { kind: SyntaxKind },
    MarkerPlaceholder,
    Error(ParseError),
}
