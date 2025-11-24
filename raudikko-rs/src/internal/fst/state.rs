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
 * Raudikko, the Java port of the Initial Code is Copyright (C) 2020 by
 * Evident Solutions Oy. All Rights Reserved.
 *
 * Alternatively, the contents of this file may be used under the terms of
 * either the GNU General Public License Version 2 or later (the "GPL"), or
 * the GNU Lesser General Public License Version 2.1 or later (the "LGPL"),
 * in which case the provisions of the GPL or the LGPL are applicable instead
 * of those above. If you wish to allow use of your version of this file only
 * under the terms of either the GPL or the LGPL, and not to allow others to
 * use your version of this file under the terms of the MPL, indicate your
 * decision by deleting the provisions above and replace them with the notice
 * and other provisions required by the GPL or the LGPL. If you do not delete
 * the provisions above, a recipient may use your version of this file under
 * the terms of any one of the MPL, the GPL or the LGPL.
 */

use crate::internal::fst::transition::{CharTransition, DiacriticTransition};

pub(crate) struct State {
    pub(crate) diacritic_transitions: Vec<DiacriticTransition>,
    pub(crate) char_transitions: Vec<CharTransition>,
}

impl State {
    pub(crate) fn new() -> Self {
        State {
            diacritic_transitions: Vec::new(),
            char_transitions: Vec::new(),
        }
    }

    pub(crate) fn is_final(&self) -> bool {
        self.diacritic_transitions.is_empty() && self.char_transitions.is_empty()
    }

    /// Returns index of first transition of given character. Assumes that
    /// transitions are sorted by character.
    pub(crate) fn first_character_transition_for(&self, ch: char) -> usize {
        if self.char_transitions.len() < 8 {
            // If the array is small enough, just loop through transitions linearly
            for (i, transition) in self.char_transitions.iter().enumerate() {
                if transition.input == ch {
                    return i;
                }
            }
        } else {
            // Otherwise use binary search
            let mut low = 0;
            let mut high = self.char_transitions.len() - 1;

            while low <= high {
                let mid = (low + high) / 2;
                let mid_val = self.char_transitions[mid].input;

                match mid_val.cmp(&ch) {
                    std::cmp::Ordering::Less => low = mid + 1,
                    std::cmp::Ordering::Greater => {
                        if mid == 0 {
                            break;
                        }
                        high = mid - 1;
                    }
                    std::cmp::Ordering::Equal => {
                        let mut result = mid;
                        while result > 0 && self.char_transitions[result - 1].input == ch {
                            result -= 1;
                        }
                        return result;
                    }
                }
            }
        }

        self.char_transitions.len()
    }
}
