
use std::collections::HashMap;
use std::io::{Read, Write, Seek};
use indexmap::IndexMap;
use uuid::Uuid;
use crate::unreal_asset::error::{Error, UnrealAssetError, UnrealAssetResult};
use crate::unreal_asset::types::{FName, PackageIndex, PackageIndexTrait, ToSerializedName};
use crate::unreal_asset::versions::{CustomVersion, CustomVersionTrait, EngineVersion, ObjectVersion, ObjectVersionUE5};
use crate::unreal_asset::containers::{NameMap, SharedResource};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ArchiveType {
    /// Raw archive
    Raw,
    /// Archive used to read .uasset/.uexp files
    UAsset,
    /// Archive used to read .usmap files
    Usmap,
    /// Archive used to read zen files
    Zen,
}

impl std::fmt::Display for ArchiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveType::Raw => write!(f, "Raw"),
            ArchiveType::UAsset => write!(f, "UAsset"),
            ArchiveType::Usmap => write!(f, "Usmap"),
            ArchiveType::Zen => write!(f, "Zen"),
        }
    }
}

pub trait ArchiveTrait<Index: PackageIndexTrait>: Seek {
    /// Get archive type
    fn get_archive_type(&self) -> ArchiveType;

    /// Get a custom version from this archive
    fn get_custom_version<T>(&self) -> CustomVersion
    where
        T: CustomVersionTrait + Into<i32>;

    /// Get if the asset has unversioned properties
    fn has_unversioned_properties(&self) -> bool;

    /// Get if the archive uses the event driven loader
    fn use_event_driven_loader(&self) -> bool;

    /// Archive data length
    fn data_length(&mut self) -> std::io::Result<u64> {
        let current_position = self.position();
        self.seek(std::io::SeekFrom::End(0))?;
        let length = self.position();
        self.seek(std::io::SeekFrom::Start(current_position))?;
        Ok(length)
    }
    
    /// Current archive cursor position
    fn position(&mut self) -> u64;
    
    /// Set archive cursor position
    fn set_position(&mut self, pos: u64) -> std::io::Result<()> {
        self.seek(std::io::SeekFrom::Start(pos))?;
        Ok(())
    }

    /// Add a string slice to this archive as an `FName`, `FName` number will be 0
    fn add_fname(&mut self, value: &str) -> FName {
        let mut binding = self.get_name_map();
        let mut name_map = binding.get_mut();
        if let Some(_index) = name_map.iter().position(|n| n == value) {
            FName::new(value)
        } else {
            name_map.push(value.to_string());
            FName::new(value)
        }
    }
    
    /// Add a string slice to this archive as an `FName`
    fn add_fname_with_number(&mut self, value: &str, number: i32) -> FName {
        let mut binding = self.get_name_map();
        let mut name_map = binding.get_mut();
        if let Some(_index) = name_map.iter().position(|n| n == value) {
            FName::with_number(value, number as u32)
        } else {
            name_map.push(value.to_string());
            FName::with_number(value, number as u32)
        }
    }

    /// Get FName name map
    fn get_name_map(&self) -> SharedResource<NameMap>;
    
    /// Get FName name reference by name map index and do something with it
    fn get_name_reference<T>(&self, index: i32, func: impl FnOnce(&str) -> T) -> T {
        let name_map = self.get_name_map();
        let name_ref = name_map.get_ref().get(index as usize).map(|s| s.as_str()).unwrap_or("");
        func(name_ref)
    }
    
    /// Get FName name by name map index as a `String`
    fn get_owned_name(&self, index: i32) -> String {
        self.get_name_map().get_ref().get(index as usize).cloned().unwrap_or_default()
    }

    /// Get struct overrides for an `ArrayProperty`
    fn get_array_struct_type_override(&self) -> &IndexMap<String, String>;
    
    /// Get map key overrides for a `MapProperty`
    fn get_map_key_override(&self) -> &IndexMap<String, String>;
    
    /// Get map value overrides for a `MapProperty`
    fn get_map_value_override(&self) -> &IndexMap<String, String>;

    /// Get archive's engine version
    fn get_engine_version(&self) -> EngineVersion;
    
    /// Get archive's object version
    fn get_object_version(&self) -> ObjectVersion;
    
    /// Get archive's UE5 object version
    fn get_object_version_ue5(&self) -> ObjectVersionUE5;

