mod database;

use crate::ast;
pub(crate) use database::Database;
use la_arena::Idx;

pub(crate) fn lower(ast: ast::Root) -> (Vec<Item>, Database) {
    let mut db = Database::default();
    let items = ast.items().map(|item| db.lower_item(item)).collect();
    (items, db)
}

#[derive(Debug)]
pub(crate) enum Item {
    CharacterDef {
        character: String,
    },
    StageDirection {
        characters: Vec<String>,
        direction: Direction,
    },
    Dialog {
        character: String,
        sentences: Vec<Sentence>,
    },
}

#[derive(Debug)]
pub(crate) enum Direction {
    Enter,
    Exit,
    Exeunt,
}

#[derive(Debug)]
pub(crate) enum Sentence {
    Statement { expr: Expr },
}

type ExprIdx = Idx<Expr>;

#[derive(Debug)]
pub(crate) enum Expr {
    Noun {
        adjectives: Vec<Adjective>,
        noun: Noun,
    },
    Bin {
        operation: Operation,
        lhs: ExprIdx,
        rhs: ExprIdx,
    },
    Nothing,
}

#[derive(Debug)]
pub(crate) enum Adjective {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug)]
pub(crate) enum Noun {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug)]
pub(crate) enum Operation {
    Remainder,
    Difference,
    Quotient,
    Product,
    Sum,
}
