// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

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

#[cfg(test)]
mod tests {

    use dashmap::DashMap;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::fmt::Debug;
    use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};
    use tower_lsp::lsp_types::*;
    use tower_lsp::{lsp_types::InitializeResult, LspService, Server};

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct LspMessage<T, R> {
        jsonrpc: String,
        method: Option<String>,
        params: Option<T>,
        result: Option<R>,
        id: Option<i32>,
    }

    /// Tests local commands that do not requires interacting with any API.
    #[mz_ore::test(tokio::test)]
    #[cfg_attr(miri, ignore)] // unsupported operation: can't call foreign function `pipe2` on OS `linux`
    async fn test_lsp() {
        let (mut req_client, mut resp_client) = start_server();
        test_initialize(&mut req_client, &mut resp_client).await;
        test_parser(&mut req_client, &mut resp_client).await;
    }

    async fn test_parser(req_client: &mut DuplexStream, resp_client: &mut DuplexStream) {
        // Test "did_open". Triggers "on_change" and the parser.
        let did_open = r#"{
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {
                    "textDocument": {
                        "uri": "file:///foo.rs",
                        "languageId": "sql",
                        "version": 1,
                        "text": "SELECT 100;"
                    }
                }
            }
        "#;

        let did_open_response: Vec<LspMessage<serde_json::Value, String>> = vec![
            LspMessage {
                jsonrpc: "2.0".to_string(),
                method: Some("window/logMessage".to_string()),
                params: Some(json!(LogMessageParams {
                    message: "file opened!".to_string(),
                    typ: MessageType::INFO,
                })),
                result: None,
                id: None,
            },
            LspMessage {
                jsonrpc: "2.0".to_string(),
                method: Some("window/logMessage".to_string()),
                params: Some(json!(json!(LogMessageParams {
                    message: r#"on_change Url { scheme: "file", cannot_be_a_base: false, username: "", password: None, host: None, port: None, path: "/foo.rs", query: None, fragment: None }"#.to_string(),
                    typ: MessageType::INFO
                }))),
                result: None,
                id: None,
            },
            LspMessage {
                jsonrpc: "2.0".to_string(),
                method: Some("window/logMessage".to_string()),
                params: Some(json!(json!(LogMessageParams {
                    message: r#"Results: [StatementParseResult { ast: Select(SelectStatement { query: Query { ctes: Simple([]), body: Select(Select { distinct: None, projection: [Expr { expr: Value(Number("100")), alias: None }], from: [], selection: None, group_by: [], having: None, options: [] }), order_by: [], limit: None, offset: None }, as_of: None }), sql: "SELECT 100" }]"#.to_string(),
                    typ: MessageType::INFO
                }))),
                result: None,
                id: None,
            },
            LspMessage {
                jsonrpc: "2.0".to_string(),
                method: Some("textDocument/publishDiagnostics".to_string()),
                params: Some(json!(json!(PublishDiagnosticsParams {
                    uri: "file:///foo.rs".parse().unwrap(),
                    diagnostics: vec![],
                    version: Some(1),
                }))),
                result: None,
                id: None,
            },
        ];

        write_and_assert(
            req_client,
            resp_client,
            &mut [0; 1024],
            did_open,
            did_open_response,
        )
        .await;
    }

    async fn test_initialize(req_client: &mut DuplexStream, resp_client: &mut DuplexStream) {
        // Test that the server initializes ok.
        let initialize = r#"{"jsonrpc":"2.0","method":"initialize","params":{"capabilities":{"textDocumentSync":1}},"id":1}"#;
        let initialize_response: Vec<LspMessage<bool, InitializeResult>> = vec![LspMessage {
            jsonrpc: "2.0".to_string(),
            method: None,
            params: None,
            result: Some(InitializeResult {
                server_info: None,
                offset_encoding: None,
                capabilities: ServerCapabilities {
                    text_document_sync: Some(TextDocumentSyncCapability::Kind(
                        TextDocumentSyncKind::FULL,
                    )),
                    code_lens_provider: Some(CodeLensOptions {
                        resolve_provider: Some(true),
                    }),
                    completion_provider: Some(CompletionOptions {
                        resolve_provider: Some(false),
                        trigger_characters: Some(vec![".".to_string()]),
                        work_done_progress_options: Default::default(),
                        all_commit_characters: None,
                        completion_item: None,
                    }),
                    workspace: Some(WorkspaceServerCapabilities {
                        workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                            supported: Some(true),
                            change_notifications: Some(OneOf::Left(true)),
                        }),
                        file_operations: None,
                    }),
                    ..ServerCapabilities::default()
                },
            }),
            id: Some(1),
        }];

        write_and_assert(
            req_client,
            resp_client,
            &mut [0; 1024],
            initialize,
            initialize_response,
        )
        .await;
    }

    async fn write_and_assert<'de, T, R>(
        req_client: &mut DuplexStream,
        resp_client: &mut DuplexStream,
        buf: &'de mut [u8],
        input_message: &str,
        expected_output_message: Vec<LspMessage<T, R>>,
    ) where
        T: Debug + Deserialize<'de> + PartialEq + ToOwned + Clone,
        R: Debug + Deserialize<'de> + PartialEq + ToOwned + Clone,
    {
        req_client
            .write_all(req(input_message).as_bytes())
            .await
            .unwrap();
        let n = resp_client.read(buf).await.unwrap();
        let buf_as = std::str::from_utf8(&buf[..n]).unwrap();

        let messages = parse_response::<T, R>(buf_as.clone());
        assert_eq!(messages, expected_output_message)
    }

    fn parse_response<'de, T, R>(response: &'de str) -> Vec<LspMessage<T, R>>
    where
        T: Debug + Deserialize<'de> + PartialEq + ToOwned + Clone,
        R: Debug + Deserialize<'de> + PartialEq + ToOwned + Clone,
    {
        let mut messages: Vec<LspMessage<T, R>> = Vec::new();
        let mut slices = response.as_bytes();

        while !slices.is_empty() {
            // parse headers to get headers length
            let mut dst = [httparse::EMPTY_HEADER; 2];
            let (headers_len, _) = match httparse::parse_headers(slices, &mut dst).unwrap() {
                httparse::Status::Complete(output) => output,
                httparse::Status::Partial => panic!("Partial headers"),
            };

            // Extract content length
            let content_length = dst
                .iter()
                .find(|header| header.name.eq_ignore_ascii_case("Content-Length"))
                .and_then(|header| std::str::from_utf8(header.value).ok())
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap();

            // Extract the message body using content length
            let str_slice: &str =
                std::str::from_utf8(&slices[headers_len..headers_len + content_length]).unwrap();
            messages.push(serde_json::from_str::<LspMessage<T, R>>(str_slice).unwrap());

            // Move the slice pointer past the current message (header + content length)
            slices = &slices[headers_len + content_length..];
        }

        messages
    }

    fn start_server() -> (tokio::io::DuplexStream, tokio::io::DuplexStream) {
        let (req_client, req_server) = tokio::io::duplex(1024);
        let (resp_server, resp_client) = tokio::io::duplex(1024);

        let (service, socket) = LspService::new(|client| mz_lsp::backend::Backend {
            client,
            document_map: DashMap::new(),
        });

        // start server as concurrent task
        mz_ore::task::spawn(|| format!("taskname:{}", "lsp_server"), Server::new(req_server, resp_server, socket).serve(service));

        (req_client, resp_client)
    }

    fn req(msg: &str) -> String {
        format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg)
    }
}
