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

//! Analysis validator for checking validity of morphological analyses

use super::symbol_buffer::SymbolBuffer;
use super::tags::Tags;
use crate::analysis::WordClass;

const VOWELS: &str = "aeiouyäö";

/// Check if an analysis is valid based on morphological rules
pub fn is_valid_analysis(tokenizer: &mut SymbolBuffer) -> bool {
    let mut before_last_char = '\0';
    let mut last_char = '\0';
    let mut boundary_passed = false;
    let mut hyphen_present = false;
    let mut hyphen_unconditionally_allowed = false;
    let mut hyphen_unconditionally_allowed_just_set = false;
    let mut hyphen_required = false;
    let mut required_hyphen_missing = false;
    let mut starts_with_proper_noun = false;
    let mut ends_with_non_ica_noun = false;

    tokenizer.move_to_start();
    while tokenizer.next_token() {
        let tag = tokenizer.get_current_tag();
        if let Some(tag) = tag {
            if tag.matches_str(Tags::ISF) {
                hyphen_unconditionally_allowed = true;
                hyphen_unconditionally_allowed_just_set = true;
            } else if tag.matches_str(Tags::ICU) {
                boundary_passed = false;
                hyphen_unconditionally_allowed = true;
                hyphen_required = true;
            } else if tag.matches_str(Tags::ICA) {
                required_hyphen_missing = false;
                ends_with_non_ica_noun = false;
            } else if tag.is_name_tag() {
                starts_with_proper_noun = true; // TODO starts?
                ends_with_non_ica_noun = false;
            } else if tag.matches_word_class(WordClass::Noun) || tag.matches_word_class(WordClass::NounAdjective) {
                ends_with_non_ica_noun = true;
            } else if tag.matches_str(Tags::DG) {
                starts_with_proper_noun = false;
            } else if tag.is_x_parameter() {
                tokenizer.skip_x_tag();
            } else if tag.matches_str(Tags::BH) {
                boundary_passed = true;
                hyphen_present = false;

                if required_hyphen_missing {
                    return false;
                }

                if hyphen_required {
                    required_hyphen_missing = true;
                }
            }
        } else {
            let current_token = tokenizer.current_token();
            for (i, current) in current_token.chars().enumerate() {
                if current == '-' {
                    starts_with_proper_noun = false;
                    ends_with_non_ica_noun = false;

                    if i == current_token.len() - 1 && tokenizer.next_token_is_tag(Tags::BH) {
                        tokenizer.next_token();
                        boundary_passed = true;
                        hyphen_present = true;
                    }
                } else {
                    if boundary_passed {
                        if last_char == '\0' || (before_last_char == 'i' && last_char == 's') {
                            hyphen_unconditionally_allowed = true;
                        }

                        if hyphen_present {
                            hyphen_required = false;
                        }

                        if !hyphen_unconditionally_allowed || !hyphen_present {
                            let last_char_lower = last_char.to_lowercase().next().unwrap_or(last_char);

                            let need_hyphen = (last_char_lower == current.to_lowercase().next().unwrap_or(current)
                                              && is_vowel(last_char_lower))
                                             || last_char.is_ascii_digit();

                            if need_hyphen != hyphen_present {
                                return false;
                            }
                        }

                        boundary_passed = false;

                        if hyphen_unconditionally_allowed_just_set {
                            hyphen_unconditionally_allowed_just_set = false;
                        } else {
                            hyphen_unconditionally_allowed = false;
                        }
                    }
                    before_last_char = last_char;
                    last_char = current;
                }
            }
        }
    }

    !required_hyphen_missing && (!starts_with_proper_noun || !ends_with_non_ica_noun)
}

fn is_vowel(ch: char) -> bool {
    VOWELS.contains(ch)
}
