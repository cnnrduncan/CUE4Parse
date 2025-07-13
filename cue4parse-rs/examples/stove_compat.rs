//! # Stove Compatibility Example
//!
//! This example demonstrates the advanced features of the unreal_asset compatibility layer
//! specifically designed for Stove and similar level editing tools. It shows how to:
//!
//! - Extract mesh data with vertices, indices, and UV coordinates
//! - Parse texture data with various compression formats
//! - Handle actor transforms and component hierarchies
//! - Work with material properties and texture references
//! - Process per-platform properties and advanced property types
//!
//! Run with: `cargo run --example stove_compat --features unrealmodding-compat`

use cue4parse_rs::{Provider, GameVersion, Result};

#[cfg(feature = "unrealmodding-compat")]
use cue4parse_rs::unreal_asset::{
    Asset, UnrealAssetCompat, Property, FName, ConversionUtils, AdvancedAssetProcessing,
    Vector, Vector2D, Vector4, Rotator, Quat, LinearColor, Transform, SoftObjectPath,
    StaticMeshData, Texture2DData, MaterialData, ActorData
};

#[cfg(feature = "unrealmodding-compat")]
fn main() -> Result<()> {
    println!("=== Stove Compatibility Demo ===\n");
    
    // Setup provider for a typical game
    let mut provider = Provider::new("C:/Game/Content/Paks", GameVersion::UE5_3);
    provider.add_key("", "0x1234567890ABCDEF1234567890ABCDEF12345678");
    provider.set_mappings("C:/Game/Mappings.usmap");
    
    println!("✓ Created CUE4Parse provider for Stove");
    
    // Demo 1: Static Mesh Processing (Critical for Stove)
    demonstrate_mesh_processing(&provider)?;
    
    // Demo 2: Texture Processing 
    demonstrate_texture_processing(&provider)?;
    
    // Demo 3: Material Processing
    demonstrate_material_processing(&provider)?;
    
    // Demo 4: Actor Transform Processing (Core Stove functionality)
    demonstrate_actor_processing(&provider)?;
    
    // Demo 5: Advanced Property Types
    demonstrate_advanced_properties();
    
    // Demo 6: Level/Map Processing
    demonstrate_level_processing(&provider)?;
    
    println!("\n=== Stove Migration Benefits ===");
    print_stove_benefits();
    
    Ok(())
}

#[cfg(feature = "unrealmodding-compat")]
fn demonstrate_mesh_processing(provider: &Provider) -> Result<()> {
    println!("--- Static Mesh Processing ---");
    
    // Try to load a static mesh asset
    match Asset::from_cue4parse(provider, "StaticMesh'/Game/Meshes/SM_Rock.SM_Rock'") {
        Ok(asset) => {
            println!("✓ Loaded static mesh asset");
            
            // Extract mesh data using the advanced processing trait
            if let Ok(Some(mesh_data)) = asset.extract_static_mesh() {
                println!("✓ Extracted mesh data:");
                println!("  Vertices: {}", mesh_data.vertex_count());
                println!("  Triangles: {}", mesh_data.triangle_count());
                println!("  UV Channels: {}", mesh_data.uv_channels.len());
                println!("  Materials: {:?}", mesh_data.materials);
                
                // Example of vertex processing (like Stove does)
                if !mesh_data.vertices.is_empty() {
                    let first_vertex = &mesh_data.vertices[0];
                    println!("  First vertex: ({:.2}, {:.2}, {:.2})", 
                             first_vertex.x, first_vertex.y, first_vertex.z);
                }
                
                // Example of UV processing
                if let Some(uv_channel) = mesh_data.get_uv_channel(0) {
                    if !uv_channel.is_empty() {
                        let first_uv = &uv_channel[0];
                        println!("  First UV: ({:.3}, {:.3})", first_uv.x, first_uv.y);
                    }
                }
                
                // Material processing
                for (i, material) in mesh_data.materials.iter().enumerate() {
                    println!("  Material {}: {}", i, material);
                }
            } else {
                println!("⚠ No mesh data found in asset");
            }
        }
        Err(e) => {
            println!("⚠ Could not load mesh (demo): {}", e);
            
            // Show how to create mesh data manually for testing
            let mut demo_mesh = StaticMeshData::new();
            demo_mesh.vertices = vec![
                Vector::new(0.0, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.5, 1.0, 0.0),
            ];
            demo_mesh.indices = vec![0, 1, 2];
            demo_mesh.uv_channels.push(vec![
                Vector2D::new(0.0, 0.0),
                Vector2D::new(1.0, 0.0),
                Vector2D::new(0.5, 1.0),
            ]);
            demo_mesh.materials.push("M_DefaultMaterial".to_string());
            
            println!("✓ Created demo mesh data for testing");
            println!("  Vertices: {}", demo_mesh.vertex_count());
            println!("  Triangles: {}", demo_mesh.triangle_count());
        }
    }
    
    println!();
    Ok(())
}

