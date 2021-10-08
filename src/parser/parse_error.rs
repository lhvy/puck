use crate::lexer::SyntaxKind;
use std::collections::BTreeSet;
use std::fmt;
use text_size::TextRange;

#[derive(Debug, PartialEq)]
pub(crate) struct ParseError {
    pub(super) expected: BTreeSet<SyntaxKind>,
    pub(super) found: Option<SyntaxKind>,
    pub(super) range: TextRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error at {}..{}: expected ",
            u32::from(self.range.start()),
            u32::from(self.range.end()),
        )?;

        let expected: Vec<_> = self
            .expected
            .iter()
            .flat_map(|kind| kind.to_strs())
            .copied()
            .collect();

        comma_separate(f, &expected)?;

        if let Some(found) = self.found {
            write!(f, " but found ")?;
            comma_separate(f, found.to_strs())?;
        }

        Ok(())
    }
}

fn comma_separate(f: &mut fmt::Formatter<'_>, items: &[&str]) -> fmt::Result {
    let is_first = |idx| idx == 0;
    let is_last = |idx| idx == items.len() - 1;
    for (idx, expected_kind) in items.iter().enumerate() {
        if is_first(idx) {
            write!(f, "{}", expected_kind)?;
        } else if is_last(idx) {
            write!(f, " or {}", expected_kind)?;
        } else {
            write!(f, ", {}", expected_kind)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range as StdRange;

    fn check<const N: usize>(
        expected: [SyntaxKind; N],
        found: Option<SyntaxKind>,
        range: StdRange<u32>,
        output: &str,
    ) {
        let error = ParseError {
            expected: IntoIterator::into_iter(expected).collect(),
            found,
            range: {
                let start = range.start.into();
                let end = range.end.into();
                TextRange::new(start, end)
            },
        };

        assert_eq!(format!("{}", error), output);
    }

    #[test]
    fn one_expected_did_find() {
        check(
            [SyntaxKind::Period],
            Some(SyntaxKind::Question),
            91..92,
            "error at 91..92: expected ‘.’ but found ‘?’",
        );
    }

    #[test]
    fn one_expected_didnt_find() {
        check(
            [SyntaxKind::Period],
            None,
            91..92,
            "error at 91..92: expected ‘.’",
        );
    }

    #[test]
    fn multiple_expected_did_find() {
        check(
            [
                SyntaxKind::Period,
                SyntaxKind::Exclamation,
                SyntaxKind::Comma,
            ],
            Some(SyntaxKind::Question),
            91..92,
            "error at 91..92: expected ‘.’, ‘!’ or ‘,’ but found ‘?’",
        );
    }

    #[test]
    fn multiple_expected_didnt_find() {
        check(
            [SyntaxKind::Period, SyntaxKind::Exclamation],
            None,
            91..92,
            "error at 91..92: expected ‘.’ or ‘!’",
        );
    }
}
