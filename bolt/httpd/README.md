# HTTPD

A lightweight HTTP server crate that provides GraphQL and REST endpoints for Bolt applications. This crate is a simplified version of the indexer HTTP server, focusing only on core Bolt query functionality without any indexing features.

## Highlights

- **GraphQL API**: Query Bolt applications, store, and simulate transactions
- **REST endpoints**: Health checks and status information
- **CORS support**: Configurable cross-origin resource sharing
- **Metrics**: Optional Prometheus metrics (with `metrics` feature)
- **Tracing**: Optional tracing support (with `tracing` feature)

## Usage

### As a Library

```rust
use httpd::{context::Context, graphql, server};
use std::sync::Arc;

// Create your Bolt app instance
let bolt_app = Arc::new(your_bolt_app);
let context = Context::new(bolt_app);

// Run the server
server::run_server(
    "127.0.0.1",
    8080,
    None, // CORS origin
    context,
    server::config_app,
    graphql::build_schema,
)
.await?;
```

### As a Binary

```bash
# Run with default settings (127.0.0.1:8080)
cargo run --bin httpd

# Run with custom settings
cargo run --bin httpd -- --ip 0.0.0.0 --port 3000 --cors-origin https://example.com
```

## GraphQL Schema

The server provides the following GraphQL queries:

- `queryApp(request: Query!, height: UInt64): QueryResponse!` - Query the Bolt application
- `queryStore(key: String!, height: UInt64, prove: Boolean!): Store!` - Query the key-value store
- `queryStatus: Status!` - Get chain status information
- `simulate(tx: UnsignedTx!): TxOutcome!` - Simulate a transaction

## REST Endpoints

- `GET /` - Health check (returns "OK")
- `GET /up` - Status endpoint with block height and git commit
- `GET /graphql` - GraphiQL playground
- `POST /graphql` - GraphQL endpoint

## Features

### Metrics

Enable Prometheus metrics with the `metrics` feature:

```bash
cargo run --bin httpd --features metrics
```

### Tracing

Enable tracing with the `tracing` feature:

```bash
cargo run --bin httpd --features tracing
```

## Differences from Indexer HTTPD

This crate is a simplified version of `indexer/httpd` with the following differences:

- **No database**: Removed all database-related functionality
- **No indexing**: Removed block, transaction, event, and message queries
- **No subscriptions**: Removed GraphQL subscriptions
- **No mutations**: Removed GraphQL mutations
- **Simplified context**: Only includes the Bolt app, no database or indexer path
- **Core functionality**: Focuses only on Bolt query operations

## Extending

To add additional functionality (like indexing features), you can:

1. Extend the `Context` struct with additional fields
2. Add new GraphQL types and queries
3. Implement additional traits for your specific use case
4. Use the existing server infrastructure as a foundation

## License

This project is licensed under the same terms as the workspace.
