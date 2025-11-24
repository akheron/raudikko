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
 * Raudikko, the Rust port is Copyright (C) 2025 by
 * the contributors. All Rights Reserved.
 */

use crate::analysis::AnalysisClass;
use crate::internal::fst::Symbol;
use crate::internal::morphology::Tags;
use crate::internal::utils::string_utils;
use crate::StructureSymbol;

/// Buffer for processing FST output symbols
pub struct SymbolBuffer {
    text_buffer: String,
    tags: Vec<Option<Symbol>>,
    index: isize,
    start_indices: Vec<usize>,
    token_count: usize,
}

impl SymbolBuffer {
    /// Create a new symbol buffer with the given capacity
    pub fn new(buffer_size: usize) -> Self {
        Self {
            text_buffer: String::with_capacity(buffer_size),
            tags: vec![None; buffer_size],
            index: -1,
            start_indices: vec![0; buffer_size / 2],
            token_count: 0,
        }
    }

    /// Parse a string into a symbol buffer (for testing)
    #[cfg(test)]
    pub fn parse(cs: &str) -> Self {
        let mut symbols = Vec::new();
        let mut offset = 0;
        let chars: Vec<char> = cs.chars().collect();

        while offset < chars.len() {
            if chars[offset] == '[' {
                let mut i = offset + 1;
                while i < chars.len() && chars[i] != ']' {
                    i += 1;
                }
                if i < chars.len() {
                    i += 1; // Include the ']'
                }
                let tag_str: String = chars[offset..i].iter().collect();
                symbols.push(Symbol::new(tag_str));
                offset = i;
            } else {
                let char_str: String = chars[offset..offset + 1].iter().collect();
                symbols.push(Symbol::new(char_str));
                offset += 1;
            }
        }

        let mut buffer = Self::new(2000);
        buffer.reset(&symbols);
        buffer
    }

    /// Reset the buffer with new symbols
    pub fn reset(&mut self, symbols: &[Symbol]) {
        self.text_buffer.clear();
        let mut index = 0;
        let mut previous_char = false;

        for symbol in symbols {
            if symbol.is_char() {
                if !previous_char {
                    previous_char = true;
                    self.tags[index] = None;
                    self.start_indices[index] = self.text_buffer.len();
                    index += 1;
                }
                self.text_buffer.push_str(symbol.as_str());
            } else if !symbol.is_epsilon() {
                self.tags[index] = Some(symbol.clone());
                self.start_indices[index] = self.text_buffer.len();
                index += 1;
                self.text_buffer.push_str(symbol.as_str());
                previous_char = false;
            }
        }

        self.token_count = index;
        self.start_indices[index] = self.text_buffer.len();
        self.index = -1;
    }

    /// Get the full text contents
    pub fn full_contents(&self) -> &str {
        &self.text_buffer
    }

    /// Move to the start of the buffer
    pub fn move_to_start(&mut self) {
        self.index = -1;
    }

    /// Move to the end of the buffer
    pub fn move_to_end(&mut self) {
        self.index = self.token_count as isize;
    }

    /// Get the current offset in the text buffer
    pub fn get_current_offset(&self) -> usize {
        self.start_indices[self.index as usize]
    }

    /// Get the total length of the text buffer
    pub fn get_total_length(&self) -> usize {
        self.text_buffer.len()
    }

    /// Check if at the first token
    pub fn is_at_first_token(&self) -> bool {
        self.index == 0
    }

    /// Check if at the last token
    pub fn is_at_last_token(&self) -> bool {
        self.index == (self.token_count as isize - 1)
    }

    /// Move to the next token
    pub fn next_token(&mut self) -> bool {
        if self.index + 1 < self.token_count as isize {
            self.index += 1;
            true
        } else {
            false
        }
    }

    /// Move to the previous token
    pub fn previous_token(&mut self) -> bool {
        if self.index > 0 {
            self.index -= 1;
            true
        } else {
            false
        }
    }

    /// Check if a relative token ends with a character
    pub fn relative_token_ends_with_char(&self, relative_index: isize, c: char) -> bool {
        let token_index = self.index + relative_index;
        if token_index >= 0 && (token_index as usize) < self.token_count {
            let next_start = self.start_indices[token_index as usize + 1];
            if next_start > 0 {
                return self
                    .text_buffer
                    .chars()
                    .nth(next_start - 1)
                    .map_or(false, |ch| ch == c);
            }
        }
        false
    }

