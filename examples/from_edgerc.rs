//! Example of using EdgeGrid authentication with .edgerc file

use akamai_edgegrid::EdgeGridClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Get .edgerc path from environment or use default
    let edgerc_path = env::var("EDGERC_PATH").unwrap_or_else(|_| "~/.edgerc".to_string());
    let section = env::var("EDGERC_SECTION").unwrap_or_else(|_| "default".to_string());

    println!("Loading credentials from {} [{}]", edgerc_path, section);

    // Create client from .edgerc file
    let client = EdgeGridClient::from_edgerc(&edgerc_path, &section)?;

    // Example: Get property manager properties
    println!("Fetching properties...");
    let response = client
        .get("/papi/v1/properties")
        .query("contractId", "ctr_123456")
        .query("groupId", "grp_123456")
        .header("Accept", "application/json")
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        println!("Properties: {}", serde_json::to_string_pretty(&json)?);
    } else {
        println!("Error: {} - {}", response.status(), response.text().await?);
    }

    // Example: Purge cache
    println!("\nPurging cache...");
    let purge_body = serde_json::json!({
        "objects": [
            "https://www.example.com/path/to/resource",
            "https://www.example.com/another/resource"
        ]
    });

    let response = client
        .post("/ccu/v3/invalidate/url/production")
        .json(&purge_body)
        .send()
        .await?;

    println!("Purge status: {}", response.status());
    if response.status().is_success() {
        let result: serde_json::Value = response.json().await?;
        println!("Purge result: {}", serde_json::to_string_pretty(&result)?);
    }

    Ok(())
}