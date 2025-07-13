# Stove & Advanced Tool Compatibility Guide

This document describes the enhanced features in the CUE4Parse Rust bindings specifically designed for compatibility with [Stove](https://github.com/bananaturtlesandwich/stove) and other advanced Unreal Engine modding tools.

## Overview

The `unrealmodding-compat` feature provides a comprehensive compatibility layer that mimics the `unreal_asset` API while leveraging CUE4Parse's superior parsing capabilities. This enables tools like Stove to migrate seamlessly while gaining access to enhanced features.

## Key Features for Stove

### 1. Static Mesh Processing

Stove requires detailed mesh information for 3D visualization and editing. The compatibility layer provides:

```rust
use cue4parse_rs::unreal_asset::{Asset, AdvancedAssetProcessing};

let mesh_asset = Asset::from_cue4parse(&provider, "StaticMesh'/Game/Props/SM_Rock.SM_Rock'")?;
if let Some(mesh_data) = mesh_asset.extract_static_mesh()? {
    // Access vertex positions for rendering
    for vertex in &mesh_data.vertices {
        println!("Vertex: ({:.2}, {:.2}, {:.2})", vertex.x, vertex.y, vertex.z);
    }
    
    // Access triangle indices
    for triangle in mesh_data.indices.chunks(3) {
        println!("Triangle: [{}, {}, {}]", triangle[0], triangle[1], triangle[2]);
    }
    
    // Access UV coordinates (multiple channels supported)
    if let Some(uv_channel_0) = mesh_data.get_uv_channel(0) {
        for uv in uv_channel_0 {
            println!("UV: ({:.3}, {:.3})", uv.x, uv.y);
        }
    }
    
    // Access material assignments
    for (i, material) in mesh_data.materials.iter().enumerate() {
        let (start_index, count) = mesh_data.material_ranges[i];
        println!("Material '{}': {} triangles starting at index {}", 
                 material, count / 3, start_index);
    }
}
```

### 2. Texture Processing

Stove supports various texture formats for material visualization:

```rust
let texture_asset = Asset::from_cue4parse(&provider, "Texture2D'/Game/Textures/T_Concrete.T_Concrete'")?;
if let Some(texture_data) = texture_asset.extract_texture_data()? {
    println!("Texture: {}x{} ({})", texture_data.width, texture_data.height, texture_data.format);
    
    // Handle different compression formats (as Stove does)
    match texture_data.format.as_str() {
        "PF_DXT1" => {
            // BC1 compression (4 bits per pixel, no alpha)
            decode_bc1(&texture_data.data, texture_data.width, texture_data.height)?;
        },
        "PF_DXT5" => {
            // BC3 compression (8 bits per pixel, with alpha)
            decode_bc3(&texture_data.data, texture_data.width, texture_data.height)?;
        },
        "PF_ASTC_4x4" => {
            // ASTC 4x4 compression (mobile)
            decode_astc_4x4(&texture_data.data, texture_data.width, texture_data.height)?;
        },
        "PF_B8G8R8A8" => {
            // Uncompressed RGBA
            let rgba_data = &texture_data.data;
        },
        _ => {
            println!("Unsupported format: {}", texture_data.format);
        }
    }
}
```

### 3. Actor Transform Processing

Core to Stove's level editing functionality is handling actor transforms:

```rust
use cue4parse_rs::unreal_asset::{ConversionUtils, Property, Vector, Rotator, Quat};

// Load a level asset
let level_asset = Asset::from_cue4parse(&provider, "World'/Game/Maps/TestLevel.TestLevel'")?;

// Extract all actors with their transforms
let actors = level_asset.extract_actors()?;
for actor in actors {
    println!("Actor: {} ({})", actor.name, actor.class);
    
    // Access transform components
    let transform = &actor.transform;
    println!("  Location: ({:.1}, {:.1}, {:.1})", 
             transform.location.x, transform.location.y, transform.location.z);
    println!("  Rotation: ({:.2}, {:.2}, {:.2}, {:.2})", 
             transform.rotation.x, transform.rotation.y, 
             transform.rotation.z, transform.rotation.w);
    println!("  Scale: ({:.2}, {:.2}, {:.2})", 
             transform.scale.x, transform.scale.y, transform.scale.z);
    
    // Find associated mesh component (for rendering)
    if let Some(mesh_path) = ConversionUtils::find_mesh_component(&level_asset, &export) {
        println!("  Mesh: {}", mesh_path);
    }
    
    // Access individual properties (for editing UI)
    for (prop_name, property) in &actor.properties {
        match property {
            Property::Vector(vec) if prop_name == "RelativeLocation" => {
                // Handle location property specifically
                println!("  Custom Location: ({:.1}, {:.1}, {:.1})", vec.x, vec.y, vec.z);
            },
            Property::Bool(value) => {
                println!("  {}: {}", prop_name, value);
            },
            Property::String(value) => {
                println!("  {}: {}", prop_name, value);
            },
            Property::SoftObjectPath(path) => {
                println!("  {}: {}", prop_name, path.asset_path.name);
            },
            _ => {
                println!("  {}: {:?}", prop_name, property);
            }
        }
    }
}
```

### 4. Material Processing

Stove needs material information for proper rendering and texture assignment:

```rust
let material_asset = Asset::from_cue4parse(&provider, "Material'/Game/Materials/M_Concrete.M_Concrete'")?;
if let Some(material_data) = material_asset.extract_material_data()? {
    println!("Material: {} ({})", material_data.name, material_data.material_type);
    
    // Access texture slots (critical for Stove's rendering)
    for (slot_name, texture_path) in &material_data.textures {
        println!("  {}: {}", slot_name, texture_path);
        
        // Load and process the referenced texture
        if let Ok(texture_asset) = Asset::from_cue4parse(&provider, texture_path) {
            if let Ok(Some(texture_data)) = texture_asset.extract_texture_data() {
                println!("    → {}x{} ({})", texture_data.width, texture_data.height, texture_data.format);
            }
        }
    }
    
    // Access material parameters
    for (param_name, value) in &material_data.scalar_parameters {
        println!("  {} = {:.3}", param_name, value);
    }
    
    for (param_name, color) in &material_data.vector_parameters {
        println!("  {} = ({:.2}, {:.2}, {:.2}, {:.2})", 
                 param_name, color.r, color.g, color.b, color.a);
    }
}

// Alternative: Get texture references directly
let texture_refs = material_asset.get_texture_references()?;
for texture_ref in texture_refs {
    println!("Referenced texture: {}", texture_ref);
}
```

### 5. Advanced Property Types

Stove works with many specialized Unreal Engine property types:

```rust
use cue4parse_rs::unreal_asset::*;

// Vector properties (transforms, positions, etc.)
let location = Property::Vector(Vector::new(100.0, 200.0, 50.0));
let rotation = Property::Rotator(Rotator::new(0.0, 90.0, 0.0)); // Pitch, Yaw, Roll
let scale = Property::Vector(Vector::new(1.0, 1.0, 2.0));

// Color properties (materials, lighting)
let color = Property::LinearColor(LinearColor::new(1.0, 0.5, 0.2, 1.0));

// Asset references (meshes, materials, textures)
let mesh_ref = Property::SoftObjectPath(SoftObjectPath::new("/Game/Meshes/SM_Rock", ""));
let material_ref = Property::SoftClassPath(SoftObjectPath::new("/Game/Materials/M_Rock", ""));

// Per-platform properties (for multi-platform games)
let platform_quality = Property::PerPlatformFloat(vec![1.0, 0.8, 0.6]); // PC, Console, Mobile
let platform_enabled = Property::PerPlatformBool(vec![true, true, false]);

// Specialized properties
let guid = Property::Guid([0x12345678, 0x9ABCDEF0, 0x12345678, 0x9ABCDEF0]);
let timestamp = Property::DateTime(637500000000000000); // .NET ticks

// Property value extraction for UI editing
if let Some(float_val) = ConversionUtils::property_as_float(&location) {
    println!("Location X as float: {:.2}", float_val);
}

// Property modification for editing
let mut mutable_prop = Property::Float(3.14);
ConversionUtils::set_property_from_float(&mut mutable_prop, 2.71);
```

## Migration from unreal_asset

Tools using the `unreal_asset` crate can migrate with minimal code changes:

### Before (unreal_asset)
```rust
use unreal_asset::{Asset, read, properties::Property};

// Load asset
let mut reader = BufReader::new(File::open("asset.uasset")?);
let asset = read(&mut reader, &engine_version, None)?;

// Access properties
for export in &asset.asset_data.exports {
    for property in &export.properties {
        match property {
            Property::VectorProperty(vec) => {
                println!("Vector: {}, {}, {}", vec.value.x, vec.value.y, vec.value.z);
            },
            _ => {}
        }
    }
}
```

### After (CUE4Parse with compatibility)
```rust
use cue4parse_rs::{Provider, GameVersion};
use cue4parse_rs::unreal_asset::{Asset, UnrealAssetCompat, Property};

// Load asset (now with better parsing and format support)
let provider = Provider::new("/game/path", GameVersion::UE5_3);
let asset = Asset::from_cue4parse(&provider, "MyAsset.MyAsset")?;

// Access properties (same API!)
for export in &asset.asset_data.exports {
    for (prop_name, property) in &export.properties {
        match property {
            Property::Vector(vec) => {
                println!("Vector: {}, {}, {}", vec.x, vec.y, vec.z);
            },
            _ => {}
        }
    }
}
```

## Performance Considerations

### Memory Usage
- The compatibility layer creates intermediate structures for seamless API compatibility
- For memory-critical applications, consider using the native CUE4Parse API directly
- Mesh and texture data are loaded on-demand to minimize memory footprint

### Processing Speed
- CUE4Parse's native parsing is generally faster than unreal_asset
- Property conversion adds minimal overhead
- Large meshes and textures benefit from streaming/chunked processing

### Caching
- Use the provider's caching mechanisms for frequently accessed assets
- Material and texture references can be cached to avoid redundant parsing
- Consider implementing application-level caching for processed mesh data

## Error Handling

The compatibility layer provides comprehensive error handling:

```rust
use cue4parse_rs::Result;

fn process_asset(provider: &Provider, path: &str) -> Result<()> {
    match Asset::from_cue4parse(provider, path) {
        Ok(asset) => {
            // Process asset...
            if let Ok(Some(mesh_data)) = asset.extract_static_mesh() {
                // Handle mesh...
            } else {
                println!("No mesh data found in asset");
            }
        },
        Err(e) => {
            eprintln!("Failed to load asset '{}': {}", path, e);
            // Handle error appropriately...
        }
    }
    Ok(())
}
```

## Best Practices for Stove Integration

1. **Asset Loading**: Use the provider's game detection and key management
2. **Transform Handling**: Always check for both relative and absolute transform properties
3. **Mesh Processing**: Handle LOD levels appropriately (Stove typically uses LOD 0)
4. **Texture Formats**: Implement fallbacks for unsupported compression formats
5. **Error Recovery**: Gracefully handle missing or corrupted assets
6. **Performance**: Cache frequently accessed data and use background loading
7. **Memory Management**: Unload unused assets to prevent memory leaks

## Integration Examples

### Stove-style Actor Duplication
```rust
// Load source actor
let source_asset = Asset::from_cue4parse(&provider, source_path)?;
let source_actor = source_asset.extract_actors()?.into_iter().next().ok_or("No actor found")?;

// Create duplicate with modified transform
let mut duplicate_actor = source_actor.clone();
duplicate_actor.name = format!("{}_Copy", duplicate_actor.name);
duplicate_actor.transform.location.x += 100.0; // Offset position

// Note: Actual serialization back to .uasset would require additional implementation
```

### Stove-style Material Preview
```rust
// Load material and extract all textures
let material_asset = Asset::from_cue4parse(&provider, material_path)?;
let texture_refs = material_asset.get_texture_references()?;

for texture_ref in texture_refs {
    if let Ok(texture_asset) = Asset::from_cue4parse(&provider, &texture_ref) {
        if let Ok(Some(texture_data)) = texture_asset.extract_texture_data() {
            // Convert to GPU texture format...
            // Apply to material shader...
        }
    }
}
```

This enhanced compatibility layer provides Stove with all the advanced features it needs while maintaining the familiar `unreal_asset` API structure.
