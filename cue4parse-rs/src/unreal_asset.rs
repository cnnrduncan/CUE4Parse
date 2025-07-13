//! # Unreal Asset Compatibility Layer
//!
//! This module provides compatibility with the `unreal_asset` module from the `unreal_modding` crate.
//! It allows users to convert CUE4Parse results into structures compatible with `unreal_asset` APIs,
//! providing a smooth migration path for projects already using `unreal_modding`.
//!
//! ## Features
//!
//! - **Asset Structure**: Compatible `Asset` struct with similar field layout
//! - **Property System**: Convert CUE4Parse JSON to property structures
//! - **Import/Export Tables**: Map CUE4Parse package data to import/export tables
//! - **Type Compatibility**: Similar type definitions for seamless migration
//! - **Trait Implementations**: Compatible traits for serialization and debugging
//!
//! ## Migration Guide
//!
//! ### Before (unreal_modding)
//! ```rust,ignore
//! use unreal_modding::unreal_asset::{Asset, read};
//!
//! let mut reader = std::io::Cursor::new(asset_data);
//! let asset = read(&mut reader, &engine_version, None)?;
//! println!("Asset name: {}", asset.asset_data.object_name);
//! ```
//!
//! ### After (CUE4Parse with compatibility)
//! ```rust,ignore
//! use cue4parse_rs::{Provider, GameVersion};
//! use cue4parse_rs::unreal_asset::{Asset, UnrealAssetCompat};
//!
//! let provider = Provider::new("/path/to/game", GameVersion::UE5_3);
//! let asset = Asset::from_cue4parse(&provider, "MyAsset.MyAsset")?;
//! println!("Asset name: {}", asset.asset_data.object_name);
//! ```
//!
//! ## Example Usage
//!
//! ```no_run
//! use cue4parse_rs::{Provider, GameVersion, Result};
//! # #[cfg(feature = "unrealmodding-compat")]
//! use cue4parse_rs::unreal_asset::{Asset, Property, UnrealAssetCompat};
//!
//! # #[cfg(feature = "unrealmodding-compat")]
//! fn migrate_from_unreal_asset() -> Result<()> {
//!     let provider = Provider::new("/path/to/fortnite", GameVersion::UE5_3);
//!     
//!     // Load asset using CUE4Parse but get unreal_asset-compatible structure
//!     let asset = Asset::from_cue4parse(&provider, "MyAsset.MyAsset")?;
//!     
//!     // Use existing unreal_asset-compatible APIs
//!     println!("Asset class: {}", asset.asset_data.object_name);
//!     println!("Export count: {}", asset.asset_data.exports.len());
//!     
//!     // Access properties in familiar format
//!     for export in &asset.asset_data.exports {
//!         for (name, property) in &export.properties {
//!             println!("Property {}: {:?}", name, property);
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```

#[cfg(feature = "unrealmodding-compat")]
use indexmap::IndexMap;
#[cfg(feature = "unrealmodding-compat")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "unrealmodding-compat")]
use std::collections::HashMap;
#[cfg(feature = "unrealmodding-compat")]
use uuid::Uuid;

#[cfg(feature = "unrealmodding-compat")]
use crate::{Provider, Result};

/// Compatibility trait for unreal_asset migration
/// 
/// This trait provides methods for converting CUE4Parse data structures
/// to formats compatible with the unreal_asset APIs.
#[cfg(feature = "unrealmodding-compat")]
pub trait UnrealAssetCompat {
    /// Create from CUE4Parse provider and object path
    fn from_cue4parse(provider: &Provider, object_path: &str) -> Result<Self>
    where
        Self: Sized;
}

/// FName-like structure for compatibility
/// 
/// Represents an Unreal Engine FName, which is an indexed string used
/// throughout the engine for performance reasons.
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FName {
    /// The string value of this name
    pub name: String,
    /// Optional numeric suffix for duplicate names
    pub number: u32,
}

#[cfg(feature = "unrealmodding-compat")]
impl FName {
    /// Create a new FName from a string
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            number: 0,
        }
    }
    
    /// Create an FName with a specific number suffix
    pub fn with_number(name: impl Into<String>, number: u32) -> Self {
        Self {
            name: name.into(),
            number,
        }
    }
    
    /// Get the string representation
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl std::fmt::Display for FName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.number == 0 {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{}_{}", self.name, self.number)
        }
    }
}

/// Property value types compatible with unreal_asset
/// 
/// Represents the different types of properties that can exist in Unreal Engine assets.
/// This enum provides compatibility with the unreal_asset property system.
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Property {
    /// Boolean property
    Bool(bool),
    /// 8-bit signed integer
    Int8(i8),
    /// 16-bit signed integer  
    Int16(i16),
    /// 32-bit signed integer
    Int32(i32),
    /// 64-bit signed integer
    Int64(i64),
    /// 8-bit unsigned integer
    UInt8(u8),
    /// 16-bit unsigned integer
    UInt16(u16),
    /// 32-bit unsigned integer
    UInt32(u32),
    /// 64-bit unsigned integer
    UInt64(u64),
    /// 32-bit floating point
    Float(f32),
    /// 64-bit floating point
    Double(f64),
    /// String property
    String(String),
    /// Name property (indexed string)
    Name(FName),
    /// Object reference property
    Object(Option<PackageIndex>),
    /// Struct property containing nested properties
    Struct {
        /// Name of the struct type
        struct_type: FName,
        /// Nested properties
        properties: IndexMap<String, Property>,
    },
    /// Array of properties
    Array(Vec<Property>),
    /// Map of key-value pairs
    Map {
        /// Map key type
        key_type: String,
        /// Map value type  
        value_type: String,
        /// Map entries
        entries: Vec<(Property, Property)>,
    },
    /// Enum property
    Enum {
        /// Enum type name
        enum_type: FName,
        /// Selected enum value
        value: FName,
    },
    /// Text property (localized string)
    Text {
        /// Text content
        text: String,
        /// Text namespace
        namespace: Option<String>,
        /// Text key
        key: Option<String>,
    },
    /// Unknown or unsupported property type
    Unknown(serde_json::Value),
}

/// Package index for object references
/// 
/// References to objects within packages, compatible with unreal_asset's indexing system.
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PackageIndex(pub i32);

#[cfg(feature = "unrealmodding-compat")]
impl PackageIndex {
    /// Create a null reference
    pub fn null() -> Self {
        Self(0)
    }
    
    /// Check if this is a null reference
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
    
    /// Check if this references an import (negative index)
    pub fn is_import(&self) -> bool {
        self.0 < 0
    }
    
    /// Check if this references an export (positive index)
    pub fn is_export(&self) -> bool {
        self.0 > 0
    }
    
    /// Get the import index (converts to positive)
    pub fn import_index(&self) -> Option<usize> {
        if self.is_import() {
            Some((-self.0 - 1) as usize)
        } else {
            None
        }
    }
    
    /// Get the export index (converts to zero-based)
    pub fn export_index(&self) -> Option<usize> {
        if self.is_export() {
            Some((self.0 - 1) as usize)
        } else {
            None
        }
    }
}

/// Import table entry
/// 
/// Represents an object that this package imports from another package.
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    /// The class of the imported object
    pub class_package: FName,
    /// The class name
    pub class_name: FName,
    /// Index of the outer object
    pub outer_index: PackageIndex,
    /// The name of the imported object
    pub object_name: FName,
    /// Optional package GUID
    pub package_guid: Option<Uuid>,
}

/// Export table entry
/// 
/// Represents an object that this package exports to other packages.
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    /// Index of the class for this export
    pub class_index: PackageIndex,
    /// Index of the super class
    pub super_index: PackageIndex,
    /// Index of the template/archetype object
    pub template_index: PackageIndex,
    /// Index of the outer object
    pub outer_index: PackageIndex,
    /// The name of this export
    pub object_name: FName,
    /// Object flags
    pub object_flags: u32,
    /// Serial size of the object data
    pub serial_size: u64,
    /// Serial offset in the package file
    pub serial_offset: u64,
    /// Export flags
    pub export_flags: u32,
    /// Properties of this object
    pub properties: IndexMap<String, Property>,
    /// Additional export data (varies by object type)
    pub extras: Option<serde_json::Value>,
}

/// Asset data structure compatible with unreal_asset
/// 
/// Main container for all package/asset information, designed to be compatible
/// with the Asset struct from the unreal_asset crate.
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetData {
    /// Engine version information
    pub engine_version: String,
    /// Package name
    pub object_name: String,
    /// Package GUID
    pub package_guid: Option<Uuid>,
    /// Import table
    pub imports: Vec<Import>,
    /// Export table
    pub exports: Vec<Export>,
    /// Package flags
    pub package_flags: u32,
    /// Total header size
    pub total_header_size: u32,
    /// Name map (indexed strings used in the package)
    pub name_map: Vec<String>,
    /// Additional package metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Main asset structure compatible with unreal_asset
/// 
/// Top-level structure representing a complete Unreal Engine asset/package,
/// designed for compatibility with the unreal_asset crate's Asset struct.
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    /// Core asset data
    pub asset_data: AssetData,
    /// Custom version information
    pub custom_versions: HashMap<String, i32>,
    /// Asset registry tags
    pub asset_tags: HashMap<String, String>,
}

#[cfg(feature = "unrealmodding-compat")]
impl Asset {
    /// Create a new empty asset
    pub fn new() -> Self {
        Self {
            asset_data: AssetData {
                engine_version: "UE5.3".to_string(),
                object_name: String::new(),
                package_guid: None,
                imports: Vec::new(),
                exports: Vec::new(),
                package_flags: 0,
                total_header_size: 0,
                name_map: Vec::new(),
                metadata: HashMap::new(),
            },
            custom_versions: HashMap::new(),
            asset_tags: HashMap::new(),
        }
    }
    
    /// Get an export by name
    pub fn get_export_by_name(&self, name: &str) -> Option<&Export> {
        self.asset_data.exports.iter().find(|e| e.object_name.name == name)
    }
    
    /// Get an import by name
    pub fn get_import_by_name(&self, name: &str) -> Option<&Import> {
        self.asset_data.imports.iter().find(|i| i.object_name.name == name)
    }
    
    /// Get the main export (usually the first one)
    pub fn get_main_export(&self) -> Option<&Export> {
        self.asset_data.exports.first()
    }
    
    /// Convert a CUE4Parse JSON value to a Property
    pub fn json_to_property(value: &serde_json::Value, property_type: Option<&str>) -> Property {
        match value {
            serde_json::Value::Bool(b) => Property::Bool(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                        Property::Int32(i as i32)
                    } else {
                        Property::Int64(i)
                    }
                } else if let Some(f) = n.as_f64() {
                    Property::Double(f)
                } else {
                    Property::Unknown(value.clone())
                }
            },
            serde_json::Value::String(s) => {
                if let Some("Name") = property_type {
                    Property::Name(FName::new(s.clone()))
                } else {
                    Property::String(s.clone())
                }
            },
            serde_json::Value::Array(arr) => {
                let properties: Vec<Property> = arr.iter()
                    .map(|v| Self::json_to_property(v, None))
                    .collect();
                Property::Array(properties)
            },
            serde_json::Value::Object(obj) => {
                // Check if this looks like a struct
                if obj.contains_key("$type") || obj.len() > 1 {
                    let mut properties = IndexMap::new();
                    for (key, val) in obj {
                        if key != "$type" {
                            properties.insert(key.clone(), Self::json_to_property(val, None));
                        }
                    }
                    
                    let struct_type = obj.get("$type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Struct");
                    
                    Property::Struct {
                        struct_type: FName::new(struct_type),
                        properties,
                    }
                } else {
                    Property::Unknown(value.clone())
                }
            },
            serde_json::Value::Null => Property::Object(None),
        }
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl Default for Asset {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl UnrealAssetCompat for Asset {
    /// Create an Asset from CUE4Parse data
    /// 
    /// This method loads an object using CUE4Parse and converts it to an Asset
    /// structure compatible with the unreal_asset crate.
    /// 
    /// # Arguments
    /// * `provider` - The CUE4Parse provider to use for loading
    /// * `object_path` - Full path to the object to load
    /// 
    /// # Returns
    /// An Asset structure with data converted from CUE4Parse JSON output
    /// 
    /// # Example
    /// ```no_run
    /// use cue4parse_rs::{Provider, GameVersion};
    /// use cue4parse_rs::unreal_asset::{Asset, UnrealAssetCompat};
    /// 
    /// # #[cfg(feature = "unrealmodding-compat")]
    /// # fn example() -> cue4parse_rs::Result<()> {
    /// let provider = Provider::new("/path/to/game", GameVersion::UE5_3);
    /// let asset = Asset::from_cue4parse(&provider, "MyAsset.MyAsset")?;
    /// 
    /// println!("Loaded asset: {}", asset.asset_data.object_name);
    /// # Ok(())
    /// # }
    /// ```
    fn from_cue4parse(provider: &Provider, object_path: &str) -> Result<Self> {
        // First get package info to build import/export tables
        let package_path = if let Some(pos) = object_path.find('.') {
            &object_path[..pos]
        } else {
            object_path
        };
        
        let package_info = provider.load_package(package_path)?;
        
        // Load the main object data
        let json_data = provider.export_object_json(object_path)?;
        
        let mut asset = Asset::new();
        
        // Set basic asset info
        asset.asset_data.object_name = package_info.name.clone();
        asset.asset_data.engine_version = match provider.config.game_version {
            crate::GameVersion::UE4_0 => "UE4.0",
            crate::GameVersion::UE4_27 => "UE4.27", 
            crate::GameVersion::UE5_0 => "UE5.0",
            crate::GameVersion::UE5_1 => "UE5.1",
            crate::GameVersion::UE5_2 => "UE5.2",
            crate::GameVersion::UE5_3 => "UE5.3",
            crate::GameVersion::UE5_4 => "UE5.4",
            crate::GameVersion::UE5_5 => "UE5.5",
        }.to_string();
        
        // Convert package exports to Asset exports
        for (_index, export_info) in package_info.exports.iter().enumerate() {
            let mut export = Export {
                class_index: PackageIndex::null(), // We'd need more data to resolve this properly
                super_index: PackageIndex::null(),
                template_index: PackageIndex::null(),
                outer_index: PackageIndex(export_info.outer_index),
                object_name: FName::new(&export_info.name),
                object_flags: 0, // Default flags
                serial_size: 0,  // Would need raw package data
                serial_offset: 0, // Would need raw package data  
                export_flags: 0,  // Default flags
                properties: IndexMap::new(),
                extras: None,
            };
            
            // If this is the main object, add its properties
            if export_info.name == object_path.split('.').last().unwrap_or("") {
                if let serde_json::Value::Object(properties) = &json_data {
                    for (key, value) in properties {
                        if !key.starts_with('$') { // Skip metadata fields
                            export.properties.insert(
                                key.clone(),
                                Self::json_to_property(value, None)
                            );
                        }
                    }
                }
                
                // Store raw JSON data in extras for advanced use cases
                export.extras = Some(json_data.clone());
            }
            
            asset.asset_data.exports.push(export);
        }
        
        // Build name map from object names
        let mut names = std::collections::HashSet::new();
        for export in &package_info.exports {
            names.insert(export.name.clone());
            names.insert(export.class_name.clone());
        }
        asset.asset_data.name_map = names.into_iter().collect();
        
        Ok(asset)
    }
}

