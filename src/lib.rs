//! # Akamai EdgeGrid Authentication for Rust
//!
//! This library implements the Akamai EdgeGrid authentication scheme for Rust,
//! providing a simple way to make authenticated HTTP requests to Akamai APIs.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use akamai_edgegrid::{EdgeGridClient, EdgeGridConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client from .edgerc file
//!     let client = EdgeGridClient::from_edgerc("~/.edgerc", "default")?;
//!
//!     // Make a GET request
//!     let response = client
//!         .get("/billing-usage/v1/reportSources")
//!         .send()
//!         .await?;
//!
//!     println!("Status: {}", response.status());
//!     Ok(())
//! }
//! ```
//!
//! ## Creating a Client
//!
//! You can create a client in several ways:
//!
//! ### From .edgerc file
//! ```rust,no_run
//! # use akamai_edgegrid::EdgeGridClient;
//! let client = EdgeGridClient::from_edgerc("~/.edgerc", "default")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### From credentials
//! ```rust,no_run
//! # use akamai_edgegrid::{EdgeGridClient, EdgeGridConfig};
//! let config = EdgeGridConfig::new(
//!     "client-token".to_string(),
//!     "client-secret".to_string(),
//!     "access-token".to_string(),
//!     "host.luna.akamaiapis.net".to_string(),
//! );
//! let client = EdgeGridClient::new(config)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### From environment variables
//! ```rust,no_run
//! # use akamai_edgegrid::{EdgeGridClient, EdgeGridConfig};
//! // Reads from AKAMAI_HOST, AKAMAI_CLIENT_TOKEN, etc.
//! let config = EdgeGridConfig::from_env("default")?;
//! let client = EdgeGridClient::new(config)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod auth;
pub mod client;
pub mod config;
pub mod error;

// Re-export main types
pub use client::EdgeGridClient;
pub use config::{EdgeGridConfig, MAX_BODY};
pub use error::{EdgeGridError, Result};