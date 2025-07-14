//! Integration tests for the unreal_asset compatibility layer
//!
//! These tests verify that the compatibility layer provides the expected
//! API surface and behavior for migrating from unreal_modding to CUE4Parse.

#[cfg(feature = "unrealmodding-compat")]
mod compatibility_tests {
    use cue4parse_rs::unreal_asset::*;
    use serde_json::json;

    #[test]
    fn test_fname_compatibility() {
        // Test FName creation and display
        let fname = FName::new("TestAsset");
        assert_eq!(fname.as_str(), "TestAsset");
        assert_eq!(fname.to_string(), "TestAsset");
        assert_eq!(fname.number, 0);
        
        let numbered_fname = FName::with_number("TestAsset", 3);
        assert_eq!(numbered_fname.to_string(), "TestAsset_3");
        assert_eq!(numbered_fname.number, 3);
        
        // Test equality
        let fname2 = FName::new("TestAsset");
        assert_eq!(fname, fname2);
        
        let fname3 = FName::with_number("TestAsset", 1);
        assert_ne!(fname, fname3);
    }
    
    #[test]
    fn test_package_index_compatibility() {
        // Null reference
        let null_ref = PackageIndex::null();
        assert!(null_ref.is_null());
        assert!(!null_ref.is_import());
        assert!(!null_ref.is_export());
        assert_eq!(null_ref.import_index(), None);
        assert_eq!(null_ref.export_index(), None);
        
        // Import reference (negative)
        let import_ref = PackageIndex(-1);
        assert!(!import_ref.is_null());
        assert!(import_ref.is_import());
        assert!(!import_ref.is_export());
        assert_eq!(import_ref.import_index(), Some(0)); // -(-1) - 1 = 0
        assert_eq!(import_ref.export_index(), None);
        
        // Export reference (positive)
        let export_ref = PackageIndex(1);
        assert!(!export_ref.is_null());
        assert!(!export_ref.is_import());
        assert!(export_ref.is_export());
        assert_eq!(export_ref.import_index(), None);
        assert_eq!(export_ref.export_index(), Some(0)); // 1 - 1 = 0
    }
    
    #[test]
    fn test_property_type_compatibility() {
        // Test all basic property types
        let bool_prop = Property::Bool(true);
        assert!(matches!(bool_prop, Property::Bool(true)));
        
        let int8_prop = Property::Int8(-5);
        assert!(matches!(int8_prop, Property::Int8(-5)));
        
        let int32_prop = Property::Int32(42);
        assert!(matches!(int32_prop, Property::Int32(42)));
        
        let float_prop = Property::Float(3.14);
        assert!(matches!(float_prop, Property::Float(f) if (f - 3.14).abs() < f32::EPSILON));
        
        let string_prop = Property::String("test".to_string());
        assert!(matches!(string_prop, Property::String(ref s) if s == "test"));
        
        let name_prop = Property::Name(FName::new("TestName"));
        if let Property::Name(fname) = &name_prop {
            assert_eq!(fname.as_str(), "TestName");
        } else {
            panic!("Expected Name property");
        }
        
        let obj_prop = Property::Object(None);
        assert!(matches!(obj_prop, Property::Object(None)));
        
        let obj_ref_prop = Property::Object(Some(PackageIndex(5)));
        assert!(matches!(obj_ref_prop, Property::Object(Some(PackageIndex(5)))));
    }
    