    /// Get .usmap mappings
    fn get_mappings(&self) -> Option<&()> { // TODO: Usmap
        None // Would need to be stored in the archive
    }

    /// Get parent class export name
    fn get_parent_class_export_name(&self) -> Option<FName>;

    /// Get object name by an `Index`
    fn get_object_name(&self, index: Index) -> Option<FName>;
    
    /// Get object name by a `PackageIndex`
    fn get_object_name_packageindex(&self, index: PackageIndex) -> Option<FName>;
    
    /// Get export class type by an `Index`
    fn get_export_class_type(&self, index: Index) -> Option<FName> {
        match index.is_import() {
            true => self.get_object_name(index),
            false => Some(FName::new(index.get_index().to_string())),
        }
    }
}

pub trait ArchiveReader<Index: PackageIndexTrait>: ArchiveTrait<Index> + Read {
    /// Read a `Guid` property
    fn read_property_guid(&mut self) -> UnrealAssetResult<Option<Uuid>> {
        if self.get_object_version().get() >= 522 { // VER_UE4_25
            let has_property_guid = self.read_bool()?;
            if has_property_guid {
                return Ok(Some(self.read_guid()?));
            }
        }
        Ok(None)
    }

    /// Read an `FName`
    fn read_fname(&mut self) -> UnrealAssetResult<FName> {
        use byteorder::{ReadBytesExt, LittleEndian};
        
        let index = self.read_i32::<LittleEndian>()?;
        let number = self.read_i32::<LittleEndian>()?;

        let name_map_size = self
            .get_name_map()
            .get_ref()
            .len();
            
        if index < 0 || index >= name_map_size as i32 {
            return Err(Error::InvalidIndex(index).into());
        }

        let name = self.get_owned_name(index);
        Ok(FName::with_number(name, number as u32))
    }

    /// Read an array with specified length
    fn read_array_with_length<T>(
        &mut self,
        length: i32,
        getter: impl Fn(&mut Self) -> UnrealAssetResult<T>,
    ) -> UnrealAssetResult<Vec<T>> {
        let mut array = Vec::with_capacity(length as usize);
        for _ in 0..length {
            array.push(getter(self)?);
        }
        Ok(array)
    }

    /// Read an array with the length being read from this archive
    fn read_array<T>(
        &mut self,
        getter: impl Fn(&mut Self) -> UnrealAssetResult<T>,
    ) -> UnrealAssetResult<Vec<T>> {
        use byteorder::{ReadBytesExt, LittleEndian};
        let length = self.read_i32::<LittleEndian>()?;
        self.read_array_with_length(length, getter)
    }

    /// Read an FString
    fn read_fstring(&mut self) -> UnrealAssetResult<Option<String>> {
        use byteorder::{ReadBytesExt, LittleEndian};
        
        // Simple string reading implementation
        let length = self.read_i32::<LittleEndian>()?;
        if length == 0 {
            return Ok(None);
        }
        
        if length < 0 {
            // Unicode string (UCS-2)
            let char_count = (-length) as usize;
            let mut buffer = vec![0u16; char_count];
            for i in 0..char_count {
                buffer[i] = self.read_u16::<LittleEndian>()?;
            }
            // Convert UTF-16 to String, removing null terminator
            let s = String::from_utf16_lossy(&buffer[..char_count.saturating_sub(1)]);
            Ok(Some(s))
        } else {
            // ANSI string
            let mut buffer = vec![0u8; length as usize];
            self.read_exact(&mut buffer)?;
            // Remove null terminator if present
            if let Some(&0) = buffer.last() {
                buffer.pop();
            }
            Ok(Some(String::from_utf8_lossy(&buffer).into_owned()))
        }
    }
    
