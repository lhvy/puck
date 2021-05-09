use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum TokenKind {
    #[regex("(?i)(am|are|art|be|is)")]
    Be,

    #[regex("(?i)(a|an|the)")]
    Article,

    #[regex("(?i)(i|me)")]
    FirstPerson,

    #[regex("(?i)(myself)")]
    FirstPersonReflexive,

    #[regex("(?i)(mine|my)")]
    FirstPersonPossessive,

    #[regex("(?i)(thee|thou|you)")]
    SecondPerson,

    #[regex("(?i)(thyself|yourself)")]
    SecondPersonReflexive,

    #[regex("(?i)(thine|thy|your)")]
    SecondPersonPossessive,

    #[regex("(?i)(his|her|its|their)")]
    ThirdPersonPossessive,

    #[error]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = TokenKind::lexer(input);

        let token_kind = lexer.next().unwrap();
        let text = lexer.slice();

        assert_eq!(token_kind, kind);
        assert_eq!(text, input);
    }

    #[test]
    fn lex_be() {
        check("am", TokenKind::Be);
        check("Are", TokenKind::Be);
        check("aRT", TokenKind::Be);
        check("be", TokenKind::Be);
        check("iS", TokenKind::Be);
    }

    #[test]
    fn lex_article() {
        check("a", TokenKind::Article);
        check("aN", TokenKind::Article);
        check("The", TokenKind::Article);
    }

    #[test]
    fn lex_first_person() {
        check("I", TokenKind::FirstPerson);
        check("me", TokenKind::FirstPerson);
    }

    #[test]
    fn lex_first_person_reflexive() {
        check("myself", TokenKind::FirstPersonReflexive);
        check("Myself", TokenKind::FirstPersonReflexive);
    }

    #[test]
    fn lex_first_person_possessive() {
        check("mine", TokenKind::FirstPersonPossessive);
        check("My", TokenKind::FirstPersonPossessive);
    }

    #[test]
    fn lex_second_person() {
        check("thee", TokenKind::SecondPerson);
        check("Thou", TokenKind::SecondPerson);
        check("yOu", TokenKind::SecondPerson);
    }

    #[test]
    fn lex_second_person_reflexive() {
        check("thyself", TokenKind::SecondPersonReflexive);
        check("Yourself", TokenKind::SecondPersonReflexive);
    }

    #[test]
    fn lex_second_person_possessive() {
        check("thine", TokenKind::SecondPersonPossessive);
        check("Thy", TokenKind::SecondPersonPossessive);
        check("yoUR", TokenKind::SecondPersonPossessive);
    }

    #[test]
    fn lex_third_person_possessive() {
        check("his", TokenKind::ThirdPersonPossessive);
        check("Her", TokenKind::ThirdPersonPossessive);
        check("iTs", TokenKind::ThirdPersonPossessive);
        check("theIR", TokenKind::ThirdPersonPossessive);
    }
}
