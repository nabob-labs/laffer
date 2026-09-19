set positional-arguments

# List available recipes
default:
  @just --list

# ------------------------------------ Git -------------------------------------

# Sync the `main` branch with the origin
git-fetch-main:
  git fetch origin main:main

# Delete all local git branches except for main
git-clear-branches:
  git branch | grep -v "main" | xargs git branch -D

# ------------------------------------ Rust ------------------------------------

# Compile and install the Velox node software
install-node:
  cargo install --path velox/cli --locked

# Compile and install the Velox client CLI
install-client:
  cargo install --path sdk/rust/cli --locked

# Run all tests
test:
  RUST_BACKTRACE=1 cargo test --all-features -- --nocapture

# Run bolt tests
test-bolt:
  RUST_BACKTRACE=1 cargo test --all-features -p bolt-testing -- --nocapture

# Run velox tests
test-velox:
  RUST_BACKTRACE=1 cargo test --all-features -p velox-testing -- --nocapture

# Run velox perp tests
test-perps:
  RUST_BACKTRACE=1 cargo test --all-features -p velox-types perps::tests -- --nocapture
  RUST_BACKTRACE=1 cargo test --all-features -p velox-perps -- --nocapture
  RUST_BACKTRACE=1 cargo test --all-features -p velox-testing --test perps -- --nocapture

# Run indexer tests
test-indexer:
  RUST_BACKTRACE=1 cargo test --all-features -p indexer-testing -- --nocapture

# Check whether the code compiles
check:
  cargo check --bins --tests --benches --examples --all-features --all-targets

# Perform linting
lint:
  cargo clippy --bins --tests --benches --examples --all-features --all-targets -- -D warnings

# Perform linting but with `--no-default-features` enabled for each crate
lint-without-features:
  #!/usr/bin/env bash
  set -euo pipefail
  crates=($(cargo metadata --format-version=1 --no-deps | jq -r '.packages[].name'))
  total=${#crates[@]}
  for i in "${!crates[@]}"; do
    crate="${crates[$i]}"
    echo "[$((i+1))/$total] Checking $crate..."
    cargo clippy -p "$crate" --bins --tests --benches --examples --no-default-features --all-targets -- -D warnings
  done

# Perform formatting
fmt:
  cargo +nightly fmt --all

# Build schema
build-graphql-schema:
  cargo run -p velox-httpd build_graphql_schema -- \
    ./velox/indexer/graphql-types/src/schemas/schema.graphql

# Build the Velox Book
book:
  mdbook build --open

# Update wasm artifacts used in tests
update-testdata:
  cp -v artifacts/bolt_{mock_*,tester}.wasm bolt/vm/wasm/testdata/

# ---------------------------------- Frontend ----------------------------------

run-website:
  pnpm i
  pnpm dev:portal-web

# ----------------------------------- Debug ------------------------------------

check-candles:
  INDEXER__CLICKHOUSE__URL="http://localhost:8123" \
    INDEXER__DATABASE__URL=postgres://postgres@localhost:5432/bolt_dev \
    INDEXER__CLICKHOUSE__DATABASE=testnet_velox_production \
    INDEXER__CLICKHOUSE__PASSWORD=${CLICKHOUSE_PASSWORD} \
    RUST_LOG=info \
    cargo run -p velox-cli indexer --home localvelox/configs/velox/ check-candles
