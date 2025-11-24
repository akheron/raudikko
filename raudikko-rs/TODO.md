# Raudikko Rust Port - Remaining Work

This document outlines the work needed to complete the Rust port of Raudikko.

## ✅ Completed

- [x] Public API design and type system
- [x] All grammatical enums (WordClass, Locative, Mood, Person, etc.)
- [x] Analysis result structures
- [x] Configuration system with builder pattern
- [x] Structure and Word/WordPart types
- [x] Error types
- [x] Basic documentation
- [x] Project compiles cleanly

## ✅ Completed (Phase 1)

### 1. Finite State Transducer (FST) ✅

**Location**: `src/internal/fst/` ✅

All core FST components have been ported from Java:
- ✅ `UnweightedTransducer` - Core FST data structure and operations
- ✅ `UnweightedVfstLoader` - Loading `.vfst` files
- ✅ `State`, `CharTransition`, `DiacriticTransition` - State and transition types
- ✅ `Symbol`, `Diacritic`, `SymbolOrDiacritic` - Symbol types with tag parsing
- ✅ `TransducerOperation` - FST operation enum (P, C, U, R, D)
- ✅ `SymbolMap` - Symbol lookup and management
- ✅ `CharMap` - Character to symbol mapping (from utils)
- ✅ `MyInputStream` - Binary data reader (from utils)

**Status**: All files ported, code compiles cleanly, basic test structure in place.

**Testing Status**: 0 of 9 Java FST unit tests (`UnweightedTransducerTest.java`) have been ported yet.
Tests require Phase 2 components: `SymbolBuffer` for output processing and morphology data loading.
Only 1 smoke test (`test_fst_module_compiles`) currently passes to verify compilation.

The FST implementation is ready for integration with the morphology analysis engine.

## 🚧 Core Implementation (Critical)

### 2. Morphological Analysis Engine

**Location**: New module `src/internal/morphology/`

Port the core analysis logic:
- `FinnishVfstAnalyzer` - Main analyzer implementation (this is what `Analyzer` will wrap)
- `SymbolBuffer` - Buffer for processing FST symbols
- `WordParser` - Parse analysis results into `Word` structures
- `StructureParser` - Parse structure strings into `Structure` objects
- `BaseFormParts` parser - Extract compound word components
- `BasicAttributes` parser - Extract grammatical attributes
- `Tags` - Tag name constants and utilities
- `TaggedValueLookupTable` - Lookup tables for tag values
- `Validator` - Validation logic for analysis results
- `Organization` - Organization name handling

**Files to port**:
- `internal/morphology/FinnishVfstAnalyzer.java` (most critical)
- `internal/morphology/SymbolBuffer.java`
- `internal/morphology/WordParser.java`
- `internal/morphology/StructureParser.java`
- `internal/morphology/BaseForm.java`
- `internal/morphology/BaseFormParts.java`
- `internal/morphology/BasicAttributes.java`
- `internal/morphology/Tags.java`
- `internal/morphology/TaggedValueLookupTable.java`
- `internal/morphology/Validator.java`
- `internal/morphology/Organization.java`

### 3. Morphology Data Loading

**Tasks**:
- Embed morphology files in the binary (currently at `resources/morpho/5/mor-morpho/mor.vfst`)
- Implement `Morphology::load_bundled()` to read embedded data
- Consider compression to reduce binary size
- Add support for loading external morphology files (optional)

**Approach**:
```rust
// Use include_bytes! macro
const MORPHOLOGY_DATA: &[u8] = include_bytes!("../resources/morpho/5/mor-morpho/mor.vfst");
```

### 4. Utility Modules

**Location**: New module `src/internal/utils/`

Port utility code:
- `StringUtils` - String manipulation helpers
- `CharMap` - Character mapping utilities
- `CollectionUtils` - Collection helpers
- `MyInputStream` - Custom input stream wrapper (or use standard Rust I/O)

**Files to port**:
- `internal/utils/StringUtils.java`
- `internal/utils/CharMap.java`
- `internal/utils/CollectionUtils.java`
- `internal/utils/MyInputStream.java` (may not be needed in Rust)

## 🧪 Testing

