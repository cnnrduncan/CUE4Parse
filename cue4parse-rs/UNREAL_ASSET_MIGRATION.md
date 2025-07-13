# Unreal Asset Compatibility Layer

This document describes the compatibility layer that allows migration from the `unreal_modding` crate's `unreal_asset` module to CUE4Parse-based parsing.

## Overview

The compatibility layer provides:

- **API-compatible structures** that mirror `unreal_asset`'s main types
- **Conversion functions** to transform CUE4Parse JSON output into familiar structures
- **Migration helpers** including macros and utilities
- **Complete type coverage** for all common Unreal Engine property types

## Quick Migration Guide

### Before (unreal_modding)

```rust
use unreal_modding::unreal_asset::{Asset, read, EngineVersion};
use std::io::Cursor;

// Read from binary data
let mut reader = Cursor::new(asset_bytes);
let asset = read(&mut reader, EngineVersion::VER_UE5_3, None)?;

// Access properties
println!("Asset: {}", asset.asset_data.object_name);
for export in &asset.asset_data.exports {
    println!("Export: {}", export.object_name);
    for (name, property) in &export.properties {
        println!("  Property {}: {:?}", name, property);
    }
}
```

### After (CUE4Parse with compatibility)

```rust
use cue4parse_rs::{Provider, GameVersion};
use cue4parse_rs::unreal_asset::{Asset, UnrealAssetCompat};

// Create provider and load asset
let provider = Provider::new("/path/to/game", GameVersion::UE5_3);
let asset = Asset::from_cue4parse(&provider, "MyAsset.MyAsset")?;

// Access properties (same API!)
println!("Asset: {}", asset.asset_data.object_name);
for export in &asset.asset_data.exports {
    println!("Export: {}", export.object_name);
    for (name, property) in &export.properties {
        println!("  Property {}: {:?}", name, property);
    }
}
```

## Type Mapping

### Core Structures

| unreal_asset | CUE4Parse Compatibility | Notes |
|--------------|-------------------------|--------|
| `Asset` | `cue4parse_rs::unreal_asset::Asset` | Main asset container |
| `AssetData` | `cue4parse_rs::unreal_asset::AssetData` | Package data |
| `Export` | `cue4parse_rs::unreal_asset::Export` | Export table entry |
| `Import` | `cue4parse_rs::unreal_asset::Import` | Import table entry |
| `FName` | `cue4parse_rs::unreal_asset::FName` | Indexed string |
| `PackageIndex` | `cue4parse_rs::unreal_asset::PackageIndex` | Object reference |

### Property Types

| unreal_asset | CUE4Parse Compatibility | Example |
|--------------|-------------------------|---------|
| `BoolProperty` | `Property::Bool(bool)` | `Property::Bool(true)` |
| `IntProperty` | `Property::Int32(i32)` | `Property::Int32(42)` |
| `Int64Property` | `Property::Int64(i64)` | `Property::Int64(123456789)` |
| `FloatProperty` | `Property::Float(f32)` | `Property::Float(3.14)` |
| `DoubleProperty` | `Property::Double(f64)` | `Property::Double(3.14159265)` |
| `StrProperty` | `Property::String(String)` | `Property::String("text".to_string())` |
| `NameProperty` | `Property::Name(FName)` | `Property::Name(FName::new("Name"))` |
| `ObjectProperty` | `Property::Object(Option<PackageIndex>)` | `Property::Object(Some(PackageIndex(5)))` |
| `StructProperty` | `Property::Struct { .. }` | See struct examples below |
| `ArrayProperty` | `Property::Array(Vec<Property>)` | `Property::Array(vec![...])` |
| `MapProperty` | `Property::Map { .. }` | See map examples below |
| `EnumProperty` | `Property::Enum { .. }` | See enum examples below |
| `TextProperty` | `Property::Text { .. }` | See text examples below |

## Usage Examples

### Basic Asset Loading

