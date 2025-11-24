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
use std::collections::HashMap;
use std::marker::PhantomData;

/// Lookup table for mapping FST symbols to enum values
pub struct TaggedValueLookupTable<T>
where
    T: AnalysisClass + Copy + 'static,
{
    map: HashMap<String, T>,
    _phantom: PhantomData<T>,
}

impl<T> TaggedValueLookupTable<T>
where
    T: AnalysisClass + Copy + 'static,
{
    /// Create a new lookup table from an array of values
    pub fn new(values: &[T]) -> Self {
        let mut map = HashMap::new();
        for &value in values {
            let tag = format!("[{}]", value.morphology_tag());
            map.insert(tag, value);
        }
        Self {
            map,
            _phantom: PhantomData,
        }
    }

    /// Get the enum value for a symbol
    pub fn get(&self, tag: &Symbol) -> Option<T> {
        self.map.get(tag.as_str()).copied()
    }
}
