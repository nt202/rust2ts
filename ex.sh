#!/bin/bash
set -e

echo "Building rust2ts..."
cargo build --features generate

echo "Building chess example..."
cd examples/chess
cargo build --features generate

echo "Generated files should be in target/rust2ts/"
