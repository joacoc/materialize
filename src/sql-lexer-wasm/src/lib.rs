// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE file at the
// root of this repository, or online at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// BEGIN LINT CONFIG
// DO NOT EDIT. Automatically generated by bin/gen-lints.
// Have complaints about the noise? See the note in misc/python/materialize/cli/gen-lints.py first.
#![allow(unknown_lints)]
#![allow(clippy::style)]
#![allow(clippy::complexity)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::mutable_key_type)]
#![allow(clippy::stable_sort_primitive)]
#![allow(clippy::map_entry)]
#![allow(clippy::box_default)]
#![allow(clippy::drain_collect)]
#![warn(clippy::bool_comparison)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::no_effect)]
#![warn(clippy::unnecessary_unwrap)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::wildcard_dependencies)]
#![warn(clippy::zero_prefixed_literal)]
#![warn(clippy::borrowed_box)]
#![warn(clippy::deref_addrof)]
#![warn(clippy::double_must_use)]
#![warn(clippy::double_parens)]
#![warn(clippy::extra_unused_lifetimes)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_question_mark)]
#![warn(clippy::needless_return)]
#![warn(clippy::redundant_pattern)]
#![warn(clippy::redundant_slicing)]
#![warn(clippy::redundant_static_lifetimes)]
#![warn(clippy::single_component_path_imports)]
#![warn(clippy::unnecessary_cast)]
#![warn(clippy::useless_asref)]
#![warn(clippy::useless_conversion)]
#![warn(clippy::builtin_type_shadow)]
#![warn(clippy::duplicate_underscore_argument)]
#![warn(clippy::double_neg)]
#![warn(clippy::unnecessary_mut_passed)]
#![warn(clippy::wildcard_in_or_patterns)]
#![warn(clippy::crosspointer_transmute)]
#![warn(clippy::excessive_precision)]
#![warn(clippy::overflow_check_conditional)]
#![warn(clippy::as_conversions)]
#![warn(clippy::match_overlapping_arm)]
#![warn(clippy::zero_divided_by_zero)]
#![warn(clippy::must_use_unit)]
#![warn(clippy::suspicious_assignment_formatting)]
#![warn(clippy::suspicious_else_formatting)]
#![warn(clippy::suspicious_unary_op_formatting)]
#![warn(clippy::mut_mutex_lock)]
#![warn(clippy::print_literal)]
#![warn(clippy::same_item_push)]
#![warn(clippy::useless_format)]
#![warn(clippy::write_literal)]
#![warn(clippy::redundant_closure)]
#![warn(clippy::redundant_closure_call)]
#![warn(clippy::unnecessary_lazy_evaluations)]
#![warn(clippy::partialeq_ne_impl)]
#![warn(clippy::redundant_field_names)]
#![warn(clippy::transmutes_expressible_as_ptr_casts)]
#![warn(clippy::unused_async)]
#![warn(clippy::disallowed_methods)]
#![warn(clippy::disallowed_macros)]
#![warn(clippy::disallowed_types)]
#![warn(clippy::from_over_into)]
// END LINT CONFIG

// Many things here only compile in the wasm32 target. We don't need to annotate it that way because
// this should only ever be built by wasm-build which specifies that.

use lol_alloc::{FreeListAllocator, LockedAllocator};
use mz_sql_lexer::lexer::{self, PosToken};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const KEYWORDS_TS_DEF: &str = r#"export function getKeywords(): string[];"#;

#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> =
    LockedAllocator::new(FreeListAllocator::new());

#[wasm_bindgen(typescript_custom_section)]
const LEX_TS_DEF: &str = r#"export function lex(query: string): PosToken[];"#;

#[wasm_bindgen(js_name = PosToken, getter_with_clone, inspectable)]
#[derive(Debug)]
pub struct JsToken {
    pub kind: String,
    pub offset: usize,
}

impl From<PosToken> for JsToken {
    fn from(value: PosToken) -> Self {
        JsToken {
            kind: value.kind.to_string(),
            offset: value.offset,
        }
    }
}

/// Lexes a SQL query.
///
/// Returns a list of tokens alongside their corresponding byte offset in the
/// input string. Returns an error if the SQL query is lexically invalid.
///
/// See the module documentation for more information about the lexical
/// structure of SQL.
#[wasm_bindgen(skip_typescript)]
pub fn lex(query: &str) -> Result<Vec<JsValue>, JsError> {
    let lexed = lexer::lex(query).map_err(|e| JsError::new(&e.message))?;
    Ok(lexed
        .into_iter()
        .map(|token| JsValue::from(JsToken::from(token)))
        .collect())
}

#[wasm_bindgen(js_name = getKeywords, skip_typescript)]
// #[wasm_bindgen] cannot be applied directly to KEYWORDS, only to functions, structs, enums,
// impls, or extern blocks. Wrap this in a function.
pub fn get_keywords() -> Vec<JsValue> {
    mz_sql_lexer::keywords::KEYWORDS
        .keys()
        .map(|k| JsValue::from(k.to_string()))
        .collect()
}
