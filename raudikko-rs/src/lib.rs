//! # Raudikko
//!
//! Raudikko is a Rust library for performing morphological analysis on Finnish language.
//! It uses Voikko's morphology files and is based on Voikko, but implemented purely in Rust
//! with no native library dependencies.
//!
//! ## Usage
//!
//! ```no_run
//! use raudikko::{Morphology, AnalyzerConfig};
//!
//! // Load the morphology (expensive operation, do once)
//! let morphology = Morphology::load_bundled()?;
//!
//! // Create an analyzer (cheap operation, can be reused)
//! let mut analyzer = morphology.new_analyzer();
//!
//! // Analyze words
//! let analyses = analyzer.analyze("kissoittansa");
//! for analysis in analyses {
//!     println!("Base form: {:?}", analysis.base_form);
//! }
//!
//! // Get just base forms
//! let base_forms = analyzer.base_forms("autokauppa");
//! # Ok::<(), raudikko::MorphologyError>(())
//! ```

pub mod analysis;
pub mod analyzer;
pub mod config;
pub mod error;
pub mod morphology;
pub mod structure;
pub mod word;

// Re-export main types at crate root
pub use analysis::Analysis;
pub use analyzer::Analyzer;
pub use config::{AnalyzerConfig, AnalyzerConfigBuilder};
pub use error::{MorphologyError, StructureError};
pub use morphology::Morphology;
pub use structure::{Structure, StructureSymbol};
pub use word::{SingleWordPart, StrongMorphemeCompoundWordPart, Word, WordPart};

// Re-export analysis enums
pub use analysis::{
    Comparison, FocusParticle, GrammaticalNumber, Locative, Mood, Negative, Participle, Person,
    Possessive, Tense, WordClass,
};