/// Conversion utilities for unreal_asset compatibility
/// 
/// Provides helper functions for converting between CUE4Parse data formats
/// and unreal_asset compatible structures.
#[cfg(feature = "unrealmodding-compat")]
pub struct ConversionUtils;

#[cfg(feature = "unrealmodding-compat")]
impl ConversionUtils {
    /// Convert a CUE4Parse GameVersion to a version string
    pub fn game_version_to_string(version: &crate::GameVersion) -> String {
        match version {
            crate::GameVersion::UE4_0 => "4.0.0".to_string(),
            crate::GameVersion::UE4_27 => "4.27.0".to_string(),
            crate::GameVersion::UE5_0 => "5.0.0".to_string(),
            crate::GameVersion::UE5_1 => "5.1.0".to_string(),
            crate::GameVersion::UE5_2 => "5.2.0".to_string(),
            crate::GameVersion::UE5_3 => "5.3.0".to_string(),
            crate::GameVersion::UE5_4 => "5.4.0".to_string(),
            crate::GameVersion::UE5_5 => "5.5.0".to_string(),
        }
    }
    
    /// Convert a JSON value to a Property with type hints
    pub fn json_to_property_typed(value: &serde_json::Value, type_hint: &str) -> Property {
        match type_hint {
            "BoolProperty" => Property::Bool(value.as_bool().unwrap_or(false)),
            "IntProperty" => Property::Int32(value.as_i64().unwrap_or(0) as i32),
            "Int64Property" => Property::Int64(value.as_i64().unwrap_or(0)),
            "FloatProperty" => Property::Float(value.as_f64().unwrap_or(0.0) as f32),
            "DoubleProperty" => Property::Double(value.as_f64().unwrap_or(0.0)),
            "StrProperty" => Property::String(value.as_str().unwrap_or("").to_string()),
            "NameProperty" => Property::Name(FName::new(value.as_str().unwrap_or(""))),
            "ObjectProperty" => {
                if value.is_null() {
                    Property::Object(None)
                } else {
                    // Would need more context to resolve object references properly
                    Property::Object(Some(PackageIndex::null()))
                }
            },
            _ => Asset::json_to_property(value, Some(type_hint)),
        }
    }
    
