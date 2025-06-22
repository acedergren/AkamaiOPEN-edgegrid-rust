//! Integration tests for EdgeGrid authentication

use akamai_edgegrid::{EdgeGridClient, EdgeGridConfig};

#[tokio::test]
async fn test_get_request() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    
    let _m = server.mock("GET", "/test")
        .match_header("authorization", mockito::Matcher::Regex(r"^EG1-HMAC-SHA256.*".to_string()))
        .with_status(200)
        .with_body(r#"{"status": "ok"}"#)
        .create_async()
        .await;

    let config = EdgeGridConfig::new(
        "test-client-token".to_string(),
        "test-client-secret".to_string(),
        "test-access-token".to_string(),
        url,
    );

    let client = EdgeGridClient::new(config).unwrap();
    let response = client.get("/test").send().await.unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_post_request_with_json() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    
    let _m = server.mock("POST", "/test")
        .match_header("authorization", mockito::Matcher::Regex(r"^EG1-HMAC-SHA256.*".to_string()))
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::Json(serde_json::json!({
            "key": "value"
        })))
        .with_status(201)
        .with_body(r#"{"id": "123"}"#)
        .create_async()
        .await;

    let config = EdgeGridConfig::new(
        "test-client-token".to_string(),
        "test-client-secret".to_string(),
        "test-access-token".to_string(),
        url,
    );

    let client = EdgeGridClient::new(config).unwrap();
    let body = serde_json::json!({
        "key": "value"
    });
    
    let response = client
        .post("/test")
        .json(&body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 201);
}

#[tokio::test]
async fn test_query_parameters() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    
    let _m = server.mock("GET", "/test")
        .match_query(mockito::Matcher::UrlEncoded("limit".to_string(), "10".to_string()))
        .match_query(mockito::Matcher::UrlEncoded("offset".to_string(), "20".to_string()))
        .with_status(200)
        .create_async()
        .await;

    let config = EdgeGridConfig::new(
        "test-client-token".to_string(),
        "test-client-secret".to_string(),
        "test-access-token".to_string(),
        url,
    );

    let client = EdgeGridClient::new(config).unwrap();
    let response = client
        .get("/test")
        .query("limit", "10")
        .query("offset", "20")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[test]
fn test_config_validation() {
    // Test missing credentials
    let config = EdgeGridConfig::new(
        "".to_string(),
        "secret".to_string(),
        "token".to_string(),
        "host".to_string(),
    );
    
    let result = EdgeGridClient::new(config);
    assert!(result.is_err());
}

#[test]
fn test_edgerc_parsing() {
    use std::io::Write;
    use tempfile::NamedTempFile;

    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "[default]").unwrap();
    writeln!(file, "client_secret = test-secret").unwrap();
    writeln!(file, "host = test.luna.akamaiapis.net").unwrap();
    writeln!(file, "access_token = test-access").unwrap();
    writeln!(file, "client_token = test-client").unwrap();

    let config = EdgeGridConfig::from_edgerc(file.path(), "default").unwrap();
    assert_eq!(config.client_token, "test-client");
    assert_eq!(config.client_secret, "test-secret");
    assert_eq!(config.access_token, "test-access");
    assert_eq!(config.host, "https://test.luna.akamaiapis.net");
}