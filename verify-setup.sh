#!/bin/bash

echo "=== Verifying Rust EdgeGrid Setup for crates.io and GitHub Pages ==="
echo

echo "1. Checking Cargo.toml metadata..."
if grep -q "documentation = " Cargo.toml && grep -q "homepage = " Cargo.toml && grep -q "readme = " Cargo.toml; then
    echo "✓ All required metadata fields present"
else
    echo "✗ Missing some metadata fields"
fi
echo

echo "2. Checking GitHub Actions workflows..."
if [ -f ".github/workflows/rust.yml" ] && [ -f ".github/workflows/release.yml" ]; then
    echo "✓ CI/CD workflows present"
else
    echo "✗ Missing workflow files"
fi
echo

echo "3. Checking documentation files..."
if [ -f "README.md" ] && [ -f "LICENSE" ] && [ -f "CHANGELOG.md" ] && [ -f "CONTRIBUTING.md" ]; then
    echo "✓ All documentation files present"
else
    echo "✗ Missing some documentation files"
fi
echo

echo "4. Verifying cargo package..."
echo "Running: cargo publish --dry-run"
cargo publish --dry-run
echo

echo "5. Building documentation..."
echo "Running: cargo doc --no-deps"
cargo doc --no-deps
echo

echo "=== Setup Complete ==="
echo
echo "Next steps:"
echo "1. Push to GitHub and ensure the repository is public"
echo "2. Enable GitHub Pages in repository settings (use GitHub Actions as source)"
echo "3. Set CARGO_REGISTRY_TOKEN secret in GitHub repository settings"
echo "4. Create and push a tag to trigger the release workflow:"
echo "   git tag v0.1.0"
echo "   git push origin v0.1.0"
echo
echo "Documentation will be available at:"
echo "https://acedergren.github.io/AkamaiOPEN-edgegrid-rust/"