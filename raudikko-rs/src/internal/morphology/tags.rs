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

// https://github.com/voikko/corevoikko/blob/master/voikko-fi/vvfst/root.lexc
#[allow(dead_code)]
pub struct Tags;

impl Tags {
    // Boundaries
    pub const PREFIX_B: &'static str = "[B";
    pub const BC: &'static str = "[Bc]";
    pub const BH: &'static str = "[Bh]";
    pub const BM: &'static str = "[Bm]";

    // Comparative
    pub const DE: &'static str = "[De]";
    pub const DG: &'static str = "[Dg]";

    // Clitic
    pub const FKO: &'static str = "[Fko]";

    pub const ISF: &'static str = "[Isf]";
    pub const ICU: &'static str = "[Icu]";
    pub const ICA: &'static str = "[Ica]";
    pub const ION: &'static str = "[Ion]";
    pub const IVJ: &'static str = "[Ivj]";
    pub const IRA: &'static str = "[Ira]";
    pub const IRM: &'static str = "[Irm]";

    // Classes
    pub const PREFIX_L: &'static str = "[L";
    pub const PREFIX_LE: &'static str = "[Le";

    // Parameters
    pub const PREFIX_X: &'static str = "[X";
    pub const XP: &'static str = "[Xp]"; // perusmuoto
    pub const XJ: &'static str = "[Xj]"; // johtimen perusmuoto
    pub const XR: &'static str = "[Xr]"; // rakenne
    pub const XS: &'static str = "[Xs]"; // sourceid
    pub const X: &'static str = "[X]"; // end marker
}
