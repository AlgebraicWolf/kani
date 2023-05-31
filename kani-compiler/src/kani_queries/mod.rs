// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Define the communication between KaniCompiler and the codegen implementation.

use std::sync::{Arc, Mutex};
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

#[derive(Debug, Default, Clone, Copy, AsRefStr, EnumString, EnumVariantNames, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum ReachabilityType {
    /// Start the cross-crate reachability analysis from all harnesses in the local crate.
    Harnesses,
    /// Don't perform any reachability analysis. This will skip codegen for this crate.
    #[default]
    None,
    /// Start the cross-crate reachability analysis from all public functions in the local crate.
    PubFns,
    /// Start the cross-crate reachability analysis from all *test* (i.e. `#[test]`) harnesses in the local crate.
    Tests,
}

/// This structure should only be used behind a synchronized reference or a snapshot.
#[derive(Debug, Clone)]
pub struct QueryDb {
    pub check_assertion_reachability: bool,
    pub emit_vtable_restrictions: bool,
    pub output_pretty_json: bool,
    pub ignore_global_asm: bool,
    /// When set, instructs the compiler to produce the symbol table for CBMC in JSON format and use symtab2gb.
    pub write_json_symtab: bool,
    pub reachability_analysis: ReachabilityType,
    pub stubbing_enabled: bool,
    pub unstable_features: Vec<String>,
}

impl QueryDb {
    pub fn new() -> Arc<Mutex<QueryDb>> {
        Arc::new(Mutex::new(QueryDb {
            check_assertion_reachability: false,
            emit_vtable_restrictions: false,
            output_pretty_json: false,
            ignore_global_asm: false,
            write_json_symtab: false,
            reachability_analysis: ReachabilityType::None,
            stubbing_enabled: false,
            unstable_features: vec![],
        }))
    }
}