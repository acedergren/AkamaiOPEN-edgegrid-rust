//! Configuration types and .edgerc file parsing

use crate::error::{EdgeGridError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Maximum body size for request signing (128KB)
pub const MAX_BODY: usize = 131072;

/// EdgeGrid configuration containing authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeGridConfig {
    /// Client token for authentication
    pub client_token: String,
    /// Client secret for signing requests
    pub client_secret: String,
    /// Access token for API access
    pub access_token: String,
    /// API host (e.g., "akab-xxxxx.luna.akamaiapis.net")
    pub host: String,
    /// Maximum body size for signing (defaults to MAX_BODY)
    #[serde(default = "default_max_body")]
    pub max_body: usize,
    /// Enable debug mode
    #[serde(default)]
    pub debug: bool,
    /// Account switch key (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_switch_key: Option<String>,
}

fn default_max_body() -> usize {
    MAX_BODY
}

impl EdgeGridConfig {
    /// Create a new EdgeGrid configuration
    pub fn new(
        client_token: String,
        client_secret: String,
        access_token: String,
        host: String,
    ) -> Self {
        let host = if host.starts_with("https://") {
            host
        } else {
            format!("https://{}", host)
        };

        Self {
            client_token,
            client_secret,
            access_token,
            host,
            max_body: MAX_BODY,
            debug: false,
            account_switch_key: None,
        }
    }

    /// Load configuration from .edgerc file
    pub fn from_edgerc(path: impl AsRef<Path>, section: &str) -> Result<Self> {
        let path = resolve_home_path(path)?;
        
        // First try environment variables
        if let Ok(config) = Self::from_env(section) {
            println!("Using configuration from environment variables");
            return Ok(config);
        }
        
        // Then try .edgerc file
        let content = fs::read_to_string(&path)
            .map_err(|e| EdgeGridError::Config(format!("Cannot read .edgerc file: {}", e)))?;
        
        let edgerc = parse_edgerc(&content)?;
        
        edgerc
            .get(section)
            .ok_or_else(|| EdgeGridError::InvalidSection(section.to_string()))
            .and_then(|config| Self::validate_config(config.clone()))
    }

    /// Load configuration from environment variables
    pub fn from_env(section: &str) -> Result<Self> {
        let prefix = if section == "default" {
            "AKAMAI_".to_string()
        } else {
            format!("AKAMAI_{}_", section.to_uppercase())
        };

        let host = env::var(format!("{}HOST", prefix))
            .map_err(|_| EdgeGridError::EnvError(format!("{}HOST not set", prefix)))?;
        let client_token = env::var(format!("{}CLIENT_TOKEN", prefix))
            .map_err(|_| EdgeGridError::EnvError(format!("{}CLIENT_TOKEN not set", prefix)))?;
        let client_secret = env::var(format!("{}CLIENT_SECRET", prefix))
            .map_err(|_| EdgeGridError::EnvError(format!("{}CLIENT_SECRET not set", prefix)))?;
        let access_token = env::var(format!("{}ACCESS_TOKEN", prefix))
            .map_err(|_| EdgeGridError::EnvError(format!("{}ACCESS_TOKEN not set", prefix)))?;

        Ok(Self::new(client_token, client_secret, access_token, host))
    }

    /// Validate that all required fields are present
    fn validate_config(mut config: EdgeGridConfig) -> Result<Self> {
        if config.client_token.trim().is_empty() {
            return Err(EdgeGridError::MissingCredential("client_token".to_string()));
        }
        if config.client_secret.trim().is_empty() {
            return Err(EdgeGridError::MissingCredential("client_secret".to_string()));
        }
        if config.access_token.trim().is_empty() {
            return Err(EdgeGridError::MissingCredential("access_token".to_string()));
        }
        if config.host.trim().is_empty() {
            return Err(EdgeGridError::MissingCredential("host".to_string()));
        }

        // Ensure host has https://
        if !config.host.starts_with("https://") {
            config.host = format!("https://{}", config.host);
        }

        // Remove trailing slash from host
        if config.host.ends_with('/') {
            config.host.pop();
        }

        Ok(config)
    }
}

