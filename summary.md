# Amazon CloudFront Client Routing Library - Summary

## Project Overview

The Amazon CloudFront Client Routing Library is a Rust library designed for CloudFront's Client Routing feature. This feature helps direct client devices to CloudFront Points of Presence (POPs) with greater precision by utilizing client subnet information encoded in DNS labels.

## Key Features

- **Encode DNS Labels**: Generate DNS labels with encoded client subnet information
- **Decode DNS Labels**: Extract client routing information from DNS labels
- **Support for IPv4 and IPv6**: Handle both IP address formats

## Main Functions

1. **`encode_request_data(client_ip, content_group_id, fqdn)`**
   - Takes client IP, content group ID, and FQDN
   - Returns domain with client routing key prepended as a subdomain

2. **`decode_request_data(domain)`**
   - Takes a domain (FQDN or just the DNS label)
   - Returns decoded client routing information or an error

## Running Locally

### Prerequisites

1. **Install Rust** (version 1.63 or higher):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Verify Installation**:
   ```bash
   rustc --version
   cargo --version
   ```

### Building the Library

```bash
# Clone the repository (if needed)
git clone https://github.com/aws/amazon-cloudfront-client-routing-library.git
cd amazon-cloudfront-client-routing-library

# Build the library
cargo build

# Build in release mode
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test files
cargo test --test encode
cargo test --test decode

# Run tests with output
cargo test -- --nocapture
```

### Using in Your Project

Add to your `Cargo.toml`:
```toml
[dependencies]
amazon-cloudfront-client-routing-lib = { git = "https://github.com/aws/amazon-cloudfront-client-routing-library.git" }
```

Example usage:
```rust
use amazon_cloudfront_client_routing_lib::{encode_request_data, decode_request_data};

fn main() {
    // Encode a client IP into a DNS label
    let encoded = encode_request_data("1.2.3.4", "", "example.com");
    println!("Encoded: {}", encoded);
    
    // Decode a DNS label
    match decode_request_data(&encoded) {
        Ok(decoded) => {
            println!("Decoded client subnet: {:?}", decoded.client_subnet);
            println!("Subnet mask: {}", decoded.subnet_mask);
            println!("Is IPv6: {}", decoded.is_ipv6);
        },
        Err(e) => println!("Error decoding: {}", e)
    }
}
```

### Documentation

Generate and view documentation:
```bash
cargo doc --open
```
