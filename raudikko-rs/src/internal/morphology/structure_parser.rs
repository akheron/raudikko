/*
 * The contents of this file are subject to the Mozilla Public License Version
 * 2.0 (the "License"); you may not use this file except in compliance with
 * the License. You may obtain a copy of the License at
 * https://www.mozilla.org/en-US/MPL/2.0/
 *
 * Software distributed under the License is distributed on an "AS IS" basis,
 * WITHOUT WARRANTY OF ANY KIND, either express or implied. See the License
 * for the specific language governing rights and limitations under the
 * License.
 *
 * The Original Code is Libvoikko: Library of natural language processing tools.
 * The Initial Developer of the Original Code is Harri Pitkänen <hatapitk@iki.fi>.
 * Portions created by the Initial Developer are Copyright (C) 2012
 * the Initial Developer. All Rights Reserved.
 *
 * Raudikko, the Rust port of the Initial Code is Copyright (C) 2025.
 * All Rights Reserved.
 */

//! Structure parser for analyzing word morpheme boundaries

use super::symbol_buffer::SymbolBuffer;
use super::tags::Tags;
use crate::analysis::WordClass;
use crate::{Structure, StructureSymbol};

/// Parse structure information from a symbol buffer
pub fn parse_structure(tokenizer: &mut SymbolBuffer, word_length: usize) -> Structure {
    let mut builder = StructureBuilder::new(word_length * 2);

    let mut chars_missing = word_length as isize;
    let mut chars_seen = 0isize;
    let mut chars_from_default = 0isize;
    let mut default_title_case = false;
    let mut is_abbr = false;

    tokenizer.move_to_start();

    while tokenizer.next_token() {
        let tag = tokenizer.get_current_tag();
        if let Some(tag) = tag {
            if tag.matches(&Tags::bc()) || tag.matches(&Tags::bm()) {
                if tokenizer.get_current_offset() == 1 {
                    builder.add(StructureSymbol::MorphemeStart);
                }

                if chars_seen > chars_from_default {
                    default_title_case = create_default_structure(
                        &mut builder,
                        chars_seen - chars_from_default,
                        default_title_case,
                        is_abbr,
                    );
                    chars_missing -= chars_seen - chars_from_default;
                }

                // TODO: Why is 'tokenizer.get_current_offset() + 5' necessary? Make the meaning clearer.
                if tokenizer.get_current_offset() != 1
                    && tokenizer.get_current_offset() + 5 < tokenizer.get_total_length()
                {
                    builder.ensure_ends_with_new_morpheme();
                }

                chars_seen = 0;
                chars_from_default = 0;
            } else if tag.matches(&Tags::xr()) {
                default_title_case = false;

                for c in tokenizer.read_structure() {
                    if chars_missing == 0 {
                        break;
                    }

                    builder.add(c);
                    if c != StructureSymbol::MorphemeStart {
                        chars_from_default += 1;
                        if c != StructureSymbol::Hyphen {
                            chars_missing -= 1;
                        }
                    }
                }
            } else if tag.is_x_parameter() {
                tokenizer.skip_x_tag();
            } else if tag.is_name_tag() {
                default_title_case = true;
                is_abbr = false;
            } else if tag.is_class_tag() {
                is_abbr = tag.matches_word_class(WordClass::Abbreviation)
                    || tag.matches_word_class(WordClass::NumeralRoman)
                    || (tag.matches_word_class(WordClass::Numeral)
                        && tokenizer.next_token_starts_with_digit());
            }
        } else {
            let current_token = tokenizer.current_token();
            for (i, c) in current_token.chars().enumerate() {
                match c {
                    '-' => {
                        if chars_seen > chars_from_default {
                            default_title_case = create_default_structure(
                                &mut builder,
                                chars_seen - chars_from_default,
                                default_title_case,
                                is_abbr,
                            );
                            chars_missing -= chars_seen - chars_from_default;
                            builder.add(StructureSymbol::Hyphen);
                            chars_seen = 0;
                            chars_from_default = 0;
                        } else if !tokenizer.is_at_first_token() || i != 0 {
                            if chars_seen == chars_from_default {
                                builder.add(StructureSymbol::Hyphen);
                            } else {
                                chars_seen += 1;
                            }
                        }
                        if chars_missing != 0 {
                            chars_missing -= 1;
                        }
                        if builder.size() == 1 {
                            builder.replace_start_with_hyphen();
                        }
                    }
                    ':' => {
                        if is_abbr {
                            if chars_seen > chars_from_default {
                                default_title_case = create_default_structure(
                                    &mut builder,
                                    chars_seen - chars_from_default,
                                    default_title_case,
                                    true,
                                );
                                chars_missing -= chars_seen - chars_from_default;
                                chars_seen = 0;
                                chars_from_default = 0;
                            }
                            is_abbr = false;
                        }
                        builder.add(StructureSymbol::Colon);
                        if chars_missing != 0 {
                            chars_missing -= 1;
                        }
                    }
                    _ => chars_seen += 1,
                }
            }
        }
    }

    create_default_structure(&mut builder, chars_missing, default_title_case, is_abbr);
    capitalize_structure(&mut builder, tokenizer);

    builder.build()
}

