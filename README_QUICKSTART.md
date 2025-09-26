# Amazon CloudFront Client Routing Library - Quick Start Guide

This repository contains the Amazon CloudFront Client Routing Library, a Rust library designed for CloudFront's Client Routing feature. This feature helps direct client devices to CloudFront Points of Presence (POPs) with greater precision by utilizing client subnet information encoded in DNS labels.

## Quick Start

For the fastest way to get started, run the setup script:

```bash
./setup_and_test.sh
```

This script will:
1. Check if Rust is installed and install it if needed
2. Build the library
3. Run the tests
4. Build and run the example application
5. Start the HTTP server for testing

## Documentation

- [Installation Guide](installation_guide.md) - Detailed instructions for installing Rust and setting up the library
- [Example Usage](example_usage.rs) - A standalone example showing how to use the library
- [Summary](summary.md) - Overview of the project and its features

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

## Example Usage

```rust
use amazon_cloudfront_client_routing_lib::{encode_request_data, decode_request_data};

fn main() {
    // Encode a client IP into a DNS label
    let encoded = encode_request_data("1.2.3.4", "my-content-group", "example.com");
    println!("Encoded: {}", encoded);
    
    // Decode a DNS label
    match decode_request_data(&encoded) {
        Ok(decoded) => {
            println!("Decoded client subnet: {:?}", decoded.client_subnet);
            println!("Subnet mask: {}", decoded.subnet_mask);
            println!("Is IPv6: {}", decoded.is_ipv6);
            println!("Content group ID: {}", decoded.content_group_id);
        },
        Err(e) => println!("Error decoding: {}", e)
    }
}
```

## HTTP Server Example

The repository includes a simple HTTP server in the `rust_test` directory that demonstrates how to use the library in a web service context. The server provides two endpoints:

- `/encode` - Encodes a client IP into a DNS label
- `/decode` - Decodes a DNS label

To run the server:

```bash
cd rust_test
cargo run
```

Then, in another terminal, you can test it with:

```bash
# Test the encode endpoint
curl -X POST -H "Content-Type: application/json" -d '{"client_ip":"1.2.3.4","content_group_id":"my-content-group","fqdn":"example.com"}' http://localhost:8080/encode

# Test the decode endpoint (use the encoded URL from the previous response)
curl -X POST -H "Content-Type: application/json" -d '{"encoded_url":"abacaqdaaaaaaaamiyjpbvsbssmxe.example.com"}' http://localhost:8080/decode
```

## License

This library is licensed under the Apache License 2.0. See the LICENSE file for details.
