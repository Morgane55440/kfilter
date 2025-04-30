#!/bin/bash
# Runs tests / builds with different features enabled
# For pre-commit checks 

set -e 

cargo clippy -- -D warnings
cargo build --features std
cargo build --features serde
cargo build --features defmt
cargo test