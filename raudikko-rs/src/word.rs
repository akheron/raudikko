//! Word and word part representations

use std::collections::HashSet;

/// Represents a word composed of one or more parts
#[derive(Debug, Clone)]
pub struct Word {
    parts: Vec<WordPart>,
}

impl Word {
    /// Create a new word from parts
    pub fn new(parts: Vec<WordPart>) -> Self {
        Self { parts }
    }

    /// Get the word parts
    pub fn parts(&self) -> &[WordPart] {
        &self.parts
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for part in &self.parts {
            write!(f, "{}", part)?;
        }
        Ok(())
    }
}

/// A part of a word (either single or compound)
#[derive(Debug, Clone)]
pub enum WordPart {
    Single(SingleWordPart),
    StrongMorphemeCompound(StrongMorphemeCompoundWordPart),
}

impl WordPart {
    /// Get base forms for this word part
    pub fn base_forms(&self) -> &[String] {
        match self {
            Self::Single(s) => &s.base_forms,
            Self::StrongMorphemeCompound(c) => std::slice::from_ref(&c.base_form),
        }
    }

    /// Check if this word part is in base form
    pub fn is_in_base_form(&self) -> bool {
        match self {
            Self::Single(s) => s.is_in_base_form(),
            Self::StrongMorphemeCompound(c) => c.is_in_base_form(),
        }
    }

    /// Check if this word part is a proper noun
    pub fn is_proper_noun(&self) -> bool {
        match self {
            Self::Single(s) => s.is_proper_noun(),
            Self::StrongMorphemeCompound(c) => c.is_proper_noun(),
        }
    }
}

impl std::fmt::Display for WordPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(s) => write!(f, "{}", s),
            Self::StrongMorphemeCompound(c) => write!(f, "{}", c),
        }
    }
}

/// A single word part with grammatical attributes
#[derive(Debug, Clone)]
pub struct SingleWordPart {
    word: String,
    base_forms: Vec<String>,
    attributes: HashSet<WordAttribute>,
}

impl SingleWordPart {
    /// Create a new single word part
    pub fn new(word: String, base_forms: Vec<String>, attributes: HashSet<WordAttribute>) -> Self {
        Self {
            word,
            base_forms,
            attributes,
        }
    }

    /// Check if this word is in nominative case
    pub fn is_nominative(&self) -> bool {
        self.attributes.contains(&WordAttribute::Sn)
    }

    /// Check if this word is singular
    pub fn is_singular(&self) -> bool {
        self.attributes.contains(&WordAttribute::Ny)
    }

    /// Check if this word has a clitic particle
    pub fn is_clitic(&self) -> bool {
        self.attributes.contains(&WordAttribute::Fko)
            || self.attributes.contains(&WordAttribute::Fkin)
            || self.attributes.contains(&WordAttribute::Fkaan)
    }

    /// Check if this word has a possessive suffix
    pub fn is_possessive_suffix(&self) -> bool {
        self.attributes.contains(&WordAttribute::O3)
            || self.attributes.contains(&WordAttribute::O2y)
            || self.attributes.contains(&WordAttribute::O2m)
            || self.attributes.contains(&WordAttribute::O1y)
            || self.attributes.contains(&WordAttribute::O1m)
    }

    /// Check if this word is in comparative form
    pub fn is_comparative(&self) -> bool {
        self.attributes.contains(&WordAttribute::Cc)
            || self.attributes.contains(&WordAttribute::Cs)
    }

    /// Check if this is a proper noun
    pub fn is_proper_noun(&self) -> bool {
        self.attributes.contains(&WordAttribute::Lee)
            || self.attributes.contains(&WordAttribute::Les)
            || self.attributes.contains(&WordAttribute::Lep)
            || self.attributes.contains(&WordAttribute::Lem)
    }

    /// Check if this word is in its base form
    pub fn is_in_base_form(&self) -> bool {
        self.is_nominative()
            && self.is_singular()
            && !self.is_clitic()
            && !self.is_possessive_suffix()
            && !self.is_comparative()
    }
}

impl std::fmt::Display for SingleWordPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word)
    }
}

/// A compound word part with strong morpheme boundaries
#[derive(Debug, Clone)]
pub struct StrongMorphemeCompoundWordPart {
    parts: Vec<SingleWordPart>,
    base_form: String,
}

impl StrongMorphemeCompoundWordPart {
    /// Create a new compound word part
    pub fn new(parts: Vec<SingleWordPart>, base_form: String) -> Self {
        Self { parts, base_form }
    }

    /// Get the individual parts
    pub fn parts(&self) -> &[SingleWordPart] {
        &self.parts
    }

    /// Get the base form
    pub fn base_form(&self) -> &str {
        &self.base_form
    }

    /// Check if this compound is in base form
    pub fn is_in_base_form(&self) -> bool {
        self.parts
            .last()
            .map(|p| p.is_in_base_form())
            .unwrap_or(false)
    }

    /// Check if this is a proper noun compound
    pub fn is_proper_noun(&self) -> bool {
        self.parts
            .first()
            .map(|p| p.is_proper_noun())
            .unwrap_or(false)
    }
}

impl std::fmt::Display for StrongMorphemeCompoundWordPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for part in &self.parts {
            write!(f, "{}", part)?;
        }
        Ok(())
    }
}