/// Parse .edgerc file format
fn parse_edgerc(content: &str) -> Result<HashMap<String, EdgeGridConfig>> {
    let mut sections = HashMap::new();
    let mut current_section = None;
    let mut current_config = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        
        // Skip comments and empty lines
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        // Check for section header
        if line.starts_with('[') && line.ends_with(']') {
            // Save previous section if exists
            if let Some(section) = current_section.take() {
                if let Ok(config) = parse_section_config(&current_config) {
                    sections.insert(section, config);
                }
            }
            
            current_section = Some(line[1..line.len()-1].to_string());
            current_config.clear();
            continue;
        }

        // Parse key-value pairs
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim();
            let value = line[eq_pos + 1..].trim();
            
            // Remove quotes and inline comments
            let value = parse_value(value);
            
            // Handle max-body -> max_body conversion
            let key = if key == "max-body" { "max_body" } else { key };
            
            current_config.insert(key.to_string(), value);
        }
    }

    // Save last section
    if let Some(section) = current_section {
        if let Ok(config) = parse_section_config(&current_config) {
            sections.insert(section, config);
        }
    }

    if sections.is_empty() {
        Err(EdgeGridError::Config("No valid sections found in .edgerc".to_string()))
    } else {
        Ok(sections)
    }
}

/// Parse a section's key-value pairs into EdgeGridConfig
fn parse_section_config(values: &HashMap<String, String>) -> Result<EdgeGridConfig> {
    let config = EdgeGridConfig {
        client_token: values.get("client_token").cloned().unwrap_or_default(),
        client_secret: values.get("client_secret").cloned().unwrap_or_default(),
        access_token: values.get("access_token").cloned().unwrap_or_default(),
        host: values.get("host").cloned().unwrap_or_default(),
        max_body: values
            .get("max_body")
            .and_then(|v| v.parse().ok())
            .unwrap_or(MAX_BODY),
        debug: false,
        account_switch_key: values.get("account_switch_key").cloned(),
    };

    EdgeGridConfig::validate_config(config)
}

/// Parse value from .edgerc, handling quotes and comments
fn parse_value(value: &str) -> String {
    let value = value.trim();
    
    // Remove surrounding quotes
    let value = if (value.starts_with('"') && value.ends_with('"')) ||
                   (value.starts_with('\'') && value.ends_with('\'')) {
        &value[1..value.len()-1]
    } else {
        value
    };
    
    // Remove inline comments
    if let Some(comment_pos) = value.find(';') {
        value[..comment_pos].trim().to_string()
    } else {
        value.to_string()
    }
}

/// Resolve ~ in file paths
fn resolve_home_path(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    
    if path.starts_with("~") {
        let home = dirs::home_dir()
            .ok_or_else(|| EdgeGridError::Config("Cannot determine home directory".to_string()))?;
        Ok(home.join(path.strip_prefix("~").unwrap()))
    } else {
        Ok(path.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_edgerc() {
        let content = r#"
[default]
client_secret = abcdefghijklmnopqrstuvwxyz1234567890ABCDEFG=
host = akab-XXXXXXXXXXXXXXXX-XXXXXXXXXXXXXXXX.luna.akamaiapis.net
access_token = akab-XXXXXXXXXXXXXXXX-XXXXXXXXXXXXXXXX
client_token = akab-XXXXXXXXXXXXXXXX-XXXXXXXXXXXXXXXX

[section1]
client_secret = secret1
host = host1.akamaiapis.net
access_token = token1
client_token = client1
max_body = 2048
"#;

        let sections = parse_edgerc(content).unwrap();
        assert_eq!(sections.len(), 2);
        
        let default = sections.get("default").unwrap();
        assert_eq!(default.client_token, "akab-XXXXXXXXXXXXXXXX-XXXXXXXXXXXXXXXX");
        assert_eq!(default.max_body, MAX_BODY);
        
        let section1 = sections.get("section1").unwrap();
        assert_eq!(section1.client_token, "client1");
        assert_eq!(section1.max_body, 2048);
    }

    #[test]
    fn test_parse_value() {
        assert_eq!(parse_value("simple"), "simple");
        assert_eq!(parse_value("\"quoted\""), "quoted");
        assert_eq!(parse_value("'single quoted'"), "single quoted");
        assert_eq!(parse_value("value ; comment"), "value");
        assert_eq!(parse_value("  spaced  "), "spaced");
    }
}