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

use crate::internal::fst::{
    CharTransition, Diacritic, DiacriticTransition, State, Symbol, SymbolMap, SymbolOrDiacritic,
    UnweightedTransducer,
};
use crate::internal::utils::MyInputStream;
use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::rc::Rc;

const HEADER_SIZE: usize = 16;
const TRANSITION_ALIGNMENT: usize = 8;

pub(crate) struct UnweightedVfstLoader;

impl UnweightedVfstLoader {
    pub(crate) fn load<R: Read>(input_stream: R) -> std::io::Result<UnweightedTransducer> {
        let mut stream = MyInputStream::new(input_stream);
        stream.skip_n_bytes(HEADER_SIZE)?;

        let mut features = SymbolMap::new();
        let mut values = SymbolMap::new();

        // initialize these to 0 and 1 (Neutral and Any)
        values.get_code("");
        values.get_code("@");

        let symbol_count = stream.read_short()?;
        let mut symbols: Vec<SymbolOrDiacritic> = Vec::with_capacity(symbol_count as usize);
        for i in 0..symbol_count {
            let s = stream.read_utf8_string()?;

            if i == 0 {
                symbols.push(Diacritic::epsilon().into());
            } else if s.starts_with('@') {
                let diacritic = Diacritic::parse(&s, &mut features, &mut values)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                symbols.push(diacritic.into());
            } else {
                symbols.push(Symbol::new(s).into());
            }
        }

        let partial = stream.get_position() % TRANSITION_ALIGNMENT;
        if partial != 0 {
            stream.skip_n_bytes(TRANSITION_ALIGNMENT - partial)?;
        }

        let mut transitions: Vec<Option<TransitionData>> = Vec::new();
        while stream.has_more()? {
            let sym_in = stream.read_short()?;
            let sym_out = stream.read_short()?;
            let target_state = stream.read_int24()?;
            let mut more_transitions = stream.read_byte()? as i32;

            assert!(sym_in >= -1);
            assert!(sym_out >= 0);

            let overflow = more_transitions == 0xff;

            if overflow {
                more_transitions = stream.read_int()?;
                let _padding = stream.read_int()?;
            }

            let sym_in_obj = if sym_in == -1 {
                Symbol::final_symbol().into()
            } else {
                symbols[sym_in as usize].clone()
            };
            let sym_out_obj = symbols[sym_out as usize].clone();
            transitions.push(Some(TransitionData {
                sym_in: sym_in_obj,
                sym_out: sym_out_obj.to_output_symbol(),
                target_state: target_state as usize,
                more_transitions: more_transitions as usize,
            }));

            if overflow {
                transitions.push(None); // add null to keep indexes correct
            }
        }

        let mut targets = HashSet::new();
        targets.insert(0);
        for transition in &transitions {
            if let Some(tr) = transition {
                targets.insert(tr.target_state);
            }
        }

        let mut states: HashMap<usize, Rc<State>> = HashMap::new();
        for (i, transition_opt) in transitions.iter().enumerate() {
            if targets.contains(&i) {
                if let Some(tr) = transition_opt {
                    assert!(!tr.sym_in.is_final() || tr.more_transitions == 0);
                }
                states.insert(i, Rc::new(State::new()));
            }
        }

        for (i, _) in transitions.iter().enumerate() {
            if !targets.contains(&i) {
                continue;
            }

            let head = transitions[i].as_ref().unwrap();
            let state = states.get(&i).unwrap();

            if head.sym_in.is_final() {
                // State is already created with empty transitions
            } else {
                let mut diacritic_transitions = Vec::new();
                let mut character_transitions = Vec::new();

                let offset = if head.has_overflow() { 1 } else { 0 };
                for j in 0..(head.more_transitions + 1 + offset) {
                    if j == 1 && head.has_overflow() {
                        continue;
                    }

                    let data = transitions[i + j].as_ref().unwrap();
                    let target_state = states.get(&data.target_state).unwrap().clone();
                    let diacritic_opt = data.sym_in.as_diacritic();

                    if let Some(diacritic) = diacritic_opt {
                        diacritic_transitions.push(DiacriticTransition::new(
                            diacritic.clone(),
                            data.sym_out.clone(),
                            target_state,
                        ));
                    } else {
                        character_transitions.push(CharTransition::new(
                            data.sym_in.char_value(),
                            data.sym_out.clone(),
                            target_state,
                        ));
                    }
                }

                // We need to mutate the state, so we need to use unsafe or interior mutability
                // For now, let's rebuild the state with the transitions
                let state_ptr = Rc::as_ptr(state) as *mut State;
                unsafe {
                    (*state_ptr).diacritic_transitions = diacritic_transitions;
                    (*state_ptr).char_transitions = character_transitions;
                }
            }
        }

        Ok(UnweightedTransducer::new(
            symbols,
            states.get(&0).unwrap().clone(),
            features.size(),
        ))
    }
}

struct TransitionData {
    sym_in: SymbolOrDiacritic,
    sym_out: Symbol,
    target_state: usize,
    more_transitions: usize,
}

impl TransitionData {
    fn has_overflow(&self) -> bool {
        self.more_transitions >= 255
    }
}
