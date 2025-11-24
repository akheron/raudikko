//! Enums for grammatical analysis

/// Trait for analysis classification types
pub trait AnalysisClass {
    /// Get the legacy Voikko code
    fn legacy_code(&self) -> &'static str;

    /// Get the morphology tag
    fn morphology_tag(&self) -> &'static str;
}

/// Word class (part of speech)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WordClass {
    Noun,
    Adjective,
    NounAdjective,
    Interjection,
    FirstName,
    LastName,
    Toponym,
    ProperNoun,
    Pronoun,
    Numeral,
    NumeralRoman,
    Verb,
    Abbreviation,
    Adverb,
    Conjunction,
    Adposition,
    Negation,
    Prefix,
}

impl AnalysisClass for WordClass {
    fn legacy_code(&self) -> &'static str {
        match self {
            Self::Noun => "nimisana",
            Self::Adjective => "laatusana",
            Self::NounAdjective => "nimisana_laatusana",
            Self::Interjection => "huudahdussana",
            Self::FirstName => "etunimi",
            Self::LastName => "sukunimi",
            Self::Toponym => "paikannimi",
            Self::ProperNoun => "nimi",
            Self::Pronoun => "asemosana",
            Self::Numeral => "lukusana",
            Self::NumeralRoman => "lukusana",
            Self::Verb => "teonsana",
            Self::Abbreviation => "lyhenne",
            Self::Adverb => "seikkasana",
            Self::Conjunction => "sidesana",
            Self::Adposition => "suhdesana",
            Self::Negation => "kieltosana",
            Self::Prefix => "etuliite",
        }
    }

    fn morphology_tag(&self) -> &'static str {
        match self {
            Self::Noun => "Ln",
            Self::Adjective => "Ll",
            Self::NounAdjective => "Lnl",
            Self::Interjection => "Lh",
            Self::FirstName => "Lee",
            Self::LastName => "Les",
            Self::Toponym => "Lep",
            Self::ProperNoun => "Lem",
            Self::Pronoun => "Lr",
            Self::Numeral => "Lu",
            Self::NumeralRoman => "Lur",
            Self::Verb => "Lt",
            Self::Abbreviation => "La",
            Self::Adverb => "Ls",
            Self::Conjunction => "Lc",
            Self::Adposition => "Ld",
            Self::Negation => "Lk",
            Self::Prefix => "Lp",
        }
    }
}

/// Finnish grammatical cases (sijamuoto)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locative {
    Nominative,
    Genitive,
    Partitive,
    Essive,
    Translative,
    Inessive,
    Elative,
    Illative,
    Adessive,
    Ablative,
    Allative,
    Abessive,
    Comitative,
    Instructive,
    Accusative,
}

impl AnalysisClass for Locative {
    fn legacy_code(&self) -> &'static str {
        match self {
            Self::Nominative => "nimentö",
            Self::Genitive => "omanto",
            Self::Partitive => "osanto",
            Self::Essive => "olento",
            Self::Translative => "tulento",
            Self::Inessive => "sisäolento",
            Self::Elative => "sisäeronto",
            Self::Illative => "sisätulento",
            Self::Adessive => "ulko-olento",
            Self::Ablative => "ulkoeronto",
            Self::Allative => "ulkotulento",
            Self::Abessive => "vajanto",
            Self::Comitative => "seuranto",
            Self::Instructive => "keinonto",
            Self::Accusative => "kohdanto",
        }
    }

    fn morphology_tag(&self) -> &'static str {
        match self {
            Self::Nominative => "Sn",
            Self::Genitive => "Sg",
            Self::Partitive => "Sp",
            Self::Essive => "Ses",
            Self::Translative => "Str",
            Self::Inessive => "Sine",
            Self::Elative => "Sela",
            Self::Illative => "Sill",
            Self::Adessive => "Sade",
            Self::Ablative => "Sabl",
            Self::Allative => "Sall",
            Self::Abessive => "Sab",
            Self::Comitative => "Sko",
            Self::Instructive => "Sin",
            Self::Accusative => "Sak",
        }
    }
}

/// Grammatical number
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GrammaticalNumber {
    Singular,
    Plural,
}

impl AnalysisClass for GrammaticalNumber {
    fn legacy_code(&self) -> &'static str {
        match self {
            Self::Singular => "singular",
            Self::Plural => "plural",
        }
    }

    fn morphology_tag(&self) -> &'static str {
        match self {
            Self::Singular => "Ny",
            Self::Plural => "Nm",
        }
    }
}

