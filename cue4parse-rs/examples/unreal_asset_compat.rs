//! # Unreal Asset Compatibility Example
//!
//! This example demonstrates how to use the unreal_asset compatibility layer
//! to migrate from the unreal_modding crate to CUE4Parse while maintaining
//! compatible APIs.
//!
//! Run with: `cargo run --example unreal_asset_compat --features unrealmodding-compat`

use cue4parse_rs::{Provider, GameVersion, Result};

#[cfg(feature = "unrealmodding-compat")]
use cue4parse_rs::unreal_asset::{Asset, UnrealAssetCompat, Property, FName};

#[cfg(feature = "unrealmodding-compat")]
fn main() -> Result<()> {
    println!("=== CUE4Parse Unreal Asset Compatibility Demo ===\n");
    
    // Create a provider (same as regular CUE4Parse usage)
    let mut provider = Provider::new("C:/Game/Content/Paks", GameVersion::UE5_3);
    provider.add_key("", "0x1234567890ABCDEF1234567890ABCDEF12345678");
    provider.set_mappings("C:/Game/Mappings.usmap");
    
    println!("✓ Created CUE4Parse provider");
    
    // Example 1: Load asset using unreal_asset-compatible API
    match Asset::from_cue4parse(&provider, "ExampleAsset.ExampleAsset") {
        Ok(asset) => {
            println!("✓ Loaded asset: {}", asset.asset_data.object_name);
            println!("  Engine version: {}", asset.asset_data.engine_version);
            println!("  Export count: {}", asset.asset_data.exports.len());
            println!("  Import count: {}", asset.asset_data.imports.len());
            
            // Example 2: Access the main export
            if let Some(main_export) = asset.get_main_export() {
                println!("✓ Main export: {}", main_export.object_name);
                println!("  Properties: {}", main_export.properties.len());
                
                // Example 3: Iterate through properties (unreal_asset-style)
                for (name, property) in &main_export.properties {
                    println!("  Property '{}': {:?}", name, property_type_name(property));
                }
                
                // Example 4: Access specific property types
                if let Some(Property::String(str_val)) = main_export.properties.get("SomeStringProperty") {
                    println!("  String property value: {}", str_val);
                }
                
                if let Some(Property::Bool(bool_val)) = main_export.properties.get("SomeBoolProperty") {
                    println!("  Bool property value: {}", bool_val);
                }
                
                if let Some(Property::Int32(int_val)) = main_export.properties.get("SomeIntProperty") {
                    println!("  Int property value: {}", int_val);
                }
            }
            
            // Example 5: Work with FName structures
            let test_name = FName::new("TestName");
            println!("✓ Created FName: {}", test_name);
            
            let numbered_name = FName::with_number("TestName", 5);
            println!("✓ Created numbered FName: {}", numbered_name);
            
            // Example 6: Property creation and manipulation
            demonstrate_property_types();
            
        }
        Err(e) => {
            println!("⚠ Could not load asset (this is expected in demo): {}", e);
            println!("  In a real scenario, make sure the asset path and encryption keys are correct");
        }
    }
    
    println!("\n=== Migration Guide ===");
    print_migration_guide();
    
    Ok(())
}

