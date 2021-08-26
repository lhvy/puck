use crate::lexer::{SyntaxKind, Token};
use text_size::TextRange;

pub(super) struct Source<'tokens, 'input> {
    tokens: &'tokens [Token<'input>],
    cursor: usize,
}

impl<'tokens, 'input> Source<'tokens, 'input> {
    pub(super) fn new(tokens: &'tokens [Token<'input>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub(super) fn next_token(&mut self) -> Option<Token<'input>> {
        self.skip_ws();

        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(*token)
    }

    fn skip_ws(&mut self) {
        while self.tokens.get(self.cursor).map(|Token { kind, .. }| *kind)
            == Some(SyntaxKind::Whitespace)
        {
            self.cursor += 1;
        }
    }

    pub(super) fn peek_kind(&self) -> Option<SyntaxKind> {
        self.lookahead_kind(0)
    }

    pub(super) fn peek_token(&self) -> Option<Token<'input>> {
        self.lookahead_token(0)
    }

    pub(super) fn lookahead_kind(&self, amount: usize) -> Option<SyntaxKind> {
        let token = self.lookahead_token(amount)?;
        Some(token.kind)
    }

    fn lookahead_token(&self, mut amount: usize) -> Option<Token<'input>> {
        let mut idx = self.cursor;
        loop {
            let token = *self.tokens.get(idx)?;
            if token.kind != SyntaxKind::Whitespace {
                if amount == 0 {
                    return Some(token);
                }
                amount -= 1;
            }
            idx += 1;
        }
    }

    pub(super) fn final_token_range(&self) -> Option<TextRange> {
        self.tokens.last().map(|Token { range, .. }| *range)
    }

    pub(super) fn previous_token_range(&self) -> TextRange {
        self.tokens[self.cursor - 1].range
    }
}
