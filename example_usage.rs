//! Example usage of the Amazon CloudFront Client Routing Library
//!
//! This example demonstrates how to use the encode_request_data and decode_request_data
//! functions from the Amazon CloudFront Client Routing Library.
//!
//! To run this example:
//! 1. Make sure you have Rust installed (see installation_guide.md)
//! 2. Run: `rustc example_usage.rs -L target/debug -l amazon_cloudfront_client_routing_lib`
//! 3. Execute: `./example_usage`

extern crate amazon_cloudfront_client_routing_lib;

use amazon_cloudfront_client_routing_lib::{encode_request_data, decode_request_data};

fn main() {
    println!("Amazon CloudFront Client Routing Library - Example Usage");
    println!("-------------------------------------------------------\n");

    // Example 1: Encode an IPv4 address
    let client_ip = "192.168.1.1";
    let content_group_id = "my-content-group";
    let fqdn = "example.com";
    
    println!("Example 1: Encoding IPv4 address");
    println!("Client IP: {}", client_ip);
    println!("Content Group ID: {}", content_group_id);
    println!("FQDN: {}", fqdn);
    
    let encoded = encode_request_data(client_ip, content_group_id, fqdn);
    println!("Encoded URL: {}\n", encoded);
    
    // Example 2: Decode the encoded URL
    println!("Example 2: Decoding the encoded URL");
    println!("Encoded URL: {}", encoded);
    
    match decode_request_data(&encoded) {
        Ok(decoded) => {
            println!("Decoded successfully:");
            println!("  Client subnet: {:?}", decoded.client_subnet);
            println!("  Subnet mask: {}", decoded.subnet_mask);
            println!("  Is IPv6: {}", decoded.is_ipv6);
            println!("  Content group ID: {}", decoded.content_group_id);
        },
        Err(e) => println!("Error decoding: {}", e)
    }
    println!();
    
    // Example 3: Encode an IPv6 address
    let ipv6_client_ip = "2001:0db8:85a3:0000:0000:8a2e:0370:7334";
    
    println!("Example 3: Encoding IPv6 address");
    println!("Client IP: {}", ipv6_client_ip);
    println!("Content Group ID: {}", content_group_id);
    println!("FQDN: {}", fqdn);
    
    let ipv6_encoded = encode_request_data(ipv6_client_ip, content_group_id, fqdn);
    println!("Encoded URL: {}\n", ipv6_encoded);
    
    // Example 4: Decode the IPv6 encoded URL
    println!("Example 4: Decoding the IPv6 encoded URL");
    println!("Encoded URL: {}", ipv6_encoded);
    
    match decode_request_data(&ipv6_encoded) {
        Ok(decoded) => {
            println!("Decoded successfully:");
            println!("  Client subnet: {:?}", decoded.client_subnet);
            println!("  Subnet mask: {}", decoded.subnet_mask);
            println!("  Is IPv6: {}", decoded.is_ipv6);
            println!("  Content group ID: {}", decoded.content_group_id);
        },
        Err(e) => println!("Error decoding: {}", e)
    }
}
