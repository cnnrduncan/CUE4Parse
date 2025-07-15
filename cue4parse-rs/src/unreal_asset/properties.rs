
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::unreal_asset::types::{
    FName, Guid, LinearColor, PackageIndex, Quat, Rotator, SoftObjectPath, Transform, Vector,
    Vector2D, Vector4,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayProperty(pub String, pub Vec<Property>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapProperty(pub String, pub String, pub Vec<(Property, Property)>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetProperty(pub String, pub Vec<Property>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructProperty {
    pub struct_type: FName,
    pub struct_guid: Option<[u32; 4]>,
    pub properties: IndexMap<String, Property>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectProperty {
    pub value: Option<PackageIndex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftObjectProperty {
    pub value: SoftObjectPath,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumProperty {
    pub enum_type: FName,
    pub value: FName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BytePropertyValue {
    /// Regular byte value
    Byte(u8),
    /// Enum byte value
    Enum {
        enum_type: FName,
        value: FName,
    },
    /// FName variant for Stove compatibility
    FName(FName),
}

pub type VectorProperty = Vector;
pub type RotatorProperty = Rotator;
pub type SoftObjectPathPropertyValue = SoftObjectPath;

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
    Byte(u8),
    
    // Missing Stove-specific property variants
    /// Weighted random sampler property
    WeightedRandomSamplerProperty(serde_json::Value),
    /// Skeletal mesh sampling LOD built data property
    SkeletalMeshSamplingLODBuiltDataProperty(serde_json::Value),
    /// Skeletal mesh area weighted triangle sampler
    SkeletalMeshAreaWeightedTriangleSampler(serde_json::Value),
    /// Soft asset path property (alias for SoftObjectPath)
    SoftAssetPathProperty(SoftObjectPath),
    /// Soft object path property (alias for SoftObjectPath)
    SoftObjectPathProperty(SoftObjectPath),
    /// Soft class path property (alias for SoftClassPath)
    SoftClassPathProperty(SoftObjectPath),
    /// Delegate property (alias for Delegate)
    DelegateProperty {
        /// Delegate object reference
        object: Option<PackageIndex>,
        /// Function name
        function_name: FName,
    },
    /// Multicast delegate property (alias for MulticastDelegate)
    MulticastDelegateProperty {
        /// Array of delegates
        delegates: Vec<Property>,
    },
    /// Multicast sparse delegate property
    MulticastSparseDelegateProperty {
        /// Array of delegates
        delegates: Vec<Property>,
    },
    /// Multicast inline delegate property
    MulticastInlineDelegateProperty {
        /// Array of delegates
        delegates: Vec<Property>,
    },
    /// Smart name property
    SmartNameProperty(FName),
    /// Struct property (alias for Struct)
    StructProperty {
        /// Name of the struct type
        struct_type: FName,
        /// Nested properties
        properties: IndexMap<String, Property>,
    },
    /// Enum property (alias for Enum)
    EnumProperty {
        /// Enum type name
        enum_type: FName,
        /// Selected enum value
        value: FName,
    },
    /// Array property (alias for Array)
    ArrayProperty(Vec<Property>),
    /// Map property (alias for Map)
    MapProperty {
        /// Map key type
        key_type: String,
        /// Map value type  
        value_type: String,
        /// Map entries
        entries: Vec<(Property, Property)>,
    },
    /// Set property (alias for Set)
    SetProperty(Vec<Property>),
    /// Object property (alias for Object)
    ObjectProperty(Option<PackageIndex>),
    
    /// Material instances with parameters
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