    /// Extract property type from CUE4Parse metadata
    pub fn extract_property_type(json: &serde_json::Value, property_name: &str) -> Option<String> {
        json.get("$types")
            .and_then(|types| types.get(property_name))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
    }
}

/// Migration helper macros
/// 
/// Provides macros to ease migration from unreal_asset to CUE4Parse.
#[cfg(feature = "unrealmodding-compat")]
pub mod migration {
    /// Create an Asset from CUE4Parse with error handling
    /// 
    /// # Example
    /// ```rust,ignore
    /// use cue4parse_rs::unreal_asset::migration::load_asset;
    /// 
    /// let asset = load_asset!(provider, "MyAsset.MyAsset")?;
    /// ```
    #[macro_export]
    macro_rules! load_asset {
        ($provider:expr, $path:expr) => {
            crate::unreal_asset::Asset::from_cue4parse($provider, $path)
        };
    }
    
    /// Access properties with type checking
    /// 
    /// # Example
    /// ```rust,ignore
    /// use cue4parse_rs::unreal_asset::migration::get_property;
    /// 
    /// let value = get_property!(export, "MyProperty", String);
    /// ```
    #[macro_export]
    macro_rules! get_property {
        ($export:expr, $name:expr, String) => {
            $export.properties.get($name)
                .and_then(|p| match p {
                    crate::unreal_asset::Property::String(s) => Some(s.clone()),
                    _ => None,
                })
        };
        ($export:expr, $name:expr, i32) => {
            $export.properties.get($name)
                .and_then(|p| match p {
                    crate::unreal_asset::Property::Int32(i) => Some(*i),
                    _ => None,
                })
        };
        ($export:expr, $name:expr, bool) => {
            $export.properties.get($name)
                .and_then(|p| match p {
                    crate::unreal_asset::Property::Bool(b) => Some(*b),
                    _ => None,
                })
        };
    }
}

