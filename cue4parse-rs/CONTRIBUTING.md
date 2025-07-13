# Contributing to CUE4Parse Rust Bindings

Thank you for your interest in contributing to the CUE4Parse Rust bindings! This document provides guidelines and information for contributors.

## Getting Started

1. **Fork the repository** and clone your fork
2. **Set up the development environment**:
   ```bash
   # Build all components
   ./build.ps1 -All
   
   # Verify Rust components work
   cd cue4parse-rs
   cargo test
   cargo run --example basic_usage
   ```

## Development Workflow

### Making Changes

1. **Create a feature branch** from `main`
2. **Make your changes** with appropriate tests
3. **Update documentation** if needed
4. **Run tests** to ensure everything works
5. **Submit a pull request**

### Code Style

- **Rust code**: Follow standard Rust formatting (`cargo fmt`)
- **Documentation**: Use `///` for public APIs with examples
- **Comments**: Explain complex logic, not obvious code
- **Error handling**: Use proper Result types and descriptive errors

### Testing

```bash
# Run all tests
cargo test

# Run tests with native features
cargo test --features native-lib

# Run tests without native features  
cargo test --no-default-features

# Test with different game scenarios
cargo test --release
```

## Types of Contributions

### Bug Reports

When reporting bugs, please include:
- Rust version and platform
- Steps to reproduce
- Expected vs actual behavior
- Relevant error messages
- Game/asset information if applicable

### Feature Requests

For new features, please:
- Describe the use case clearly
- Explain the expected behavior
- Consider backward compatibility
- Provide examples if possible

### Code Contributions

We welcome contributions for:
- **Bug fixes** - Always appreciated!
- **Performance improvements** - Especially for large asset processing
- **New features** - Discuss in an issue first for major changes
- **Documentation** - Help make the project more accessible
- **Examples** - Real-world usage examples are valuable

## Architecture Guidelines

### Design Principles

1. **Safety first** - Prefer safe Rust patterns over performance
2. **Clear APIs** - Public interfaces should be intuitive and well-documented
3. **Error transparency** - Provide meaningful error messages
4. **Cross-platform** - Support Windows, macOS, and Linux
5. **Backward compatibility** - Avoid breaking changes when possible

### Code Organization

- `src/lib.rs` - Main public API and documentation
- `src/` - Core functionality modules
- `examples/` - Usage examples and demonstrations
- `tests/` - Integration and unit tests
- `build.rs` - Build-time configuration

### FFI Guidelines

- **Minimize unsafe code** - Use process-based communication when possible
- **Document safety** - Explain why unsafe code is safe
- **Handle errors gracefully** - Convert C errors to Rust Result types
- **Memory management** - Use RAII and proper Drop implementations

## Documentation

### API Documentation

- Use `///` for all public items
- Include examples for non-trivial functions
- Document error conditions
- Explain safety requirements for unsafe code

### Examples

- Keep examples focused and practical
- Include error handling
- Show real-world usage patterns
- Test examples in CI when possible

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- **Major**: Breaking API changes
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes, backward compatible

### Release Checklist

1. Update version numbers in `Cargo.toml`
2. Update `CHANGELOG.md` with changes
3. Run full test suite
4. Update documentation if needed
5. Create release PR
6. Tag release after merge

## Getting Help

- **Questions**: Open a discussion or issue
- **Chat**: Join the CUE4Parse community channels
- **Documentation**: Check the README and API docs first

## Code of Conduct

Please be respectful and constructive in all interactions. We want this to be a welcoming environment for all contributors.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

Thank you for contributing to CUE4Parse Rust bindings!
