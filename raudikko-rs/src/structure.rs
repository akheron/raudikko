//! Morpheme structure and hyphenation information

use crate::StructureError;

/// Describes morpheme boundaries, character case, and hyphenation restrictions.
///
/// # Examples
///
/// - "Matti-niminen" → "=ipppp-=ppppppp"
/// - "DNA-näyte" → "=jjj-=ppppp"
/// - "autokauppa" → "=pppp=pppppp"
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Structure {
    symbols: Vec<StructureSymbol>,
}

impl Structure {
    /// Create a new structure from symbols
    ///
    /// # Errors
    ///
    /// Returns an error if the symbol list is empty
    pub fn new(symbols: Vec<StructureSymbol>) -> Result<Self, StructureError> {
        if symbols.is_empty() {
            return Err(StructureError::Empty);
        }
        Ok(Self { symbols })
    }

    /// Get the number of morphemes in this structure
    pub fn morpheme_count(&self) -> usize {
        self.symbols
            .iter()
            .filter(|s| **s == StructureSymbol::MorphemeStart)
            .count()
    }

    /// Returns a structure that is otherwise identical to this one, but starts with a capital letter
    pub fn capitalized(&self) -> Self {
        if self.symbols.len() <= 1 || self.symbols[1] == StructureSymbol::Uppercase {
            return self.clone();
        }

        let mut symbols = self.symbols.clone();
        symbols[1] = StructureSymbol::Uppercase;
        Self { symbols }
    }

    /// Returns an iterator over non-morpheme symbols
    pub fn non_morphemes(&self) -> impl Iterator<Item = &StructureSymbol> {
        self.symbols
            .iter()
            .filter(|s| **s != StructureSymbol::MorphemeStart)
    }

    /// Converts given word to follow this structure
    pub fn apply(&self, word: &str) -> String {
        let mut non_morphemes = self.non_morphemes();

        word.chars()
            .map(|ch| {
                non_morphemes
                    .next()
                    .map(|sym| sym.convert(ch))
                    .unwrap_or(ch)
            })
            .collect()
    }
}

impl std::fmt::Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for symbol in &self.symbols {
            write!(f, "{}", symbol.code())?;
        }
        Ok(())
    }
}

/// Individual symbols in a structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructureSymbol {
    /// Start of a new morpheme (=)
    MorphemeStart,

    /// Letter written in upper case in standard form (i)
    Uppercase,

    /// Letter in upper case, hyphenation forbidden before (j)
    UppercaseNoHyphenation,

    /// Letter written in lower case in standard form (p)
    Lowercase,

    /// Letter in lower case, hyphenation forbidden before (q)
    LowercaseNoHyphenation,

    /// Hyphen - word can be split without adding extra hyphen (-)
    Hyphen,

    /// Colon in the word (:)
    Colon,
}

impl StructureSymbol {
    /// Get the character code for this symbol
    pub fn code(&self) -> char {
        match self {
            Self::MorphemeStart => '=',
            Self::Uppercase => 'i',
            Self::UppercaseNoHyphenation => 'j',
            Self::Lowercase => 'p',
            Self::LowercaseNoHyphenation => 'q',
            Self::Hyphen => '-',
            Self::Colon => ':',
        }
    }

    /// Create a symbol from its character code
    pub fn from_code(c: char) -> Result<Self, StructureError> {
        match c {
            '=' => Ok(Self::MorphemeStart),
            'i' => Ok(Self::Uppercase),
            'j' => Ok(Self::UppercaseNoHyphenation),
            'p' => Ok(Self::Lowercase),
            'q' => Ok(Self::LowercaseNoHyphenation),
            '-' => Ok(Self::Hyphen),
            ':' => Ok(Self::Colon),
            _ => Err(StructureError::UnknownCode(c)),
        }
    }

    /// Convert a character according to this symbol's case
    pub fn convert(&self, ch: char) -> char {
        match self {
            Self::Uppercase | Self::UppercaseNoHyphenation => {
                ch.to_uppercase().next().unwrap_or(ch)
            }
            Self::Lowercase | Self::LowercaseNoHyphenation => {
                ch.to_lowercase().next().unwrap_or(ch)
            }
            _ => ch,
        }
    }

    /// Check if this symbol agrees with the given character
    pub fn agrees(&self, ch: char) -> bool {
        match self {
            Self::MorphemeStart => true,
            Self::Uppercase | Self::UppercaseNoHyphenation => !ch.is_lowercase(),
            Self::Lowercase | Self::LowercaseNoHyphenation => !ch.is_uppercase(),
            Self::Hyphen => ch == '-',
            Self::Colon => ch == ':',
        }
    }

    /// Check if this is a lowercase symbol
    pub fn is_lowercase(&self) -> bool {
        matches!(self, Self::Lowercase | Self::LowercaseNoHyphenation)
    }

    /// Check if this is an uppercase symbol
    pub fn is_uppercase(&self) -> bool {
        matches!(self, Self::Uppercase | Self::UppercaseNoHyphenation)
    }
}
