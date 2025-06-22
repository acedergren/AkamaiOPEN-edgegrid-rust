# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2025-06-22

### Fixed
- Added missing permissions to GitHub Actions workflows
- Fixed release automation for crates.io publishing
- Updated documentation with correct package name

## [0.1.0] - 2025-06-22

### Added
- Initial release of the Akamai EdgeGrid authentication library for Rust
- Full EdgeGrid authentication protocol implementation
- Support for `.edgerc` configuration files
- Environment variable configuration support
- Async/await support with Tokio
- Comprehensive error handling
- Request builder pattern for all HTTP methods
- JSON request/response handling
- Query parameter and header management
- Examples for basic usage and `.edgerc` configuration
- Comprehensive test suite
- GitHub Actions CI/CD pipeline
- Documentation with GitHub Pages deployment

### Security
- Uses `rustls` by default for TLS connections
- Optional `native-tls` feature for platform-specific TLS

[Unreleased]: https://github.com/acedergren/AkamaiOPEN-edgegrid-rust/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/acedergren/AkamaiOPEN-edgegrid-rust/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/acedergren/AkamaiOPEN-edgegrid-rust/releases/tag/v0.1.0