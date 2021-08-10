use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (SyntaxKind, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let syntax_kind = self.inner.next()?;
        let slice = self.inner.slice();

        Some((syntax_kind, slice))
    }
}

#[derive(Logos, Debug, PartialEq, FromPrimitive, ToPrimitive, Clone, Copy)]
pub(crate) enum SyntaxKind {
    Root,

    CharacterDef,

    Comment,

    Skip,

    StageDirection,

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

    #[regex("(?i)(better|bigger|fresher|friendlier|nicer|jollier)")]
    PositiveComparative,

    #[regex("(?i)(punier|smaller|worse)")]
    NegativeComparative,

    #[regex("(?i)(more)")]
    More,

    #[regex("(?i)(than)")]
    Than,

    #[regex("(?i)(as)")]
    As,

    #[regex("(?i)(and)")]
    And,

    #[regex("(?i)(to)")]
    To,

    #[regex("(?i)(of)")]
    Of,

    #[regex("(?i)(if)")]
    If,

    #[regex("(?i)(not)")]
    Not,

    #[regex("(?i)(so)")]
    So,

    #[regex("(?i)(proceed|return)")]
    Jump,

    #[regex("(?i)(let)")]
    Let,

    #[regex("(?i)(we)")]
    We,

    #[regex("(?i)(us)")]
    Us,

    #[regex("(?i)(shall|must)")]
    Must,

    #[regex("(?i)(difference)")]
    Difference,

    #[regex("(?i)(product)")]
    Product,

    #[regex("(?i)(quotient)")]
    Quotient,

    #[regex("(?i)(sum)")]
    Sum,

    #[regex("(?i)(remainder)")]
    Remainder,

    #[regex("(?i)(cube)")]
    Cube,

    #[regex("(?i)(square)")]
    Square,

    #[regex("(?i)(square root)")]
    SquareRoot,

    #[regex("(?i)(factorial)")]
    Factorial,

    #[regex("(?i)(twice)")]
    Twice,

    #[regex("(?i)(between)")]
    Between,

    #[regex("(?i)(bad|cowardly|cursed|damned|dirty|disgusting|distasteful|dusty|evil|fat|fat-kidneyed|fatherless|foul|hairy|half-witted|horrible|horrid|infected|lying|miserable|misused|oozing|rotten|rotten|smelly|snotty|sorry|stinking|stuffed|stupid|vile|villainous|worried)")]
    NegativeAdjective,

    #[regex("(?i)(big|black|blue|bluest|bottomless|furry|green|hard|huge|large|little|normal|old|purple|red|rural|small|tiny|white|yellow)")]
    NeutralAdjective,

    #[regex("(?i)(amazing|beautiful|blossoming|bold|brave|charming|clearest|cunning|cute|delicious|embroidered|fair|fine|gentle|golden|good|handsome|happy|healthy|honest|lovely|loving|mighty|noble|peaceful|pretty|prompt|proud|reddest|rich|smooth|sunny|sweet|sweetest|trustworthy|warm)")]
    PositiveAdjective,

    #[regex("(?i)(hell|bastard|beggar|blister|codpiece|coward|curse|death|devil|draught|famine|flirt-gill|goat|hate|hog|hound|leech|lie|pig|plague|starvation|toad|war|wolf)")]
    NegativeNoun,

    #[regex("(?i)(animal|aunt|brother|cat|chihuahua|cousin|cow|daughter|door|face|father|fellow|granddaughter|grandfather|grandmother|grandson|hair|hamster|horse|lamp|lantern|mistletoe|moon|morning|mother|nephew|niece|nose|purse|road|roman|sister|sky|son|squirrel|stone wall|thing|town|tree|uncle|wind)")]
    NeutralNoun,

    #[regex("(?i)(heaven|king|lord|angel|flower|happiness|joy|plum|summer's day|hero|rose|kingdom|pony)")]
    PositiveNoun,

    #[regex("(?i)(achilles|adonis|adriana|aegeon|aemilia|agamemnon|agrippa|ajax|alonso|andromache|angelo|antiochus|antonio|arthur|autolycus|balthazar|banquo|beatrice|benedick|benvolio|bianca|brabantio|brutus|capulet|cassandra|cassius|christopher sly|cicero|claudio|claudius|cleopatra|cordelia|cornelius|cressida|cymberline|demetrius|desdemona|dionyza|doctor caius|dogberry|don john|don pedro|donalbain|dorcas|duncan|egeus|emilia|escalus|falstaff|fenton|ferdinand|ford|fortinbras|francisca|friar john|friar laurence|gertrude|goneril|hamlet|hecate|hector|helen|helena|hermia|hermonie|hippolyta|horatio|imogen|isabella|john of gaunt|john of lancaster|julia|juliet|julius caesar|king henry|king john|king lear|king richard|lady capulet|lady macbeth|lady macduff|lady montague|lennox|leonato|luciana|lucio|lychorida|lysander|macbeth|macduff|malcolm|mariana|mark antony|mercutio|miranda|mistress ford|mistress overdone|mistress page|montague|mopsa|oberon|octavia|octavius caesar|olivia|ophelia|orlando|orsino|othello|page|pantino|paris|pericles|pinch|polonius|pompeius|portia|priam|prince henry|prospero|proteus|publius|puck|queen elinor|regan|robin|romeo|rosalind|sebastian|shallow|shylock|slender|solinus|stephano|thaisa|the abbot of westminster|the apothecary|the archbishop of canterbury|the duke of milan|the duke of venice|the ghost|theseus|thurio|timon|titania|titus|troilus|tybalt|ulysses|valentine|venus|vincentio|viol)")]
    Character,