    #[test]
    fn test_complex_property_types() {
        use indexmap::IndexMap;
        
        // Array property
        let array_prop = Property::Array(vec![
            Property::Int32(1),
            Property::Int32(2),
            Property::String("test".to_string()),
        ]);
        
        if let Property::Array(arr) = &array_prop {
            assert_eq!(arr.len(), 3);
            assert!(matches!(arr[0], Property::Int32(1)));
            assert!(matches!(arr[1], Property::Int32(2)));
            assert!(matches!(arr[2], Property::String(ref s) if s == "test"));
        } else {
            panic!("Expected Array property");
        }
        
        // Struct property
        let mut struct_props = IndexMap::new();
        struct_props.insert("X".to_string(), Property::Float(1.0));
        struct_props.insert("Y".to_string(), Property::Float(2.0));
        
        let struct_prop = Property::Struct {
            struct_type: FName::new("Vector2D"),
            properties: struct_props,
        };
        
        if let Property::Struct { struct_type, properties } = &struct_prop {
            assert_eq!(struct_type.as_str(), "Vector2D");
            assert_eq!(properties.len(), 2);
            assert!(matches!(properties.get("X"), Some(Property::Float(f)) if (*f - 1.0).abs() < f32::EPSILON));
            assert!(matches!(properties.get("Y"), Some(Property::Float(f)) if (*f - 2.0).abs() < f32::EPSILON));
        } else {
            panic!("Expected Struct property");
        }
        
        // Map property
        let map_prop = Property::Map {
            key_type: "String".to_string(),
            value_type: "Int32".to_string(),
            entries: vec![
                (Property::String("key1".to_string()), Property::Int32(100)),
                (Property::String("key2".to_string()), Property::Int32(200)),
            ],
        };
        
        if let Property::Map { key_type, value_type, entries } = &map_prop {
            assert_eq!(key_type, "String");
            assert_eq!(value_type, "Int32");
            assert_eq!(entries.len(), 2);
        } else {
            panic!("Expected Map property");
        }
        
        // Enum property
        let enum_prop = Property::Enum {
            enum_type: FName::new("ETestEnum"),
            value: FName::new("Value1"),
        };
        
        if let Property::Enum { enum_type, value } = &enum_prop {
            assert_eq!(enum_type.as_str(), "ETestEnum");
            assert_eq!(value.as_str(), "Value1");
        } else {
            panic!("Expected Enum property");
        }
        
        // Text property
        let text_prop = Property::Text {
            text: "Hello World".to_string(),
            namespace: Some("Game".to_string()),
            key: Some("Greeting".to_string()),
        };
        
        if let Property::Text { text, namespace, key } = &text_prop {
            assert_eq!(text, "Hello World");
            assert_eq!(namespace.as_ref().unwrap(), "Game");
            assert_eq!(key.as_ref().unwrap(), "Greeting");
        } else {
            panic!("Expected Text property");
        }
    }
    
    #[test]
    fn test_asset_structure_compatibility() {
        let asset: Asset<std::io::Cursor<Vec<u8>>> = Asset::new();
        
        // Test default values
        assert_eq!(asset.asset_data.object_name, "");
        assert_eq!(asset.asset_data.engine_version, "UE5.3");
        assert_eq!(asset.asset_data.exports.len(), 0);
        assert_eq!(asset.asset_data.imports.len(), 0);
        assert_eq!(asset.asset_data.package_flags, 0);
        assert_eq!(asset.asset_data.total_header_size, 0);
        assert_eq!(asset.asset_data.name_map.len(), 0);
        assert_eq!(asset.custom_versions.len(), 0);
        assert_eq!(asset.asset_tags.len(), 0);
        
        // Test that it implements expected traits
        let _debug_str = format!("{:?}", asset);
        let _cloned = asset.clone();
        
        // Test serialization/deserialization
        let serialized = serde_json::to_string(&asset).expect("Should serialize");
        let _deserialized: Asset = serde_json::from_str(&serialized).expect("Should deserialize");
    }
    
    #[test]
    fn test_export_structure_compatibility() {
        use indexmap::IndexMap;
        
        let mut properties = IndexMap::new();
        properties.insert("TestProp".to_string(), Property::Bool(true));
        
        let export = Export {
            class_index: PackageIndex(1),
            super_index: PackageIndex::null(),
            template_index: PackageIndex::null(),
            outer_index: PackageIndex::null(),
            object_name: FName::new("TestObject"),
            object_flags: 0x12345678,
            serial_size: 1024,
            serial_offset: 512,
            export_flags: 0x00000001,
            create_before_serialization_dependencies: Vec::new(),
            properties,
            extras: Some(json!({"custom_data": "test"})),
        };
        
        assert_eq!(export.object_name.as_str(), "TestObject");
        assert_eq!(export.object_flags, 0x12345678);
        assert_eq!(export.serial_size, 1024);
        assert_eq!(export.serial_offset, 512);
        assert_eq!(export.export_flags, 0x00000001);
        assert_eq!(export.properties.len(), 1);
        assert!(export.properties.contains_key("TestProp"));
        assert!(export.extras.is_some());
        
        // Test serialization
        let _serialized = serde_json::to_string(&export).expect("Should serialize");
    }
    
    #[test]
    fn test_import_structure_compatibility() {
        let import = Import {
            class_package: FName::new("CoreUObject"),
            class_name: FName::new("Class"),
            outer_index: PackageIndex::null(),
            object_name: FName::new("TestClass"),
            package_name: FName::new("TestPackage"),
            package_guid: Some(uuid::Uuid::new_v4()),
        };
        
        assert_eq!(import.class_package.as_str(), "CoreUObject");
        assert_eq!(import.class_name.as_str(), "Class");
        assert_eq!(import.object_name.as_str(), "TestClass");
        assert!(import.package_guid.is_some());
        
        // Test serialization
        let _serialized = serde_json::to_string(&import).expect("Should serialize");
    }
    