/// Word attributes (grammatical tags)
///
/// These correspond to the internal morphology tags used by Voikko.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum WordAttribute {
    // Word classes
    Ln,
    Lee,
    Les,
    Lep,
    Lem,
    Ll,
    Lnl,
    Lt,
    Lh,
    Lp,
    La,
    Ls,
    Lc,
    Lu,
    Lur,
    Lr,
    Ld,
    Lk,

    // Cases
    Sn,
    Sg,
    Sp,
    Ses,
    Str,
    Sine,
    Sela,
    Sill,
    Sade,
    Sabl,
    Sall,
    Sab,
    Sko,
    Sin,
    Ssti,
    Sak,

    // Numbers
    Ny,
    Nm,

    // Possessive suffixes
    O1y,
    O2y,
    O1m,
    O2m,
    O3,

    // Comparison
    Cc,
    Cs,

    // Focus particles
    Fkin,
    Fkaan,
    Fko,

    // Moods
    Tn1,
    Tn2,
    Tn3,
    Tn4,
    Tn5,
    Tt,
    Te,
    Tk,
    Tm,

    // Participles
    Rv,
    Ra,
    Ru,
    Rt,
    Rm,
    Re,

    // Person
    P1,
    P2,
    P3,
    P4,

    // Tense
    Ap,
    Ai,

    // Negative
    Et,
    Ef,
    Eb,

    // Other flags
    Dg,
    De,
    Ips,
    Ipu,
    Isf,
    Icu,
    Ica,
    Ivj,
    Ion,
    Ira,
    Irm,
    Bc,
    Bh,
    Bm,

    // Vowel harmony
    Va,
    Vä,
    Vää,
}

impl WordAttribute {
    /// Parse a WordAttribute from an FST symbol tag.
    /// Tags are in the format `[AttributeName]`.
    pub fn from_tag(tag: &crate::internal::fst::Symbol) -> Option<Self> {
        let tag_str = tag.to_string();

        if !tag_str.starts_with('[') || !tag_str.ends_with(']') {
            return None;
        }

        let name = &tag_str[1..tag_str.len() - 1];

        match name {
            "Ln" => Some(Self::Ln),
            "Lee" => Some(Self::Lee),
            "Les" => Some(Self::Les),
            "Lep" => Some(Self::Lep),
            "Lem" => Some(Self::Lem),
            "Ll" => Some(Self::Ll),
            "Lnl" => Some(Self::Lnl),
            "Lt" => Some(Self::Lt),
            "Lh" => Some(Self::Lh),
            "Lp" => Some(Self::Lp),
            "La" => Some(Self::La),
            "Ls" => Some(Self::Ls),
            "Lc" => Some(Self::Lc),
            "Lu" => Some(Self::Lu),
            "Lur" => Some(Self::Lur),
            "Lr" => Some(Self::Lr),
            "Ld" => Some(Self::Ld),
            "Lk" => Some(Self::Lk),
            "Sn" => Some(Self::Sn),
            "Sg" => Some(Self::Sg),
            "Sp" => Some(Self::Sp),
            "Ses" => Some(Self::Ses),
            "Str" => Some(Self::Str),
            "Sine" => Some(Self::Sine),
            "Sela" => Some(Self::Sela),
            "Sill" => Some(Self::Sill),
            "Sade" => Some(Self::Sade),
            "Sabl" => Some(Self::Sabl),
            "Sall" => Some(Self::Sall),
            "Sab" => Some(Self::Sab),
            "Sko" => Some(Self::Sko),
            "Sin" => Some(Self::Sin),
            "Ssti" => Some(Self::Ssti),
            "Sak" => Some(Self::Sak),
            "Ny" => Some(Self::Ny),
            "Nm" => Some(Self::Nm),
            "O1y" => Some(Self::O1y),
            "O2y" => Some(Self::O2y),
            "O1m" => Some(Self::O1m),
            "O2m" => Some(Self::O2m),
            "O3" => Some(Self::O3),
            "Cc" => Some(Self::Cc),
            "Cs" => Some(Self::Cs),
            "Fkin" => Some(Self::Fkin),
            "Fkaan" => Some(Self::Fkaan),
            "Fko" => Some(Self::Fko),
            "Tn1" => Some(Self::Tn1),
            "Tn2" => Some(Self::Tn2),
            "Tn3" => Some(Self::Tn3),
            "Tn4" => Some(Self::Tn4),
            "Tn5" => Some(Self::Tn5),
            "Tt" => Some(Self::Tt),
            "Te" => Some(Self::Te),
            "Tk" => Some(Self::Tk),
            "Tm" => Some(Self::Tm),
            "Rv" => Some(Self::Rv),
            "Ra" => Some(Self::Ra),
            "Ru" => Some(Self::Ru),
            "Rt" => Some(Self::Rt),
            "Rm" => Some(Self::Rm),
            "Re" => Some(Self::Re),
            "P1" => Some(Self::P1),
            "P2" => Some(Self::P2),
            "P3" => Some(Self::P3),
            "P4" => Some(Self::P4),
            "Ap" => Some(Self::Ap),
            "Ai" => Some(Self::Ai),
            "Et" => Some(Self::Et),
            "Ef" => Some(Self::Ef),
            "Eb" => Some(Self::Eb),
            "Dg" => Some(Self::Dg),
            "De" => Some(Self::De),
            "Ips" => Some(Self::Ips),
            "Ipu" => Some(Self::Ipu),
            "Isf" => Some(Self::Isf),
            "Icu" => Some(Self::Icu),
            "Ica" => Some(Self::Ica),
            "Ivj" => Some(Self::Ivj),
            "Ion" => Some(Self::Ion),
            "Ira" => Some(Self::Ira),
            "Irm" => Some(Self::Irm),
            "Bc" => Some(Self::Bc),
            "Bh" => Some(Self::Bh),
            "Bm" => Some(Self::Bm),
            "Va" => Some(Self::Va),
            "Vä" => Some(Self::Vä),
            "Vää" => Some(Self::Vää),
            _ => None,
        }
    }
}
