//! Analysis results and related types

use std::collections::HashMap;

mod enums;
pub use enums::*;

use crate::{Structure, Word};

/// Result of analyzing a word.
///
/// Note that some properties are named in English and some in Finnish
/// for naming compatibility with Voikko.
#[derive(Debug, Clone, Default)]
pub struct Analysis {
    /// Base form (dictionary form) of the word
    pub base_form: Option<String>,

    /// Word class (noun, verb, adjective, etc.)
    pub word_class: Option<WordClass>,

    /// Grammatical case
    pub locative: Option<Locative>,

    /// Comparison form (comparative, superlative)
    pub comparison: Option<Comparison>,

    /// Focus particle
    pub focus: Option<FocusParticle>,

    /// Raw FST output
    pub fst_output: Option<String>,

    /// Morpheme structure information
    pub structure: Option<Structure>,

    /// Grammatical number (singular/plural)
    pub number: Option<GrammaticalNumber>,

    /// Negative form
    pub negative: Option<Negative>,

    /// Mood
    pub mood: Option<Mood>,

    /// Participle form
    pub participle: Option<Participle>,

    /// Person
    pub person: Option<Person>,

    /// Possessive suffix
    pub possessive: Option<Possessive>,

    /// Tense
    pub tense: Option<Tense>,

    /// Interrogative form
    pub interrogative: bool,

    /// Malaga vapaa jälkiosa
    pub malaga_vapaa_jalkiosa: bool,

    /// Possible geographical name
    pub possible_geographical_name: bool,

    /// Requires following verb
    pub require_following_verb: Option<Mood>,

    /// Base form parts for compound words
    pub base_form_parts: Option<Vec<String>>,

    /// Detailed word part information
    pub word: Option<Word>,
}

impl Analysis {
    /// Converts the analysis results to Voikko-compatible format.
    ///
    /// Returns a map with string keys and values matching Voikko's output format.
    pub fn to_voikko_format(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        if let Some(ref v) = self.base_form {
            map.insert("BASEFORM".to_string(), v.clone());
        }
        if let Some(ref v) = self.word_class {
            map.insert("CLASS".to_string(), v.legacy_code().to_string());
        }
        if let Some(ref v) = self.locative {
            map.insert("SIJAMUOTO".to_string(), v.legacy_code().to_string());
        }
        if let Some(ref v) = self.comparison {
            map.insert("COMPARISON".to_string(), v.legacy_code().to_string());
        }
        if let Some(ref v) = self.focus {
            map.insert("FOCUS".to_string(), v.legacy_code().to_string());
        }
        if let Some(ref v) = self.fst_output {
            map.insert("FSTOUTPUT".to_string(), v.clone());
        }
        if let Some(ref v) = self.structure {
            map.insert("STRUCTURE".to_string(), v.to_string());
        }
        if let Some(ref v) = self.number {
            map.insert("NUMBER".to_string(), v.legacy_code().to_string());
        }
        if let Some(ref v) = self.negative {
            map.insert("NEGATIVE".to_string(), v.legacy_code().to_string());
        }
        if let Some(ref v) = self.mood {
            map.insert("MOOD".to_string(), v.legacy_code().to_string());
        }
        if let Some(ref v) = self.participle {
            map.insert("PARTICIPLE".to_string(), v.legacy_code().to_string());
        }
        if let Some(ref v) = self.person {
            map.insert("PERSON".to_string(), v.legacy_code().to_string());
        }
        if let Some(ref v) = self.possessive {
            map.insert("POSSESSIVE".to_string(), v.legacy_code().to_string());
        }
        if let Some(ref v) = self.tense {
            map.insert("TENSE".to_string(), v.legacy_code().to_string());
        }
        if let Some(ref v) = self.require_following_verb {
            map.insert("REQUIRE_FOLLOWING_VERB".to_string(), v.legacy_code().to_string());
        }
        if self.interrogative {
            map.insert("KYSYMYSLIITE".to_string(), "true".to_string());
        }
        if self.malaga_vapaa_jalkiosa {
            map.insert("MALAGA_VAPAA_JALKIOSA".to_string(), "true".to_string());
        }
        if self.possible_geographical_name {
            map.insert("POSSIBLE_GEOGRAPHICAL_NAME".to_string(), "true".to_string());
        }

        map
    }
}
