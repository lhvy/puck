use crate::lexer::{SyntaxKind, Token};

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

    pub(super) fn peek(&mut self) -> Option<SyntaxKind> {
        self.lookahead(0)
    }

    pub(super) fn lookahead(&self, mut amount: usize) -> Option<SyntaxKind> {
        let mut idx = self.cursor;
        loop {
            let kind = self.tokens.get(idx)?.kind;
            if kind != SyntaxKind::Whitespace {
                if amount == 0 {
                    return Some(kind);
                }
                amount -= 1;
            }
            idx += 1;
        }
    }
}
