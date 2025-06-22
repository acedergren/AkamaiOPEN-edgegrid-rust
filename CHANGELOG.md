# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

## [0.1.0] - TBD

Initial release - see Unreleased section above for features.

[Unreleased]: https://github.com/acedergren/AkamaiOPEN-edgegrid-rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/acedergren/AkamaiOPEN-edgegrid-rust/releases/tag/v0.1.0