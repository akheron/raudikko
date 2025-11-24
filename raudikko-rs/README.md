# Raudikko (Rust Port)

Raudikko is a Rust library for performing morphological analysis on Finnish language. This is a port of the original Java implementation.

Raudikko uses [Voikko](https://voikko.puimula.org)'s morphology files and is based on Voikko, but unlike Voikko's native interface, it is implemented purely in Rust with no native library dependencies.

## Status

🚧 **Work in Progress** - This is a skeleton port. Core functionality is not yet implemented.

## Planned Usage

```rust
use raudikko::{Morphology, AnalyzerConfig};

// Load the morphology (expensive operation, do once)
let morphology = Morphology::load_bundled()?;

// Create an analyzer (cheap operation, can be reused)
let mut analyzer = morphology.new_analyzer();

// Analyze words
let analyses = analyzer.analyze("kissoittansa");
for analysis in analyses {
    println!("Base form: {:?}", analysis.base_form);
    println!("Word class: {:?}", analysis.word_class);
}

// Get just base forms
let base_forms = analyzer.base_forms("kahdellakymmenelläseitsemällä");
println!("Base forms: {:?}", base_forms);
```

### With Custom Configuration

```rust
let config = AnalyzerConfig::builder()
    .include_word(true)
    .include_structure(true)
    .build();

let mut analyzer = morphology.new_analyzer_with_config(config);
```

## Architecture

The port maintains the same overall architecture as the Java version:

- **Morphology**: Immutable, thread-safe morphology rules (can be shared via `Arc`)
- **Analyzer**: Mutable analyzer with cached state (not thread-safe, one per thread)
- **Analysis**: Rich result object with grammatical information
- **Structure**: Morpheme boundaries and hyphenation information
- **Word/WordPart**: Detailed word composition for compound words

## TODO

- [ ] Implement FST (Finite State Transducer) loading and operations
- [ ] Port morphology analysis logic
- [ ] Embed bundled morphology data
- [ ] Add comprehensive tests
- [ ] Add benchmarks
- [ ] Add examples
- [ ] Validate against Voikko for compatibility

## License

Dual-licensed under:
- GNU General Public License v3.0 or later ([GPL-3.0-or-later](https://opensource.org/licenses/GPL-3.0))
- Mozilla Public License 2.0 ([MPL-2.0](https://opensource.org/licenses/MPL-2.0))

You may choose either license.

## Acknowledgements

This is a port of the original [Raudikko Java library](https://github.com/evidentsolutions/raudikko) by Evident Solutions Oy.

Raudikko is based heavily on Voikko and uses morphology files directly from Voikko. None of this would be possible without the great work of [Harri Pitkänen](https://github.com/hatapitk) and other contributors to Voikko.