#[cfg(feature = "unrealmodding-compat")]
fn property_type_name(property: &Property) -> &'static str {
    match property {
        Property::Bool(_) => "Bool",
        Property::Int8(_) => "Int8",
        Property::Int16(_) => "Int16", 
        Property::Int32(_) => "Int32",
        Property::Int64(_) => "Int64",
        Property::UInt8(_) => "UInt8",
        Property::UInt16(_) => "UInt16",
        Property::UInt32(_) => "UInt32",
        Property::UInt64(_) => "UInt64",
        Property::Float(_) => "Float",
        Property::Double(_) => "Double",
        Property::String(_) => "String",
        Property::Name(_) => "Name",
        Property::Object(_) => "Object",
        Property::Struct { .. } => "Struct",
        Property::Array(_) => "Array",
        Property::Map { .. } => "Map",
        Property::Enum { .. } => "Enum",
        Property::Text { .. } => "Text",
        Property::Vector(_) => "Vector",
        Property::Vector4(_) => "Vector4",
        Property::Vector2D(_) => "Vector2D",
        Property::Rotator(_) => "Rotator",
        Property::Quat(_) => "Quat",
        Property::LinearColor(_) => "LinearColor",
        Property::Transform(_) => "Transform",
        Property::SoftObjectPath(_) => "SoftObjectPath",
        Property::SoftClassPath(_) => "SoftClassPath",
        Property::AssetObjectProperty(_) => "AssetObjectProperty",
        Property::PerPlatformBool(_) => "PerPlatformBool",
        Property::PerPlatformInt(_) => "PerPlatformInt",
        Property::PerPlatformFloat(_) => "PerPlatformFloat",
        Property::Guid(_) => "Guid",
        Property::DateTime(_) => "DateTime",
        Property::TimeSpan(_) => "TimeSpan",
        Property::Delegate { .. } => "Delegate",
        Property::MulticastDelegate { .. } => "MulticastDelegate",
        Property::MaterialInterface(_) => "MaterialInterface",
        Property::StaticMesh(_) => "StaticMesh",
        Property::SkeletalMesh(_) => "SkeletalMesh",
        Property::Texture2D(_) => "Texture2D",
        Property::Set(_) => "Set",
        Property::ByteEnum { .. } => "ByteEnum",
        Property::Byte(_) => "Byte",
        Property::MaterialInstance(_) => "MaterialInstance",
        Property::LevelSequence(_) => "LevelSequence",
        Property::ComponentReference(_) => "ComponentReference",
        Property::Blueprint(_) => "Blueprint",
        Property::WorldContext(_) => "WorldContext",
        Property::LandscapeComponent(_) => "LandscapeComponent",
        Property::Unknown(_) => "Unknown",
    }
}

#[cfg(feature = "unrealmodding-compat")]
fn demonstrate_property_types() {
    use cue4parse_rs::unreal_asset::{Property, FName, PackageIndex};
    use indexmap::IndexMap;
    
    println!("✓ Property type demonstrations:");
    
    // Basic types
    let bool_prop = Property::Bool(true);
    let int_prop = Property::Int32(42);
    let string_prop = Property::String("Hello, World!".to_string());
    let name_prop = Property::Name(FName::new("TestName"));
    
    println!("  Bool: {:?}", bool_prop);
    println!("  Int32: {:?}", int_prop);
    println!("  String: {:?}", string_prop);
    println!("  Name: {:?}", name_prop);
    
    // Complex types
    let array_prop = Property::Array(vec![
        Property::Int32(1),
        Property::Int32(2),
        Property::Int32(3),
    ]);
    println!("  Array: {:?}", array_prop);
    
    let mut struct_props = IndexMap::new();
    struct_props.insert("X".to_string(), Property::Float(1.0));
    struct_props.insert("Y".to_string(), Property::Float(2.0));
    struct_props.insert("Z".to_string(), Property::Float(3.0));
    
    let struct_prop = Property::Struct {
        struct_type: FName::new("Vector"),
        properties: struct_props,
    };
    println!("  Struct: {:?}", struct_prop);
    
    // Object reference
    let obj_prop = Property::Object(Some(PackageIndex(5)));
    println!("  Object: {:?}", obj_prop);
}

#[cfg(feature = "unrealmodding-compat")]
fn print_migration_guide() {
    println!("Before (unreal_modding):");
    println!("```rust");
    println!("use unreal_modding::unreal_asset::{{Asset, read}};");
    println!();
    println!("let mut reader = std::io::Cursor::new(asset_data);");
    println!("let asset = read(&mut reader, &engine_version, None)?;");
    println!("println!(\"Asset: {{}}\", asset.asset_data.object_name);");
    println!("```");
    println!();
    println!("After (CUE4Parse with compatibility):");
    println!("```rust");
    println!("use cue4parse_rs::{{Provider, GameVersion}};");
    println!("use cue4parse_rs::unreal_asset::{{Asset, UnrealAssetCompat}};");
    println!();
    println!("let provider = Provider::new(\"/path/to/game\", GameVersion::UE5_3);");
    println!("let asset = Asset::from_cue4parse(&provider, \"MyAsset.MyAsset\")?;");
    println!("println!(\"Asset: {{}}\", asset.asset_data.object_name);");
    println!("```");
    println!();
    println!("Benefits of migration:");
    println!("• Better performance with native compression support");
    println!("• More complete Unreal Engine format support");
    println!("• Active development and updates");
    println!("• Cross-platform compatibility");
    println!("• No need to handle raw binary parsing");
}

#[cfg(not(feature = "unrealmodding-compat"))]
fn main() {
    println!("This example requires the 'unrealmodding-compat' feature.");
    println!("Run with: cargo run --example unreal_asset_compat --features unrealmodding-compat");
}
