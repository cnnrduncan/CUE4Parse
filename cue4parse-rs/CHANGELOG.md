# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial Rust FFI bindings for CUE4Parse
- Process-based communication with CUE4Parse.CLI tool
- Safe Rust API with comprehensive error handling
- Support for all Unreal Engine versions (UE4.0 to UE5.5)
- AES encryption key management
- Type mappings support (.usmap files)
- JSON-based asset serialization
- Optional native FFI for compression library feature detection
- Cross-platform support (Windows, macOS, Linux)
- Comprehensive documentation with examples
- Build script for easy setup (`build.ps1`)

### Features
- `Provider` struct for managing game asset access
- `list_packages()` method to discover available assets
- `load_package()` method to get package information
- `export_object_json()` method to serialize objects to JSON
- `export_object()` method for file-based exports (extensible)
- `is_feature_available()` function for native feature detection
- Support for multiple AES keys per provider
- Configurable CLI tool path

### Documentation
- Complete API documentation with examples
- Comprehensive README with setup instructions
- Inline code documentation for all public APIs
- Usage examples for different game types
- Troubleshooting guide
- Architecture explanation

### Build System
- Cargo.toml with proper dependencies and features
- build.rs script for optional native library integration
- PowerShell build script for complete setup
- CMake integration for native components
- Feature flags for optional components

## [0.1.0] - 2025-07-12

### Added
- Initial release of CUE4Parse Rust FFI bindings
- Basic functionality for parsing Unreal Engine assets
- CLI-based approach for safe memory management
- Support for encrypted assets and mappings files