#[cfg(all(test, feature = "unrealmodding-compat"))]
mod tests {
    use super::*;
    use crate::GameVersion;

    #[test]
    fn test_fname_creation() {
        let fname = FName::new("TestName");
        assert_eq!(fname.as_str(), "TestName");
        assert_eq!(fname.number, 0);
        assert_eq!(fname.to_string(), "TestName");
        
        let fname_with_number = FName::with_number("TestName", 5);
        assert_eq!(fname_with_number.to_string(), "TestName_5");
    }
    
    #[test]
    fn test_package_index() {
        let null_ref = PackageIndex::null();
        assert!(null_ref.is_null());
        assert!(!null_ref.is_import());
        assert!(!null_ref.is_export());
        
        let import_ref = PackageIndex(-5);
        assert!(import_ref.is_import());
        assert_eq!(import_ref.import_index(), Some(4)); // -(-5) - 1 = 4
        
        let export_ref = PackageIndex(3);
        assert!(export_ref.is_export());
        assert_eq!(export_ref.export_index(), Some(2)); // 3 - 1 = 2
    }
    
    #[test]
    fn test_property_creation() {
        let bool_prop = Property::Bool(true);
        assert!(matches!(bool_prop, Property::Bool(true)));
        
        let string_prop = Property::String("test".to_string());
        assert!(matches!(string_prop, Property::String(_)));
        
        let name_prop = Property::Name(FName::new("TestName"));
        if let Property::Name(fname) = &name_prop {
            assert_eq!(fname.as_str(), "TestName");
        }
    }
    