/// Comparison forms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comparison {
    Comparative,
    Superlative,
}

impl AnalysisClass for Comparison {
    fn legacy_code(&self) -> &'static str {
        match self {
            Self::Comparative => "comparative",
            Self::Superlative => "superlative",
        }
    }

    fn morphology_tag(&self) -> &'static str {
        match self {
            Self::Comparative => "Cc",
            Self::Superlative => "Cs",
        }
    }
}

/// Verb moods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mood {
    Indicative,
    Conditional,
    Imperative,
    Potential,
}

impl AnalysisClass for Mood {
    fn legacy_code(&self) -> &'static str {
        match self {
            Self::Indicative => "indicative",
            Self::Conditional => "conditional",
            Self::Imperative => "imperative",
            Self::Potential => "potential",
        }
    }

    fn morphology_tag(&self) -> &'static str {
        match self {
            Self::Indicative => "Tt",
            Self::Conditional => "Te",
            Self::Imperative => "Tk",
            Self::Potential => "Tm",
        }
    }
}

/// Verb tenses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tense {
    Present,
    Past,
}

impl AnalysisClass for Tense {
    fn legacy_code(&self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Past => "past",
        }
    }

    fn morphology_tag(&self) -> &'static str {
        match self {
            Self::Present => "Ap",
            Self::Past => "Ai",
        }
    }
}

/// Grammatical person
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Person {
    First,
    Second,
    Third,
    Fourth,
}

impl AnalysisClass for Person {
    fn legacy_code(&self) -> &'static str {
        match self {
            Self::First => "1",
            Self::Second => "2",
            Self::Third => "3",
            Self::Fourth => "4",
        }
    }

    fn morphology_tag(&self) -> &'static str {
        match self {
            Self::First => "P1",
            Self::Second => "P2",
            Self::Third => "P3",
            Self::Fourth => "P4",
        }
    }
}

/// Participle forms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Participle {
    PresentActive,
    PresentPassive,
    PastActive,
    PastPassive,
    Agent,
    Negative,
}

impl AnalysisClass for Participle {
    fn legacy_code(&self) -> &'static str {
        match self {
            Self::PresentActive => "present_active",
            Self::PresentPassive => "present_passive",
            Self::PastActive => "past_active",
            Self::PastPassive => "past_passive",
            Self::Agent => "agent",
            Self::Negative => "negative",
        }
    }

    fn morphology_tag(&self) -> &'static str {
        match self {
            Self::PresentActive => "Rv",
            Self::PresentPassive => "Ra",
            Self::PastActive => "Ru",
            Self::PastPassive => "Rt",
            Self::Agent => "Rm",
            Self::Negative => "Re",
        }
    }
}

/// Negative forms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Negative {
    True,
    False,
    Both,
}

impl AnalysisClass for Negative {
    fn legacy_code(&self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
            Self::Both => "both",
        }
    }

    fn morphology_tag(&self) -> &'static str {
        match self {
            Self::True => "Et",
            Self::False => "Ef",
            Self::Both => "Eb",
        }
    }
}

/// Possessive suffixes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Possessive {
    FirstSingular,
    SecondSingular,
    FirstPlural,
    SecondPlural,
    Third,
}

impl AnalysisClass for Possessive {
    fn legacy_code(&self) -> &'static str {
        match self {
            Self::FirstSingular => "1s",
            Self::SecondSingular => "2s",
            Self::FirstPlural => "1p",
            Self::SecondPlural => "2p",
            Self::Third => "3",
        }
    }

    fn morphology_tag(&self) -> &'static str {
        match self {
            Self::FirstSingular => "O1y",
            Self::SecondSingular => "O2y",
            Self::FirstPlural => "O1m",
            Self::SecondPlural => "O2m",
            Self::Third => "O3",
        }
    }
}

/// Focus particles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FocusParticle {
    Kin,
    Kaan,
}

impl AnalysisClass for FocusParticle {
    fn legacy_code(&self) -> &'static str {
        match self {
            Self::Kin => "kin",
            Self::Kaan => "kaan",
        }
    }

    fn morphology_tag(&self) -> &'static str {
        match self {
            Self::Kin => "Fkin",
            Self::Kaan => "Fkaan",
        }
    }
}
