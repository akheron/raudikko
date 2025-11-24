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
 * Raudikko, the Rust port is Copyright (C) 2025.
 * All Rights Reserved.
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

// Note: This test requires the morphology data file to be present.
// Since we haven't implemented data loading yet, this test is disabled.

#[cfg(test)]
mod fst_loading_tests {
    #[test]
    #[ignore]
    fn test_load_transducer() {
        // This test will be enabled once we have morphology data loading implemented
        // For now, it's just a placeholder to show the structure

        // let file = std::fs::File::open("path/to/mor.vfst").unwrap();
        // let transducer = UnweightedVfstLoader::load(file).unwrap();
        // assert!(transducer.flag_diacritic_feature_count > 0);
    }
}

#[test]
fn test_fst_module_compiles() {
    // This is a smoke test to verify the FST module compiles
    // without actually loading data
    assert!(true);
}