    #[test]
    fn test_json_to_property_conversion() {
        // Test basic JSON to Property conversion
        let bool_json = json!(true);
        let prop = Asset::<std::io::Cursor<Vec<u8>>>::json_to_property(&bool_json, None);
        assert!(matches!(prop, Property::Bool(true)));
        
        let number_json = json!(42);
        let prop = Asset::<std::io::Cursor<Vec<u8>>>::json_to_property(&number_json, None);
        assert!(matches!(prop, Property::Int32(42)));
        
        let large_number_json = json!(9223372036854775807i64);
        let prop = Asset::<std::io::Cursor<Vec<u8>>>::json_to_property(&large_number_json, None);
        assert!(matches!(prop, Property::Int64(9223372036854775807)));
        
        let float_json = json!(3.14159);
        let prop = Asset::json_to_property(&float_json, None);
        assert!(matches!(prop, Property::Double(f) if (f - 3.14159).abs() < f64::EPSILON));
        
        let string_json = json!("test_string");
        let prop = Asset::json_to_property(&string_json, None);
        assert!(matches!(prop, Property::String(ref s) if s == "test_string"));
        
        let name_json = json!("test_name");
        let prop = Asset::json_to_property(&name_json, Some("Name"));
        if let Property::Name(fname) = prop {
            assert_eq!(fname.as_str(), "test_name");
        } else {
            panic!("Expected Name property");
        }
        
        let array_json = json!([1, 2, 3]);
        let prop = Asset::json_to_property(&array_json, None);
        if let Property::Array(arr) = prop {
            assert_eq!(arr.len(), 3);
            assert!(matches!(arr[0], Property::Int32(1)));
            assert!(matches!(arr[1], Property::Int32(2)));
            assert!(matches!(arr[2], Property::Int32(3)));
        } else {
            panic!("Expected Array property");
        }
        
        let struct_json = json!({
            "$type": "Vector",
            "X": 1.0,
            "Y": 2.0,
            "Z": 3.0
        });
        let prop = Asset::json_to_property(&struct_json, None);
        if let Property::Struct { struct_type, properties } = prop {
            assert_eq!(struct_type.as_str(), "Vector");
            assert_eq!(properties.len(), 3);
            assert!(properties.contains_key("X"));
            assert!(properties.contains_key("Y"));
            assert!(properties.contains_key("Z"));
        } else {
            panic!("Expected Struct property");
        }
        
        let null_json = json!(null);
        let prop = Asset::json_to_property(&null_json, None);
        assert!(matches!(prop, Property::Object(None)));
    }
    
    #[test]
    fn test_conversion_utils() {
        // Test game version conversion
        let version_str = ConversionUtils::game_version_to_string(&cue4parse_rs::GameVersion::UE5_3);
        assert_eq!(version_str, "5.3.0");
        
        let version_str = ConversionUtils::game_version_to_string(&cue4parse_rs::GameVersion::UE4_27);
        assert_eq!(version_str, "4.27.0");
        
        let version_str = ConversionUtils::game_version_to_string(&cue4parse_rs::GameVersion::UE5_0);
        assert_eq!(version_str, "5.0.0");
        
        // Test typed property conversion
        let bool_json = json!(true);
        let prop = ConversionUtils::json_to_property_typed(&bool_json, "BoolProperty");
        assert!(matches!(prop, Property::Bool(true)));
        
        let int_json = json!(42);
        let prop = ConversionUtils::json_to_property_typed(&int_json, "IntProperty");
        assert!(matches!(prop, Property::Int32(42)));
        
        let int64_json = json!(9223372036854775807i64);
        let prop = ConversionUtils::json_to_property_typed(&int64_json, "Int64Property");
        assert!(matches!(prop, Property::Int64(9223372036854775807)));
        
        let float_json = json!(3.14);
        let prop = ConversionUtils::json_to_property_typed(&float_json, "FloatProperty");
        assert!(matches!(prop, Property::Float(f) if (f - 3.14).abs() < 0.01));
        
        let double_json = json!(3.14159265359);
        let prop = ConversionUtils::json_to_property_typed(&double_json, "DoubleProperty");
        assert!(matches!(prop, Property::Double(f) if (f - 3.14159265359).abs() < f64::EPSILON));
        
        let str_json = json!("test_string");
        let prop = ConversionUtils::json_to_property_typed(&str_json, "StrProperty");
        if let Property::String(s) = prop {
            assert_eq!(s, "test_string");
        } else {
            panic!("Expected String property");
        }
        
        let name_json = json!("test_name");
        let prop = ConversionUtils::json_to_property_typed(&name_json, "NameProperty");
        if let Property::Name(fname) = prop {
            assert_eq!(fname.as_str(), "test_name");
        } else {
            panic!("Expected Name property");
        }
        
        let null_obj_json = json!(null);
        let prop = ConversionUtils::json_to_property_typed(&null_obj_json, "ObjectProperty");
        assert!(matches!(prop, Property::Object(None)));
    }
    
