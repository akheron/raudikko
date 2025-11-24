//! Analyzer configuration

/// Configuration for the analyzer.
///
/// Default configuration includes sensible defaults (most analysis features enabled,
/// except expensive ones like detailed word analysis).
#[derive(Debug, Clone)]
pub struct AnalyzerConfig {
    /// Include morpheme structure information
    pub include_structure: bool,

    /// Include base form (dictionary form) of words
    pub include_base_form: bool,

    /// Include basic grammatical attributes
    pub include_basic_attributes: bool,

    /// Include organization name analysis
    pub include_organization_name_analysis: bool,

    /// Include raw FST output
    pub include_fst_output: bool,

    /// Include base form parts for compound words
    pub include_base_form_parts: bool,

    /// Include detailed word part analysis (more expensive)
    pub include_word: bool,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            include_structure: true,
            include_base_form: true,
            include_basic_attributes: true,
            include_organization_name_analysis: true,
            include_fst_output: true,
            include_base_form_parts: true,
            include_word: false,
        }
    }
}

impl AnalyzerConfig {
    /// Create a new builder for configuring the analyzer
    pub fn builder() -> AnalyzerConfigBuilder {
        AnalyzerConfigBuilder::default()
    }
}

/// Builder for [`AnalyzerConfig`]
#[derive(Default)]
pub struct AnalyzerConfigBuilder {
    config: AnalyzerConfig,
}

impl AnalyzerConfigBuilder {
    /// Include morpheme structure information
    pub fn include_structure(mut self, value: bool) -> Self {
        self.config.include_structure = value;
        self
    }

    /// Include base form (dictionary form) of words
    pub fn include_base_form(mut self, value: bool) -> Self {
        self.config.include_base_form = value;
        self
    }

    /// Include basic grammatical attributes
    pub fn include_basic_attributes(mut self, value: bool) -> Self {
        self.config.include_basic_attributes = value;
        self
    }

    /// Include organization name analysis
    pub fn include_organization_name_analysis(mut self, value: bool) -> Self {
        self.config.include_organization_name_analysis = value;
        self
    }

    /// Include raw FST output
    pub fn include_fst_output(mut self, value: bool) -> Self {
        self.config.include_fst_output = value;
        self
    }

    /// Include base form parts for compound words
    pub fn include_base_form_parts(mut self, value: bool) -> Self {
        self.config.include_base_form_parts = value;
        self
    }

    /// Include detailed word part analysis (more expensive)
    pub fn include_word(mut self, value: bool) -> Self {
        self.config.include_word = value;
        self
    }

    /// Build the configuration
    pub fn build(self) -> AnalyzerConfig {
        self.config
    }
}