    #[regex("(?i)(nothing|zero)")]
    Nothing,

    #[regex("(?i)(open)")]
    Open,

    #[regex("(?i)(speak)")]
    Speak,

    #[regex("(?i)(listen)")]
    Listen,

    #[regex("(?i)(heart)")]
    Heart,

    #[regex("(?i)(mind)")]
    Mind,

    #[regex("(?i)(remember)")]
    Remember,

    #[regex("(?i)(recall)")]
    Recall,

    #[regex("(?i)(scene)")]
    Scene,

    #[regex("(?i)(act)")]
    Act,

    #[regex("(?i)(pause)")]
    Pause,

    #[regex("(?i)(enter)")]
    Enter,

    #[regex("(?i)(exit)")]
    Exit,

    #[regex("(?i)(exeunt)")]
    Exeunt,

    #[token(".")]
    Period,

    #[token("!")]
    Exclamation,

    #[token("?")]
    Question,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[regex("M*(CM|CD|D?C*)(XC|XL|L?X*)(IX|IV|V?I*)", roman_numeral)]
    RomanNumeral,

    #[token(" ")]
    Whitespace,

    #[token("\n")]
    Newline,

    #[error]
    Error,
}

fn roman_numeral(lex: &mut logos::Lexer<SyntaxKind>) -> bool {
    let slice = lex.slice();
    let regex =
        regex::Regex::new("M{0,4}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})").unwrap();

    if let Some(roman_numeral) = regex.find(slice) {
        return roman_numeral.start() == 0 && roman_numeral.end() == slice.len();
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = SyntaxKind::lexer(input);

        let token_kind = lexer.next().unwrap();
        let text = lexer.slice();

        assert_eq!(token_kind, kind);
        assert_eq!(text, input);
    }

    #[test]
    fn lex_be() {
        check("am", SyntaxKind::Be);
        check("Are", SyntaxKind::Be);
        check("aRT", SyntaxKind::Be);
        check("be", SyntaxKind::Be);
        check("iS", SyntaxKind::Be);
    }

    #[test]
    fn lex_article() {
        check("a", SyntaxKind::Article);
        check("aN", SyntaxKind::Article);
        check("The", SyntaxKind::Article);
    }

    #[test]
    fn lex_first_person() {
        check("I", SyntaxKind::FirstPerson);
        check("me", SyntaxKind::FirstPerson);
    }

    #[test]
    fn lex_first_person_reflexive() {
        check("myself", SyntaxKind::FirstPersonReflexive);
        check("Myself", SyntaxKind::FirstPersonReflexive);
    }

    #[test]
    fn lex_first_person_possessive() {
        check("mine", SyntaxKind::FirstPersonPossessive);
        check("My", SyntaxKind::FirstPersonPossessive);
    }

    #[test]
    fn lex_second_person() {
        check("thee", SyntaxKind::SecondPerson);
        check("Thou", SyntaxKind::SecondPerson);
        check("yOu", SyntaxKind::SecondPerson);
    }

    #[test]
    fn lex_second_person_reflexive() {
        check("thyself", SyntaxKind::SecondPersonReflexive);
        check("Yourself", SyntaxKind::SecondPersonReflexive);
    }

    #[test]
    fn lex_second_person_possessive() {
        check("thine", SyntaxKind::SecondPersonPossessive);
        check("Thy", SyntaxKind::SecondPersonPossessive);
        check("yoUR", SyntaxKind::SecondPersonPossessive);
    }

    #[test]
    fn lex_third_person_possessive() {
        check("his", SyntaxKind::ThirdPersonPossessive);
        check("Her", SyntaxKind::ThirdPersonPossessive);
        check("iTs", SyntaxKind::ThirdPersonPossessive);
        check("theIR", SyntaxKind::ThirdPersonPossessive);
    }

    #[test]
    fn lex_with_spaces() {
        check("square root", SyntaxKind::SquareRoot);
        check("stone wall", SyntaxKind::NeutralNoun);
        check("lady macbeth", SyntaxKind::Character);
        check(" ", SyntaxKind::Whitespace);
    }
}