### Unit Tests

**Location**: `tests/` and inline module tests

Port existing tests:
- FST tests (`UnweightedTransducerTest.java`) - **Blocked**: Requires `SymbolBuffer` and morphology data loading
  - 9 tests total: baseForm, baseFormForCompoundWord1/2, baseFormForNounDerivedFromVerb,
    baseFormForNumeral, baseFormForOrdinal, baseFormForWordHavingNoInflections,
    baseFormForCompoundProperNoun, baseFormForCapitalizedWord
- Morphology parser tests:
  - `FinnishVfstAnalyzerTest.java`
  - `BaseFormParserTest.java`
  - `StructureParserTest.java`
  - `SymbolBufferTest.java`
- Utility tests (`StringUtilsTest.java`)

### Integration Tests

**Location**: `tests/integration/`

Port integration tests:
- `MorphoTest.java` - Comprehensive morphological analysis tests
- `BaseFormPartsTest.java` - Compound word analysis tests
- `VoikkoComparisonTest.java` - Compatibility tests with Voikko (optional, requires Voikko bindings)

### Test Data

Copy test resources:
- Word lists from `src/test/resources/`
- Expected output files

## 📚 Documentation & Examples

### Examples

**Location**: `examples/`

Create usage examples:
- `basic.rs` - Simple analysis example
- `custom_config.rs` - Using custom configuration
- `compound_words.rs` - Analyzing compound words
- `batch_analysis.rs` - Processing multiple words efficiently

### Documentation

- Add rustdoc comments to all public items
- Create tutorial in README.md
- Add architecture documentation
- Document performance characteristics
- Add troubleshooting guide

## 🔧 Tooling & Infrastructure

### Build & CI

- Set up GitHub Actions for CI
- Add `cargo fmt` and `cargo clippy` checks
- Add test coverage reporting
- Set up automatic releases

### Performance

**Location**: `benches/`

- Uncomment and implement `analyzer_bench.rs`
- Add benchmarks for:
  - FST operations
  - Full word analysis
  - Batch processing
- Compare performance with Java version

### Packaging

- Verify Cargo.toml metadata is complete
- Add license files
- Prepare for crates.io publication
- Create installation guide

## 🎯 Nice-to-Have Features

### Additional APIs

- Async/streaming analysis API
- WASM support for browser usage
- C FFI for interoperability
- Python bindings (via PyO3)

### Optimizations

- Cache commonly analyzed words
- Parallel batch processing
- Memory-mapped morphology files
- SIMD optimizations for FST operations

### Debugging Tools

- FST visualization
- Analysis step debugging
- Morphology file inspection tools

## 📊 Validation

### Compatibility Testing

Before declaring the port complete:
1. Run all tests from Java version
2. Compare output with Java version for large word list (e.g., 10,000+ words)
3. Verify edge cases match exactly
4. Benchmark against Java version
5. Test thread safety and concurrent usage

## 🎬 Recommended Implementation Order

1. **Phase 1: Core FST** (2-3 weeks)
   - Implement FST data structures
   - Implement VFST loader
   - Unit tests for FST

2. **Phase 2: Analysis Engine** (2-3 weeks)
   - Port FinnishVfstAnalyzer
   - Port parsers (Structure, Word, BaseForm)
   - Basic integration tests

3. **Phase 3: Data & Integration** (1 week)
   - Embed morphology data
   - Wire up Morphology and Analyzer
   - End-to-end testing

4. **Phase 4: Completeness** (1-2 weeks)
   - Port all remaining utilities
   - Complete test coverage
   - Voikko compatibility validation

5. **Phase 5: Polish** (1 week)
   - Documentation
   - Examples
   - Benchmarks
   - CI/CD

**Total estimated effort**: 7-10 weeks for a complete, production-ready port

## 📝 Notes

- The Java version is ~46 source files. The Rust version currently has 12 files.
- Most complex parts are the FST implementation and the analysis engine.
- Consider using existing Rust FST libraries (e.g., `fst` crate) instead of porting from scratch.
- The current skeleton provides a solid, idiomatic Rust foundation - the hard part is the algorithmic logic, not the API design.
