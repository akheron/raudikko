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

use crate::analysis::{AnalysisClass, WordClass};
use crate::internal::fst::{SymbolMap, TransducerOperation};
use std::rc::Rc;

// Constants for tag prefixes (from Tags.java)
const PREFIX_LE: &str = "[Le";
const PREFIX_X: &str = "[X";
const PREFIX_L: &str = "[L";
const PREFIX_B: &str = "[B";
const XP: &str = "[Xp]";
const XJ: &str = "[Xj]";

#[derive(Debug, Clone)]
pub(crate) struct Symbol {
    s: Rc<str>,
}

impl Symbol {
    pub(crate) fn new(s: String) -> Self {
        Symbol { s: s.into() }
    }

    pub(crate) fn final_symbol() -> Self {
        Symbol {
            s: "<final>".into(),
        }
    }

    pub(crate) fn is_epsilon(&self) -> bool {
        matches!(self.as_diacritic(), Some(d) if d.is_epsilon())
    }

    pub(crate) fn as_diacritic(&self) -> Option<&Diacritic> {
        None
    }

    pub(crate) fn is_final(&self) -> bool {
        &*self.s == "<final>"
    }

    pub(crate) fn is_char(&self) -> bool {
        self.s.len() == 1
    }

    pub(crate) fn is_diacritic(&self) -> bool {
        false
    }

    pub(crate) fn to_output_symbol(&self) -> Symbol {
        self.clone()
    }

    pub(crate) fn char_value(&self) -> char {
        assert!(self.s.len() == 1, "not a char '{}'", self.s);
        self.s.chars().next().unwrap()
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.s
    }

    pub(crate) fn matches_str(&self, s: &str) -> bool {
        &*self.s == s
    }

    pub(crate) fn matches_word_class(&self, c: WordClass) -> bool {
        let tag = c.morphology_tag();
        self.s.len() == tag.len() + 2
            && self.s.starts_with('[')
            && self.s.ends_with(']')
            && self.s[1..].starts_with(tag)
    }

    pub(crate) fn is_name_tag(&self) -> bool {
        self.starts_with(PREFIX_LE)
    }

    pub(crate) fn is_x_parameter(&self) -> bool {
        self.starts_with(PREFIX_X)
    }

    pub(crate) fn is_class_tag(&self) -> bool {
        self.starts_with(PREFIX_L)
    }

    pub(crate) fn is_base_form_tag(&self) -> bool {
        self.matches_str(XP) || self.matches_str(XJ)
    }

    pub(crate) fn is_boundary(&self) -> bool {
        self.starts_with(PREFIX_B)
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.s.starts_with(prefix)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Diacritic {
    base: Symbol,
    pub(crate) op: TransducerOperation,
    pub(crate) feature: i16,
    pub(crate) value: i16,
}

impl Diacritic {
    pub(crate) const NEUTRAL: i16 = 0;
    pub(crate) const ANY: i16 = 1;

    pub(crate) fn epsilon() -> Self {
        Diacritic {
            base: Symbol::new("".to_string()),
            op: TransducerOperation::C,
            feature: 0,
            value: 0,
        }
    }

    pub(crate) fn is_epsilon(&self) -> bool {
        self.base.s.is_empty() && self.op == TransducerOperation::C && self.feature == 0 && self.value == 0
    }

    pub(crate) fn parse(
        symbol: &str,
        features: &mut SymbolMap,
        values: &mut SymbolMap,
    ) -> Result<Self, String> {
        if symbol.len() < 4 {
            return Err(format!("Malformed flag diacritic: '{}'.", symbol));
        }

        let feature_and_value = &symbol[3..symbol.len() - 1];
        let value_start = feature_and_value.find('.');
        let feature = match value_start {
            Some(pos) => &feature_and_value[0..pos],
            None => feature_and_value,
        };
        let value = match value_start {
            Some(pos) => &feature_and_value[pos + 1..],
            None => "@",
        };

        let op_char = symbol.chars().nth(1).ok_or_else(|| {
            format!("Invalid diacritic format: '{}'", symbol)
        })?;

        let op = TransducerOperation::for_code(op_char)?;
        let feature_code = features.get_code(feature);
        let value_code = values.get_code(value);

        Ok(Diacritic {
            base: Symbol::new(symbol.to_string()),
            op,
            feature: feature_code,
            value: value_code,
        })
    }

    pub(crate) fn as_symbol(&self) -> &Symbol {
        &self.base
    }

    pub(crate) fn to_output_symbol(&self) -> Symbol {
        Symbol::new("".to_string())
    }
}

impl std::fmt::Display for Diacritic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}

// Wrapper enum to handle Symbol or Diacritic
#[derive(Debug, Clone)]
pub(crate) enum SymbolOrDiacritic {
    Symbol(Symbol),
    Diacritic(Diacritic),
}

impl SymbolOrDiacritic {
    pub(crate) fn is_epsilon(&self) -> bool {
        match self {
            SymbolOrDiacritic::Symbol(s) => s.is_epsilon(),
            SymbolOrDiacritic::Diacritic(d) => d.is_epsilon(),
        }
    }

    pub(crate) fn as_diacritic(&self) -> Option<&Diacritic> {
        match self {
            SymbolOrDiacritic::Diacritic(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn is_final(&self) -> bool {
        match self {
            SymbolOrDiacritic::Symbol(s) => s.is_final(),
            SymbolOrDiacritic::Diacritic(d) => d.as_symbol().is_final(),
        }
    }

    pub(crate) fn is_char(&self) -> bool {
        match self {
            SymbolOrDiacritic::Symbol(s) => s.is_char(),
            SymbolOrDiacritic::Diacritic(d) => d.as_symbol().is_char(),
        }
    }

    pub(crate) fn char_value(&self) -> char {
        match self {
            SymbolOrDiacritic::Symbol(s) => s.char_value(),
            SymbolOrDiacritic::Diacritic(d) => d.as_symbol().char_value(),
        }
    }

    pub(crate) fn to_output_symbol(&self) -> Symbol {
        match self {
            SymbolOrDiacritic::Symbol(s) => s.clone(),
            SymbolOrDiacritic::Diacritic(_) => Symbol::new("".to_string()),
        }
    }

    pub(crate) fn as_symbol(&self) -> &Symbol {
        match self {
            SymbolOrDiacritic::Symbol(s) => s,
            SymbolOrDiacritic::Diacritic(d) => d.as_symbol(),
        }
    }
}

impl From<Symbol> for SymbolOrDiacritic {
    fn from(s: Symbol) -> Self {
        SymbolOrDiacritic::Symbol(s)
    }
}

impl From<Diacritic> for SymbolOrDiacritic {
    fn from(d: Diacritic) -> Self {
        SymbolOrDiacritic::Diacritic(d)
    }
}
