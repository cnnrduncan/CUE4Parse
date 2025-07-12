use cue4parse_rs::{Provider, GameVersion, is_feature_available};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CUE4Parse Rust FFI Example");
    
    // Check available features from native library (if compiled with native-lib feature)
    println!("Available features:");
    println!("  ACL: {}", is_feature_available("ACL"));
    println!("  Oodle: {}", is_feature_available("Oodle"));
    
    // Create a provider for your game files
    let mut provider = Provider::new("path/to/game/files", GameVersion::UE5_3);
    
    // Set the path to CUE4Parse.CLI.exe if it's not in the default location
    // provider.set_executable_path("C:/path/to/CUE4Parse.CLI.exe");
    
    // Add encryption keys if needed
    provider.add_key("your-key-guid", "your-aes-key");
    
    // Set mappings for better parsing
    provider.set_mappings("path/to/mappings.usmap");
    
    // Example operations (commented out since they need real game files):
    
    // List all packages
    // let packages = provider.list_packages()?;
    // println!("Found {} packages", packages.len());
    // for package in packages.iter().take(5) {
    //     println!("Package: {}", package);
    // }
    
    // Load package information
    // let package_info = provider.load_package("GameAssets/SomePackage.uasset")?;
    // println!("Package '{}' has {} exports", package_info.name, package_info.exports.len());
    // for export in package_info.exports.iter().take(3) {
    //     println!("  Export: {} ({})", export.name, export.class_name);
    // }
    
    // Export an object to JSON
    // let json = provider.export_object_json("GameAssets/SomeObject")?;
    // println!("Object JSON: {}", serde_json::to_string_pretty(&json)?);
    
    // Export an object to a file
    // provider.export_object("GameAssets/SomeTexture", "output.png", "png")?;
    // println!("Exported texture to output.png");
    
    println!("Example completed successfully!");
    Ok(())
}