    /// Check if the next token is a specific tag
    pub fn next_token_is_tag(&self, tag: &str) -> bool {
        let next_index = self.index + 1;
        if next_index >= 0 && (next_index as usize) < self.token_count {
            if let Some(ref next_tag) = self.tags[next_index as usize] {
                return next_tag.matches_str(tag);
            }
        }
        false
    }

    /// Check if the previous token is a specific tag
    pub fn previous_token_is_tag(&self, tag: &str) -> bool {
        let previous = self.index - 1;
        if previous >= 0 && (previous as usize) < self.token_count {
            if let Some(ref prev_tag) = self.tags[previous as usize] {
                return prev_tag.matches_str(tag);
            }
        }
        false
    }

    /// Check if the next token starts with a digit
    pub fn next_token_starts_with_digit(&self) -> bool {
        let next_index = self.index + 1;
        if next_index >= 0 && (next_index as usize) < self.token_count {
            let start = self.start_indices[next_index as usize];
            return self
                .text_buffer
                .chars()
                .nth(start)
                .map_or(false, |c| c.is_ascii_digit());
        }
        false
    }

    /// Check if the next token is a boundary
    pub fn next_token_is_boundary(&self) -> bool {
        let next_index = self.index + 1;
        if next_index >= 0 && (next_index as usize) < self.token_count {
            if let Some(ref next_tag) = self.tags[next_index as usize] {
                return next_tag.is_boundary();
            }
        }
        false
    }

    /// Check if the first token starts with a character
    pub fn first_token_starts_with(&self, c: char) -> bool {
        !self.text_buffer.is_empty() && self.text_buffer.chars().next() == Some(c)
    }

    /// Check if the first token matches an analysis class
    pub fn first_token_is(&self, tag: &impl AnalysisClass) -> bool {
        if self.token_count == 0 {
            return false;
        }
        if let Some(ref first) = self.tags[0] {
            let expected = format!("[{}]", tag.morphology_tag());
            return first.matches_str(&expected);
        }
        false
    }

    /// Check if the last token matches an analysis class
    pub fn last_token_is(&self, cl: &impl AnalysisClass) -> bool {
        if self.token_count == 0 {
            return false;
        }
        if let Some(ref last) = self.tags[self.token_count - 1] {
            let expected = format!("[{}]", cl.morphology_tag());
            return last.matches_str(&expected);
        }
        false
    }

    /// Read the contents of an X-tag
    pub fn read_x_tag_contents(&mut self) -> String {
        self.next_token();
        if self.matches_tag(Tags::X) {
            String::new()
        } else {
            let content = self.current_token_string();
            self.next_token();
            assert!(
                self.matches_tag(Tags::X),
                "{} from {}",
                self.current_token_string(),
                self.full_contents()
            );
            content
        }
    }

    /// Read a structure
    pub fn read_structure(&mut self) -> Vec<StructureSymbol> {
        self.next_token();
        let mut result = Vec::new();

        if !self.matches_tag(Tags::X) {
            let token = self.current_token_string();
            for c in token.chars() {
                result.push(StructureSymbol::for_code(c));
            }

            self.next_token();
            assert!(
                self.matches_tag(Tags::X),
                "{} from {}",
                self.current_token_string(),
                self.full_contents()
            );
        }

        result
    }

    /// Skip to the next X-tag
    pub fn skip_x_tag(&mut self) {
        while self.next_token() {
            if self.matches_tag(Tags::X) {
                break;
            }
        }
    }

    /// Create a shallow copy of the buffer
    pub fn copy(&self) -> Self {
        Self {
            text_buffer: self.text_buffer.clone(),
            tags: self.tags.clone(),
            index: self.index,
            start_indices: self.start_indices.clone(),
            token_count: self.token_count,
        }
    }

    /// Get the current tag
    pub fn get_current_tag(&self) -> Option<&Symbol> {
        if self.index >= 0 && (self.index as usize) < self.token_count {
            self.tags[self.index as usize].as_ref()
        } else {
            None
        }
    }

