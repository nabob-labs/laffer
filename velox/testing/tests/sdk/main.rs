//! Integration tests for the Rust SDK (`velox-sdk`).
//!
//! These exercise the SDK's HTTP and WebSocket clients against a mock
//! velox-httpd backed by `velox-testing`'s in-process chain. They live here,
//! rather than in `velox-sdk`, so that `velox-testing` is the importer — which
//! keeps the test harness a leaf that no shipped crate depends on.

mod utils;

mod client;
mod core;
mod smoke;