    #[test]
    fn test_property_metadata_extraction() {
        let test_json = json!({
            "$types": {
                "TestProperty": "BoolProperty",
                "AnotherProperty": "IntProperty",
                "StringProp": "StrProperty"
            },
            "TestProperty": true,
            "AnotherProperty": 42,
            "StringProp": "hello"
        });
        
        let bool_type = ConversionUtils::extract_property_type(&test_json, "TestProperty");
        assert_eq!(bool_type, Some("BoolProperty".to_string()));
        
        let int_type = ConversionUtils::extract_property_type(&test_json, "AnotherProperty");
        assert_eq!(int_type, Some("IntProperty".to_string()));
        
        let str_type = ConversionUtils::extract_property_type(&test_json, "StringProp");
        assert_eq!(str_type, Some("StrProperty".to_string()));
        
        let missing_type = ConversionUtils::extract_property_type(&test_json, "NonExistent");
        assert_eq!(missing_type, None);
    }
    
    #[test]
    fn test_asset_helper_methods() {
        use indexmap::IndexMap;
        
        let mut asset: Asset<std::io::Cursor<Vec<u8>>> = Asset::new();
        
        // Add some test exports
        let mut props1 = IndexMap::new();
        props1.insert("TestProp".to_string(), Property::Bool(true));
        
        let export1 = Export {
            class_index: PackageIndex::null(),
            super_index: PackageIndex::null(),
            template_index: PackageIndex::null(),
            outer_index: PackageIndex::null(),
            object_name: FName::new("TestExport1"),
            object_flags: 0,
            serial_size: 0,
            serial_offset: 0,
            export_flags: 0,
            create_before_serialization_dependencies: Vec::new(),
            properties: props1,
            extras: None,
        };
        
        let mut props2 = IndexMap::new();
        props2.insert("AnotherProp".to_string(), Property::Int32(42));
        
        let export2 = Export {
            class_index: PackageIndex::null(),
            super_index: PackageIndex::null(),
            template_index: PackageIndex::null(),
            outer_index: PackageIndex::null(),
            object_name: FName::new("TestExport2"),
            object_flags: 0,
            serial_size: 0,
            serial_offset: 0,
            export_flags: 0,
            create_before_serialization_dependencies: Vec::new(),
            properties: props2,
            extras: None,
        };
        
        asset.asset_data.exports.push(export1);
        asset.asset_data.exports.push(export2);
        
        // Test get_export_by_name
        let found_export = asset.get_export_by_name("TestExport1");
        assert!(found_export.is_some());
        assert_eq!(found_export.unwrap().object_name.as_str(), "TestExport1");
        
        let not_found_export = asset.get_export_by_name("NonExistent");
        assert!(not_found_export.is_none());
        
        // Test get_main_export
        let main_export = asset.get_main_export();
        assert!(main_export.is_some());
        assert_eq!(main_export.unwrap().object_name.as_str(), "TestExport1");
        
        // Add test imports
        let import1 = Import {
            class_package: FName::new("CoreUObject"),
            class_name: FName::new("Class"),
            outer_index: PackageIndex::null(),
            object_name: FName::new("TestImport1"),
            package_name: FName::new("TestPackage"),
            package_guid: None,
        };
        
        asset.asset_data.imports.push(import1);
        
        // Test get_import_by_name
        let found_import = asset.get_import_by_name("TestImport1");
        assert!(found_import.is_some());
        assert_eq!(found_import.unwrap().object_name.as_str(), "TestImport1");
        
        let not_found_import = asset.get_import_by_name("NonExistent");
        assert!(not_found_import.is_none());
    }
}

#[cfg(not(feature = "unrealmodding-compat"))]
mod feature_disabled_tests {
    #[test]
    fn test_feature_disabled() {
        // When the feature is disabled, the module should not be available
        // This test just ensures the crate compiles without the feature
        assert!(true);
    }
}
