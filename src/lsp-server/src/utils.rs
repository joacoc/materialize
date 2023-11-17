// Copyright (c) 2023 IWANABETHATGUY
// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.
//
// Portions of this file are derived from the tower-lsp-boilerplate project. The original source
// code was retrieved on 10/02/2023 from:
//
//     https://github.com/IWANABETHATGUY/tower-lsp-boilerplate/blob/86e3f8603ce97c235f3af81bd784b8b4fbe9f81e/src/main.rs
//
// The original source code is subject to the terms of the <APACHE|MIT> license, a copy
// of which can be found in the LICENSE file at the root of this repository.

use ropey::Rope;
use tower_lsp::{
    jsonrpc::{Error, ErrorCode},
    lsp_types::Position,
};

/// This functions is a helper function that converts an offset in the file to a (line, column).
///
/// It is useful when translating an ofsset returned by [mz_sql_mz_sql_lexer::keywords::Keywordparser::parse_statements]
/// to an (x,y) position in the text to represent the error in the correct token.
pub fn offset_to_position(offset: usize, rope: &Rope) -> Option<Position> {
    let line = rope.try_char_to_line(offset).ok()?;
    let first_char_of_line = rope.try_line_to_char(line).ok()?;
    let column = offset - first_char_of_line;

    // Convert to u32.
    let line_u32 = line.try_into().ok()?;
    let column_u32 = column.try_into().ok()?;

    Some(Position::new(line_u32, column_u32))
}

/// Builds a [tower_lsp::jsonrpc::Error]
///
/// Use this function to map normal errors to the one the trait expects
pub fn build_error(message: &'static str) -> tower_lsp::jsonrpc::Error {
    Error {
        code: ErrorCode::InternalError,
        message: std::borrow::Cow::Borrowed(message),
        data: None,
    }
}