```rust
use cue4parse_rs::{Provider, GameVersion};
use cue4parse_rs::unreal_asset::{Asset, UnrealAssetCompat};

let mut provider = Provider::new("/path/to/fortnite/paks", GameVersion::UE5_3);
provider.add_key("", "0xYOUR_AES_KEY");
provider.set_mappings("/path/to/mappings.usmap");

// Load an asset
let asset = Asset::from_cue4parse(&provider, "GameData/Items/Weapons/Rifle.Rifle")?;

println!("Loaded asset: {}", asset.asset_data.object_name);
println!("Engine version: {}", asset.asset_data.engine_version);
println!("Exports: {}", asset.asset_data.exports.len());
```

### Property Access

```rust
// Get the main export (usually the first one)
if let Some(main_export) = asset.get_main_export() {
    println!("Main export: {}", main_export.object_name);
    
    // Access specific property types
    if let Some(Property::String(display_name)) = main_export.properties.get("DisplayName") {
        println!("Display name: {}", display_name);
    }
    
    if let Some(Property::Bool(is_epic)) = main_export.properties.get("IsEpic") {
        println!("Is epic: {}", is_epic);
    }
    
    if let Some(Property::Int32(item_level)) = main_export.properties.get("ItemLevel") {
        println!("Item level: {}", item_level);
    }
}
```

### Working with Structs

```rust
use cue4parse_rs::unreal_asset::Property;

// Access a struct property (e.g., a Vector)
if let Some(Property::Struct { struct_type, properties }) = export.properties.get("Location") {
    println!("Struct type: {}", struct_type);
    
    if let Some(Property::Float(x)) = properties.get("X") {
        println!("X: {}", x);
    }
    if let Some(Property::Float(y)) = properties.get("Y") {
        println!("Y: {}", y);
    }
    if let Some(Property::Float(z)) = properties.get("Z") {
        println!("Z: {}", z);
    }
}
```

### Working with Arrays

```rust
// Access an array property
if let Some(Property::Array(items)) = export.properties.get("ItemList") {
    println!("Array has {} items", items.len());
    
    for (i, item) in items.iter().enumerate() {
        match item {
            Property::String(s) => println!("Item {}: {}", i, s),
            Property::Int32(n) => println!("Item {}: {}", i, n),
            Property::Struct { struct_type, properties } => {
                println!("Item {}: {} struct with {} properties", i, struct_type, properties.len());
            }
            _ => println!("Item {}: {:?}", i, item),
        }
    }
}
```

### Working with Maps

```rust
// Access a map property
if let Some(Property::Map { key_type, value_type, entries }) = export.properties.get("AttributeMap") {
    println!("Map<{}, {}> with {} entries", key_type, value_type, entries.len());
    
    for (key, value) in entries {
        if let (Property::String(k), Property::Float(v)) = (key, value) {
            println!("  {}: {}", k, v);
        }
    }
}
```

### Working with Enums

```rust
// Access an enum property
if let Some(Property::Enum { enum_type, value }) = export.properties.get("Rarity") {
    println!("Enum {}: {}", enum_type, value);
    
    match value.as_str() {
        "Common" => println!("This is a common item"),
        "Rare" => println!("This is a rare item"),
        "Epic" => println!("This is an epic item"),
        "Legendary" => println!("This is a legendary item"),
        _ => println!("Unknown rarity: {}", value),
    }
}
```

### Using Helper Macros

```rust
use cue4parse_rs::{load_asset, get_property};

// Load asset with error handling
let asset = load_asset!(provider, "MyAsset.MyAsset")?;

// Access properties with type checking
if let Some(main_export) = asset.get_main_export() {
    let display_name = get_property!(main_export, "DisplayName", String);
    let item_level = get_property!(main_export, "ItemLevel", i32);
    let is_stackable = get_property!(main_export, "IsStackable", bool);
    
    println!("Item: {:?}, Level: {:?}, Stackable: {:?}", 
             display_name, item_level, is_stackable);
}
```

## Advanced Usage

### Custom Property Conversion