#[cfg(feature = "unrealmodding-compat")]
fn demonstrate_texture_processing(provider: &Provider) -> Result<()> {
    println!("--- Texture Processing ---");
    
    match Asset::from_cue4parse(provider, "Texture2D'/Game/Textures/T_Rock_Diffuse.T_Rock_Diffuse'") {
        Ok(asset) => {
            println!("✓ Loaded texture asset");
            
            if let Ok(Some(texture_data)) = asset.extract_texture_data() {
                println!("✓ Extracted texture data:");
                println!("  Dimensions: {}x{}", texture_data.width, texture_data.height);
                println!("  Format: {}", texture_data.format);
                println!("  Compressed: {}", texture_data.is_compressed());
                println!("  Data size: {} bytes", texture_data.size_bytes());
                println!("  Mip levels: {}", texture_data.mip_count);
                println!("  Normal map: {}", texture_data.is_normal_map);
                
                // Stove-specific format handling
                match texture_data.format.as_str() {
                    "PF_DXT1" => println!("  → BC1 compression (no alpha)"),
                    "PF_DXT5" => println!("  → BC3 compression (with alpha)"),
                    "PF_BC7" => println!("  → BC7 compression (high quality)"),
                    "PF_ASTC_4x4" => println!("  → ASTC 4x4 compression (mobile)"),
                    "PF_B8G8R8A8" => println!("  → Uncompressed RGBA"),
                    _ => println!("  → Other format: {}", texture_data.format),
                }
            }
        }
        Err(e) => {
            println!("⚠ Could not load texture (demo): {}", e);
            
            // Demo texture data creation
            let demo_texture = Texture2DData::new(512, 512, "PF_DXT5".to_string());
            println!("✓ Created demo texture data:");
            println!("  Format: {} (compressed: {})", demo_texture.format, demo_texture.is_compressed());
        }
    }
    
    println!();
    Ok(())
}

#[cfg(feature = "unrealmodding-compat")]
fn demonstrate_material_processing(provider: &Provider) -> Result<()> {
    println!("--- Material Processing ---");
    
    match Asset::from_cue4parse(provider, "Material'/Game/Materials/M_Rock.M_Rock'") {
        Ok(asset) => {
            println!("✓ Loaded material asset");
            
            if let Ok(Some(material_data)) = asset.extract_material_data() {
                println!("✓ Extracted material data:");
                println!("  Name: {}", material_data.name);
                println!("  Type: {}", material_data.material_type);
                
                // Texture references (critical for Stove)
                println!("  Textures:");
                for (slot, texture_path) in &material_data.textures {
                    println!("    {}: {}", slot, texture_path);
                }
                
                // Parameters
                println!("  Scalar Parameters:");
                for (name, value) in &material_data.scalar_parameters {
                    println!("    {}: {:.3}", name, value);
                }
                
                println!("  Vector Parameters:");
                for (name, color) in &material_data.vector_parameters {
                    println!("    {}: ({:.2}, {:.2}, {:.2}, {:.2})", 
                             name, color.r, color.g, color.b, color.a);
                }
            }
            
            // Get texture references using the trait method
            if let Ok(texture_refs) = asset.get_texture_references() {
                println!("✓ Found {} texture references:", texture_refs.len());
                for texture_ref in texture_refs {
                    println!("  → {}", texture_ref);
                }
            }
        }
        Err(e) => {
            println!("⚠ Could not load material (demo): {}", e);
            
            // Demo material creation
            let mut demo_material = MaterialData::new("M_Demo".to_string(), "Material".to_string());
            demo_material.textures.insert("BaseColor".to_string(), "T_Demo_Diffuse".to_string());
            demo_material.textures.insert("Normal".to_string(), "T_Demo_Normal".to_string());
            demo_material.scalar_parameters.insert("Roughness".to_string(), 0.5);
            demo_material.vector_parameters.insert("EmissiveColor".to_string(), 
                                                   LinearColor::new(1.0, 0.5, 0.0, 1.0));
            
            println!("✓ Created demo material with {} textures", demo_material.textures.len());
        }
    }
    
    println!();
    Ok(())
}