    /// Read a guid
    fn read_guid(&mut self) -> std::io::Result<Uuid> {
        use byteorder::{ReadBytesExt, LittleEndian};
        
        let a = self.read_u32::<LittleEndian>()?;
        let b = self.read_u32::<LittleEndian>()?;
        let c = self.read_u32::<LittleEndian>()?;
        let d = self.read_u32::<LittleEndian>()?;
        
        // Convert UE4 GUID format to UUID
        let bytes = [
            (a & 0xFF) as u8,
            ((a >> 8) & 0xFF) as u8,
            ((a >> 16) & 0xFF) as u8,
            ((a >> 24) & 0xFF) as u8,
            (b & 0xFF) as u8,
            ((b >> 8) & 0xFF) as u8,
            ((b >> 16) & 0xFF) as u8,
            ((b >> 24) & 0xFF) as u8,
            (c & 0xFF) as u8,
            ((c >> 8) & 0xFF) as u8,
            ((c >> 16) & 0xFF) as u8,
            ((c >> 24) & 0xFF) as u8,
            (d & 0xFF) as u8,
            ((d >> 8) & 0xFF) as u8,
            ((d >> 16) & 0xFF) as u8,
            ((d >> 24) & 0xFF) as u8,
        ];
        
        Ok(Uuid::from_bytes(bytes))
    }
    
    /// Read `bool`
    fn read_bool(&mut self) -> std::io::Result<bool> {
        use byteorder::ReadBytesExt;
        Ok(self.read_u8()? != 0)
    }
}

pub trait ArchiveWriter<Index: PackageIndexTrait>: ArchiveTrait<Index> + Write {
    /// Write a `Guid` property
    fn write_property_guid(&mut self, guid: Option<&Uuid>) -> UnrealAssetResult<()> {
        if self.get_object_version().get() >= 522 { // VER_UE4_25
            self.write_bool(guid.is_some())?;
            if let Some(data) = guid {
                self.write_guid(data)?;
            }
        }
        Ok(())
    }

    /// Write an `FName`
    fn write_fname(&mut self, fname: &FName) -> UnrealAssetResult<()> {
        use byteorder::{WriteBytesExt, LittleEndian};
        
        // For compatibility, we'll need to find the name in the map
        let name_map = self.get_name_map();
        let name_ref = name_map.get_ref();
        
        if let Some(index) = name_ref.iter().position(|n| n == &fname.name) {
            self.write_i32::<LittleEndian>(index as i32)?;
            self.write_i32::<LittleEndian>(fname.number as i32)?;
        } else {
            return Err(UnrealAssetError::new(&format!("FName '{}' not found in name map", fname.name)));
        }
        
        Ok(())
    }

    /// Write an FString
    fn write_fstring(&mut self, value: Option<&str>) -> UnrealAssetResult<usize> {
        use byteorder::{WriteBytesExt, LittleEndian};
        
        match value {
            Some(s) if !s.is_empty() => {
                // Write as ANSI string
                let bytes = s.as_bytes();
                self.write_i32::<LittleEndian>((bytes.len() + 1) as i32)?; // +1 for null terminator
                self.write_all(bytes)?;
                self.write_u8(0)?; // null terminator
                Ok(bytes.len() + 1)
            }
            _ => {
                // Empty string
                self.write_i32::<LittleEndian>(0)?;
                Ok(0)
            }
        }
    }
    
    /// Write a guid
    fn write_guid(&mut self, guid: &Uuid) -> std::io::Result<()> {
        use byteorder::{WriteBytesExt, LittleEndian};
        
        let bytes = guid.as_bytes();
        
        // Convert UUID to UE4 GUID format (4 u32s)
        let a = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let b = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let c = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let d = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
        
        self.write_u32::<LittleEndian>(a)?;
        self.write_u32::<LittleEndian>(b)?;
        self.write_u32::<LittleEndian>(c)?;
        self.write_u32::<LittleEndian>(d)?;
        
        Ok(())
    }
    
    /// Write `bool`
    fn write_bool(&mut self, value: bool) -> std::io::Result<()> {
        use byteorder::WriteBytesExt;
        self.write_u8(if value { 1 } else { 0 })
    }
}

pub struct BinaryArchive<R: Read + Seek> {
    reader: R,
    name_map: SharedResource<NameMap>,
    object_version: ObjectVersion,
    object_version_ue5: ObjectVersionUE5,
    engine_version: EngineVersion,
    custom_versions: HashMap<String, i32>,
}

impl<R: Read + Seek> BinaryArchive<R> {
    /// Create a new binary archive
    pub fn new(reader: R, engine_version: EngineVersion) -> Self {
        Self {
            reader,
            name_map: SharedResource::new(NameMap::new()),
            object_version: ObjectVersion::new(match engine_version {
                EngineVersion::VerUe4_27 => 522,
                EngineVersion::VerUe5_0 => 524,
                EngineVersion::VerUe5_1 => 525,
                EngineVersion::VerUe5_2 => 526,
                EngineVersion::VerUe5_3 => 527,
                _ => 522,
            }),
            object_version_ue5: ObjectVersionUE5::new(0),
            engine_version,
            custom_versions: HashMap::new(),
        }
    }
    
