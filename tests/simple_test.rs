//! Simple integration tests for EdgeGrid authentication

use akamai_edgegrid::{EdgeGridClient, EdgeGridConfig};

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
fn test_config_validation_with_empty_secret() {
    let config = EdgeGridConfig::new(
        "client".to_string(),
        "".to_string(),
        "token".to_string(),
        "host".to_string(),
    );
    
    let result = EdgeGridClient::new(config);
    assert!(result.is_err());
}

#[test]
fn test_valid_config() {
    let config = EdgeGridConfig::new(
        "test-client-token".to_string(),
        "test-client-secret".to_string(),
        "test-access-token".to_string(),
        "https://test.luna.akamaiapis.net".to_string(),
    );
    
    let result = EdgeGridClient::new(config);
    assert!(result.is_ok());
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

#[test]
fn test_edgerc_parsing_with_comments() {
    use std::io::Write;
    use tempfile::NamedTempFile;

    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "; This is a comment").unwrap();
    writeln!(file, "[default]").unwrap();
    writeln!(file, "client_secret = test-secret ; inline comment").unwrap();
    writeln!(file, "host = \"test.luna.akamaiapis.net\"").unwrap();
    writeln!(file, "access_token = 'test-access'").unwrap();
    writeln!(file, "client_token = test-client").unwrap();
    writeln!(file, "; Another comment").unwrap();

    let config = EdgeGridConfig::from_edgerc(file.path(), "default").unwrap();
    assert_eq!(config.client_token, "test-client");
    assert_eq!(config.client_secret, "test-secret");
    assert_eq!(config.access_token, "test-access");
    assert_eq!(config.host, "https://test.luna.akamaiapis.net");
}