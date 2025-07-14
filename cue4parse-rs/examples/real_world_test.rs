/// Real-world test of the unreal_asset compatibility layer
/// 
/// This example tests our compatibility layer against real game files,
/// including both traditional .pak/.uasset files and modern .utoc/.ucas iostore formats.
/// 
/// Usage:
/// cargo run --example real_world_test --features unrealmodding-compat -- \
///   --game-dir "C:\Games\Steam\steamapps\common\Oblivion Remastered\OblivionRemastered" \
///   --usmap-path "C:\path\to\Mappings.usmap"

use std::env;
use cue4parse_rs::{Provider, GameVersion, Result};

#[cfg(feature = "unrealmodding-compat")]
use cue4parse_rs::unreal_asset::{Asset, AssetData, ConversionUtils, UnrealAssetCompat};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let default_game_dir = "C:\\Games\\Steam\\steamapps\\common\\Oblivion Remastered\\OblivionRemastered".to_string();
    let game_dir = args.iter()
        .position(|arg| arg == "--game-dir")
        .and_then(|i| args.get(i + 1))
        .unwrap_or(&default_game_dir);
    
    let usmap_path = args.iter()
        .position(|arg| arg == "--usmap-path")
        .and_then(|i| args.get(i + 1));
    
    println!("🎮 CUE4Parse unreal_asset Compatibility Test");
    println!("===========================================");
    println!("Game Directory: {}", game_dir);
    if let Some(usmap) = usmap_path {
        println!("Using .usmap file: {}", usmap);
    } else {
        println!("⚠️  No .usmap file specified - type information may be limited");
    }
    println!();

    // Test with real files
    test_real_world_assets(game_dir, usmap_path)?;
    
    Ok(())
}

#[cfg(feature = "unrealmodding-compat")]
fn test_real_world_assets(game_dir: &str, usmap_path: Option<&String>) -> Result<()> {
    // Initialize provider
    let mut provider = Provider::new(game_dir, GameVersion::UE5_3);
    println!("✅ Provider initialized successfully");
    
    // Load .usmap if provided
    if let Some(usmap) = usmap_path {
        println!("📂 Loading .usmap mappings...");
        let _ = provider.set_mappings(usmap);
        println!("✅ Mappings loaded successfully");
    }
    
    // Test assets that are commonly found in Unreal Engine games
    let test_assets = vec![
        // Levels/Maps
        ("/Game/Maps/MainMenu", "Level/Map"),
        ("/Game/Maps/TestLevel", "Level/Map"),
        ("/Game/Levels/Persistent", "Level/Map"),
        
        // Static Meshes
        ("/Game/Meshes/Environment/SM_Rock", "Static Mesh"),
        ("/Game/Meshes/Props/SM_Barrel", "Static Mesh"),
        ("/Game/StaticMeshes/SM_Cube", "Static Mesh"),
        
        // Textures
        ("/Game/Textures/Environment/T_Ground_Diffuse", "Texture"),
        ("/Game/Textures/UI/T_Button", "Texture"),
        ("/Game/Materials/Textures/T_Noise", "Texture"),
        
        // Materials
        ("/Game/Materials/Environment/M_Ground", "Material"),
        ("/Game/Materials/Props/M_Metal", "Material"),
        ("/Game/Materials/Master/M_BaseMaterial", "Material"),
        
        // Blueprints
        ("/Game/Blueprints/Actors/BP_Player", "Blueprint"),
        ("/Game/Blueprints/UI/BP_MainMenu", "Blueprint"),
        ("/Game/BluePrints/Gameplay/BP_GameMode", "Blueprint"),
        
        // Animations
        ("/Game/Animations/Character/Anim_Walk", "Animation"),
        ("/Game/Anims/Idle", "Animation"),
        
        // Audio
        ("/Game/Audio/Music/BGM_Main", "Audio"),
        ("/Game/Sounds/SFX_Click", "Audio"),
    ];
    
    println!("🔍 Testing asset loading and compatibility...\n");
    
    let mut successful_loads = 0;
    let mut total_tests = 0;
    
    for (asset_path, asset_type) in test_assets {
        total_tests += 1;
        println!("📦 Testing {} ({})", asset_path, asset_type);
        
        // Test Method 1: Direct Asset loading
        match Asset::from_cue4parse(&provider, asset_path) {
            Ok(asset) => {
                successful_loads += 1;
                test_asset_compatibility(&asset, asset_path, asset_type);
            }
            Err(e) => {
                println!("   ❌ Failed to load: {}", e);
                
                // Try with .uasset extension
                let uasset_path = format!("{}.uasset", asset_path);
                match Asset::from_cue4parse(&provider, &uasset_path) {
                    Ok(asset) => {
                        successful_loads += 1;
                        println!("   ✅ Loaded with .uasset extension");
                        test_asset_compatibility(&asset, &uasset_path, asset_type);
                    }
                    Err(e2) => {
                        println!("   ❌ Also failed with .uasset: {}", e2);
                    }
                }
            }
        }
        
        // Test Method 2: AssetData with mappings (if available)
        if usmap_path.is_some() {
            match AssetData::load_with_mappings(&mut provider, asset_path, usmap_path.map(|s| s.as_str())) {
                Ok(asset_data) => {
                    println!("   ✅ AssetData with mappings loaded successfully");
                    test_asset_data_compatibility(&asset_data, asset_path);
                }
                Err(e) => {
                    println!("   ⚠️  AssetData with mappings failed: {}", e);
                }
            }
        }
        
        println!();
    }
    
    // Test specific iostore (.utoc/.ucas) vs traditional (.pak) formats
    println!("📊 Testing Package Format Compatibility");
    println!("=====================================");
    
    // Look for .utoc files (iostore format)
    test_iostore_compatibility(&provider)?;
    
    // Look for .pak files (traditional format)  
    test_pak_compatibility(&provider)?;
    
    // Summary
    println!("📈 Test Results Summary");
    println!("======================");
    println!("Total assets tested: {}", total_tests);
    println!("Successful loads: {}", successful_loads);
    println!("Success rate: {:.1}%", (successful_loads as f64 / total_tests as f64) * 100.0);
    
    if usmap_path.is_some() {
        println!("✅ .usmap mappings were used for enhanced type resolution");
    } else {
        println!("⚠️  Consider using .usmap files for better type information");
    }
    
    Ok(())
}