    /// Static method to read FString from a reader
    pub fn read_fstring_static<R2: Read + Seek>(reader: &mut R2) -> UnrealAssetResult<Option<String>> {
        use byteorder::{ReadBytesExt, LittleEndian};
        
        let length = reader.read_i32::<LittleEndian>()?;
        
        if length == 0 {
            return Ok(None);
        }
        
        if length < 0 {
            // Unicode string (UTF-16)
            let char_count = (-length) as usize;
            let mut buffer = vec![0u16; char_count];
            for i in 0..char_count {
                buffer[i] = reader.read_u16::<LittleEndian>()?;
            }
            // Remove null terminator if present
            if !buffer.is_empty() && buffer[buffer.len() - 1] == 0 {
                buffer.pop();
            }
            String::from_utf16(&buffer)
                .map(Some)
                .map_err(|_| Error::Serialization("Invalid UTF-16 string".to_string()).into())
        } else {
            // ASCII string
            let byte_count = length as usize;
            let mut buffer = vec![0u8; byte_count];
            reader.read_exact(&mut buffer)?;
            // Remove null terminator if present
            if !buffer.is_empty() && buffer[buffer.len() - 1] == 0 {
                buffer.pop();
            }
            String::from_utf8(buffer)
                .map(Some)
                .map_err(|_| Error::Serialization("Invalid UTF-8 string".to_string()).into())
        }
    }
}

impl<R: Read + Seek> ArchiveTrait<PackageIndex> for BinaryArchive<R> {
    fn get_archive_type(&self) -> ArchiveType {
        ArchiveType::UAsset
    }
    
    fn get_object_version(&self) -> ObjectVersion {
        self.object_version
    }
    
    fn get_object_version_ue5(&self) -> ObjectVersionUE5 {
        self.object_version_ue5
    }
    
    fn get_engine_version(&self) -> EngineVersion {
        self.engine_version
    }
    
    fn get_custom_version<T: CustomVersionTrait>(&self) -> CustomVersion {
        // Create a default CustomVersion - in a real implementation this would look up the actual version
        CustomVersion {
            guid: T::guid(),
            version: self.custom_versions.get(&T::guid().to_string()).copied().unwrap_or(0),
            friendly_name: "Unknown".to_string(),
        }
    }
    
    fn get_mappings(&self) -> Option<&()> { // TODO: Usmap
        None // Would need to be stored in the archive
    }
    
    fn use_event_driven_loader(&self) -> bool {
        false // Default implementation
    }
    
    fn get_name_map(&self) -> SharedResource<NameMap> {
        self.name_map.clone()
    }
    
    fn get_parent_class_export_name(&self) -> Option<FName> {
        None // Would need export table access
    }
    
    fn get_object_name(&self, _index: PackageIndex) -> Option<FName> {
        None // Would need import/export tables
    }
    
    fn get_object_name_packageindex(&self, _index: PackageIndex) -> Option<FName> {
        None // Would need import/export tables
    }
    
    fn has_unversioned_properties(&self) -> bool {
        false // Default implementation - could be configured per archive
    }
    
    fn position(&mut self) -> u64 {
        self.reader.stream_position().unwrap_or(0)
    }
    
    fn get_array_struct_type_override(&self) -> &IndexMap<String, String> {
        // Return empty map - in a real implementation this would be populated
        static EMPTY_MAP: std::sync::OnceLock<IndexMap<String, String>> = std::sync::OnceLock::new();
        EMPTY_MAP.get_or_init(|| IndexMap::new())
    }
    
    fn get_map_key_override(&self) -> &IndexMap<String, String> {
        // Return empty map - in a real implementation this would be populated
        static EMPTY_MAP: std::sync::OnceLock<IndexMap<String, String>> = std::sync::OnceLock::new();
        EMPTY_MAP.get_or_init(|| IndexMap::new())
    }
    
    fn get_map_value_override(&self) -> &IndexMap<String, String> {
        // Return empty map - in a real implementation this would be populated
        static EMPTY_MAP: std::sync::OnceLock<IndexMap<String, String>> = std::sync::OnceLock::new();
        EMPTY_MAP.get_or_init(|| IndexMap::new())
    }
}

