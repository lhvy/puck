use crate::hir;
use arrayvec::ArrayVec;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub(crate) struct Evaluator {
    states: HashMap<String, CharacterState>,
    on_stage: ArrayVec<String, 2>,
}

impl Evaluator {
    pub(crate) fn eval(&mut self, items: &[hir::Item], db: hir::Database) {
        for item in items {
            match item {
                hir::Item::CharacterDef { character } => {
                    self.states
                        .insert(character.clone(), CharacterState::default());
                }
                hir::Item::StageDirection {
                    characters,
                    direction,
                } => match direction {
                    hir::Direction::Exeunt => {
                        for state in self.states.values_mut() {
                            state.on_stage = false;
                        }
                    }
                    hir::Direction::Enter => {
                        for character in characters {
                            let state = self.states.get_mut(character).unwrap();
                            assert!(!state.on_stage);
                            state.on_stage = true;
                            self.on_stage.push(character.clone());
                        }
                    }
                    hir::Direction::Exit => {
                        for character in characters {
                            let state = self.states.get_mut(character).unwrap();
                            assert!(state.on_stage);
                            state.on_stage = false;
                            let idx = self.on_stage.iter().position(|c| c == character).unwrap();
                            self.on_stage.remove(idx);
                        }
                    }
                },
                hir::Item::Dialog {
                    character,
                    sentences,
                } => {
                    assert_eq!(self.on_stage.len(), 2);
                    let speaker_idx = self.on_stage.iter().position(|c| c == character).unwrap();
                    let speaker = self.on_stage[speaker_idx].clone();
                    let listener = match speaker_idx {
                        0 => self.on_stage[1].clone(),
                        1 => self.on_stage[0].clone(),
                        _ => unreachable!(),
                    };

                    for sentence in sentences {
                        match sentence {
                            hir::Sentence::Statement { expr } => {
                                self.states.get_mut(&listener).unwrap().value =
                                    self.eval_expr(expr, &db);
                                println!(
                                    "{} set to {} by {}",
                                    listener, self.states[&listener].value, speaker
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    fn eval_expr(&mut self, expr: &hir::Expr, db: &hir::Database) -> i64 {
        match expr {
            hir::Expr::Noun { adjectives, noun } => {
                let value = match noun {
                    hir::Noun::Positive | hir::Noun::Neutral => 1,
                    hir::Noun::Negative => -1,
                };

                value * 2_i64.pow(adjectives.len() as u32)
            }
            hir::Expr::Bin {
                operation,
                lhs,
                rhs,
            } => {
                let lhs = self.eval_expr(&db[*lhs], db);
                let rhs = self.eval_expr(&db[*rhs], db);

                match operation {
                    hir::Operation::Remainder => lhs % rhs,
                    hir::Operation::Difference => lhs - rhs,
                    hir::Operation::Quotient => lhs / rhs,
                    hir::Operation::Product => lhs * rhs,
                    hir::Operation::Sum => lhs + rhs,
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct CharacterState {
    value: i64,
    on_stage: bool,
}
