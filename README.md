# Akamai EdgeGrid Authentication for Rust

A Rust implementation of the Akamai EdgeGrid authentication scheme, providing a simple way to make authenticated HTTP requests to Akamai APIs.

> **⚠️ IMPORTANT: Community Project**
> 
> This is a **community-created** Rust implementation of the EdgeGrid authentication protocol. This project is **NOT** officially supported, endorsed, or affiliated with Akamai Technologies. It is maintained by the open source community.
>
> For official Akamai-supported libraries, please refer to:
> - [Node.js](https://github.com/akamai/AkamaiOPEN-edgegrid-node)
> - [Python](https://github.com/akamai/AkamaiOPEN-edgegrid-python)
> - [Java](https://github.com/akamai/AkamaiOPEN-edgegrid-java)
> - [Go](https://github.com/akamai/AkamaiOPEN-edgegrid-golang)
>
> Use this library at your own risk. For production use cases requiring official support, please use one of the officially supported libraries listed above.

## Features

- Full EdgeGrid authentication protocol implementation
- Support for `.edgerc` configuration files
- Environment variable configuration
- Async/await support with Tokio
- Automatic request signing
- Support for all HTTP methods
- Query parameter and header management
- JSON request/response handling
- Comprehensive error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
akamai-edgegrid = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use akamai_edgegrid::EdgeGridClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from .edgerc file
    let client = EdgeGridClient::from_edgerc("~/.edgerc", "default")?;

    // Make an authenticated request
    let response = client
        .get("/billing-usage/v1/reportSources")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Body: {}", response.text().await?);

    Ok(())
}
```

## Configuration

### Using .edgerc File

The library supports the standard Akamai `.edgerc` configuration file format:

```ini
[default]
client_secret = C113nt533KR3TN6N90yVuAgICxIRwsObLi0E67/N8eRN=
host = akab-h05tnam3wl42son7nktnlnnx-kbob3i3v.luna.akamaiapis.net
access_token = akab-acc35t0k3nodujqunph3w7hzp7-gtm6ij
client_token = akab-c113ntt0k3n4qtari252bfyqjkq-kkeh5a
max_body = 131072

[staging]
client_secret = C113nt533KR3TN6N90yVuAgICxIRwsObLi0E67/N8eRN=
host = akab-staging.luna.akamaiapis.net
access_token = akab-acc35t0k3nodujqunph3w7hzp7-gtm6ij
client_token = akab-c113ntt0k3n4qtari252bfyqjkq-kkeh5a
```

### Using Environment Variables

You can also configure the client using environment variables:

```bash
# For default section
export AKAMAI_HOST="akab-h05tnam3wl42son7nktnlnnx-kbob3i3v.luna.akamaiapis.net"
export AKAMAI_CLIENT_TOKEN="akab-c113ntt0k3n4qtari252bfyqjkq-kkeh5a"
export AKAMAI_CLIENT_SECRET="C113nt533KR3TN6N90yVuAgICxIRwsObLi0E67/N8eRN="
export AKAMAI_ACCESS_TOKEN="akab-acc35t0k3nodujqunph3w7hzp7-gtm6ij"

# For custom section (e.g., staging)
export AKAMAI_STAGING_HOST="..."
export AKAMAI_STAGING_CLIENT_TOKEN="..."
# etc.
```

### Programmatic Configuration

```rust
use akamai_edgegrid::{EdgeGridClient, EdgeGridConfig};

let config = EdgeGridConfig::new(
    "client-token".to_string(),
    "client-secret".to_string(), 
    "access-token".to_string(),
    "host.luna.akamaiapis.net".to_string(),
);

let client = EdgeGridClient::new(config)?;
```

## Usage Examples

### GET Request

```rust
let response = client
    .get("/identity-management/v2/user-profile")
    .send()
    .await?;
```

### POST Request with JSON

```rust
let body = serde_json::json!({
    "hostname": "www.example.com",
    "notes": "Production hostname"
});

let response = client
    .post("/papi/v1/properties/prp_123456/versions/1/hostnames")
    .json(&body)
    .send()
    .await?;
```

### Request with Query Parameters

```rust
let response = client
    .get("/papi/v1/properties")
    .query("contractId", "ctr_123456")
    .query("groupId", "grp_123456")
    .send()
    .await?;
```

### Request with Custom Headers

```rust
let response = client
    .get("/papi/v1/properties/prp_123456")
    .header("Accept", "application/vnd.akamai.papirules.v2023-01-05+json")
    .send()
    .await?;
```

### Handling JSON Responses

```rust
#[derive(Deserialize)]
struct Property {
    property_id: String,
    property_name: String,
}

let property: Property = client
    .get("/papi/v1/properties/prp_123456")
    .send_json()
    .await?;

println!("Property: {} ({})", property.property_name, property.property_id);
```

## Error Handling

The library provides comprehensive error handling through the `EdgeGridError` enum:

```rust
use akamai_edgegrid::{EdgeGridError, EdgeGridClient};

match EdgeGridClient::from_edgerc("~/.edgerc", "default") {
    Ok(client) => { /* use client */ },
    Err(EdgeGridError::FileError(e)) => eprintln!("Cannot read .edgerc: {}", e),
    Err(EdgeGridError::InvalidSection(s)) => eprintln!("Section '{}' not found", s),
    Err(EdgeGridError::MissingCredential(c)) => eprintln!("Missing: {}", c),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Features

- `rustls` (default): Use rustls for TLS
- `native-tls`: Use native TLS implementation

## Running Examples

```bash
# Basic usage example
cargo run --example basic_usage

# Using .edgerc file
cargo run --example from_edgerc
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with logging
RUST_LOG=debug cargo test -- --nocapture
```

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

This is an independent project and is not affiliated with, officially maintained, authorized, endorsed, or sponsored by Akamai Technologies or any of its affiliates. Akamai is a registered trademark of Akamai Technologies, Inc. All product and company names are trademarks™ or registered® trademarks of their respective holders. Use of them does not imply any affiliation with or endorsement by them.

The authors and contributors of this project are not responsible for any damages or issues that may arise from using this library. Users should thoroughly test this implementation before using it in any production environment.