    #[test]
    fn test_asset_creation() {
        let asset = Asset::new();
        assert_eq!(asset.asset_data.object_name, "");
        assert_eq!(asset.asset_data.exports.len(), 0);
        assert_eq!(asset.asset_data.imports.len(), 0);
    }
    
    #[test]
    fn test_json_to_property_conversion() {
        use serde_json::json;
        
        let bool_json = json!(true);
        let prop = Asset::json_to_property(&bool_json, None);
        assert!(matches!(prop, Property::Bool(true)));
        
        let string_json = json!("test_string");
        let prop = Asset::json_to_property(&string_json, Some("Name"));
        if let Property::Name(fname) = prop {
            assert_eq!(fname.as_str(), "test_string");
        } else {
            panic!("Expected Name property");
        }
        
        let number_json = json!(42);
        let prop = Asset::json_to_property(&number_json, None);
        assert!(matches!(prop, Property::Int32(42)));
        
        let array_json = json!([1, 2, 3]);
        let prop = Asset::json_to_property(&array_json, None);
        if let Property::Array(arr) = prop {
            assert_eq!(arr.len(), 3);
        } else {
            panic!("Expected Array property");
        }
    }
    
    #[test]
    fn test_conversion_utils() {
        let version_str = ConversionUtils::game_version_to_string(&GameVersion::UE5_3);
        assert_eq!(version_str, "5.3.0");
        
        let version_str = ConversionUtils::game_version_to_string(&GameVersion::UE4_27);
        assert_eq!(version_str, "4.27.0");
    }
    
    #[test]
    fn test_property_type_conversion() {
        use serde_json::json;
        
        let bool_val = json!(true);
        let prop = ConversionUtils::json_to_property_typed(&bool_val, "BoolProperty");
        assert!(matches!(prop, Property::Bool(true)));
        
        let int_val = json!(42);
        let prop = ConversionUtils::json_to_property_typed(&int_val, "IntProperty");
        assert!(matches!(prop, Property::Int32(42)));
        
        let str_val = json!("test");
        let prop = ConversionUtils::json_to_property_typed(&str_val, "StrProperty");
        if let Property::String(s) = prop {
            assert_eq!(s, "test");
        } else {
            panic!("Expected String property");
        }
    }
}
