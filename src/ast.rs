use crate::lexer::SyntaxKind;
use crate::syntax::{SyntaxElement, SyntaxNode, SyntaxToken};

#[derive(Debug)]
pub(crate) struct Root(SyntaxNode);

impl Root {
    pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Root => Some(Self(node)),
            _ => None,
        }
    }

    pub(crate) fn items(&self) -> impl Iterator<Item = Item> {
        self.0.children().filter_map(Item::cast)
    }
}

#[derive(Debug)]
pub(crate) enum Item {
    CharacterDef(CharacterDef),
    StageDirection(StageDirection),
    Dialog(Dialog),
}

impl Item {
    pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::CharacterDef => Self::CharacterDef(CharacterDef(node)),
            SyntaxKind::StageDirection => Self::StageDirection(StageDirection(node)),
            SyntaxKind::Dialog => Self::Dialog(Dialog(node)),
            _ => return None,
        };

        Some(result)
    }
}

#[derive(Debug)]
pub(crate) struct CharacterDef(SyntaxNode);

impl CharacterDef {
    pub(crate) fn character(&self) -> Option<String> {
        self.0.first_token().map(|token| token.text().to_string())
    }
}

#[derive(Debug)]
pub(crate) struct StageDirection(SyntaxNode);

impl StageDirection {
    pub(crate) fn direction(&self) -> Option<Direction> {
        let token = self
            .0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| {
                matches!(
                    token.kind(),
                    SyntaxKind::Enter | SyntaxKind::Exit | SyntaxKind::Exeunt
                )
            })?;

        match token.kind() {
            SyntaxKind::Enter => Some(Direction::Enter),
            SyntaxKind::Exit => Some(Direction::Exit),
            SyntaxKind::Exeunt => Some(Direction::Exeunt),
            _ => unreachable!(),
        }
    }

    pub(crate) fn characters(&self) -> impl Iterator<Item = String> {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .filter(|token| token.kind() == SyntaxKind::Character)
            .map(|token| token.text().to_string())
    }
}

#[derive(Debug)]
pub(crate) enum Direction {
    Enter,
    Exit,
    Exeunt,
}

#[derive(Debug)]
pub(crate) struct Dialog(SyntaxNode);

impl Dialog {
    pub(crate) fn character(&self) -> Option<String> {
        self.0.first_token().map(|token| token.text().to_string())
    }

    pub(crate) fn sentences(&self) -> impl Iterator<Item = Sentence> {
        self.0.children().filter_map(Sentence::cast)
    }
}

#[derive(Debug)]
pub(crate) enum Sentence {
    Statement(Statement),
    IntOutput(IntOutput),
    CharOutput(CharOutput),
}

impl Sentence {
    pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::Statement => Self::Statement(Statement(node)),
            SyntaxKind::IntOutput => Self::IntOutput(IntOutput(node)),
            SyntaxKind::CharOutput => Self::CharOutput(CharOutput(node)),
            _ => return None,
        };

        Some(result)
    }
}

#[derive(Debug)]
pub(crate) struct Statement(SyntaxNode);

impl Statement {
    pub(crate) fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

#[derive(Debug)]
pub(crate) enum Expr {
    Noun(NounExpr),
    Bin(BinExpr),
    Nothing(NothingExpr),
}

impl Expr {
    pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::NounExpr => Self::Noun(NounExpr(node)),
            SyntaxKind::BinExpr => Self::Bin(BinExpr(node)),
            SyntaxKind::NothingExpr => Self::Nothing(NothingExpr(node)),
            _ => return None,
        };

        Some(result)
    }
}

#[derive(Debug)]
pub(crate) struct NounExpr(SyntaxNode);

impl NounExpr {
    pub(crate) fn adjectives(&self) -> impl Iterator<Item = Adjective> {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .filter_map(Adjective::cast)
    }

    pub(crate) fn noun(&self) -> Option<Noun> {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find_map(Noun::cast)
    }
}

#[derive(Debug)]
pub(crate) enum Adjective {
    Positive,
    Negative,
    Neutral,
}

impl Adjective {
    pub(crate) fn cast(token: SyntaxToken) -> Option<Self> {
        match token.kind() {
            SyntaxKind::PositiveAdjective => Some(Self::Positive),
            SyntaxKind::NegativeAdjective => Some(Self::Negative),
            SyntaxKind::NeutralAdjective => Some(Self::Neutral),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct BinExpr(SyntaxNode);

impl BinExpr {
    pub(crate) fn operation(&self) -> Option<Operation> {
        let operation = self
            .0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| {
                matches!(
                    token.kind(),
                    SyntaxKind::Remainder
                        | SyntaxKind::Difference
                        | SyntaxKind::Quotient
                        | SyntaxKind::Product
                        | SyntaxKind::Sum
                )
            })?;

        match operation.kind() {
            SyntaxKind::Remainder => Some(Operation::Remainder),
            SyntaxKind::Difference => Some(Operation::Difference),
            SyntaxKind::Quotient => Some(Operation::Quotient),
            SyntaxKind::Product => Some(Operation::Product),
            SyntaxKind::Sum => Some(Operation::Sum),
            _ => unreachable!(),
        }
    }

    pub(crate) fn lhs(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub(crate) fn rhs(&self) -> Option<Expr> {
        self.0.children().filter_map(Expr::cast).nth(1) // Second expr should be rhs
    }
}

#[derive(Debug)]
pub(crate) struct NothingExpr(SyntaxNode);

#[derive(Debug)]
pub(crate) enum Noun {
    Positive,
    Negative,
    Neutral,
}

impl Noun {
    pub(crate) fn cast(token: SyntaxToken) -> Option<Self> {
        match token.kind() {
            SyntaxKind::PositiveNoun => Some(Self::Positive),
            SyntaxKind::NegativeNoun => Some(Self::Negative),
            SyntaxKind::NeutralNoun => Some(Self::Neutral),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum Operation {
    Difference,
    Quotient,
    Product,
    Sum,
    Remainder,
}

#[derive(Debug)]
pub(crate) struct IntOutput(SyntaxNode);

#[derive(Debug)]
pub(crate) struct CharOutput(SyntaxNode);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn character_def() {
        let syntax_node = parse("Romeo, a test.", false).syntax_node();
        let root = Root::cast(syntax_node).unwrap();
        let item = root.items().next().unwrap();

        let character_def = if let Item::CharacterDef(character_def) = item {
            character_def
        } else {
            unreachable!()
        };

        assert_eq!(character_def.character().unwrap(), "Romeo")
    }
}
