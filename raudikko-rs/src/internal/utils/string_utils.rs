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

/// Replace a character at a specific position in a string
pub fn replace_char_at(s: &str, i: usize, c: char) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    if i < chars.len() && chars[i] == c {
        return s.to_string();
    }
    if i < chars.len() {
        chars[i] = c;
    }
    chars.into_iter().collect()
}

/// Remove all occurrences of a character from a string
pub fn without_char(s: &str, removed: char) -> String {
    s.chars().filter(|&c| c != removed).collect()
}

/// Count occurrences of a character in a string
pub fn count_occurrences(s: &str, c: char) -> usize {
    s.chars().filter(|&ch| ch == c).count()
}

/// Check if a string ends with a specific character
pub fn ends_with_char(s: &str, c: char) -> bool {
    s.chars().last() == Some(c)
}

/// Check if a string starts with a specific character
pub fn starts_with_char(s: &str, c: char) -> bool {
    s.chars().next() == Some(c)
}

/// Capitalize the first character if the string is all lowercase
pub fn capitalize_if_lower(s: &str) -> String {
    if is_all_lower(s) {
        capitalize(s)
    } else {
        s.to_string()
    }
}

/// Check if all characters are uppercase
pub fn is_all_upper(s: &str) -> bool {
    s.chars().all(|c| !c.is_lowercase())
}

/// Check if all characters are lowercase
pub fn is_all_lower(s: &str) -> bool {
    s.chars().all(|c| !c.is_uppercase())
}

/// Capitalize the first character
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let mut result = String::new();
            result.extend(first.to_uppercase());
            result.extend(chars);
            result
        }
    }
}

/// Remove a range of characters from a string
pub fn remove_range(s: &str, start_index: usize, end_index: usize) -> String {
    if end_index < start_index {
        panic!("end_index < start_index");
    } else if end_index == start_index {
        s.to_string()
    } else {
        let chars: Vec<char> = s.chars().collect();
        let mut result = String::new();
        result.extend(chars.iter().take(start_index));
        result.extend(chars.iter().skip(end_index));
        result
    }
}

/// Check if a string contains a character
pub fn contains(s: &str, c: char) -> bool {
    index_of(s, c, 0).is_some()
}

/// Find the index of a character in a string
pub fn index_of(s: &str, c: char, from_index: usize) -> Option<usize> {
    s.chars()
        .enumerate()
        .skip(from_index)
        .find(|(_, ch)| *ch == c)
        .map(|(i, _)| i)
}

/// Check if haystack matches needle at a specific offset
pub fn matches_at(haystack: &str, offset: usize, needle: &str) -> bool {
    let haystack_chars: Vec<char> = haystack.chars().collect();
    let needle_chars: Vec<char> = needle.chars().collect();

    if offset + needle_chars.len() > haystack_chars.len() {
        return false;
    }

    for i in 0..needle_chars.len() {
        if haystack_chars[i + offset] != needle_chars[i] {
            return false;
        }
    }

    true
}

/// Remove leading and trailing occurrences of a character
pub fn remove_leading_and_trailing(s: &str, c: char) -> String {
    if s.is_empty() || (s.len() == 1 && s.chars().next() == Some(c)) {
        return String::new();
    }

    let start_offset = if starts_with_char(s, c) { 1 } else { 0 };
    let end_offset = if ends_with_char(s, c) { 1 } else { 0 };

    let chars: Vec<char> = s.chars().collect();
    chars[start_offset..(chars.len() - end_offset)]
        .iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_occurrences() {
        assert_eq!(count_occurrences("hello", 'l'), 2);
        assert_eq!(count_occurrences("hello", 'o'), 1);
        assert_eq!(count_occurrences("hello", 'x'), 0);
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("a"), "A");
    }

    #[test]
    fn test_is_all_lower() {
        assert!(is_all_lower("hello"));
        assert!(!is_all_lower("Hello"));
        assert!(!is_all_lower("HELLO"));
    }

    #[test]
    fn test_is_all_upper() {
        assert!(is_all_upper("HELLO"));
        assert!(!is_all_upper("Hello"));
        assert!(!is_all_upper("hello"));
    }

    #[test]
    fn test_matches_at() {
        assert!(matches_at("hello world", 6, "world"));
        assert!(!matches_at("hello world", 6, "word"));
        assert!(!matches_at("hello", 3, "world"));
    }
}