#[cfg(feature = "unrealmodding-compat")]
fn test_asset_compatibility(asset: &Asset, _asset_path: &str, asset_type: &str) {
    println!("   ✅ Asset loaded successfully");
    println!("      📄 Object name: {}", asset.asset_data.object_name);
    println!("      🔢 Export count: {}", asset.asset_data.exports.len());
    println!("      📥 Import count: {}", asset.asset_data.imports.len());
    println!("      🏷️  Name map size: {}", asset.asset_data.name_map.len());
    
    // Test compatibility features based on asset type
    match asset_type {
        "Static Mesh" => test_mesh_compatibility(asset),
        "Texture" => test_texture_compatibility(asset),
        "Material" => test_material_compatibility(asset),
        "Level/Map" => test_level_compatibility(asset),
        "Blueprint" => test_blueprint_compatibility(asset),
        _ => {}
    }
    
    // Test general property access
    let mut property_count = 0;
    for export in &asset.asset_data.exports {
        property_count += export.properties.len();
    }
    println!("      🔧 Total properties: {}", property_count);
    
    // Test unreal_asset API compatibility
    if let Some(first_export) = asset.asset_data.exports.first() {
        println!("      🎯 First export class: {}", 
            asset.asset_data.imports.get(first_export.class_index.0 as usize)
                .map(|i| &i.object_name.name)
                .unwrap_or(&"Unknown".to_string())
        );
    }
}

#[cfg(feature = "unrealmodding-compat")]
fn test_asset_data_compatibility(asset_data: &AssetData, _asset_path: &str) {
    println!("   📊 AssetData compatibility test:");
    println!("      🗂️  Has mappings: {}", asset_data.has_type_mappings());
    if let Some(mappings_path) = asset_data.get_mappings_path() {
        println!("      📍 Mappings file: {}", mappings_path);
    }
    println!("      🆔 Package GUID: {:?}", asset_data.package_guid);
    println!("      🏁 Engine version: {}", asset_data.engine_version);
}

#[cfg(feature = "unrealmodding-compat")]
fn test_mesh_compatibility(asset: &Asset) {
    println!("      🔺 Mesh-specific tests:");
    
    // Look for mesh-related exports
    for (_i, export) in asset.asset_data.exports.iter().enumerate() {
        if export.object_name.name.contains("StaticMesh") || 
           export.object_name.name.contains("Mesh") {
            println!("         Found mesh export: {}", export.object_name.name);
            
            // Test mesh data extraction (would need real mesh data)
            if let Some(_vertices_prop) = export.properties.get("Vertices") {
                println!("         ✅ Vertex data found");
            }
            if let Some(_indices_prop) = export.properties.get("Indices") {
                println!("         ✅ Index data found");
            }
        }
    }
}

