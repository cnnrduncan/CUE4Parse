# CUE4Parse Rust Integration - Commit Summary

## Overview

This commit introduces comprehensive Rust FFI bindings for CUE4Parse, enabling safe and efficient parsing of Unreal Engine assets from Rust applications.

## Key Components Added

### 1. Rust Crate (`cue4parse-rs/`)
- **Safe API**: Memory-safe wrappers around CUE4Parse functionality
- **Process-based approach**: Uses CLI tool for main parsing to avoid complex FFI
- **Optional native bindings**: FFI for compression library feature detection
- **Comprehensive error handling**: Proper Rust Result types with detailed errors
- **Full documentation**: API docs, examples, and usage guides

### 2. CLI Tool (`CUE4Parse.CLI/`)
- **Command-line interface**: Structured CLI for all CUE4Parse operations
- **JSON output**: Machine-readable output for Rust integration
- **Comprehensive options**: Support for encryption keys, mappings, multiple formats
- **Cross-platform**: Works on all platforms where .NET 8 runs

### 3. Unreal Asset Compatibility Layer (NEW)
- **Migration support**: Optional `unrealmodding-compat` feature for seamless migration
- **API compatibility**: Drop-in replacement types for `unreal_asset` module
- **Property system**: Complete property type support (Bool, Int, String, Struct, Array, Map, Enum, Text)
- **Conversion utilities**: Tools for transforming CUE4Parse JSON to `unreal_asset` format
- **Helper macros**: `load_asset!` and `get_property!` for easier code migration
- **Migration guide**: Complete documentation for users switching from `unreal_modding`

### 4. Build Infrastructure
- **PowerShell build script**: `build.ps1` for easy setup and building
- **CMake integration**: Native library building with optional features
- **Cargo integration**: Proper Rust package with features and dependencies

## Features

✅ **Memory Safety**: No manual memory management, uses process communication
✅ **Cross-platform**: Windows, macOS, Linux support via .NET 8
✅ **Feature Complete**: Access to all CUE4Parse functionality
✅ **Type Safety**: Proper Rust types and error handling
✅ **Documentation**: Comprehensive docs with examples
✅ **Testing**: Unit tests and integration examples
✅ **Flexible**: Support for different UE versions, encryption, mappings
✅ **Migration Ready**: Optional compatibility layer for `unreal_modding` users

## API Highlights

```rust
// Create provider and configure
let mut provider = Provider::new("/path/to/game", GameVersion::UE5_3);
provider.add_key("", "0xYOUR_AES_KEY");
provider.set_mappings("/path/to/mappings.usmap");

// Discover assets
let packages = provider.list_packages()?;

// Load package info
let package = provider.load_package("GameAssets/Hero.uasset")?;

// Export to JSON
let json = provider.export_object_json("GameAssets/Texture.Texture")?;

// Check native features
if is_feature_available("Oodle") {
    println!("Oodle compression supported");
}
```

## Files Added/Modified

### New Files
- `cue4parse-rs/src/lib.rs` - Main Rust library
- `cue4parse-rs/Cargo.toml` - Package configuration
- `cue4parse-rs/build.rs` - Build script for optional FFI
- `cue4parse-rs/wrapper.h` - FFI wrapper header
- `cue4parse-rs/examples/basic_usage.rs` - Usage example
- `cue4parse-rs/README.md` - Comprehensive documentation
- `cue4parse-rs/CHANGELOG.md` - Version history
- `cue4parse-rs/CONTRIBUTING.md` - Contribution guidelines
- `cue4parse-rs/LICENSE` - MIT license
- `CUE4Parse.CLI/Program.cs` - CLI tool implementation
- `CUE4Parse.CLI/CUE4Parse.CLI.csproj` - CLI project file
- `build.ps1` - PowerShell build script

### Modified Files
- `CUE4Parse.sln` - Added CLI project to solution
- `CUE4Parse-Natives/cue4parse_c_api.h` - Simplified for FFI approach

## Testing

```bash
# Build everything
./build.ps1 -All

# Test Rust components
cd cue4parse-rs
cargo test --all-features
cargo run --example basic_usage

# Test CLI tool
../CUE4Parse.CLI/bin/Release/net8.0/CUE4Parse.CLI.exe --help
```

## Usage Scenarios

1. **Game modding tools** - Extract and analyze game assets
2. **Asset conversion** - Convert UE assets to other formats
3. **Data mining** - Analyze game content and structure
4. **Reverse engineering** - Understand game asset formats
5. **Automation** - Batch processing of game assets

## Integration Benefits

- **Easy adoption**: Simple Rust API, no complex setup
- **Maintainable**: Uses existing C# codebase, no large rewrites needed
- **Safe**: Memory-safe by design, proper error handling
- **Extensible**: Easy to add new features through CLI interface
- **Cross-platform**: Works everywhere .NET 8 runs

## Future Enhancements

- Direct .NET interop for even better performance
- More export formats (PNG, FBX, WAV, etc.) via CUE4Parse-Conversion
- Streaming APIs for large asset processing
- Plugin system for custom asset types
- GUI tools built on the Rust bindings

This implementation provides a solid foundation for Rust applications to work with Unreal Engine assets while maintaining safety, performance, and cross-platform compatibility.