impl<R: Read + Seek> Read for BinaryArchive<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read + Seek> Seek for BinaryArchive<R> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.reader.seek(pos)
    }
}

impl<R: Read + Seek> ArchiveReader<PackageIndex> for BinaryArchive<R> {}

pub struct BinaryArchiveWriter<W: Write + Seek> {
    writer: W,
    name_map: SharedResource<NameMap>,
    object_version: ObjectVersion,
    object_version_ue5: ObjectVersionUE5,
    engine_version: EngineVersion,
    custom_versions: HashMap<String, i32>,
}

impl<W: Write + Seek> BinaryArchiveWriter<W> {
    /// Create a new binary archive writer
    pub fn new(writer: W, engine_version: EngineVersion) -> Self {
        Self {
            writer,
            name_map: SharedResource::new(NameMap::new()),
            object_version: ObjectVersion::new(match engine_version {
                EngineVersion::VerUe4_27 => 522,
                EngineVersion::VerUe5_0 => 524,
                EngineVersion::VerUe5_1 => 525,
                EngineVersion::VerUe5_2 => 526,
                EngineVersion::VerUe5_3 => 527,
                _ => 522,
            }),
            object_version_ue5: ObjectVersionUE5::new(0),
            engine_version,
            custom_versions: HashMap::new(),
        }
    }
}

impl<W: Write + Seek> ArchiveTrait<PackageIndex> for BinaryArchiveWriter<W> {
    fn get_archive_type(&self) -> ArchiveType {
        ArchiveType::UAsset
    }
    
    fn get_object_version(&self) -> ObjectVersion {
        self.object_version
    }
    
    fn get_object_version_ue5(&self) -> ObjectVersionUE5 {
        self.object_version_ue5
    }
    
    fn get_engine_version(&self) -> EngineVersion {
        self.engine_version
    }
    
    fn get_custom_version<T: CustomVersionTrait>(&self) -> CustomVersion {
        // Create a default CustomVersion - in a real implementation this would look up the actual version
        CustomVersion {
            guid: T::guid(),
            version: self.custom_versions.get(&T::guid().to_string()).copied().unwrap_or(0),
            friendly_name: "Unknown".to_string(),
        }
    }
    
    fn get_mappings(&self) -> Option<&()> { // TODO: Usmap
        None
    }
    
    fn use_event_driven_loader(&self) -> bool {
        false
    }
    
    fn get_name_map(&self) -> SharedResource<NameMap> {
        self.name_map.clone()
    }
    
    fn get_parent_class_export_name(&self) -> Option<FName> {
        None
    }
    
    fn get_object_name(&self, _index: PackageIndex) -> Option<FName> {
        None
    }
    
    fn get_object_name_packageindex(&self, _index: PackageIndex) -> Option<FName> {
        None
    }
    
    fn has_unversioned_properties(&self) -> bool {
        false // Default implementation - could be configured per archive
    }
    
    fn position(&mut self) -> u64 {
        self.writer.stream_position().unwrap_or(0)
    }
    
    fn get_array_struct_type_override(&self) -> &IndexMap<String, String> {
        // Return empty map - in a real implementation this would be populated
        static EMPTY_MAP: std::sync::OnceLock<IndexMap<String, String>> = std::sync::OnceLock::new();
        EMPTY_MAP.get_or_init(|| IndexMap::new())
    }
    
    fn get_map_key_override(&self) -> &IndexMap<String, String> {
        // Return empty map - in a real implementation this would be populated
        static EMPTY_MAP: std::sync::OnceLock<IndexMap<String, String>> = std::sync::OnceLock::new();
        EMPTY_MAP.get_or_init(|| IndexMap::new())
    }
    
    fn get_map_value_override(&self) -> &IndexMap<String, String> {
        // Return empty map - in a real implementation this would be populated
        static EMPTY_MAP: std::sync::OnceLock<IndexMap<String, String>> = std::sync::OnceLock::new();
        EMPTY_MAP.get_or_init(|| IndexMap::new())
    }
}

impl<W: Write + Seek> Write for BinaryArchiveWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for BinaryArchiveWriter<W> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.writer.seek(pos)
    }
}

impl<W: Write + Seek> ArchiveWriter<PackageIndex> for BinaryArchiveWriter<W> {} 