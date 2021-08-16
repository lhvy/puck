use la_arena::Arena;

use super::{Adjective, Direction, Expr, Item, Noun, Operation, Sentence};
use crate::ast;

#[derive(Debug, Default)]
pub(crate) struct Database {
    exprs: Arena<Expr>,
}

impl Database {
    pub(crate) fn lower_item(&mut self, ast: ast::Item) -> Item {
        match ast {
            ast::Item::CharacterDef(character_def) => Item::CharacterDef {
                character: character_def.character().unwrap(),
            },
            ast::Item::StageDirection(stage_direction) => Item::StageDirection {
                characters: stage_direction.characters().collect(),
                direction: match stage_direction.direction().unwrap() {
                    ast::Direction::Enter => Direction::Enter,
                    ast::Direction::Exit => Direction::Exit,
                    ast::Direction::Exeunt => Direction::Exeunt,
                },
            },
            ast::Item::Dialog(dialog) => Item::Dialog {
                character: dialog.character().unwrap(),
                sentences: dialog
                    .sentences()
                    .map(|sentence| self.lower_sentence(sentence))
                    .collect(),
            },
        }
    }

    fn lower_sentence(&mut self, ast: ast::Sentence) -> Sentence {
        match ast {
            ast::Sentence::Statement(statement) => Sentence::Statement {
                expr: self.lower_expr(statement.expr().unwrap()),
            },
        }
    }

    fn lower_expr(&mut self, ast: ast::Expr) -> Expr {
        match ast {
            ast::Expr::Noun(noun_expr) => Expr::Noun {
                adjectives: noun_expr
                    .adjectives()
                    .map(|adjective| self.lower_adjective(adjective))
                    .collect(),
                noun: self.lower_noun(noun_expr.noun().unwrap()),
            },
            ast::Expr::Bin(bin) => {
                let lhs = self.lower_expr(bin.lhs().unwrap());
                let rhs = self.lower_expr(bin.rhs().unwrap());
                Expr::Bin {
                    operation: match bin.operation().unwrap() {
                        ast::Operation::Difference => Operation::Difference,
                        ast::Operation::Quotient => Operation::Quotient,
                        ast::Operation::Product => Operation::Product,
                        ast::Operation::Sum => Operation::Sum,
                        ast::Operation::Remainder => Operation::Remainder,
                    },
                    lhs: self.exprs.alloc(lhs),
                    rhs: self.exprs.alloc(rhs),
                }
            }
        }
    }

    fn lower_adjective(&mut self, ast: ast::Adjective) -> Adjective {
        match ast {
            ast::Adjective::Positive => Adjective::Positive,
            ast::Adjective::Negative => Adjective::Negative,
            ast::Adjective::Neutral => Adjective::Neutral,
        }
    }

    fn lower_noun(&mut self, ast: ast::Noun) -> Noun {
        match ast {
            ast::Noun::Positive => Noun::Positive,
            ast::Noun::Negative => Noun::Negative,
            ast::Noun::Neutral => Noun::Neutral,
        }
    }
}