fn create_default_structure(
    result: &mut StructureBuilder,
    chars_missing: isize,
    title_case: bool,
    abbr: bool,
) -> bool {
    if chars_missing == 0 {
        return title_case;
    }

    let mut chars_left = chars_missing;

    if title_case {
        result.add(if abbr {
            StructureSymbol::UppercaseNoHyphenation
        } else {
            StructureSymbol::Uppercase
        });
        chars_left -= 1;
    }

    if chars_left > 0 {
        result.add_repeated(
            if abbr {
                StructureSymbol::LowercaseNoHyphenation
            } else {
                StructureSymbol::Lowercase
            },
            chars_left as usize,
        );
    }

    false
}

fn capitalize_structure(structure: &mut StructureBuilder, tokenizer: &mut SymbolBuffer) {
    let mut is_de = false;
    let mut total_hyphens = 0;

    tokenizer.move_to_start();

    while tokenizer.next_token() {
        let tag = tokenizer.get_current_tag();
        if let Some(tag) = tag {
            if tag.matches(&Tags::dg()) {
                structure.change_to_lower_case_at_hyphen_index(total_hyphens);
            } else if tag.matches(&Tags::de()) {
                is_de = true;
            } else if tag.matches_word_class(WordClass::Noun) {
                is_de = false;
            }
        } else {
            let hyphens = tokenizer.current_token().chars().filter(|&c| c == '-').count();

            if hyphens > 0 && is_de {
                if tokenizer.contains_tag_after_current(&WordClass::Toponym)
                    || tokenizer.is_at_last_token()
                {
                    structure.capitalize();
                    return;
                }
            }

            total_hyphens += hyphens;
        }
    }
}

/// Helper for building Structure objects
struct StructureBuilder {
    symbols: Vec<StructureSymbol>,
}

impl StructureBuilder {
    fn new(capacity: usize) -> Self {
        let mut symbols = Vec::with_capacity(capacity);
        symbols.push(StructureSymbol::MorphemeStart);
        Self { symbols }
    }

    fn add(&mut self, symbol: StructureSymbol) {
        self.symbols.push(symbol);
    }

    fn add_repeated(&mut self, symbol: StructureSymbol, count: usize) {
        for _ in 0..count {
            self.add(symbol);
        }
    }

    fn ensure_ends_with_new_morpheme(&mut self) {
        if self.symbols.last() != Some(&StructureSymbol::MorphemeStart) {
            self.add(StructureSymbol::MorphemeStart);
        }
    }

    fn size(&self) -> usize {
        self.symbols.len()
    }

    fn replace_start_with_hyphen(&mut self) {
        if !self.symbols.is_empty() {
            self.symbols[0] = StructureSymbol::Hyphen;
        }
    }

    fn build(self) -> Structure {
        Structure::from_symbols(self.symbols)
    }

    fn capitalize(&mut self) {
        // TODO: The code does not handle UPPERCASE_NO_HYPHENATION and LOWERCASE_NO_HYPHENATION, which is somewhat
        //       suspicious. They probably can't occur in the call-path, but it's still a bit sketchy.
        for i in 0..self.symbols.len() {
            let sym = self.symbols[i];
            if sym == StructureSymbol::Uppercase {
                break;
            } else if sym == StructureSymbol::Lowercase {
                self.symbols[i] = StructureSymbol::Uppercase;
                break;
            }
        }
    }

    fn change_to_lower_case_at_hyphen_index(&mut self, hyphen_index: usize) {
        let mut seen_hyphens = 0;

        for i in 0..self.symbols.len() {
            let current = self.symbols[i];
            if current == StructureSymbol::Uppercase {
                if seen_hyphens == hyphen_index {
                    self.symbols[i] = StructureSymbol::Lowercase;
                }
            } else if current == StructureSymbol::Hyphen {
                seen_hyphens += 1;
            }
        }
    }
}
