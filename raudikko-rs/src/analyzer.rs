//! Morphological analyzer

use crate::{Analysis, AnalyzerConfig, Morphology};

/// Interface for morphological analyzer.
///
/// The analyzer is **not thread-safe** due to internal caching. Create one analyzer
/// per thread if needed.
pub struct Analyzer<'m> {
    #[allow(dead_code)]
    morphology: &'m Morphology,
    #[allow(dead_code)]
    config: AnalyzerConfig,
    // TODO: Cached state for analysis
}

impl<'m> Analyzer<'m> {
    pub(crate) fn new(morphology: &'m Morphology, config: AnalyzerConfig) -> Self {
        Self {
            morphology,
            config,
        }
    }

    /// Analyze given word and return a list of possible interpretations.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use raudikko::Morphology;
    ///
    /// let morphology = Morphology::load_bundled()?;
    /// let mut analyzer = morphology.new_analyzer();
    /// let analyses = analyzer.analyze("kissoittansa");
    ///
    /// for analysis in analyses {
    ///     println!("{:?}", analysis.base_form);
    /// }
    /// # Ok::<(), raudikko::MorphologyError>(())
    /// ```
    pub fn analyze(&mut self, word: &str) -> Vec<Analysis> {
        self.analyze_with_limit(word, usize::MAX)
    }

    /// Analyze given word and return a list of possible interpretations.
    ///
    /// At most `max_results` results are returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use raudikko::Morphology;
    ///
    /// let morphology = Morphology::load_bundled()?;
    /// let mut analyzer = morphology.new_analyzer();
    /// let analyses = analyzer.analyze_with_limit("kissa", 5);
    /// # Ok::<(), raudikko::MorphologyError>(())
    /// ```
    pub fn analyze_with_limit(&mut self, _word: &str, _max_results: usize) -> Vec<Analysis> {
        // TODO: Implement actual analysis using FST
        vec![]
    }

    /// Analyze given word and return a list of unique base forms.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use raudikko::Morphology;
    ///
    /// let morphology = Morphology::load_bundled()?;
    /// let mut analyzer = morphology.new_analyzer();
    /// let base_forms = analyzer.base_forms("kissoittansa");
    /// assert!(base_forms.contains(&"kissa".to_string()));
    /// # Ok::<(), raudikko::MorphologyError>(())
    /// ```
    pub fn base_forms(&mut self, word: &str) -> Vec<String> {
        let analyses = self.analyze(word);
        let mut base_forms = Vec::new();

        for analysis in analyses {
            if let Some(base_form) = analysis.base_form {
                if !base_forms.contains(&base_form) {
                    base_forms.push(base_form);
                }
            }
        }

        base_forms
    }
}
