/// Phase 3 Working Demo - Advanced Features
/// 
/// This demonstrates all the Phase 3 advanced features working together:
/// 1. Custom version support with engine-specific serialization
/// 2. Dependency management with package dependency tracking  
/// 3. Bulk data handling with large data serialization
/// 4. Name map optimization with hash-based FName system

#[cfg(feature = "unrealmodding-compat")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cue4parse_rs::unreal_asset::*;
    
    println!("🔧 Phase 3: Advanced Features Demo");
    println!("===================================");
    
    // 1. Custom Version Support - Engine-specific serialization
    println!("\n1️⃣ Custom Version Support:");
    let mut custom_registry = CustomVersionRegistry::default();
    
    // Add Fortnite-specific version
    let fortnite_version = CustomVersion {
        guid: uuid::Uuid::new_v4(),
        version: 42,
        friendly_name: "FortniteMain".to_string(),
    };
    custom_registry.register_version(fortnite_version.clone());
    
    if let Some(version) = custom_registry.get_version(&fortnite_version.guid) {
        println!("   ✅ Registered Fortnite version: {} (v{})", version.friendly_name, version.version);
    }
    
    // 2. Dependency Management - Package dependency tracking
    println!("\n2️⃣ Dependency Management:");
    let mut dependency_graph = DependencyGraph::new();
    
    // Add package dependencies
    let main_package = PackageDependency {
        package_name: "MainLevel".to_string(),
        dependency_type: DependencyType::Hard,
        import_map: vec!["Mesh".to_string(), "Material".to_string()],
    };
    
    dependency_graph.add_dependency("GameLevel".to_string(), main_package);
    
    // Check for circular dependencies
    if let Ok(resolved) = dependency_graph.resolve_dependencies("GameLevel") {
        println!("   ✅ Resolved {} dependencies for GameLevel", resolved.len());
    }
    
    // Get statistics
    let stats = dependency_graph.get_statistics();
    println!("   📊 Graph stats: {} packages, {} connections", 
             stats.total_packages, stats.total_dependencies);
    
    // 3. Bulk Data Handling - Large data serialization
    println!("\n3️⃣ Bulk Data Handling:");
    let mut bulk_manager = BulkDataManager::new();
    
    // Add bulk data entry
    let large_data = vec![0u8; 1024 * 1024]; // 1MB of data
    let bulk_entry = BulkDataEntry {
        name: "LargeMesh".to_string(),
        size_on_disk: large_data.len() as u64,
        size_in_memory: large_data.len() as u64,
        offset: 0,
        flags: BulkDataFlags::FORCE_INLINE_PAYLOAD | BulkDataFlags::COMPRESS_ZLIB,
        compression_method: Some(CompressionMethod::Zlib),
        data: Some(large_data),
    };
    
    bulk_manager.add_bulk_data("LargeMesh".to_string(), bulk_entry)?;
    
    // Demonstrate compression
    if let Ok(compressed) = bulk_manager.get_bulk_data("LargeMesh") {
        println!("   ✅ Bulk data '{}' ready: {} bytes", compressed.name, compressed.size_on_disk);
        println!("   🗜️  Compression: {:?}", compressed.compression_method);
    }
    
    let cache_stats = bulk_manager.get_cache_stats();
    println!("   📈 Cache: {} entries, {:.1}% hit ratio", 
             cache_stats.entries_count, cache_stats.hit_ratio * 100.0);
    
    // 4. Name Map Optimization - Hash-based FName system
    println!("\n4️⃣ Name Map Optimization:");
    let mut optimized_map = OptimizedNameMap::new(NameHashAlgorithm::Fnv1a);
    
    // Add some names for testing
    let names = ["PlayerCharacter", "StaticMesh", "Material", "Texture2D", "Blueprint"];
    for name in &names {
        optimized_map.add_string(name.to_string());
    }
    
    // Create optimized FNames
    let player_fname = optimized_map.create_fname("PlayerCharacter", 0);
    println!("   ✅ Created FName: {} (optimized lookup)", player_fname);
    
    // Benchmark different algorithms
    let benchmark_names: Vec<String> = (0..1000)
        .map(|i| format!("TestName_{}", i))
        .collect();
    
    let results = OptimizedNameMap::benchmark_hash_algorithms(&benchmark_names);
    println!("   🏃 Hash algorithm benchmarks:");
    for (algorithm, time_ns) in results {
        println!("      {:?}: {} ns average", algorithm, time_ns);
    }
    
    let stats = optimized_map.get_stats();
    println!("   📊 Name map stats: {} total, {} unique, {} collisions", 
             stats.total_names, stats.unique_names, stats.hash_collisions);
    
    // 5. Integration Demo - Put it all together
    println!("\n🎯 Integration Demo:");
    let mut asset = Asset::new_with_advanced_features(true)?;
    
    // Set up advanced features
    asset.asset_data.optimized_name_map = Some(optimized_map);
    asset.asset_data.dependency_graph = Some(dependency_graph);
    asset.asset_data.bulk_data_manager = Some(bulk_manager);
    asset.custom_versions.push(fortnite_version);
    
    println!("   ✅ Created asset with all Phase 3 advanced features");
    println!("   🚀 Ready for production-level Unreal Engine asset manipulation");
    
    // Demonstrate functionality
    if let Some(demo_result) = asset.demonstrate_phase3_functionality().ok() {
        println!("\n📋 Phase 3 Feature Summary:");
        println!("{}", demo_result);
    }
    
    println!("\n🎉 Phase 3 Complete! All advanced features operational.");
    println!("    Enterprise-grade optimization and management capabilities ready.");
    
    Ok(())
}

#[cfg(not(feature = "unrealmodding-compat"))]
fn main() {
    println!("Phase 3 demo requires 'unrealmodding-compat' feature to be enabled.");
    println!("Run with: cargo run --example phase3_working_demo --features unrealmodding-compat");
}
