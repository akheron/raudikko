//! Morphology rules and loading

use crate::{Analyzer, AnalyzerConfig, MorphologyError};

/// Represents morphology rules. This type is immutable and can be shared between threads.
///
/// Create using [`Morphology::load_bundled()`] and then create per-thread analyzers
/// using [`new_analyzer()`](Self::new_analyzer).
#[derive(Clone)]
pub struct Morphology {
    // Internal: UnweightedTransducer
    // This would contain the actual FST data structure
    _transducer: (),
}

impl Morphology {
    /// Loads the morphology rules bundled with the library.
    ///
    /// This is a relatively expensive operation that should be done only once.
    /// The loaded morphology is immutable and can be shared between threads.
    ///
    /// # Errors
    ///
    /// Returns an error if the bundled morphology cannot be found or loaded.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use raudikko::Morphology;
    ///
    /// let morphology = Morphology::load_bundled()?;
    /// # Ok::<(), raudikko::MorphologyError>(())
    /// ```
    pub fn load_bundled() -> Result<Self, MorphologyError> {
        // TODO: Load from embedded resources
        // const MORPHOLOGY_DATA: &[u8] = include_bytes!("../resources/morpho/5/mor-morpho/mor.vfst");

        // For now, placeholder
        Ok(Self { _transducer: () })
    }

    /// Create a new [`Analyzer`] for this morphology with default configuration.
    ///
    /// The analyzer is a mutable object that can be used repeatedly, but may not be
    /// shared between threads. Creating an analyzer is a cheap operation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use raudikko::Morphology;
    ///
    /// let morphology = Morphology::load_bundled()?;
    /// let mut analyzer = morphology.new_analyzer();
    /// let analyses = analyzer.analyze("kissa");
    /// # Ok::<(), raudikko::MorphologyError>(())
    /// ```
    pub fn new_analyzer(&self) -> Analyzer<'_> {
        self.new_analyzer_with_config(AnalyzerConfig::default())
    }

    /// Create a new [`Analyzer`] for this morphology with custom configuration.
    ///
    /// The analyzer is a mutable object that can be used repeatedly, but may not be
    /// shared between threads.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use raudikko::{Morphology, AnalyzerConfig};
    ///
    /// let morphology = Morphology::load_bundled()?;
    /// let config = AnalyzerConfig::builder()
    ///     .include_word(true)
    ///     .build();
    /// let mut analyzer = morphology.new_analyzer_with_config(config);
    /// # Ok::<(), raudikko::MorphologyError>(())
    /// ```
    pub fn new_analyzer_with_config(&self, config: AnalyzerConfig) -> Analyzer<'_> {
        Analyzer::new(self, config)
    }
}