#[cfg(feature = "unrealmodding-compat")]
fn demonstrate_actor_processing(provider: &Provider) -> Result<()> {
    println!("--- Actor Transform Processing (Core Stove Feature) ---");
    
    match Asset::from_cue4parse(provider, "World'/Game/Maps/TestLevel.TestLevel'") {
        Ok(asset) => {
            println!("✓ Loaded level asset");
            
            if let Ok(actors) = asset.extract_actors() {
                println!("✓ Found {} actors:", actors.len());
                
                for (i, actor) in actors.iter().take(5).enumerate() {
                    println!("  Actor {}: {} ({})", i, actor.name, actor.class);
                    
                    // Transform information (critical for Stove editing)
                    let transform = &actor.transform;
                    println!("    Location: ({:.2}, {:.2}, {:.2})", 
                             transform.location.x, transform.location.y, transform.location.z);
                    println!("    Rotation: ({:.2}, {:.2}, {:.2}, {:.2})", 
                             transform.rotation.x, transform.rotation.y, 
                             transform.rotation.z, transform.rotation.w);
                    println!("    Scale: ({:.2}, {:.2}, {:.2})", 
                             transform.scale.x, transform.scale.y, transform.scale.z);
                    
                    // Find mesh components (what Stove visualizes)
                    if let Some(mesh_path) = ConversionUtils::find_mesh_component(&asset, 
                        asset.asset_data.exports.get(i).unwrap_or(&asset.asset_data.exports[0])) {
                        println!("    Mesh: {}", mesh_path);
                    }
                    
                    // Show key properties
                    for (prop_name, property) in actor.properties.iter().take(3) {
                        println!("    {}: {:?}", prop_name, property_type_name(property));
                    }
                }
            }
        }
        Err(e) => {
            println!("⚠ Could not load level (demo): {}", e);
            
            // Demo actor creation
            let demo_actor = ActorData {
                name: "StaticMeshActor_1".to_string(),
                class: "StaticMeshActor".to_string(),
                transform: Transform::new(
                    Vector::new(100.0, 200.0, 50.0),
                    Quat::identity(),
                    Vector::new(1.0, 1.0, 1.0)
                ),
                properties: indexmap::IndexMap::new(),
                components: Vec::new(),
            };
            
            println!("✓ Created demo actor at location ({:.1}, {:.1}, {:.1})", 
                     demo_actor.transform.location.x,
                     demo_actor.transform.location.y, 
                     demo_actor.transform.location.z);
        }
    }
    
    println!();
    Ok(())
}

#[cfg(feature = "unrealmodding-compat")]
fn demonstrate_advanced_properties() {
    println!("--- Advanced Property Types (Stove Features) ---");
    
    // Vector properties (essential for transforms)
    let location = Property::Vector(Vector::new(100.0, 200.0, 50.0));
    let rotation = Property::Rotator(Rotator::new(0.0, 90.0, 0.0));
    let scale = Property::Vector(Vector::new(2.0, 1.0, 1.5));
    
    println!("✓ Transform properties:");
    println!("  Location: {:?}", location);
    println!("  Rotation: {:?}", rotation);
    println!("  Scale: {:?}", scale);
    
    // Color properties (for materials and lighting)
    let color = Property::LinearColor(LinearColor::new(1.0, 0.5, 0.2, 1.0));
    println!("✓ Color property: {:?}", color);
    
    // Soft object references (for asset references)
    let mesh_ref = Property::SoftObjectPath(SoftObjectPath::new("/Game/Meshes/SM_Rock", ""));
    let material_ref = Property::SoftClassPath(SoftObjectPath::new("/Game/Materials/M_Rock", ""));
    
    println!("✓ Asset references:");
    println!("  Mesh: {:?}", mesh_ref);
    println!("  Material: {:?}", material_ref);
    
    // Per-platform properties (for multi-platform games)
    let platform_floats = Property::PerPlatformFloat(vec![1.0, 0.8, 1.2]); // PC, Console, Mobile
    let platform_bools = Property::PerPlatformBool(vec![true, false, true]);
    
    println!("✓ Per-platform properties:");
    println!("  Platform floats: {:?}", platform_floats);
    println!("  Platform bools: {:?}", platform_bools);
    
    // GUID properties (for unique identification)
    let guid = Property::Guid([0x12345678, 0x9ABCDEF0, 0x12345678, 0x9ABCDEF0]);
    println!("✓ GUID property: {:?}", guid);
    
    // DateTime properties (for timestamps)
    let datetime = Property::DateTime(637500000000000000); // .NET ticks
    let timespan = Property::TimeSpan(600000000); // 1 minute in ticks
    
    println!("✓ Time properties:");
    println!("  DateTime: {:?}", datetime);
    println!("  TimeSpan: {:?}", timespan);
    
    // Demonstrate property value extraction (for UI editing)
    if let Some(float_val) = ConversionUtils::property_as_float(&Property::Float(3.14)) {
        println!("✓ Extracted float value: {:.2}", float_val);
    }
    
    println!();
}