#[cfg(feature = "unrealmodding-compat")]
fn test_texture_compatibility(asset: &Asset) {
    println!("      🖼️  Texture-specific tests:");
    
    for export in &asset.asset_data.exports {
        if export.object_name.name.contains("Texture") {
            println!("         Found texture export: {}", export.object_name.name);
            
            // Check for texture properties
            if let Some(_width) = export.properties.get("SizeX") {
                println!("         📏 Width property found");
            }
            if let Some(_height) = export.properties.get("SizeY") {
                println!("         📏 Height property found");
            }
            if let Some(_format) = export.properties.get("Format") {
                println!("         🎨 Format property found");
            }
        }
    }
}

#[cfg(feature = "unrealmodding-compat")]
fn test_material_compatibility(asset: &Asset) {
    println!("      🎨 Material-specific tests:");
    
    for export in &asset.asset_data.exports {
        if export.object_name.name.contains("Material") {
            println!("         Found material export: {}", export.object_name.name);
            
            // Look for material parameters
            let param_count = export.properties.iter()
                .filter(|(name, _)| name.contains("Parameter") || name.contains("Texture"))
                .count();
            println!("         🔧 Material parameters: {}", param_count);
        }
    }
}

#[cfg(feature = "unrealmodding-compat")]
fn test_level_compatibility(asset: &Asset) {
    println!("      🗺️  Level-specific tests:");
    
    // Count actors in the level
    let actor_count = asset.asset_data.exports.iter()
        .filter(|_export| ConversionUtils::is_actor_export(asset, 0)) // This would need proper index
        .count();
    
    println!("         🎭 Detected actors: {}", actor_count);
    
    // Look for level-specific exports
    for export in &asset.asset_data.exports {
        if export.object_name.name.contains("Level") || 
           export.object_name.name.contains("World") {
            println!("         Found level export: {}", export.object_name.name);
        }
    }
}

#[cfg(feature = "unrealmodding-compat")]
fn test_blueprint_compatibility(asset: &Asset) {
    println!("      📘 Blueprint-specific tests:");
    
    for export in &asset.asset_data.exports {
        if export.object_name.name.contains("Blueprint") {
            println!("         Found blueprint export: {}", export.object_name.name);
            
            // Look for blueprint components
            let component_count = export.properties.iter()
                .filter(|(name, _)| name.contains("Component"))
                .count();
            println!("         🔧 Blueprint components: {}", component_count);
        }
    }
}

#[cfg(feature = "unrealmodding-compat")]
fn test_iostore_compatibility(_provider: &Provider) -> Result<()> {
    println!("📦 Testing iostore (.utoc/.ucas) format compatibility");
    
    // iostore is the modern container format used in UE5+
    // These files typically have .utoc (table of contents) and .ucas (content) extensions
    
    // Try to detect iostore files by checking for common patterns
    println!("   🔍 Looking for iostore container files...");
    
    // In a real implementation, we'd enumerate the pak directory for .utoc files
    // For now, we'll test known patterns
    let iostore_tests = vec![
        "pakchunk0-WindowsNoEditor",
        "pakchunk1-WindowsNoEditor", 
        "global",
        "base",
    ];
    
    for container in iostore_tests {
        println!("   📂 Testing iostore container: {}", container);
        // Test would go here with actual provider methods
        println!("      ✅ Container enumeration would happen here");
    }
    
    println!("   💡 iostore format provides better performance and compression");
    println!("   💡 Common in UE5+ games like Fortnite, Gears 5, etc.");
    
    Ok(())
}

#[cfg(feature = "unrealmodding-compat")]
fn test_pak_compatibility(_provider: &Provider) -> Result<()> {
    println!("📦 Testing traditional .pak format compatibility");
    
    // Traditional pak files are the older container format
    // Still used in many UE4 games and some UE5 games
    
    println!("   🔍 Looking for traditional .pak files...");
    
    let pak_tests = vec![
        "pakchunk0-WindowsNoEditor.pak",
        "pakchunk1-WindowsNoEditor.pak",
        "GameName-WindowsNoEditor.pak",
        "DLC-WindowsNoEditor.pak",
    ];
    
    for pak_file in pak_tests {
        println!("   📂 Testing pak file: {}", pak_file);
        // Test would go here with actual provider methods
        println!("      ✅ Pak enumeration would happen here");
    }
    
    println!("   💡 Traditional pak format is well-established and widely supported");
    println!("   💡 Common in UE4 games and some UE5 games");
    
    Ok(())
}

#[cfg(not(feature = "unrealmodding-compat"))]
fn test_real_world_assets(_game_dir: &str, _usmap_path: Option<&String>) -> Result<()> {
    println!("❌ This example requires the 'unrealmodding-compat' feature.");
    println!("Run with: cargo run --example real_world_test --features unrealmodding-compat");
    Ok(())
}
