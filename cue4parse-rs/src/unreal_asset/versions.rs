
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectVersionUE5(pub i32);

impl ObjectVersionUE5 {
    pub const fn new(version: i32) -> Self {
        ObjectVersionUE5(version)
    }

    pub fn get(&self) -> i32 {
        self.0
    }

    pub fn supports_feature(&self, feature: UE5Feature) -> bool {
        self.0 >= feature.min_version()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UE5Feature {
    /// Enhanced name map with hashing
    OptimizedNameMap,
    /// Large world coordinates
    LargeWorldCoordinates,
    /// Improved bulk data handling
    BulkDataV2,
    /// New property serialization
    PropertySerializationV2,
    /// Enhanced dependency tracking
    DependencyTracking,
}

impl UE5Feature {
    pub fn min_version(self) -> i32 {
        match self {
            UE5Feature::OptimizedNameMap => 1001,
            UE5Feature::LargeWorldCoordinates => 1002,
            UE5Feature::BulkDataV2 => 1003,
            UE5Feature::PropertySerializationV2 => 1004,
            UE5Feature::DependencyTracking => 1005,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EngineVersion {
    // UE4 versions
    VerUe4_0 = 342,
    VerUe4_1 = 352,
    VerUe4_2 = 363,
    VerUe4_3 = 382,
    VerUe4_4 = 385,
    VerUe4_5 = 401,
    VerUe4_6 = 413,
    VerUe4_7 = 434,
    VerUe4_8 = 451,
    VerUe4_9 = 482,
    VerUe4_10 = 491,
    VerUe4_11 = 498,
    VerUe4_12 = 504,
    VerUe4_13 = 505,
    VerUe4_14 = 508,
    VerUe4_15 = 509,
    VerUe4_16 = 510,
    VerUe4_17 = 513,
    VerUe4_18 = 514,
    VerUe4_19 = 516,
    VerUe4_20 = 517,
    VerUe4_21 = 518,
    VerUe4_22 = 519,
    VerUe4_23 = 520,
    VerUe4_24 = 521,
    VerUe4_25 = 522,
    VerUe4_26 = 523,
    VerUe4_27 = 524,
    
    /// Automatic version plus one (used for version detection)
    VerUe4AutomaticVersionPlusOne = 525,
    
    // UE5 versions
    VerUe5_0 = 1001,
    VerUe5_1 = 1002,
    VerUe5_2 = 1003,
    VerUe5_3 = 1004,
    VerUe5_4 = 1005,
    VerUe5_5 = 1006,
}

impl EngineVersion {
    /// Create an `EngineVersion` from a raw version number
    pub fn from_version(version: i32) -> Option<Self> {
        match version {
            342 => Some(EngineVersion::VerUe4_0),
            352 => Some(EngineVersion::VerUe4_1),
            363 => Some(EngineVersion::VerUe4_2),
            382 => Some(EngineVersion::VerUe4_3),
            385 => Some(EngineVersion::VerUe4_4),
            401 => Some(EngineVersion::VerUe4_5),
            413 => Some(EngineVersion::VerUe4_6),
            434 => Some(EngineVersion::VerUe4_7),
            451 => Some(EngineVersion::VerUe4_8),
            482 => Some(EngineVersion::VerUe4_9),
            491 => Some(EngineVersion::VerUe4_10),
            498 => Some(EngineVersion::VerUe4_11),
            504 => Some(EngineVersion::VerUe4_12),
            505 => Some(EngineVersion::VerUe4_13),
            508 => Some(EngineVersion::VerUe4_14),
            509 => Some(EngineVersion::VerUe4_15),
            510 => Some(EngineVersion::VerUe4_16),
            513 => Some(EngineVersion::VerUe4_17),
            514 => Some(EngineVersion::VerUe4_18),
            516 => Some(EngineVersion::VerUe4_19),
            517 => Some(EngineVersion::VerUe4_20),
            518 => Some(EngineVersion::VerUe4_21),
            519 => Some(EngineVersion::VerUe4_22),
            520 => Some(EngineVersion::VerUe4_23),
            521 => Some(EngineVersion::VerUe4_24),
            522 => Some(EngineVersion::VerUe4_25),
            523 => Some(EngineVersion::VerUe4_26),
            524 => Some(EngineVersion::VerUe4_27),
            525 => Some(EngineVersion::VerUe4AutomaticVersionPlusOne),
            1001 => Some(EngineVersion::VerUe5_0),
            1002 => Some(EngineVersion::VerUe5_1),
            1003 => Some(EngineVersion::VerUe5_2),
            1004 => Some(EngineVersion::VerUe5_3),
            1005 => Some(EngineVersion::VerUe5_4),
            1006 => Some(EngineVersion::VerUe5_5),
            _ => None,
        }
    }
    
    /// Get the raw version number
    pub fn version(&self) -> i32 {
        *self as i32
    }
    
    /// Check if this version is from Unreal Engine 4
    pub fn is_ue4(&self) -> bool {
        let version = self.version();
        version >= 342 && version <= 525
    }
    
    /// Check if this version is from Unreal Engine 5
    pub fn is_ue5(&self) -> bool {
        let version = self.version();
        version >= 1001 && version <= 1006
    }
}

/// Helper enum for categorizing game engine versions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GameEngineVersion {
    /// UE4 game version
    GameUe4(EngineVersion),
    /// UE5 game version
    GameUe5(EngineVersion),
}

impl GameEngineVersion {
    /// Create from a generic `EngineVersion`
    pub fn from_engine_version(version: EngineVersion) -> Self {
        if version.is_ue5() {
            GameEngineVersion::GameUe5(version)
        } else {
            GameEngineVersion::GameUe4(version)
        }
    }
    
    /// Get the underlying `EngineVersion`
    pub fn engine_version(&self) -> EngineVersion {
        match self {
            GameEngineVersion::GameUe4(v) => *v,
            GameEngineVersion::GameUe5(v) => *v,
        }
    }
}

/// Legacy object version enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ObjectVersion(pub i32);

impl ObjectVersion {
    pub const fn new(version: i32) -> Self {
        ObjectVersion(version)
    }

    pub fn get(&self) -> i32 {
        self.0
    }
    
    pub const VER_UE4_STATIC_MESH_STORE_NAV_COLLISION: i32 = 1_000_000;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CustomVersion {
    /// GUID identifying the custom version
    pub guid: Uuid,
    /// Version number
    pub version: i32,
    /// Friendly name for this custom version
    pub friendly_name: String,
}

impl CustomVersion {
    /// Create a new `CustomVersion`
    pub fn new(guid: Uuid, version: i32, friendly_name: String) -> Self {
        Self {
            guid,
            version,
            friendly_name,
        }
    }
    
    /// Check if this custom version is compatible with another
    pub fn is_compatible_with(&self, other: &CustomVersion) -> bool {
        self.guid == other.guid && self.version >= other.version
    }
}

pub trait CustomVersionTrait {
    fn guid() -> Uuid;
}

/// Registry for managing game-specific custom versions
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomVersionRegistry {
    /// Registered custom versions by GUID
    versions: HashMap<Uuid, CustomVersion>,
    /// Game-specific version mappings
    game_versions: HashMap<String, Vec<CustomVersion>>,
}

impl CustomVersionRegistry {
    /// Create a new registry and register default versions
    pub fn new() -> Self {
        let mut registry = Self {
            versions: HashMap::new(),
            game_versions: HashMap::new(),
        };
        registry.register_unreal_engine_versions();
        registry.register_game_specific_versions();
        registry
    }
    
    /// Register a new custom version
    pub fn register(&mut self, version: CustomVersion) {
        self.versions.insert(version.guid, version);
    }
    
    /// Get a custom version by GUID
    pub fn get(&self, guid: &Uuid) -> Option<&CustomVersion> {
        self.versions.get(guid)
    }
    
    /// Get all custom versions for a specific game
    pub fn get_game_versions(&self, game: &str) -> Option<&Vec<CustomVersion>> {
        self.game_versions.get(game)
    }
    
    /// Register all standard Unreal Engine custom versions
    fn register_unreal_engine_versions(&mut self) {
        // Example: self.register(CustomVersion::new(guid, version, name));
        // This would be populated with known UE GUIDs
    }
    
    /// Register custom versions for known games
    fn register_game_specific_versions(&mut self) {
        // Example for Fortnite
        let fortnite_versions = vec![
            // Populate with Fortnite-specific versions
        ];
        self.game_versions.insert("Fortnite".to_string(), fortnite_versions);
    }
    
    /// Validate a list of versions against the registry
    pub fn validate_versions(&self, versions: &[CustomVersion]) -> Vec<String> {
        let mut errors = Vec::new();
        for version in versions {
            if let Some(registered) = self.get(&version.guid) {
                if version.version > registered.version {
                    errors.push(format!(
                        "Version mismatch for {}: asset version {} > registered version {}",
                        registered.friendly_name, version.version, registered.version
                    ));
                }
            } else {
                errors.push(format!("Unregistered GUID: {}", version.guid));
            }
        }
        errors
    }
}

impl Default for CustomVersionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global static custom version registry
static REGISTRY: once_cell::sync::Lazy<CustomVersionRegistry> =
    once_cell::sync::Lazy::new(CustomVersionRegistry::new);

/// Get a reference to the global `CustomVersionRegistry`
pub fn get_custom_version_registry() -> &'static CustomVersionRegistry {
    &REGISTRY
} 