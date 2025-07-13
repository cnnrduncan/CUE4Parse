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

## Stove Compatibility & Advanced Features

This crate provides enhanced compatibility with [Stove](https://github.com/bananaturtlesandwich/stove) and other advanced Unreal Engine modding tools. The `unrealmodding-compat` feature includes:

### Core Stove Features
- **🎮 Static Mesh Processing**: Extract vertex positions, indices, UV coordinates, and material assignments
- **🎨 Texture Data Access**: Support for DXT, ASTC, BC, and other compression formats
- **📦 Material Properties**: Parse material parameters, texture references, and shader properties  
- **🎯 Actor Transforms**: Full support for location, rotation, and scale properties
- **🔧 Component Hierarchies**: Navigate actor-component relationships and mesh assignments

### Advanced Property Types
- **Vector Properties**: `Vector`, `Vector2D`, `Vector4`, `Rotator`, `Quat` for 3D transforms
- **Asset References**: `SoftObjectPath`, `SoftClassPath` for cross-asset dependencies
- **Per-Platform Data**: `PerPlatformBool`, `PerPlatformInt`, `PerPlatformFloat` for multi-target games
- **Specialized Types**: `LinearColor`, `Transform`, `Guid`, `DateTime`, `Delegate` properties
- **Material Types**: `MaterialInterface`, `StaticMesh`, `Texture2D` object references

### Example Usage (Stove-style)

```rust
use cue4parse_rs::{Provider, GameVersion};
use cue4parse_rs::unreal_asset::{Asset, UnrealAssetCompat, AdvancedAssetProcessing, ConversionUtils};

// Load a level for editing (like Stove does)
let provider = Provider::new("/path/to/game", GameVersion::UE5_3);
let level_asset = Asset::from_cue4parse(&provider, "World'/Game/Maps/MainLevel.MainLevel'")?;

// Extract all actors with transforms (core Stove functionality)
let actors = level_asset.extract_actors()?;
for actor in actors {
    println!("Actor: {} at location ({:.1}, {:.1}, {:.1})", 
             actor.name, 
             actor.transform.location.x,
             actor.transform.location.y, 
             actor.transform.location.z);
    
    // Find associated mesh (for 3D rendering)
    if let Some(mesh_path) = ConversionUtils::find_mesh_component(&level_asset, &export) {
        let mesh_asset = Asset::from_cue4parse(&provider, &mesh_path)?;
        let mesh_data = mesh_asset.extract_static_mesh()?;
        
        // Render mesh with vertices and materials...
    }
}

// Load and process a static mesh (for visualization)
let mesh_asset = Asset::from_cue4parse(&provider, "StaticMesh'/Game/Meshes/SM_Rock.SM_Rock'")?;
if let Some(mesh_data) = mesh_asset.extract_static_mesh()? {
    println!("Mesh has {} vertices and {} triangles", 
             mesh_data.vertex_count(), mesh_data.triangle_count());
    
    // Access vertex data for rendering
    for vertex in &mesh_data.vertices {
        // vertex.x, vertex.y, vertex.z
    }
    
    // Access UV coordinates
    if let Some(uvs) = mesh_data.get_uv_channel(0) {
        for uv in uvs {
            // uv.x, uv.y
        }
    }
}

// Load texture with format support (like Stove does)
let texture_asset = Asset::from_cue4parse(&provider, "Texture2D'/Game/Textures/T_Rock.T_Rock'")?;
if let Some(texture_data) = texture_asset.extract_texture_data()? {
    match texture_data.format.as_str() {
        "PF_DXT1" => println!("BC1 compressed texture"),
        "PF_DXT5" => println!("BC3 compressed texture"),
        "PF_ASTC_4x4" => println!("ASTC mobile texture"),
        _ => println!("Other format: {}", texture_data.format),
    }
}
```

### Migration from unreal_asset

The compatibility layer provides a seamless migration path:

```rust
// Before (unreal_asset)
use unreal_asset::{Asset, read};
let asset = read(&mut reader, &engine_version, None)?;

// After (CUE4Parse with compatibility)
use cue4parse_rs::unreal_asset::{Asset, UnrealAssetCompat};
let asset = Asset::from_cue4parse(&provider, "MyAsset.MyAsset")?;

// Same API, enhanced capabilities!
```

Run the Stove compatibility example:
```bash
cargo run --example stove_compat --features unrealmodding-compat
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cue4parse-rs = { path = "path/to/cue4parse-rs" }

# For unreal_asset compatibility
cue4parse-rs = { path = "path/to/cue4parse-rs", features = ["unrealmodding-compat"] }
```

### Migrating from unreal_modding

If you're migrating from the `unreal_modding` crate, enable the compatibility feature:

```rust
use cue4parse_rs::{Provider, GameVersion};
use cue4parse_rs::unreal_asset::{Asset, UnrealAssetCompat};

let provider = Provider::new("/path/to/game", GameVersion::UE5_3);
let asset = Asset::from_cue4parse(&provider, "MyAsset.MyAsset")?;

// Use familiar unreal_asset APIs
println!("Asset: {}", asset.asset_data.object_name);
for export in &asset.asset_data.exports {
    println!("Export: {}", export.object_name);
    for (name, property) in &export.properties {
        println!("  Property {}: {:?}", name, property);
    }
}
```

See [UNREAL_ASSET_MIGRATION.md](UNREAL_ASSET_MIGRATION.md) for a complete migration guide.

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
- `unrealmodding-compat` - Enable compatibility layer for `unreal_modding` crate migration (default: disabled)

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
