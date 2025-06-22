//! EdgeGrid HTTP client implementation

use crate::auth::EdgeGridAuth;
use crate::config::EdgeGridConfig;
use crate::error::{EdgeGridError, Result};
use reqwest::{Client, Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use url::Url;

/// EdgeGrid client for making authenticated requests to Akamai APIs
#[derive(Debug, Clone)]
pub struct EdgeGridClient {
    client: Client,
    auth: EdgeGridAuth,
    base_url: Url,
}

impl EdgeGridClient {
    /// Create a new EdgeGrid client with the given configuration
    pub fn new(config: EdgeGridConfig) -> Result<Self> {
        // Validate configuration first
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
        
        let base_url = Url::parse(&config.host)?;
        let auth = EdgeGridAuth::new(config);
        let client = Client::new();

        Ok(Self {
            client,
            auth,
            base_url,
        })
    }

    /// Create a client from .edgerc file
    pub fn from_edgerc(path: impl AsRef<std::path::Path>, section: &str) -> Result<Self> {
        let config = EdgeGridConfig::from_edgerc(path, section)?;
        Self::new(config)
    }

    /// Build a request with the given method and path
    pub fn request(&self, method: Method, path: &str) -> EdgeGridRequestBuilder {
        let url = self.base_url.join(path).unwrap_or_else(|_| {
            // If join fails, try to parse as absolute URL
            Url::parse(&format!("{}{}", self.base_url, path))
                .unwrap_or_else(|_| self.base_url.clone())
        });

        EdgeGridRequestBuilder {
            client: self.client.clone(),
            auth: self.auth.clone(),
            builder: self.client.request(method, url),
            query_params: HashMap::new(),
        }
    }

    /// Convenience method for GET requests
    pub fn get(&self, path: &str) -> EdgeGridRequestBuilder {
        self.request(Method::GET, path)
    }

    /// Convenience method for POST requests
    pub fn post(&self, path: &str) -> EdgeGridRequestBuilder {
        self.request(Method::POST, path)
    }

    /// Convenience method for PUT requests
    pub fn put(&self, path: &str) -> EdgeGridRequestBuilder {
        self.request(Method::PUT, path)
    }

    /// Convenience method for DELETE requests
    pub fn delete(&self, path: &str) -> EdgeGridRequestBuilder {
        self.request(Method::DELETE, path)
    }

    /// Convenience method for PATCH requests
    pub fn patch(&self, path: &str) -> EdgeGridRequestBuilder {
        self.request(Method::PATCH, path)
    }
}

/// Builder for EdgeGrid requests
pub struct EdgeGridRequestBuilder {
    client: Client,
    auth: EdgeGridAuth,
    builder: RequestBuilder,
    query_params: HashMap<String, String>,
}

impl EdgeGridRequestBuilder {
    /// Add a query parameter
    pub fn query<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.query_params.insert(key.into(), value.into());
        self
    }

    /// Add multiple query parameters
    pub fn queries<I, K, V>(mut self, params: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in params {
            self.query_params.insert(key.into(), value.into());
        }
        self
    }

    /// Add a header to the request
    pub fn header<K: AsRef<str>, V: AsRef<str>>(mut self, key: K, value: V) -> Self {
        self.builder = self.builder.header(key.as_ref(), value.as_ref());
        self
    }

    /// Add multiple headers
    pub fn headers<I, K, V>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        for (key, value) in headers {
            self.builder = self.builder.header(key.as_ref(), value.as_ref());
        }
        self
    }

    /// Set the request body as JSON
    pub fn json<T: Serialize + ?Sized>(mut self, json: &T) -> Self {
        self.builder = self.builder.json(json);
        self
    }

    /// Set the request body as text
    pub fn body<B: Into<reqwest::Body>>(mut self, body: B) -> Self {
        self.builder = self.builder.body(body);
        self
    }

    /// Send the request and return the response
    pub async fn send(mut self) -> Result<Response> {
        // Add query parameters
        for (key, value) in self.query_params {
            self.builder = self.builder.query(&[(key, value)]);
        }

        // Build the request
        let mut request = self.builder
            .build()
            .map_err(EdgeGridError::HttpError)?;

        // Sign the request
        self.auth.sign_request(&mut request)?;

        // Send the request
        self.client
            .execute(request)
            .await
            .map_err(EdgeGridError::HttpError)
    }

    /// Send the request and deserialize the JSON response
    pub async fn send_json<T: DeserializeOwned>(self) -> Result<T> {
        let response = self.send().await?;
        let status = response.status();
        
        if status.is_success() {
            response
                .json()
                .await
                .map_err(EdgeGridError::HttpError)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response".to_string());
            
            // Create a custom error using anyhow
            Err(EdgeGridError::Config(format!("HTTP {}: {}", status, error_text)))
        }
    }

    /// Send the request and return the response as text
    pub async fn send_text(self) -> Result<String> {
        let response = self.send().await?;
        response
            .text()
            .await
            .map_err(EdgeGridError::HttpError)
    }

    /// Send the request and return the response as bytes
    pub async fn send_bytes(self) -> Result<Vec<u8>> {
        let response = self.send().await?;
        response
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(EdgeGridError::HttpError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = EdgeGridConfig::new(
            "test-client-token".to_string(),
            "test-client-secret".to_string(),
            "test-access-token".to_string(),
            "https://test.luna.akamaiapis.net".to_string(),
        );

        let client = EdgeGridClient::new(config);
        assert!(client.is_ok());
    }
}