    /// Check if the current tag matches a string
    pub fn matches_tag(&self, s: &str) -> bool {
        if let Some(tag) = self.get_current_tag() {
            tag.matches_str(s)
        } else {
            false
        }
    }

    /// Check if the current tag matches an analysis class
    pub fn matches_tag_class(&self, s: &impl AnalysisClass) -> bool {
        if let Some(tag) = self.get_current_tag() {
            let expected = format!("[{}]", s.morphology_tag());
            tag.matches_str(&expected)
        } else {
            false
        }
    }

    /// Check if a tag exists after the current position
    pub fn contains_tag_after_current(&self, s: &str) -> bool {
        for i in (self.index + 1) as usize..self.token_count {
            if let Some(ref tag) = self.tags[i] {
                if tag.matches_str(s) {
                    return true;
                }
            }
        }
        false
    }

    /// Check if an analysis class tag exists after the current position
    pub fn contains_tag_class_after_current(&self, s: &impl AnalysisClass) -> bool {
        let expected = format!("[{}]", s.morphology_tag());
        self.contains_tag_after_current(&expected)
    }

    /// Check if a tag exists before the current position
    pub fn contains_tag_before_current(&self, s: &str) -> bool {
        if self.index <= 0 {
            return false;
        }
        for i in (0..self.index as usize).rev() {
            if let Some(ref tag) = self.tags[i] {
                if tag.matches_str(s) {
                    return true;
                }
            }
        }
        false
    }

    /// Get the current token as a string
    fn current_token_string(&self) -> String {
        if self.index < 0 || self.index as usize >= self.token_count {
            return String::new();
        }
        let start = self.start_indices[self.index as usize];
        let end = self.start_indices[self.index as usize + 1];
        self.text_buffer[start..end].to_string()
    }

    /// Get the current token
    pub fn current_token(&self) -> CurrentToken<'_> {
        CurrentToken {
            buffer: self,
            index: self.index,
        }
    }
}

/// A view into the current token
pub struct CurrentToken<'a> {
    buffer: &'a SymbolBuffer,
    index: isize,
}

impl<'a> CurrentToken<'a> {
    /// Get the length of the current token
    pub fn len(&self) -> usize {
        if self.index < 0 || self.index as usize >= self.buffer.token_count {
            return 0;
        }
        let start = self.buffer.start_indices[self.index as usize];
        let end = self.buffer.start_indices[self.index as usize + 1];
        end - start
    }

    /// Check if the current token is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a character at a specific index
    pub fn char_at(&self, index: usize) -> Option<char> {
        if self.index < 0 || self.index as usize >= self.buffer.token_count {
            return None;
        }
        let start = self.buffer.start_indices[self.index as usize];
        self.buffer.text_buffer.chars().nth(start + index)
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        if self.index < 0 || self.index as usize >= self.buffer.token_count {
            return String::new();
        }
        let start = self.buffer.start_indices[self.index as usize];
        let end = self.buffer.start_indices[self.index as usize + 1];
        self.buffer.text_buffer[start..end].to_string()
    }

    /// Get the first character
    pub fn start(&self) -> Option<char> {
        self.char_at(0)
    }

    /// Check if starts with a digit
    pub fn starts_with_digit(&self) -> bool {
        self.start().map_or(false, |c| c.is_ascii_digit())
    }

    /// Check if starts with a character
    pub fn starts_with_char(&self, c: char) -> bool {
        self.start() == Some(c)
    }

    /// Check if matches at a specific position
    pub fn matches_at(&self, i: usize, s: &str) -> bool {
        let token_str = self.to_string();
        string_utils::matches_at(&token_str, i, s)
    }

    /// Count occurrences of a character
    pub fn count(&self, c: char) -> usize {
        let token_str = self.to_string();
        string_utils::count_occurrences(&token_str, c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_buffer_parse() {
        let buffer = SymbolBuffer::parse("hello[Ln]world");
        assert_eq!(buffer.full_contents(), "hello[Ln]world");
    }

    #[test]
    fn test_symbol_buffer_navigation() {
        let mut buffer = SymbolBuffer::parse("hello[Ln]world");
        assert_eq!(buffer.next_token(), true);
        assert_eq!(buffer.next_token(), true);
        assert_eq!(buffer.next_token(), true);
        assert_eq!(buffer.previous_token(), true);
    }
}
