# CUE4Parse Rust FFI Bindings

[![Crates.io](https://img.shields.io/crates/v/cue4parse-rs.svg)](https://crates.io/crates/cue4parse-rs)
[![Documentation](https://docs.rs/cue4parse-rs/badge.svg)](https://docs.rs/cue4parse-rs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/FabianFG/CUE4Parse)

This crate provides Rust bindings for the [CUE4Parse](https://github.com/FabianFG/CUE4Parse) library, enabling parsing and extraction of Unreal Engine assets from Rust applications using an FFI wrapper approach.

## Features

- 🦀 **Safe Rust API** - Memory-safe wrappers with proper error handling
- 🌍 **Cross-platform** - Works on Windows, macOS, and Linux (wherever .NET 8 runs)
- ⚡ **Native performance** - Optional FFI bindings for ACL and Oodle compression libraries
- 📄 **JSON-based communication** - Structured data exchange with the C# library
- 🎮 **Comprehensive asset support** - Parse textures, meshes, animations, blueprints, and more
- 🔐 **Encryption support** - Handle encrypted game assets with AES keys
- 🗺️ **Mappings support** - Use .usmap files for better parsing accuracy

## Architecture

This crate uses a hybrid approach:
1. **Main parsing**: Process-based communication with a .NET CLI tool
2. **Native features**: Optional FFI for compression library detection
3. **Safety**: Memory-safe by avoiding complex FFI for main functionality

## Prerequisites

Before using this crate, you need to build the CUE4Parse projects:

### Building CUE4Parse.CLI

1. Navigate to the CUE4Parse root directory
2. Build the solution: `dotnet build --configuration Release`
3. The CLI tool will be available in `CUE4Parse.CLI/bin/Release/net8.0/CUE4Parse.CLI.exe`

### Building Native Libraries (Optional)

If you want to use the native feature checking:

1. Navigate to the `CUE4Parse-Natives` directory
2. Create a build directory: `mkdir build && cd build`
3. Configure with CMake: `cmake ..`
4. Build the library: `cmake --build . --config Release`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cue4parse-rs = { path = "path/to/cue4parse-rs" }
```

### Basic Example

```rust
use cue4parse_rs::{Provider, GameVersion, is_feature_available};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check available native features (requires native-lib feature)
    println!("ACL support: {}", is_feature_available("ACL"));
    println!("Oodle support: {}", is_feature_available("Oodle"));
    
    // Create a provider for your game files
    let mut provider = Provider::new("path/to/game/files", GameVersion::UE5_3);
    
    // Set the path to CUE4Parse.CLI.exe if it's not in the default location
    provider.set_executable_path("C:/path/to/CUE4Parse.CLI.exe");
    
    // Add encryption keys if needed
    provider.add_key("your-key-guid", "your-aes-key");
    
    // Set mappings for better parsing
    provider.set_mappings("path/to/mappings.usmap");
    
    // List all packages
    let packages = provider.list_packages()?;
    println!("Found {} packages", packages.len());
    
    // Load package information
    let package_info = provider.load_package("GameAssets/SomePackage")?;
    println!("Package '{}' has {} exports", package_info.name, package_info.exports.len());
    
    // Export an object to JSON
    let json = provider.export_object_json("GameAssets/SomeObject")?;
    println!("Object JSON: {}", serde_json::to_string_pretty(&json)?);
    
    Ok(())
}
```

## API Reference

### Provider

The main entry point for parsing game assets via CLI interface.

- `Provider::new(directory_path, version)` - Create a new provider
- `provider.set_executable_path(path)` - Set path to CUE4Parse.CLI.exe
- `provider.add_key(guid, key)` - Add an AES encryption key
- `provider.set_mappings(path)` - Set type mappings for better parsing
- `provider.list_packages()` - List all available packages
- `provider.load_package(path)` - Load package information
- `provider.export_object_json(path)` - Export object as JSON
- `provider.export_object(path, output, format)` - Export object to file

### Package Information

The `PackageInfo` struct contains:
- `name` - Package name
- `exports` - List of `ExportInfo` with name, class_name, and outer_index

## Error Handling

All operations return `Result<T, CUE4ParseError>` where `CUE4ParseError` provides detailed information about any failures.

## Memory Management

All resources are automatically cleaned up when dropped. The library communicates with the CLI tool via JSON, ensuring memory safety.

## Building

To build the crate:

```bash
cd cue4parse-rs
cargo build --release
```

To run the example:

```bash
cargo run --example basic_usage
```

## Features

This crate supports the following optional features:

- `native-lib` - Enable native library bindings for feature checking (default: enabled)
- `dotnet-interop` - Enable direct .NET interop (experimental, default: disabled)

To build with specific features:

```bash
cargo build --features "native-lib"
```

## CLI Tool

The `CUE4Parse.CLI` project provides a command-line interface that the Rust bindings use internally. You can also use it directly:

```bash
# List packages
CUE4Parse.CLI.exe --directory "game/files" --version "GAME_UE5_3" --list-packages

# Get package info
CUE4Parse.CLI.exe --directory "game/files" --version "GAME_UE5_3" --package "SomePackage" --package-info

# Export object to JSON
CUE4Parse.CLI.exe --directory "game/files" --version "GAME_UE5_3" --object "SomeObject" --export --output-format json

# With AES keys and mappings
CUE4Parse.CLI.exe --directory "game/files" --version "GAME_UE5_3" --aes-key "guid:key" --mappings "mappings.usmap" --object "SomeObject" --export
```

## License

This project follows the same license as the original CUE4Parse project.
