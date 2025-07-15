
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::unreal_asset::properties::Property;
use crate::unreal_asset::types::{FName, PackageIndex};

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
    pub package_guid: Option<uuid::Uuid>,
    /// Package name for compatibility
    pub package_name: FName,
}

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
    /// Dependencies created before serialization
    pub create_before_serialization_dependencies: Vec<PackageIndex>,
}

pub trait ExportBaseTrait {
    /// Get the export's object name
    fn get_object_name(&self) -> &FName;
    /// Get the export's class index
    fn get_class_index(&self) -> PackageIndex;
    /// Get the export's outer index  
    fn get_outer_index(&self) -> PackageIndex;
}

pub trait ExportNormalTrait: ExportBaseTrait {
    /// Get export properties
    fn get_properties(&self) -> &IndexMap<String, Property>;
    /// Get mutable export properties
    fn get_properties_mut(&mut self) -> &mut IndexMap<String, Property>;
    /// Get export extras
    fn get_extras(&self) -> Option<&serde_json::Value>;
}

impl ExportBaseTrait for Export {
    fn get_object_name(&self) -> &FName {
        &self.object_name
    }
    
    fn get_class_index(&self) -> PackageIndex {
        self.class_index
    }
    
    fn get_outer_index(&self) -> PackageIndex {
        self.outer_index
    }
}

impl ExportNormalTrait for Export {
    fn get_properties(&self) -> &IndexMap<String, Property> {
        &self.properties
    }
    
    fn get_properties_mut(&mut self) -> &mut IndexMap<String, Property> {
        &mut self.properties
    }
    
    fn get_extras(&self) -> Option<&serde_json::Value> {
        self.extras.as_ref()
    }
}

impl Export {
    /// Create a new export
    pub fn new(object_name: FName, class_index: PackageIndex) -> Self {
        Self {
            class_index,
            super_index: PackageIndex::null(),
            template_index: PackageIndex::null(),
            outer_index: PackageIndex::null(),
            object_name,
            object_flags: 0,
            serial_size: 0,
            serial_offset: 0,
            export_flags: 0,
            properties: IndexMap::new(),
            extras: None,
            create_before_serialization_dependencies: Vec::new(),
        }
    }
    
    /// Get base export data
    pub fn get_base_export(&self) -> &Self {
        self
    }
    
    /// Get mutable base export data
    pub fn get_base_export_mut(&mut self) -> &mut Self {
        self
    }
    
    /// Get normal export data
    pub fn get_normal_export(&self) -> &Self {
        self
    }
    
    /// Get mutable normal export data
    pub fn get_normal_export_mut(&mut self) -> &mut Self {
        self
    }
} 