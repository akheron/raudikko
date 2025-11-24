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

use crate::internal::fst::{Diacritic, State, Symbol, SymbolOrDiacritic, TransducerOperation};
use crate::internal::utils::CharMap;
use std::rc::Rc;

pub(crate) struct UnweightedTransducer {
    char_to_symbol: CharMap<Symbol>,
    root_state: Rc<State>,
    pub(crate) flag_diacritic_feature_count: usize,
}

impl UnweightedTransducer {
    pub(crate) fn new(
        symbols: Vec<SymbolOrDiacritic>,
        root_state: Rc<State>,
        flag_diacritic_feature_count: usize,
    ) -> Self {
        let mut char_to_symbol = CharMap::new();

        for sym in symbols {
            if sym.is_char() {
                char_to_symbol.put(sym.char_value(), sym.as_symbol().clone());
            }
        }

        UnweightedTransducer {
            char_to_symbol,
            root_state,
            flag_diacritic_feature_count,
        }
    }

    pub(crate) fn transduce<F>(
        &self,
        input: &str,
        input_buffer: &mut Vec<Symbol>,
        flags: &mut [i16],
        output_stack: &mut [Symbol],
        callback: F,
    ) where
        F: FnMut(usize),
    {
        if !self.prepare_input(input_buffer, input) {
            return;
        }

        self.enter_state(
            &self.root_state,
            0,
            input_buffer,
            flags,
            output_stack,
            callback,
            0,
        );
    }

    fn prepare_input(&self, input_symbols: &mut Vec<Symbol>, input: &str) -> bool {
        input_symbols.clear();

        for ch in input.chars() {
            let lower_ch = ch.to_lowercase().next().unwrap();
            match self.char_to_symbol.get(lower_ch) {
                Some(symbol) => input_symbols.push(symbol.clone()),
                None => return false,
            }
        }

        true
    }

    fn enter_state<F>(
        &self,
        st: &State,
        input_pos: usize,
        input: &[Symbol],
        flags: &mut [i16],
        output: &mut [Symbol],
        mut callback: F,
        depth: usize,
    ) where
        F: FnMut(usize),
    {
        if depth >= output.len() {
            return;
        }

        if st.is_final() && input_pos == input.len() {
            callback(depth);
            return;
        }

        for transition in &st.diacritic_transitions {
            let diacritic = &transition.input;

            if diacritic.is_epsilon() {
                output[depth] = transition.sym_out.clone();
                self.enter_state(
                    &transition.target,
                    input_pos,
                    input,
                    flags,
                    output,
                    &mut callback,
                    depth + 1,
                );
            } else {
                let old_value = flags[diacritic.feature as usize];
                if self.flag_diacritic_check(flags, diacritic, old_value) {
                    output[depth] = transition.sym_out.clone();
                    self.enter_state(
                        &transition.target,
                        input_pos,
                        input,
                        flags,
                        output,
                        &mut callback,
                        depth + 1,
                    );
                    flags[diacritic.feature as usize] = old_value;
                }
            }
        }

        if input_pos < input.len() {
            let ch = input[input_pos].char_value();

            let transitions = &st.char_transitions;
            let start_idx = st.first_character_transition_for(ch);
            for i in start_idx..transitions.len() {
                let transition = &transitions[i];
                if ch != transition.input {
                    break;
                }

                output[depth] = transition.sym_out.clone();
                self.enter_state(
                    &transition.target,
                    input_pos + 1,
                    input,
                    flags,
                    output,
                    &mut callback,
                    depth + 1,
                );
            }
        }
    }

    fn flag_diacritic_check(&self, flags: &mut [i16], ofv: &Diacritic, value: i16) -> bool {
        match ofv.op {
            TransducerOperation::P => {
                flags[ofv.feature as usize] = ofv.value;
                true
            }
            TransducerOperation::C => {
                flags[ofv.feature as usize] = Diacritic::NEUTRAL;
                true
            }
            TransducerOperation::U => {
                if value != Diacritic::NEUTRAL {
                    value == ofv.value
                } else {
                    flags[ofv.feature as usize] = ofv.value;
                    true
                }
            }
            TransducerOperation::R => {
                let ok = (ofv.value != Diacritic::ANY || value != Diacritic::NEUTRAL)
                    && (ofv.value == Diacritic::ANY || value == ofv.value);
                if ok {
                    flags[ofv.feature as usize] = value;
                }
                ok
            }
            TransducerOperation::D => {
                let ok = (ofv.value != Diacritic::ANY || value == Diacritic::NEUTRAL)
                    && value != ofv.value;
                if ok {
                    flags[ofv.feature as usize] = value;
                }
                ok
            }
        }
    }
}
