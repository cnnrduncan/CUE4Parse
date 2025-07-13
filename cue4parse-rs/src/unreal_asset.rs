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

/// 3D Vector structure compatible with unreal_asset
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[cfg(feature = "unrealmodding-compat")]
impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

/// 4D Vector structure compatible with unreal_asset
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[cfg(feature = "unrealmodding-compat")]
impl Vector4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl Default for Vector {
    fn default() -> Self {
        Self::zero()
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl Default for Vector4 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl Default for Vector2D {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl Default for Rotator {
    fn default() -> Self {
        Self::zero()
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl Default for Quat {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl Default for LinearColor {
    fn default() -> Self {
        Self::white()
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl Default for SoftObjectPath {
    fn default() -> Self {
        Self::new("", "")
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl Default for StaticMeshData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "unrealmodding-compat")]
impl Default for MaterialData {
    fn default() -> Self {
        Self::new("Unknown".to_string(), "Material".to_string())
    }
}

/// 2D Vector structure compatible with unreal_asset
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

#[cfg(feature = "unrealmodding-compat")]
impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Rotator structure for rotation representation
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rotator {
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}

#[cfg(feature = "unrealmodding-compat")]
impl Rotator {
    pub fn new(pitch: f64, yaw: f64, roll: f64) -> Self {
        Self { pitch, yaw, roll }
    }
    
    pub fn zero() -> Self {
        Self { pitch: 0.0, yaw: 0.0, roll: 0.0 }
    }
}

/// Quaternion structure for rotation representation
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[cfg(feature = "unrealmodding-compat")]
impl Quat {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
    
    pub fn identity() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }
}

/// Color structure
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[cfg(feature = "unrealmodding-compat")]
impl LinearColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn white() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
    
    pub fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
}

/// Transform structure combining location, rotation, and scale
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub location: Vector,
    pub rotation: Quat,
    pub scale: Vector,
}

#[cfg(feature = "unrealmodding-compat")]
impl Transform {
    pub fn new(location: Vector, rotation: Quat, scale: Vector) -> Self {
        Self { location, rotation, scale }
    }
    
    pub fn identity() -> Self {
        Self {
            location: Vector::zero(),
            rotation: Quat::identity(),
            scale: Vector::new(1.0, 1.0, 1.0),
        }
    }
}

/// Soft object path for referencing assets
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoftObjectPath {
    pub asset_path: FName,
    pub sub_path: String,
}

#[cfg(feature = "unrealmodding-compat")]
impl SoftObjectPath {
    pub fn new(asset_path: impl Into<String>, sub_path: impl Into<String>) -> Self {
        Self {
            asset_path: FName::new(asset_path),
            sub_path: sub_path.into(),
        }
    }
    
    pub fn is_null(&self) -> bool {
        self.asset_path.name.is_empty() && self.sub_path.is_empty()
    }
}

/// Property value types compatible with unreal_asset
/// 
/// Represents the different types of properties that can exist in Unreal Engine assets.
/// This enum provides compatibility with the unreal_asset property system and includes
/// advanced features required by tools like Stove.
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
    
    // Advanced property types required by Stove and other tools
    
    /// 3D Vector property (Location, Scale, etc.)
    Vector(Vector),
    /// 4D Vector property
    Vector4(Vector4),
    /// 2D Vector property
    Vector2D(Vector2D),
    /// Rotation property
    Rotator(Rotator),
    /// Quaternion property
    Quat(Quat),
    /// Color property
    LinearColor(LinearColor),
    /// Transform property (location, rotation, scale)
    Transform(Transform),
    /// Soft object path property
    SoftObjectPath(SoftObjectPath),
    /// Soft class path property
    SoftClassPath(SoftObjectPath),
    /// Asset object property
    AssetObjectProperty(SoftObjectPath),
    
    // Per-platform properties
    /// Per-platform boolean values
    PerPlatformBool(Vec<bool>),
    /// Per-platform integer values
    PerPlatformInt(Vec<i32>),
    /// Per-platform float values
    PerPlatformFloat(Vec<f32>),
    
    // Advanced data types
    /// GUID property
    Guid([u32; 4]),
    /// DateTime property (ticks since epoch)
    DateTime(i64),
    /// TimeSpan property (duration in ticks)
    TimeSpan(i64),
    /// Delegate property
    Delegate {
        /// Delegate object reference
        object: Option<PackageIndex>,
        /// Function name
        function_name: FName,
    },
    /// Multicast delegate property
    MulticastDelegate {
        /// Array of delegates
        delegates: Vec<Property>, // Each is a Delegate property
    },
    
    // Material and mesh specific properties
    /// Material interface property
    MaterialInterface(Option<PackageIndex>),
    /// Static mesh property
    StaticMesh(Option<PackageIndex>),
    /// Skeletal mesh property
    SkeletalMesh(Option<PackageIndex>),
    /// Texture property
    Texture2D(Option<PackageIndex>),
    
    /// Set property (unique values)
    Set(Vec<Property>),
    /// Byte property with enum value
    ByteEnum {
        /// Enum type
        enum_type: FName,
        /// Enum value
        value: FName,
    },
    /// Raw byte property
    Byte(u8),        /// Material instances with parameters
        MaterialInstance(HashMap<String, Property>),
        /// Level sequence property
        LevelSequence(Option<PackageIndex>),
        /// Component reference property
        ComponentReference(Option<PackageIndex>),
        /// Blueprint property
        Blueprint(Option<PackageIndex>),
        /// World context property
        WorldContext(Option<PackageIndex>),
        /// Landscape component property
        LandscapeComponent(Option<PackageIndex>),
        
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

/// Mesh data structure for static mesh exports
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticMeshData {
    /// Vertex positions
    pub vertices: Vec<Vector>,
    /// Triangle indices
    pub indices: Vec<u32>,
    /// UV coordinates (multiple channels supported)
    pub uv_channels: Vec<Vec<Vector2D>>,
    /// Material names/paths
    pub materials: Vec<String>,
    /// Material data ranges (start index, count)
    pub material_ranges: Vec<(u32, u32)>,
    /// Normals (if available)
    pub normals: Option<Vec<Vector>>,
    /// Tangents (if available)
    pub tangents: Option<Vec<Vector>>,
}

#[cfg(feature = "unrealmodding-compat")]
impl StaticMeshData {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            uv_channels: Vec::new(),
            materials: Vec::new(),
            material_ranges: Vec::new(),
            normals: None,
            tangents: None,
        }
    }
    
    /// Get the number of triangles
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
    
    /// Get the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
    
    /// Get UV coordinates for a specific channel
    pub fn get_uv_channel(&self, channel: usize) -> Option<&Vec<Vector2D>> {
        self.uv_channels.get(channel)
    }
}

/// Texture data structure for texture exports
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Texture2DData {
    /// Texture width
    pub width: u32,
    /// Texture height
    pub height: u32,
    /// Pixel format (e.g., "PF_DXT1", "PF_DXT5", "PF_ASTC_4x4")
    pub format: String,
    /// Raw texture data
    pub data: Vec<u8>,
    /// Whether the texture is a normal map
    pub is_normal_map: bool,
    /// Number of mip levels
    pub mip_count: u32,
}

#[cfg(feature = "unrealmodding-compat")]
impl Texture2DData {
    pub fn new(width: u32, height: u32, format: String) -> Self {
        Self {
            width,
            height,
            format,
            data: Vec::new(),
            is_normal_map: false,
            mip_count: 1,
        }
    }
    
    /// Get the size in bytes of the texture data
    pub fn size_bytes(&self) -> usize {
        self.data.len()
    }
    
    /// Check if the format is compressed
    pub fn is_compressed(&self) -> bool {
        matches!(self.format.as_str(), 
            "PF_DXT1" | "PF_DXT3" | "PF_DXT5" | 
            "PF_BC1" | "PF_BC2" | "PF_BC3" | "PF_BC4" | "PF_BC5" | "PF_BC6H" | "PF_BC7" |
            "PF_ASTC_4x4" | "PF_ASTC_6x6" | "PF_ASTC_8x8" | "PF_ASTC_10x10" | "PF_ASTC_12x12" |
            "PF_ETC1" | "PF_ETC2_RGB" | "PF_ETC2_RGBA"
        )
    }
}

/// Material data structure
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialData {
    /// Material name
    pub name: String,
    /// Material type/class
    pub material_type: String,
    /// Texture references
    pub textures: HashMap<String, String>,
    /// Scalar parameters
    pub scalar_parameters: HashMap<String, f32>,
    /// Vector parameters
    pub vector_parameters: HashMap<String, LinearColor>,
    /// Boolean parameters
    pub boolean_parameters: HashMap<String, bool>,
}

#[cfg(feature = "unrealmodding-compat")]
impl MaterialData {
    pub fn new(name: String, material_type: String) -> Self {
        Self {
            name,
            material_type,
            textures: HashMap::new(),
            scalar_parameters: HashMap::new(),
            vector_parameters: HashMap::new(),
            boolean_parameters: HashMap::new(),
        }
    }
}

/// Actor data structure for level editing
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorData {
    /// Actor name
    pub name: String,
    /// Actor class
    pub class: String,
    /// Actor transform
    pub transform: Transform,
    /// Actor properties
    pub properties: IndexMap<String, Property>,
    /// Component data
    pub components: Vec<ComponentData>,
}

/// Component data structure
#[cfg(feature = "unrealmodding-compat")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentData {
    /// Component name
    pub name: String,
    /// Component class
    pub class: String,
    /// Component properties
    pub properties: IndexMap<String, Property>,
}

/// Advanced asset processing capabilities
#[cfg(feature = "unrealmodding-compat")]
pub trait AdvancedAssetProcessing {
    /// Extract static mesh data from an asset
    fn extract_static_mesh(&self) -> Result<Option<StaticMeshData>>;
    
    /// Extract texture data from an asset
    fn extract_texture_data(&self) -> Result<Option<Texture2DData>>;
    
    /// Extract material data from an asset
    fn extract_material_data(&self) -> Result<Option<MaterialData>>;
    
    /// Extract actor data from a level asset
    fn extract_actors(&self) -> Result<Vec<ActorData>>;
    
    /// Get texture references from a material
    fn get_texture_references(&self) -> Result<Vec<String>>;
}

#[cfg(feature = "unrealmodding-compat")]
impl AdvancedAssetProcessing for Asset {
    fn extract_static_mesh(&self) -> Result<Option<StaticMeshData>> {
        // Find the static mesh export
        let mesh_export = self.asset_data.exports
            .iter()
            .find(|export| {
                // Check if this export is a StaticMesh
                export.object_name.name.contains("StaticMesh") ||
                export.class_index.0 != 0 // Would need proper class resolution
            });
            
        if let Some(export) = mesh_export {
            let mut mesh_data = StaticMeshData::new();
            
            // Extract mesh data from the extras field
            if let Some(extras) = &export.extras {
                // Parse mesh data from CUE4Parse JSON output
                if let Some(render_data) = extras.get("RenderData") {
                    if let Some(lods) = render_data.get("LODs") {
                        if let Some(lod0) = lods.get(0) {
                            // Extract vertices
                            if let Some(vertices) = lod0.get("Vertices") {
                                if let serde_json::Value::Array(vertex_array) = vertices {
                                    for vertex in vertex_array {
                                        if let Some(pos) = vertex.get("Position") {
                                            if let (Some(x), Some(y), Some(z)) = (
                                                pos.get("X").and_then(|v| v.as_f64()),
                                                pos.get("Y").and_then(|v| v.as_f64()),
                                                pos.get("Z").and_then(|v| v.as_f64())
                                            ) {
                                                mesh_data.vertices.push(Vector::new(x, y, z));
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // Extract indices
                            if let Some(indices) = lod0.get("Indices") {
                                if let serde_json::Value::Array(index_array) = indices {
                                    for index in index_array {
                                        if let Some(idx) = index.as_u64() {
                                            mesh_data.indices.push(idx as u32);
                                        }
                                    }
                                }
                            }
                            
                            // Extract UV coordinates
                            if let Some(uvs) = lod0.get("UVs") {
                                if let serde_json::Value::Array(uv_channels) = uvs {
                                    for channel in uv_channels {
                                        if let serde_json::Value::Array(uv_array) = channel {
                                            let mut channel_uvs = Vec::new();
                                            for uv in uv_array {
                                                if let (Some(u), Some(v)) = (
                                                    uv.get("U").and_then(|v| v.as_f64()),
                                                    uv.get("V").and_then(|v| v.as_f64())
                                                ) {
                                                    channel_uvs.push(Vector2D::new(u, v));
                                                }
                                            }
                                            mesh_data.uv_channels.push(channel_uvs);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Extract material information
                if let Some(materials) = extras.get("StaticMaterials") {
                    if let serde_json::Value::Array(material_array) = materials {
                        for material in material_array {
                            if let Some(material_ref) = material.get("MaterialInterface") {
                                if let Some(name) = material_ref.as_str() {
                                    mesh_data.materials.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
            
            Ok(Some(mesh_data))
        } else {
            Ok(None)
        }
    }
    
    fn extract_texture_data(&self) -> Result<Option<Texture2DData>> {
        // Find the texture export
        let texture_export = self.asset_data.exports
            .iter()
            .find(|export| {
                export.object_name.name.contains("Texture2D") ||
                export.class_index.0 != 0 // Would need proper class resolution
            });
            
        if let Some(export) = texture_export {
            if let Some(extras) = &export.extras {
                // Extract texture properties
                let width = extras.get("SizeX")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                    
                let height = extras.get("SizeY")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                    
                let format = extras.get("Format")
                    .and_then(|v| v.as_str())
                    .unwrap_or("PF_Unknown")
                    .to_string();
                    
                let mut texture_data = Texture2DData::new(width, height, format);
                
                // Extract texture data if available
                if let Some(platform_data) = extras.get("PlatformData") {
                    if let Some(mips) = platform_data.get("Mips") {
                        if let serde_json::Value::Array(mip_array) = mips {
                            if let Some(first_mip) = mip_array.get(0) {
                                if let Some(bulk_data) = first_mip.get("BulkData") {
                                    if let Some(data) = bulk_data.get("Data") {
                                        if let serde_json::Value::Array(data_array) = data {
                                            texture_data.data = data_array
                                                .iter()
                                                .filter_map(|v| v.as_u64().map(|u| u as u8))
                                                .collect();
                                        }
                                    }
                                }
                            }
                            texture_data.mip_count = mip_array.len() as u32;
                        }
                    }
                }
                
                // Check if it's a normal map
                texture_data.is_normal_map = extras.get("CompressionSettings")
                    .and_then(|v| v.as_str())
                    .map(|s| s.contains("Normal"))
                    .unwrap_or(false);
                    
                Ok(Some(texture_data))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    fn extract_material_data(&self) -> Result<Option<MaterialData>> {
        // Find the material export
        let material_export = self.asset_data.exports
            .iter()
            .find(|export| {
                export.object_name.name.contains("Material") ||
                export.class_index.0 != 0 // Would need proper class resolution
            });
            
        if let Some(export) = material_export {
            let material_name = export.object_name.name.clone();
            let material_type = "Material".to_string(); // Would need class resolution
            
            let mut material_data = MaterialData::new(material_name, material_type);
            
            // Extract material parameters from properties
            for (prop_name, property) in &export.properties {
                match property {
                    Property::Object(Some(obj_ref)) => {
                        // Texture reference
                        if prop_name.contains("Texture") || prop_name.contains("texture") {
                            // Would need to resolve the object reference
                            material_data.textures.insert(
                                prop_name.clone(),
                                format!("Object_{}", obj_ref.0)
                            );
                        }
                    },
                    Property::Float(value) => {
                        material_data.scalar_parameters.insert(prop_name.clone(), *value);
                    },
                    Property::LinearColor(color) => {
                        material_data.vector_parameters.insert(prop_name.clone(), color.clone());
                    },
                    Property::Bool(value) => {
                        material_data.boolean_parameters.insert(prop_name.clone(), *value);
                    },
                    _ => {}
                }
            }
            
            Ok(Some(material_data))
        } else {
            Ok(None)
        }
    }
    
    fn extract_actors(&self) -> Result<Vec<ActorData>> {
        let mut actors = Vec::new();
        
        // Look for actor exports in the asset
        for export in &self.asset_data.exports {
            // Check if this export represents an actor
            if export.object_name.name.contains("Actor") || 
               export.properties.contains_key("RootComponent") ||
               export.properties.contains_key("ActorLocation") {
                
                let mut actor = ActorData {
                    name: export.object_name.name.clone(),
                    class: "Actor".to_string(), // Would need class resolution
                    transform: Transform::identity(),
                    properties: export.properties.clone(),
                    components: Vec::new(),
                };
                
                // Extract transform from properties
                if let Some(Property::Vector(location)) = export.properties.get("ActorLocation") {
                    actor.transform.location = location.clone();
                }
                if let Some(Property::Rotator(_rotation)) = export.properties.get("ActorRotation") {
                    // Convert rotator to quaternion (simplified)
                    actor.transform.rotation = Quat::new(0.0, 0.0, 0.0, 1.0);
                }
                if let Some(Property::Vector(scale)) = export.properties.get("ActorScale3D") {
                    actor.transform.scale = scale.clone();
                }
                
                actors.push(actor);
            }
        }
        
        Ok(actors)
    }
    
    fn get_texture_references(&self) -> Result<Vec<String>> {
        let mut texture_refs = Vec::new();
        
        // Look through all imports for texture references
        for import in &self.asset_data.imports {
            if import.class_name.name == "Texture2D" {
                texture_refs.push(import.object_name.name.clone());
            }
        }
        
        // Also check object properties in exports
        for export in &self.asset_data.exports {
            for (prop_name, property) in &export.properties {
                if prop_name.to_lowercase().contains("texture") {
                    match property {
                        Property::Object(Some(obj_ref)) => {
                            // Would need to resolve the reference
                            texture_refs.push(format!("Object_{}", obj_ref.0));
                        },
                        Property::SoftObjectPath(path) => {
                            if !path.asset_path.name.is_empty() {
                                texture_refs.push(path.asset_path.name.clone());
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
        
        Ok(texture_refs)
    }
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
    
    /// Convert a CUE4Parse JSON value to a Property with enhanced type support
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
                match property_type {
                    Some("NameProperty") | Some("Name") => Property::Name(FName::new(s.clone())),
                    Some("ObjectProperty") => {
                        // Parse object path
                        if s.is_empty() || s == "None" {
                            Property::Object(None)
                        } else {
                            // Would need proper object resolution
                            Property::Object(Some(PackageIndex(1)))
                        }
                    },
                    Some("SoftObjectProperty") => {
                        Property::SoftObjectPath(SoftObjectPath::new(s.clone(), ""))
                    },
                    Some("SoftClassProperty") => {
                        Property::SoftClassPath(SoftObjectPath::new(s.clone(), ""))
                    },
                    Some("AssetObjectProperty") => {
                        Property::AssetObjectProperty(SoftObjectPath::new(s.clone(), ""))
                    },
                    _ => Property::String(s.clone()),
                }
            },
            serde_json::Value::Array(arr) => {
                let properties: Vec<Property> = arr.iter()
                    .map(|v| Self::json_to_property(v, None))
                    .collect();
                Property::Array(properties)
            },
            serde_json::Value::Object(obj) => {
                // Handle special struct types
                if let Some(struct_type) = obj.get("$type").and_then(|v| v.as_str()) {
                    match struct_type {
                        "Vector" => {
                            let x = obj.get("X").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let y = obj.get("Y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let z = obj.get("Z").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            Property::Vector(Vector::new(x, y, z))
                        },
                        "Vector4" => {
                            let x = obj.get("X").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let y = obj.get("Y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let z = obj.get("Z").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let w = obj.get("W").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            Property::Vector4(Vector4::new(x, y, z, w))
                        },
                        "Vector2D" => {
                            let x = obj.get("X").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let y = obj.get("Y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            Property::Vector2D(Vector2D::new(x, y))
                        },
                        "Rotator" => {
                            let pitch = obj.get("Pitch").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let yaw = obj.get("Yaw").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let roll = obj.get("Roll").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            Property::Rotator(Rotator::new(pitch, yaw, roll))
                        },
                        "Quat" => {
                            let x = obj.get("X").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let y = obj.get("Y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let z = obj.get("Z").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let w = obj.get("W").and_then(|v| v.as_f64()).unwrap_or(1.0);
                            Property::Quat(Quat::new(x, y, z, w))
                        },
                        "LinearColor" => {
                            let r = obj.get("R").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                            let g = obj.get("G").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                            let b = obj.get("B").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                            let a = obj.get("A").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32;
                            Property::LinearColor(LinearColor::new(r, g, b, a))
                        },
                        "Transform" => {
                            let location = obj.get("Translation")
                                .map(|v| Self::json_to_property(v, Some("Vector")))
                                .and_then(|p| match p {
                                    Property::Vector(v) => Some(v),
                                    _ => None,
                                })
                                .unwrap_or_else(Vector::zero);
                                
                            let rotation = obj.get("Rotation")
                                .map(|v| Self::json_to_property(v, Some("Quat")))
                                .and_then(|p| match p {
                                    Property::Quat(q) => Some(q),
                                    _ => None,
                                })
                                .unwrap_or_else(Quat::identity);
                                
                            let scale = obj.get("Scale3D")
                                .map(|v| Self::json_to_property(v, Some("Vector")))
                                .and_then(|p| match p {
                                    Property::Vector(v) => Some(v),
                                    _ => None,
                                })
                                .unwrap_or_else(|| Vector::new(1.0, 1.0, 1.0));
                                
                            Property::Transform(Transform::new(location, rotation, scale))
                        },
                        "SoftObjectPath" => {
                            let asset_path = obj.get("AssetPathName")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");
                            let sub_path = obj.get("SubPathString")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");
                            Property::SoftObjectPath(SoftObjectPath::new(asset_path, sub_path))
                        },
                        "Guid" => {
                            let a = obj.get("A").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                            let b = obj.get("B").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                            let c = obj.get("C").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                            let d = obj.get("D").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                            Property::Guid([a, b, c, d])
                        },
                        _ => {
                            // Generic struct handling
                            let mut properties = IndexMap::new();
                            for (key, val) in obj {
                                if key != "$type" {
                                    properties.insert(key.clone(), Self::json_to_property(val, None));
                                }
                            }
                            Property::Struct {
                                struct_type: FName::new(struct_type),
                                properties,
                            }
                        }
                    }
                } else if obj.contains_key("EnumType") && obj.contains_key("EnumValue") {
                    // Handle enum properties
                    let enum_type = obj.get("EnumType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown");
                    let enum_value = obj.get("EnumValue")
                        .and_then(|v| v.as_str())
                        .unwrap_or("None");
                    Property::Enum {
                        enum_type: FName::new(enum_type),
                        value: FName::new(enum_value),
                    }
                } else {
                    // Generic struct or map
                    if obj.keys().any(|k| k.chars().all(|c| c.is_ascii_digit())) {
                        // Looks like an array represented as object with numeric keys
                        let mut array = Vec::new();
                        let mut keys: Vec<_> = obj.keys().collect();
                        keys.sort_by_key(|k| k.parse::<usize>().unwrap_or(0));
                        for key in keys {
                            if let Some(value) = obj.get(key) {
                                array.push(Self::json_to_property(value, None));
                            }
                        }
                        Property::Array(array)
                    } else {
                        // Generic struct
                        let mut properties = IndexMap::new();
                        for (key, val) in obj {
                            properties.insert(key.clone(), Self::json_to_property(val, None));
                        }
                        
                        Property::Struct {
                            struct_type: FName::new("Struct"),
                            properties,
                        }
                    }
                }
            },
            serde_json::Value::Null => Property::Object(None),
        }
    }
    
    /// Enhanced property extraction with type hints
    pub fn extract_property_with_type(&self, export_index: usize, property_name: &str, expected_type: &str) -> Option<Property> {
        self.asset_data.exports.get(export_index)
            .and_then(|export| export.properties.get(property_name))
            .cloned()
            .or_else(|| {
                // Try to find property in extras with type information
                self.asset_data.exports.get(export_index)
                    .and_then(|export| export.extras.as_ref())
                    .and_then(|extras| extras.get(property_name))
                    .map(|value| Self::json_to_property(value, Some(expected_type)))
            })
    }
    
    /// Get actor transform components (Stove compatibility)
    pub fn get_actor_transform(&self, export_index: usize) -> Option<Transform> {
        let export = self.asset_data.exports.get(export_index)?;
        
        let mut transform = Transform::identity();
        
        // Look for transform properties commonly used by Stove
        for (prop_name, property) in &export.properties {
            match (prop_name.as_str(), property) {
                ("RelativeLocation" | "Location", Property::Vector(v)) => {
                    transform.location = v.clone();
                },
                ("RelativeRotation" | "Rotation", Property::Rotator(r)) => {
                    // Convert rotator to quaternion (simplified conversion)
                    transform.rotation = Quat::new(
                        r.pitch.to_radians() * 0.5,
                        r.yaw.to_radians() * 0.5, 
                        r.roll.to_radians() * 0.5,
                        1.0
                    );
                },
                ("RelativeScale3D" | "Scale3D", Property::Vector(s)) => {
                    transform.scale = s.clone();
                },
                _ => {}
            }
        }
        
        Some(transform)
    }
    
    /// Set actor transform components (Stove compatibility)
    pub fn set_actor_transform(&mut self, export_index: usize, transform: &Transform) -> bool {
        if let Some(export) = self.asset_data.exports.get_mut(export_index) {
            // Set location
            export.properties.insert(
                "RelativeLocation".to_string(),
                Property::Vector(transform.location.clone())
            );
            
            // Set rotation (convert from quaternion to rotator)
            let (roll, pitch, yaw) = quaternion_to_euler(&transform.rotation);
            export.properties.insert(
                "RelativeRotation".to_string(),
                Property::Rotator(Rotator::new(pitch, yaw, roll))
            );
            
            // Set scale
            export.properties.insert(
                "RelativeScale3D".to_string(),
                Property::Vector(transform.scale.clone())
            );
            
            true
        } else {
            false
        }
    }
    
    /// Find mesh component for an actor (Stove-specific)
    pub fn find_mesh_component(&self, actor_export_index: usize) -> Option<usize> {
        let actor_export = self.asset_data.exports.get(actor_export_index)?;
        
        // Look for RootComponent or StaticMeshComponent references
        for (prop_name, property) in &actor_export.properties {
            if prop_name.contains("Mesh") || prop_name == "RootComponent" {
                if let Property::Object(Some(component_ref)) = property {
                    if component_ref.is_export() {
                        return component_ref.export_index();
                    }
                }
            }
        }
        None
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

/// Helper function to convert quaternion to euler angles (roll, pitch, yaw)
/// Used for Stove compatibility when converting between quaternions and rotators
#[cfg(feature = "unrealmodding-compat")]
fn quaternion_to_euler(quat: &Quat) -> (f64, f64, f64) {
    // Simplified conversion - more sophisticated conversion would be needed for production use
    let roll = (2.0 * (quat.w * quat.x + quat.y * quat.z)).atan2(1.0 - 2.0 * (quat.x * quat.x + quat.y * quat.y));
    let pitch = (2.0 * (quat.w * quat.y - quat.z * quat.x)).asin();
    let yaw = (2.0 * (quat.w * quat.z + quat.x * quat.y)).atan2(1.0 - 2.0 * (quat.y * quat.y + quat.z * quat.z));
    (roll, pitch, yaw)
}

/// Conversion utilities for unreal_asset compatibility
/// 
/// Provides helper functions for converting between CUE4Parse data formats
/// and unreal_asset compatible structures. Enhanced with Stove-specific features
/// and full compatibility with the unrealmodding/unreal_asset crate structure.
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
            "VectorProperty" => {
                if let serde_json::Value::Object(obj) = value {
                    let x = obj.get("X").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let y = obj.get("Y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let z = obj.get("Z").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    Property::Vector(Vector::new(x, y, z))
                } else {
                    Property::Unknown(value.clone())
                }
            },
            "Vector4Property" => {
                if let serde_json::Value::Object(obj) = value {
                    let x = obj.get("X").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let y = obj.get("Y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let z = obj.get("Z").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let w = obj.get("W").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    Property::Vector4(Vector4::new(x, y, z, w))
                } else {
                    Property::Unknown(value.clone())
                }
            },
            "Vector2DProperty" => {
                if let serde_json::Value::Object(obj) = value {
                    let x = obj.get("X").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let y = obj.get("Y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    Property::Vector2D(Vector2D::new(x, y))
                } else {
                    Property::Unknown(value.clone())
                }
            },
            "RotatorProperty" => {
                if let serde_json::Value::Object(obj) = value {
                    let pitch = obj.get("Pitch").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let yaw = obj.get("Yaw").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let roll = obj.get("Roll").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    Property::Rotator(Rotator::new(pitch, yaw, roll))
                } else {
                    Property::Unknown(value.clone())
                }
            },
            "QuatProperty" => {
                if let serde_json::Value::Object(obj) = value {
                    let x = obj.get("X").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let y = obj.get("Y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let z = obj.get("Z").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let w = obj.get("W").and_then(|v| v.as_f64()).unwrap_or(1.0);
                    Property::Quat(Quat::new(x, y, z, w))
                } else {
                    Property::Unknown(value.clone())
                }
            },
            "LinearColorProperty" => {
                if let serde_json::Value::Object(obj) = value {
                    let r = obj.get("R").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                    let g = obj.get("G").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                    let b = obj.get("B").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                    let a = obj.get("A").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32;
                    Property::LinearColor(LinearColor::new(r, g, b, a))
                } else {
                    Property::Unknown(value.clone())
                }
            },
            "TransformProperty" => {
                if let serde_json::Value::Object(obj) = value {
                    let location = obj.get("Translation")
                        .map(|v| Self::json_to_property_typed(v, "VectorProperty"))
                        .and_then(|p| match p {
                            Property::Vector(v) => Some(v),
                            _ => None,
                        })
                        .unwrap_or_else(Vector::zero);
                        
                    let rotation = obj.get("Rotation")
                        .map(|v| Self::json_to_property_typed(v, "QuatProperty"))
                        .and_then(|p| match p {
                            Property::Quat(q) => Some(q),
                            _ => None,
                        })
                        .unwrap_or_else(Quat::identity);
                        
                    let scale = obj.get("Scale3D")
                        .map(|v| Self::json_to_property_typed(v, "VectorProperty"))
                        .and_then(|p| match p {
                            Property::Vector(v) => Some(v),
                            _ => None,
                        })
                        .unwrap_or_else(|| Vector::new(1.0, 1.0, 1.0));
                        
                    Property::Transform(Transform::new(location, rotation, scale))
                } else {
                    Property::Unknown(value.clone())
                }
            },
            "SoftObjectProperty" => {
                if let serde_json::Value::Object(obj) = value {
                    let asset_path = obj.get("AssetPathName")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let sub_path = obj.get("SubPathString")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    Property::SoftObjectPath(SoftObjectPath::new(asset_path, sub_path))
                } else if let serde_json::Value::String(s) = value {
                    Property::SoftObjectPath(SoftObjectPath::new(s, ""))
                } else {
                    Property::Unknown(value.clone())
                }
            },
            "SoftClassProperty" => {
                if let serde_json::Value::Object(obj) = value {
                    let asset_path = obj.get("AssetPathName")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let sub_path = obj.get("SubPathString")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    Property::SoftClassPath(SoftObjectPath::new(asset_path, sub_path))
                } else if let serde_json::Value::String(s) = value {
                    Property::SoftClassPath(SoftObjectPath::new(s, ""))
                } else {
                    Property::Unknown(value.clone())
                }
            },
            "GuidProperty" => {
                if let serde_json::Value::Object(obj) = value {
                    let a = obj.get("A").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                    let b = obj.get("B").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                    let c = obj.get("C").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                    let d = obj.get("D").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                    Property::Guid([a, b, c, d])
                } else {
                    Property::Unknown(value.clone())
                }
            },
            "DateTimeProperty" => {
                Property::DateTime(value.as_i64().unwrap_or(0))
            },
            "TimeSpanProperty" => {
                Property::TimeSpan(value.as_i64().unwrap_or(0))
            },
            "ByteProperty" => {
                if let serde_json::Value::Object(obj) = value {
                    // Enum byte property
                    let enum_type = obj.get("EnumType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("None");
                    let enum_value = obj.get("EnumValue")
                        .and_then(|v| v.as_str())
                        .unwrap_or("None");
                    Property::ByteEnum {
                        enum_type: FName::new(enum_type),
                        value: FName::new(enum_value),
                    }
                } else {
                    Property::Byte(value.as_u64().unwrap_or(0) as u8)
                }
            },
            "ArrayProperty" => {
                if let serde_json::Value::Array(arr) = value {
                    let properties: Vec<Property> = arr.iter()
                        .map(|v| Asset::json_to_property(v, None))
                        .collect();
                    Property::Array(properties)
                } else {
                    Property::Unknown(value.clone())
                }
            },
            "SetProperty" => {
                if let serde_json::Value::Array(arr) = value {
                    let properties: Vec<Property> = arr.iter()
                        .map(|v| Asset::json_to_property(v, None))
                        .collect();
                    Property::Set(properties)
                } else {
                    Property::Unknown(value.clone())
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
    
    /// Extract transform components from an actor export (Stove-specific)
    pub fn extract_actor_transform(export: &Export) -> Transform {
        let mut transform = Transform::identity();
        
        // Look for transform properties
        for (prop_name, property) in &export.properties {
            match prop_name.as_str() {
                "RelativeLocation" | "ActorLocation" => {
                    if let Property::Vector(location) = property {
                        transform.location = location.clone();
                    }
                },
                "RelativeRotation" | "ActorRotation" => {
                    if let Property::Rotator(rotator) = property {
                        // Convert rotator to quaternion (simplified conversion)
                        let pitch = rotator.pitch.to_radians();
                        let yaw = rotator.yaw.to_radians();
                        let roll = rotator.roll.to_radians();
                        
                        let cp = (pitch / 2.0).cos();
                        let sp = (pitch / 2.0).sin();
                        let cy = (yaw / 2.0).cos();
                        let sy = (yaw / 2.0).sin();
                        let cr = (roll / 2.0).cos();
                        let sr = (roll / 2.0).sin();
                        
                        transform.rotation = Quat::new(
                            sr * cp * cy - cr * sp * sy,
                            cr * sp * cy + sr * cp * sy,
                            cr * cp * sy - sr * sp * cy,
                            cr * cp * cy + sr * sp * sy,
                        );
                    }
                },
                "RelativeScale3D" | "ActorScale3D" => {
                    if let Property::Vector(scale) = property {
                        transform.scale = scale.clone();
                    }
                },
                _ => {}
            }
        }
        
        transform
    }
    
    /// Extract mesh component from an actor (Stove-specific)
    pub fn find_mesh_component(asset: &Asset, actor_export: &Export) -> Option<String> {
        // Look for mesh component references
        for (prop_name, property) in &actor_export.properties {
            if prop_name.contains("MeshComponent") || prop_name.contains("StaticMesh") {
                match property {
                    Property::Object(Some(package_index)) => {
                        // Try to resolve the mesh reference
                        if let Some(mesh_export) = asset.asset_data.exports.get((package_index.0 - 1) as usize) {
                            if let Some(Property::SoftObjectPath(path)) = mesh_export.properties.get("StaticMesh") {
                                return Some(path.asset_path.name.clone());
                            }
                        }
                    },
                    Property::SoftObjectPath(path) => {
                        return Some(path.asset_path.name.clone());
                    },
                    _ => {}
                }
            }
        }
        None
    }
    
    /// Extract material references from a mesh (Stove-specific)
    pub fn extract_material_references(asset: &Asset) -> Vec<String> {
        let mut materials = Vec::new();
        
        for export in &asset.asset_data.exports {
            // Look for material properties
            for (prop_name, property) in &export.properties {
                if prop_name.to_lowercase().contains("material") {
                    match property {
                        Property::Object(Some(package_index)) => {
                            // Resolve material reference
                            if package_index.is_import() {
                                if let Some(import_index) = package_index.import_index() {
                                    if let Some(import) = asset.asset_data.imports.get(import_index) {
                                        materials.push(import.object_name.name.clone());
                                    }
                                }
                            }
                        },
                        Property::SoftObjectPath(path) => {
                            materials.push(path.asset_path.name.clone());
                        },
                        _ => {}
                    }
                }
            }
        }
        
        materials
    }
    
    /// Convert a property to a specific type (useful for UI editing)
    pub fn property_to_value<T>(property: &Property) -> Option<T> 
    where 
        T: std::str::FromStr + Clone,
    {
        match property {
            Property::String(s) => s.parse().ok(),
            Property::Name(name) => name.name.parse().ok(),
            _ => None,
        }
    }
    
    /// Get property as float for numerical editing (Stove-specific)
    pub fn property_as_float(property: &Property) -> Option<f64> {
        match property {
            Property::Float(f) => Some(*f as f64),
            Property::Double(d) => Some(*d),
            Property::Int32(i) => Some(*i as f64),
            Property::Int64(i) => Some(*i as f64),
            _ => None,
        }
    }
    
    /// Set property from float value (Stove-specific) 
    pub fn set_property_from_float(property: &mut Property, value: f64) {
        match property {
            Property::Float(f) => *f = value as f32,
            Property::Double(d) => *d = value,
            Property::Int32(i) => *i = value as i32,
            Property::Int64(i) => *i = value as i64,
            _ => {}
        }
    }
    
    /// Get all actor components for Stove-style editing
    pub fn get_actor_components(asset: &Asset, actor_export_index: usize) -> Vec<usize> {
        let mut components = Vec::new();
        
        if let Some(actor_export) = asset.asset_data.exports.get(actor_export_index) {
            // Look for component references in the actor's properties
            for (prop_name, property) in &actor_export.properties {
                if prop_name.contains("Component") {
                    if let Property::Object(Some(comp_ref)) = property {
                        if comp_ref.is_export() {
                            if let Some(comp_index) = comp_ref.export_index() {
                                components.push(comp_index);
                            }
                        }
                    }
                }
            }
            
            // Also check BlueprintCreatedComponents array
            if let Some(Property::Array(blueprint_components)) = actor_export.properties.get("BlueprintCreatedComponents") {
                for comp in blueprint_components {
                    if let Property::Object(Some(comp_ref)) = comp {
                        if comp_ref.is_export() {
                            if let Some(comp_index) = comp_ref.export_index() {
                                components.push(comp_index);
                            }
                        }
                    }
                }
            }
        }
        
        components
    }
    
    /// Check if an export is an actor (Stove-compatible check)
    pub fn is_actor_export(asset: &Asset, export_index: usize) -> bool {
        if let Some(export) = asset.asset_data.exports.get(export_index) {
            // Check class name through import table
            if export.class_index.is_import() {
                if let Some(import_idx) = export.class_index.import_index() {
                    if let Some(import) = asset.asset_data.imports.get(import_idx) {
                        return import.class_name.name.contains("Actor");
                    }
                }
            }
            
            // Check for common actor properties
            export.properties.contains_key("RootComponent") ||
            export.properties.contains_key("RelativeLocation") ||
            export.properties.contains_key("RelativeRotation") ||
            export.properties.contains_key("RelativeScale3D")
        } else {
            false
        }
    }
    
    /// Get component transform relative to its parent (Stove-specific)
    pub fn get_component_transform(asset: &Asset, component_export_index: usize) -> Option<Transform> {
        let export = asset.asset_data.exports.get(component_export_index)?;
        
        let mut transform = Transform::identity();
        
        // Look for relative transform properties
        for (prop_name, property) in &export.properties {
            match prop_name.as_str() {
                "RelativeLocation" => {
                    if let Property::Vector(location) = property {
                        transform.location = location.clone();
                    }
                },
                "RelativeRotation" => {
                    if let Property::Rotator(rotation) = property {
                        // Convert rotator to quaternion
                        transform.rotation = Quat::new(
                            rotation.pitch.to_radians() * 0.5,
                            rotation.yaw.to_radians() * 0.5,
                            rotation.roll.to_radians() * 0.5,
                            1.0
                        );
                    }
                },
                "RelativeScale3D" => {
                    if let Property::Vector(scale) = property {
                        transform.scale = scale.clone();
                    }
                },
                _ => {}
            }
        }
        
        Some(transform)
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
    fn test_vector_operations() {
        let vec = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
        
        let zero = Vector::zero();
        assert_eq!(zero.x, 0.0);
        assert_eq!(zero.y, 0.0);
        assert_eq!(zero.z, 0.0);
    }
    
    #[test]
    fn test_transform_operations() {
        let transform = Transform::identity();
        assert_eq!(transform.location, Vector::zero());
        assert_eq!(transform.rotation, Quat::identity());
        assert_eq!(transform.scale, Vector::new(1.0, 1.0, 1.0));
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
        
        // Test vector conversion
        let vector_json = json!({
            "$type": "Vector",
            "X": 1.0,
            "Y": 2.0,
            "Z": 3.0
        });
        let prop = Asset::json_to_property(&vector_json, None);
        if let Property::Vector(vec) = prop {
            assert_eq!(vec.x, 1.0);
            assert_eq!(vec.y, 2.0);
            assert_eq!(vec.z, 3.0);
        } else {
            panic!("Expected Vector property");
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
        
        // Test vector property conversion
        let vector_val = json!({
            "X": 1.5,
            "Y": 2.5,
            "Z": 3.5
        });
        let prop = ConversionUtils::json_to_property_typed(&vector_val, "VectorProperty");
        if let Property::Vector(vec) = prop {
            assert_eq!(vec.x, 1.5);
            assert_eq!(vec.y, 2.5);
            assert_eq!(vec.z, 3.5);
        } else {
            panic!("Expected Vector property");
        }
    }
    
    #[test]
    fn test_mesh_data() {
        let mut mesh = StaticMeshData::new();
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.triangle_count(), 0);
        
        mesh.vertices = vec![
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.5, 1.0, 0.0),
        ];
        mesh.indices = vec![0, 1, 2];
        
        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
    }
    
    #[test]
    fn test_texture_data() {
        let texture = Texture2DData::new(512, 512, "PF_DXT5".to_string());
        assert_eq!(texture.width, 512);
        assert_eq!(texture.height, 512);
        assert_eq!(texture.format, "PF_DXT5");
        assert!(texture.is_compressed());
        
        let uncompressed = Texture2DData::new(256, 256, "PF_B8G8R8A8".to_string());
        assert!(!uncompressed.is_compressed());
    }
    
    #[test]
    fn test_soft_object_path() {
        let path = SoftObjectPath::new("/Game/Meshes/SM_Rock", "");
        assert_eq!(path.asset_path.name, "/Game/Meshes/SM_Rock");
        assert_eq!(path.sub_path, "");
        assert!(!path.is_null());
        
        let null_path = SoftObjectPath::new("", "");
        assert!(null_path.is_null());
    }
    
    #[test]
    fn test_property_numeric_conversion() {
        let float_prop = Property::Float(3.14);
        assert_eq!(ConversionUtils::property_as_float(&float_prop), Some(3.14f64));
        
        let int_prop = Property::Int32(42);
        assert_eq!(ConversionUtils::property_as_float(&int_prop), Some(42.0f64));
        
        let string_prop = Property::String("not a number".to_string());
        assert_eq!(ConversionUtils::property_as_float(&string_prop), None);
    }
}
