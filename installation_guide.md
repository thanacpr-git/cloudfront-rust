# Amazon CloudFront Client Routing Library - Installation Guide

This guide will walk you through the process of installing Rust and running the Amazon CloudFront Client Routing Library on your local machine.

## Step 1: Install Rust

Rust consists of two main components: `rustc` (the compiler) and `cargo` (the package manager and build tool). The recommended way to install Rust is using rustup.

### For macOS, Linux, or WSL (Windows Subsystem for Linux):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions. The default installation options are recommended for most users.

After installation, add Rust to your current shell's path:

```bash
source $HOME/.cargo/env
```

### For Windows (using PowerShell):

```powershell
Invoke-WebRequest -Uri https://win.rustup.rs -OutFile rustup-init.exe
.\rustup-init.exe
```

Follow the on-screen instructions and restart your terminal after installation.

### Verify Installation

Verify that Rust and Cargo are correctly installed:

```bash
rustc --version
cargo --version
```

You should see version information for both commands.

## Step 2: Clone the Repository

Clone the Amazon CloudFront Client Routing Library repository:

```bash
git clone https://github.com/aws/amazon-cloudfront-client-routing-library.git
cd amazon-cloudfront-client-routing-library
```

## Step 3: Build the Library

Build the library using Cargo:

```bash
cargo build
```

This will compile the library in debug mode. For a release build (optimized and without debug information), use:

```bash
cargo build --release
```

## Step 4: Run Tests

Run the tests to ensure everything is working correctly:

```bash
cargo test
```

To run specific test files:

```bash
cargo test --test encode
cargo test --test decode
```

To see test output (including println! statements):

```bash
cargo test -- --nocapture
```

## Step 5: Run the Example Application

The repository includes a simple HTTP server example in the `rust_test` directory. To run it:

```bash
cd rust_test
cargo run
```

This will start a server on http://localhost:8080 that provides two endpoints:
- `/encode` - Encodes a client IP into a DNS label
- `/decode` - Decodes a DNS label

### Testing the Server

While the server is running, open a new terminal and use curl to test the endpoints:

```bash
# Test the encode endpoint
curl -X POST -H "Content-Type: application/json" -d '{"client_ip":"1.2.3.4","content_group_id":"my-content-group","fqdn":"example.com"}' http://localhost:8080/encode

# Test the decode endpoint (use the encoded URL from the previous response)
curl -X POST -H "Content-Type: application/json" -d '{"encoded_url":"abacaqdaaaaaaaamiyjpbvsbssmxe.example.com"}' http://localhost:8080/decode
```

## Using the Library in Your Project

To use this library in your own Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
amazon-cloudfront-client-routing-lib = { git = "https://github.com/aws/amazon-cloudfront-client-routing-library.git" }
```

Then, in your Rust code:

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

## Troubleshooting

### Common Issues

1. **Rust not found after installation**: Make sure to restart your terminal or run `source $HOME/.cargo/env`.

2. **Build errors**: Ensure you have the necessary development tools installed:
   - On Ubuntu/Debian: `sudo apt install build-essential`
   - On Fedora/RHEL: `sudo dnf install gcc`
   - On macOS: Install Xcode Command Line Tools with `xcode-select --install`

3. **Permission errors**: If you encounter permission errors when installing Rust or running cargo commands, you may need to use `sudo` or fix directory permissions.

4. **Network errors during build**: If cargo fails to download dependencies, check your internet connection and any proxy settings.

### Getting Help

If you encounter issues not covered here:

1. Check the [Rust documentation](https://www.rust-lang.org/learn)
2. Visit the [Rust users forum](https://users.rust-lang.org/)
3. Ask questions on [Stack Overflow](https://stackoverflow.com/questions/tagged/rust)
4. Open an issue on the [GitHub repository](https://github.com/aws/amazon-cloudfront-client-routing-library/issues)
