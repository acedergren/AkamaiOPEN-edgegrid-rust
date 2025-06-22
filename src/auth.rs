//! EdgeGrid authentication implementation

use crate::config::EdgeGridConfig;
use crate::error::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::{Method, Request};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

/// EdgeGrid authentication handler
#[derive(Debug, Clone)]
pub struct EdgeGridAuth {
    config: EdgeGridConfig,
}

impl EdgeGridAuth {
    /// Create a new EdgeGrid authenticator
    pub fn new(config: EdgeGridConfig) -> Self {
        Self { config }
    }

    /// From .edgerc file
    pub fn from_edgerc(path: impl AsRef<std::path::Path>, section: &str) -> Result<Self> {
        let config = EdgeGridConfig::from_edgerc(path, section)?;
        Ok(Self::new(config))
    }

    /// Sign a request with EdgeGrid authentication
    pub fn sign_request(&self, request: &mut Request) -> Result<()> {
        let timestamp = create_timestamp();
        let nonce = Uuid::new_v4().to_string();
        
        // Get request details
        let method = request.method().as_str();
        let url = request.url().clone();
        let path = url.path();
        let query = url.query().unwrap_or("");
        let full_path = if query.is_empty() {
            path.to_string()
        } else {
            format!("{}?{}", path, query)
        };

        // Get headers to sign
        let headers_to_sign = self.get_headers_to_sign(request);
        
        // Calculate content hash if needed
        let content_hash = self.calculate_content_hash(request)?;
        
        // Create auth header
        let auth_header = self.create_auth_header(
            method,
            &url,
            &full_path,
            &headers_to_sign,
            &content_hash,
            &timestamp,
            &nonce,
        )?;

        // Set the authorization header
        if let Ok(header_value) = auth_header.parse() {
            request.headers_mut().insert("Authorization", header_value);
        }

        Ok(())
    }

    /// Get headers that should be included in the signature
    fn get_headers_to_sign(&self, _request: &Request) -> HashMap<String, String> {
        let headers = HashMap::new();
        
        // In the Node.js version, headersToSign can be passed in the request
        // For now, we'll return empty map as default behavior
        // This can be extended to read specific headers if needed
        
        headers
    }

    /// Calculate content hash for POST requests
    fn calculate_content_hash(&self, request: &Request) -> Result<String> {
        if request.method() != Method::POST {
            return Ok(String::new());
        }

        if let Some(body) = request.body() {
            if let Some(bytes) = body.as_bytes() {
                let body_len = bytes.len();
                
                // Truncate to max_body if needed
                let bytes_to_hash = if body_len > self.config.max_body {
                    log::warn!(
                        "Request body size ({}) exceeds max_body ({}), truncating for signing",
                        body_len,
                        self.config.max_body
                    );
                    &bytes[..self.config.max_body]
                } else {
                    bytes
                };

                let hash = Sha256::digest(bytes_to_hash);
                return Ok(BASE64.encode(hash));
            }
        }

        Ok(String::new())
    }

    /// Create the EdgeGrid authorization header
    fn create_auth_header(
        &self,
        method: &str,
        url: &Url,
        path: &str,
        headers_to_sign: &HashMap<String, String>,
        content_hash: &str,
        timestamp: &str,
        nonce: &str,
    ) -> Result<String> {
        // Build the data to sign
        let data_to_sign = self.build_data_to_sign(
            method,
            url.scheme(),
            url.host_str().unwrap_or(""),
            path,
            headers_to_sign,
            content_hash,
            timestamp,
            nonce,
        );

        // Calculate signature
        let signing_key = self.create_signing_key(timestamp)?;
        let signature = self.sign_data(&data_to_sign, &signing_key)?;

        // Build authorization header
        Ok(format!(
            "EG1-HMAC-SHA256 client_token={};access_token={};timestamp={};nonce={};signature={}",
            self.config.client_token,
            self.config.access_token,
            timestamp,
            nonce,
            signature
        ))
    }

    /// Build the string that will be signed
    fn build_data_to_sign(
        &self,
        method: &str,
        scheme: &str,
        host: &str,
        path: &str,
        headers_to_sign: &HashMap<String, String>,
        content_hash: &str,
        timestamp: &str,
        nonce: &str,
    ) -> String {
        let canonicalized_headers = self.canonicalize_headers(headers_to_sign);
        let auth_header = format!(
            "EG1-HMAC-SHA256 client_token={};access_token={};timestamp={};nonce={};",
            self.config.client_token,
            self.config.access_token,
            timestamp,
            nonce
        );

        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            method.to_uppercase(),
            scheme,
            host,
            path,
            canonicalized_headers,
            content_hash,
            auth_header
        )
    }

    /// Canonicalize headers for signing
    fn canonicalize_headers(&self, headers: &HashMap<String, String>) -> String {
        let mut sorted_headers: Vec<_> = headers.iter().collect();
        sorted_headers.sort_by_key(|&(k, _)| k.to_lowercase());

        sorted_headers
            .iter()
            .map(|(k, v)| format!("{}:{}", k.to_lowercase(), v.trim()))
            .collect::<Vec<_>>()
            .join("\t")
    }

    /// Create the signing key
    fn create_signing_key(&self, timestamp: &str) -> Result<String> {
        let mut mac = HmacSha256::new_from_slice(self.config.client_secret.as_bytes())
            .map_err(|e| crate::error::EdgeGridError::AuthError(e.to_string()))?;
        mac.update(timestamp.as_bytes());
        let result = mac.finalize();
        Ok(BASE64.encode(result.into_bytes()))
    }

    /// Sign the data with the signing key
    fn sign_data(&self, data: &str, signing_key: &str) -> Result<String> {
        let key_bytes = BASE64
            .decode(signing_key)
            .map_err(|e| crate::error::EdgeGridError::AuthError(e.to_string()))?;
        
        let mut mac = HmacSha256::new_from_slice(&key_bytes)
            .map_err(|e| crate::error::EdgeGridError::AuthError(e.to_string()))?;
        mac.update(data.as_bytes());
        let result = mac.finalize();
        Ok(BASE64.encode(result.into_bytes()))
    }
}

/// Create timestamp in the required format: yyyyMMddTHH:mm:ss+0000
fn create_timestamp() -> String {
    Utc::now().format("%Y%m%dT%H:%M:%S+0000").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_timestamp() {
        let timestamp = create_timestamp();
        assert_eq!(timestamp.len(), 22); // Fixed: format is "20140321T19:34:21+0000"
        assert!(timestamp.contains('T'));
        assert!(timestamp.ends_with("+0000"));
        
        // Test the actual format matches expectation
        let parts: Vec<&str> = timestamp.split('T').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].len(), 8); // YYYYMMDD
        assert_eq!(parts[1].len(), 13); // HH:MM:SS+0000
    }

    #[test]
    fn test_canonicalize_headers() {
        let config = EdgeGridConfig::new(
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test.com".to_string(),
        );
        let auth = EdgeGridAuth::new(config);
        
        let mut headers = HashMap::new();
        headers.insert("X-Test".to_string(), "value1".to_string());
        headers.insert("X-Another".to_string(), "value2".to_string());
        
        let result = auth.canonicalize_headers(&headers);
        assert_eq!(result, "x-another:value2\tx-test:value1");
    }
}