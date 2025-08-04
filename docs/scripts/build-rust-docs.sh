#!/bin/bash

# Build Rust documentation and copy to Docusaurus static folder
set -e

echo "Building Rust documentation..."

# Navigate to project root
cd "$(dirname "$0")/../.."

# Generate Rust docs without external dependencies
cargo doc --no-deps --release

# Create rust-api directory in static folder
mkdir -p docs/static/rust-api

# Copy generated docs to static folder
cp -r target/doc/* docs/static/rust-api/

echo "Rust documentation copied to docs/static/rust-api/"
echo "Access at: http://localhost:3000/rust-api/reactive_tui/"