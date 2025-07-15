
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::unreal_asset::error::UnrealAssetError;
use crate::unreal_asset::error::UnrealAssetResult;
use crate::unreal_asset::exports::Export;
use crate::unreal_asset::Import;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PackageIndex(pub i32);

impl PackageIndex {
    /// Create a null package index
    pub const fn null() -> Self {
        PackageIndex(0)
    }
    
    /// Check if this is a null reference
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
    
    /// Check if this references an import (negative value)
    pub fn is_import(&self) -> bool {
        self.0 < 0
    }
    
    /// Check if this references an export (positive value)
    pub fn is_export(&self) -> bool {
        self.0 > 0
    }
    
    /// Get import index (converts from negative package index)
    pub fn import_index(&self) -> Option<usize> {
        if self.is_import() {
            Some((-self.0 - 1) as usize)
        } else {
            None
        }
    }
    
    /// Get export index (converts from positive package index)
    pub fn export_index(&self) -> Option<usize> {
        if self.is_export() {
            Some((self.0 - 1) as usize)
        } else {
            None
        }
    }
    
    /// Create from import index
    pub fn from_import(index: usize) -> Self {
        PackageIndex(-(index as i32) - 1)
    }
    
    /// Create from export index
    pub fn from_export(index: usize) -> Self {
        PackageIndex((index as i32) + 1)
    }
    
    /// Stove compatibility: Create a new PackageIndex (alias for from_export)
    pub fn new(index: i32) -> Self {
        PackageIndex(index)
    }
    
    /// Stove compatibility: Get the raw index value (alias for .0)
    pub fn index(&self) -> i32 {
        self.0
    }
}

pub trait PackageIndexTrait {
    /// Get the raw package index value
    fn get_index(&self) -> i32;
    /// Check if this is a null reference
    fn is_null(&self) -> bool;
    /// Check if this references an import
    fn is_import(&self) -> bool;
    /// Check if this references an export  
    fn is_export(&self) -> bool;
}

impl PackageIndexTrait for PackageIndex {
    fn get_index(&self) -> i32 { self.0 }
    fn is_null(&self) -> bool { PackageIndex::is_null(self) }
    fn is_import(&self) -> bool { PackageIndex::is_import(self) }
    fn is_export(&self) -> bool { PackageIndex::is_export(self) }
}

pub type Guid = [u32; 4];

pub trait ToSerializedName {
    fn to_serialized_name(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FName {
    /// The string value of this name
    pub name: String,
    /// Optional numeric suffix for duplicate names
    pub number: u32,
}

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
    
    /// Compare content with another FName
    pub fn eq_content(&self, other: &FName) -> bool {
        self.name == other.name
    }
    
    /// Get owned content
    pub fn get_owned_content(&self) -> String {
        self.name.clone()
    }
    
    /// Get content with callback
    pub fn get_content<R>(&self, callback: impl FnOnce(&str) -> R) -> R {
        callback(&self.name)
    }
}

impl ToSerializedName for FName {
    fn to_serialized_name(&self) -> String {
        if self.number == 0 {
            self.name.clone()
        } else {
            format!("{}_{}", self.name, self.number)
        }
    }
}

impl Default for FName {
    fn default() -> Self {
        Self {
            name: String::new(),
            number: 0,
        }
    }
}

impl std::fmt::Display for FName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.number == 0 {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{}_{}", self.name, self.number)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::zero()
    }
}

impl Default for Vector4 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Default for Vector2D {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rotator {
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}

impl Rotator {
    pub fn new(pitch: f64, yaw: f64, roll: f64) -> Self {
        Self { pitch, yaw, roll }
    }
    
    pub fn zero() -> Self {
        Self { pitch: 0.0, yaw: 0.0, roll: 0.0 }
    }
}

impl Default for Rotator {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quat {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
    
    pub fn identity() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }

