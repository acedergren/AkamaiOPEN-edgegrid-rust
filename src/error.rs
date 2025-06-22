//! Error types for the EdgeGrid authentication library

use thiserror::Error;

/// Main error type for EdgeGrid operations
#[derive(Error, Debug)]
pub enum EdgeGridError {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// Missing required credential
    #[error("Missing credential: {0}")]
    MissingCredential(String),
    
    /// File I/O errors
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),
    
    /// URL parsing errors
    #[error("URL error: {0}")]
    UrlError(#[from] url::ParseError),
    
    /// HTTP request errors
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
    
    /// TOML parsing errors
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),
    
    /// Authentication errors
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    /// Invalid section in .edgerc file
    #[error("Invalid section '{0}' in .edgerc file")]
    InvalidSection(String),
    
    /// Environment variable errors
    #[error("Environment variable error: {0}")]
    EnvError(String),
}

/// Result type alias for EdgeGrid operations
pub type Result<T> = std::result::Result<T, EdgeGridError>;