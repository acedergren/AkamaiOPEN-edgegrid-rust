# Contributing to Akamai EdgeGrid Rust

Thank you for your interest in contributing to this community-maintained Akamai EdgeGrid authentication library for Rust!

## Code of Conduct

Please note that this project follows a standard code of conduct. By participating, you are expected to uphold this code and treat all contributors with respect.

## How to Contribute

### Reporting Issues

- Check if the issue has already been reported
- Include a clear description of the problem
- Provide minimal reproducible example code
- Include your Rust version and OS

### Submitting Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Check formatting (`cargo fmt`)
7. Run clippy (`cargo clippy -- -D warnings`)
8. Commit your changes with clear messages
9. Push to your fork
10. Open a Pull Request

### Development Setup

```bash
# Clone your fork
git clone https://github.com/your-username/AkamaiOPEN-edgegrid-rust.git
cd AkamaiOPEN-edgegrid-rust

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic_usage
```

### Testing

- Write tests for any new functionality
- Ensure existing tests continue to pass
- Use meaningful test names that describe what is being tested
- Include both positive and negative test cases

### Documentation

- Update documentation for any changed functionality
- Add rustdoc comments for new public APIs
- Include examples in documentation where appropriate
- Update README.md if adding new features

### Code Style

- Follow standard Rust naming conventions
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common mistakes
- Keep functions focused and small
- Add comments for complex logic

### Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense (e.g., "Add", "Fix", "Update")
- Keep the first line under 50 characters
- Add detailed description after a blank line if needed

Example:
```
Add support for custom timeout configuration

- Allow users to specify request timeout in EdgeGridConfig
- Default to 30 seconds if not specified
- Add tests for timeout behavior
```

## Release Process

Releases are automated through GitHub Actions when a new tag is pushed:

```bash
git tag v0.1.1
git push origin v0.1.1
```

This will:
1. Create a GitHub release
2. Build binaries for multiple platforms
3. Publish to crates.io (requires CARGO_REGISTRY_TOKEN secret)

## Questions?

If you have questions about contributing, feel free to open an issue for discussion.