    pub fn from_euler(roll: f64, pitch: f64, yaw: f64) -> Self {
        let cr = (roll * 0.5).cos();
        let sr = (roll * 0.5).sin();
        let cp = (pitch * 0.5).cos();
        let sp = (pitch * 0.5).sin();
        let cy = (yaw * 0.5).cos();
        let sy = (yaw * 0.5).sin();
 
        Self {
            w: cr * cp * cy + sr * sp * sy,
            x: sr * cp * cy - cr * sp * sy,
            y: cr * sp * cy + sr * cp * sy,
            z: cr * cp * sy - sr * sp * cy,
        }
    }
}

impl Default for Quat {
    fn default() -> Self {
        Self::identity()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

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

impl Default for LinearColor {
    fn default() -> Self {
        Self::white()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub location: Vector,
    pub rotation: Quat,
    pub scale: Vector,
}

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

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoftObjectPath {
    pub asset_path: FName,
    pub sub_path: String,
}

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

impl Default for SoftObjectPath {
    fn default() -> Self {
        Self::new("", "")
    }
}

#[derive(Debug, Clone)]
pub enum ObjectReference {
    /// Null reference
    Null,
    /// Reference to an imported object
    Import {
        index: usize,
        class_package: FName,
        class_name: FName,
        object_name: FName,
        outer_index: PackageIndex,
        package_guid: Option<Uuid>,
    },
    /// Reference to an exported object
    Export {
        index: usize,
        class_index: PackageIndex,
        super_index: PackageIndex,
        template_index: PackageIndex,
        outer_index: PackageIndex,
        object_name: FName,
        object_flags: u32,
    },
}

impl ObjectReference {
    /// Get the object name
    pub fn get_name(&self) -> Option<&FName> {
        match self {
            ObjectReference::Import { object_name, .. } => Some(object_name),
            ObjectReference::Export { object_name, .. } => Some(object_name),
            ObjectReference::Null => None,
        }
    }
    
    /// Check if this is an import reference
    pub fn is_import(&self) -> bool {
        matches!(self, ObjectReference::Import { .. })
    }
    
    /// Check if this is an export reference
    pub fn is_export(&self) -> bool {
        matches!(self, ObjectReference::Export { .. })
    }
    
    /// Check if this is a null reference
    pub fn is_null(&self) -> bool {
        matches!(self, ObjectReference::Null)
    }
}

pub struct PackageIndexResolver<'a> {
    imports: &'a [Import],
    exports: &'a [Export],
    name_map: &'a [String],
}

impl<'a> PackageIndexResolver<'a> {
    /// Create a new package index resolver
    pub fn new(imports: &'a [Import], exports: &'a [Export], name_map: &'a [String]) -> Self {
        Self {
            imports,
            exports,
            name_map,
        }
    }
    
    /// Resolve a package index to object information
    pub fn resolve(&self, index: PackageIndex) -> UnrealAssetResult<ObjectReference> {
        if index.is_null() {
            return Ok(ObjectReference::Null);
        }
        
        if index.is_import() {
            let import_idx = index.import_index()
                .ok_or_else(|| UnrealAssetError::new("Invalid import index"))?;
            
            if import_idx >= self.imports.len() {
                return Err(UnrealAssetError::new("Import index out of bounds"));
            }
            
            let import = &self.imports[import_idx];
            Ok(ObjectReference::Import {
                index: import_idx,
                class_package: import.class_package.clone(),
                class_name: import.class_name.clone(),
                object_name: import.object_name.clone(),
                outer_index: import.outer_index,
                package_guid: import.package_guid,
            })
        } else if index.is_export() {
            let export_idx = index.export_index()
                .ok_or_else(|| UnrealAssetError::new("Invalid export index"))?;
            
            if export_idx >= self.exports.len() {
                return Err(UnrealAssetError::new("Export index out of bounds"));
            }
            
            let export = &self.exports[export_idx];
            Ok(ObjectReference::Export {
                index: export_idx,
                class_index: export.class_index,
                super_index: export.super_index,
                template_index: export.template_index,
                outer_index: export.outer_index,
                object_name: export.object_name.clone(),
                object_flags: export.object_flags,
            })
        } else {
            Err(UnrealAssetError::new("Invalid package index"))
        }
    }
} 