#[cfg(feature = "unrealmodding-compat")]
fn demonstrate_level_processing(provider: &Provider) -> Result<()> {
    println!("--- Level/Map Processing ---");
    
    // This is what Stove primarily works with - level assets
    match Asset::from_cue4parse(provider, "World'/Game/Maps/MainLevel.MainLevel'") {
        Ok(asset) => {
            println!("✓ Loaded level asset");
            println!("  Exports: {}", asset.asset_data.exports.len());
            println!("  Imports: {}", asset.asset_data.imports.len());
            
            // Find PersistentLevel (where actors are stored)
            if let Some(persistent_level) = asset.get_export_by_name("PersistentLevel") {
                println!("✓ Found PersistentLevel export");
                
                // Look for actor arrays
                for (prop_name, property) in &persistent_level.properties {
                    if prop_name.contains("Actor") && matches!(property, Property::Array(_)) {
                        if let Property::Array(actors) = property {
                            println!("  {} contains {} actors", prop_name, actors.len());
                        }
                    }
                }
            }
            
            // Extract material references (needed for rendering)
            let materials = ConversionUtils::extract_material_references(&asset);
            println!("✓ Found {} material references", materials.len());
            for material in materials.iter().take(5) {
                println!("  → {}", material);
            }
            
        }
        Err(e) => {
            println!("⚠ Could not load level (demo): {}", e);
            println!("  In Stove, you would typically:");
            println!("  1. Load a .umap file");
            println!("  2. Extract all actors from PersistentLevel");
            println!("  3. Build transform hierarchies");
            println!("  4. Load associated meshes and materials");
            println!("  5. Render the scene in 3D");
        }
    }
    
    println!();
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
        Property::Unknown(_) => "Unknown",
    }
}

#[cfg(feature = "unrealmodding-compat")]
fn print_stove_benefits() {
    println!("🎯 Enhanced CUE4Parse provides Stove with:");
    println!("  • Direct mesh vertex/index access for 3D rendering");
    println!("  • Comprehensive texture format support (DXT, ASTC, BC, etc.)");
    println!("  • Full transform property handling (location, rotation, scale)");
    println!("  • Material parameter extraction and texture references");
    println!("  • Actor component hierarchy resolution");
    println!("  • Per-platform property support for multi-target games");
    println!("  • Advanced property types (GUID, DateTime, Delegates)");
    println!("  • Soft object path resolution for asset references");
    println!("  • Level/map structure parsing with actor arrays");
    println!("  • Type-safe property conversion and UI editing support");
    println!();
    println!("🚀 Migration from unreal_asset is seamless:");
    println!("  • Same API structure and method names");
    println!("  • Compatible property system and type hierarchy");
    println!("  • Enhanced with CUE4Parse's superior parsing capabilities");
    println!("  • Better performance and format support");
    println!("  • Active development and game compatibility updates");
}

#[cfg(not(feature = "unrealmodding-compat"))]
fn main() {
    println!("This example requires the 'unrealmodding-compat' feature to be enabled.");
    println!("Run with: cargo run --example stove_compat --features unrealmodding-compat");
}