```rust
use cue4parse_rs::unreal_asset::{ConversionUtils, Property};
use serde_json::json;

// Convert JSON with type hints
let json_value = json!({
    "Health": 100,
    "Name": "Player",
    "IsAlive": true
});

// Extract type metadata
if let Some(health_type) = ConversionUtils::extract_property_type(&json_value, "Health") {
    let health_prop = ConversionUtils::json_to_property_typed(
        &json_value["Health"], 
        &health_type
    );
    println!("Health property: {:?}", health_prop);
}
```

### Engine Version Compatibility

```rust
use cue4parse_rs::unreal_asset::ConversionUtils;
use cue4parse_rs::GameVersion;

let version_string = ConversionUtils::game_version_to_string(&GameVersion::UE5_3);
println!("Engine version: {}", version_string); // "5.3.0"
```

### Working with Package References

```rust
use cue4parse_rs::unreal_asset::PackageIndex;

// Check reference types
let package_ref = PackageIndex(-5);
if package_ref.is_import() {
    if let Some(import_idx) = package_ref.import_index() {
        println!("References import #{}", import_idx);
        if let Some(import) = asset.asset_data.imports.get(import_idx) {
            println!("Import: {}.{}", import.class_package, import.object_name);
        }
    }
}

let export_ref = PackageIndex(3);
if export_ref.is_export() {
    if let Some(export_idx) = export_ref.export_index() {
        println!("References export #{}", export_idx);
        if let Some(export) = asset.asset_data.exports.get(export_idx) {
            println!("Export: {}", export.object_name);
        }
    }
}
```

## Migration Checklist

When migrating from `unreal_asset` to CUE4Parse:

1. **Update Dependencies**
   ```toml
   [dependencies]
   # Remove: unreal_modding = "..."
   cue4parse-rs = { version = "0.1.0", features = ["unrealmodding-compat"] }
   ```

2. **Update Imports**
   ```rust
   // Before
   use unreal_modding::unreal_asset::{Asset, read, EngineVersion};
   
   // After
   use cue4parse_rs::{Provider, GameVersion};
   use cue4parse_rs::unreal_asset::{Asset, UnrealAssetCompat};
   ```

3. **Update Asset Loading**
   ```rust
   // Before: Binary file reading
   let mut reader = Cursor::new(asset_bytes);
   let asset = read(&mut reader, engine_version, None)?;
   
   // After: Provider-based loading
   let provider = Provider::new("/path/to/game", GameVersion::UE5_3);
   let asset = Asset::from_cue4parse(&provider, "Asset.Asset")?;
   ```

4. **Update Property Access** (mostly unchanged)
   ```rust
   // This stays the same!
   for export in &asset.asset_data.exports {
       for (name, property) in &export.properties {
           match property {
               Property::String(s) => println!("String: {}", s),
               Property::Int32(i) => println!("Int: {}", i),
               // ... etc
           }
       }
   }
   ```

5. **Add Game Configuration**
   ```rust
   // Configure the provider with your game's settings
   provider.add_key("", "0xYOUR_MAIN_AES_KEY");
   provider.add_key("guid", "0xYOUR_SPECIFIC_KEY");
   provider.set_mappings("/path/to/mappings.usmap");
   ```

## Benefits of Migration

- **Better Performance**: Native compression support (ACL, Oodle)
- **More Formats**: Complete UE4/UE5 support including newer versions
- **Active Development**: Regular updates and bug fixes
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **No Binary Parsing**: Let CUE4Parse handle the complex binary formats
- **JSON-based**: Easier debugging and inspection of asset data
- **Future-Proof**: Support for new Unreal Engine versions as they're released

## Limitations

- **Feature Flag Required**: Must enable `unrealmodding-compat` feature
- **Provider Setup**: Requires setting up a CUE4Parse provider
- **Process-based**: Uses CLI tool for asset parsing (vs. direct binary reading)
- **Some Metadata Missing**: Some low-level binary metadata may not be available

## Getting Help

- Check the [CUE4Parse documentation](https://github.com/FabianFG/CUE4Parse)
- Look at the [examples](../examples/) directory
- Review the [integration tests](../tests/) for usage patterns
- Open an issue on GitHub for specific migration questions
