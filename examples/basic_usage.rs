//! Basic usage example for EdgeGrid authentication

use akamai_edgegrid::{EdgeGridClient, EdgeGridConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Create configuration with credentials
    let config = EdgeGridConfig::new(
        "your-client-token".to_string(),
        "your-client-secret".to_string(),
        "your-access-token".to_string(),
        "your-host.luna.akamaiapis.net".to_string(),
    );

    // Create client
    let client = EdgeGridClient::new(config)?;

    // Make a simple GET request
    println!("Making GET request...");
    let response = client
        .get("/billing-usage/v1/reportSources")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Response: {}", response.text().await?);

    // Make a POST request with JSON body
    println!("\nMaking POST request...");
    let body = serde_json::json!({
        "key": "value",
        "number": 42
    });

    let response = client
        .post("/some/api/endpoint")
        .json(&body)
        .send()
        .await?;

    println!("Status: {}", response.status());

    // Make a request with query parameters
    println!("\nMaking request with query params...");
    let response = client
        .get("/some/api/endpoint")
        .query("limit", "10")
        .query("offset", "0")
        .send()
        .await?;

    println!("Status: {}", response.status());

    Ok(())
}