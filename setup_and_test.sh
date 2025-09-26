#!/bin/bash
# Setup and test script for Amazon CloudFront Client Routing Library
# This script automates the steps in the installation guide

set -e  # Exit on error

echo "=== Amazon CloudFront Client Routing Library Setup and Test ==="
echo

# Check if Rust is installed
if ! command -v rustc &> /dev/null || ! command -v cargo &> /dev/null; then
    echo "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "Rust installed successfully!"
else
    echo "Rust is already installed:"
    rustc --version
    cargo --version
fi

echo
echo "=== Building the library ==="
cargo build

echo
echo "=== Running tests ==="
cargo test

echo
echo "=== Building the example application ==="
rustc example_usage.rs -L target/debug -l amazon_cloudfront_client_routing_lib

echo
echo "=== Running the example application ==="
./example_usage

echo
echo "=== Starting the HTTP server (press Ctrl+C to stop) ==="
echo "The server will be available at http://localhost:8080"
echo "You can test it with:"
echo "curl -X POST -H \"Content-Type: application/json\" -d '{\"client_ip\":\"1.2.3.4\",\"content_group_id\":\"my-content-group\",\"fqdn\":\"example.com\"}' http://localhost:8080/encode"
echo
cd rust_test && cargo run
