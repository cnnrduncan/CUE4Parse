//! # CUE4Parse Rust FFI Bindings
//!
//! This crate provides Rust bindings for the CUE4Parse library, enabling parsing and extraction
//! of Unreal Engine assets from Rust applications. It uses a hybrid approach combining FFI for
//! native compression libraries and process-based communication with a .NET CLI tool for the
//! main parsing functionality.
//!
//! ## Features
//!
//! - **Safe Rust API**: Memory-safe wrappers with proper error handling
//! - **Cross-platform**: Works on Windows, macOS, and Linux (wherever .NET 8 runs)
//! - **Native performance**: Optional FFI bindings for ACL and Oodle compression
//! - **JSON-based communication**: Structured data exchange with the C# library
//! - **Comprehensive asset support**: Parse textures, meshes, animations, and more
//!
//! ## Quick Start
//!
//! ```no_run
//! use cue4parse_rs::{Provider, GameVersion, Result};
//!
//! fn main() -> Result<()> {
//!     // Create a provider for your game files
//!     let mut provider = Provider::new("/path/to/game/files", GameVersion::UE5_3);
//!     
//!     // Add encryption keys if needed
//!     provider.add_key("your-key-guid", "your-aes-key");
//!     
//!     // Set mappings for better parsing
//!     provider.set_mappings("/path/to/mappings.usmap");
//!     
//!     // List available packages
//!     let packages = provider.list_packages()?;
//!     println!("Found {} packages", packages.len());
//!     
//!     // Load and export an object as JSON
//!     let json = provider.export_object_json("GameAssets/SomeObject")?;
//!     println!("Object data: {}", serde_json::to_string_pretty(&json)?);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! This crate uses a process-based approach where:
//! 1. The main parsing logic runs in a .NET CLI tool (`CUE4Parse.CLI`)
//! 2. Rust communicates with it via command-line arguments and JSON
//! 3. Optional native FFI is used for compression library feature detection
//!
//! This design provides:
//! - Memory safety (no complex FFI for main functionality)
//! - Full access to CUE4Parse features
//! - Easy maintenance and updates
//! - Cross-platform compatibility
//!
//! ## Error Handling
//!
//! All operations return `Result<T, CUE4ParseError>` with detailed error information:
//!
//! ```no_run
//! use cue4parse_rs::{Provider, GameVersion, CUE4ParseError};
//!
//! let provider = Provider::new("/path/to/game", GameVersion::UE5_3);
//! match provider.load_package("invalid/path") {
//!     Ok(package) => println!("Loaded package: {:?}", package),
//!     Err(CUE4ParseError::ProcessFailed(msg)) => {
//!         eprintln!("CLI tool failed: {}", msg);
//!     }
//!     Err(e) => eprintln!("Other error: {}", e),
//! }
//! ```

use std::process::Command;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "native-lib")]
use std::ffi::{CStr, CString};

// Re-export compatibility modules when features are enabled
#[cfg(feature = "unrealmodding-compat")]
pub mod unreal_asset;

#[cfg(feature = "native-lib")]
mod native {
    //! Native FFI bindings for CUE4Parse compression libraries.
    //! 
    //! This module provides direct FFI access to the native CUE4Parse library
    //! for checking feature availability (ACL, Oodle compression support).
    //!
    //! ## Safety
    //!
    //! The functions in this module are `unsafe` because they call into native C code.
    //! However, they are considered safe to use as:
    //! - The input strings are properly null-terminated
    //! - The native functions only read the strings and return boolean values
    //! - No memory allocation or complex data structures are involved
    //!
    //! ## Features
    //!
    //! - **ACL Support**: Check for ACL compression support
    //! - **Oodle Support**: Check for Oodle compression support
    //!
    //! ## Example
    //!
    //! ```no_run
    //! use cue4parse_rs::is_feature_available;
    //!
    //! if is_feature_available("ACL") {
    //!     println!("ACL compression is supported");
    //! } else {
    //!     println!("ACL compression is not supported");
    //! }
    //! ```
    use std::ffi::{CStr, CString};
    
    // Include the generated bindings only if native-lib feature is enabled
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    
    /// Check if a feature is available in the native library
    /// 
    /// # Arguments
    /// * `feature` - The feature name to check ("ACL", "Oodle", etc.)
    /// 
    /// # Returns
    /// `true` if the feature is available, `false` otherwise
    /// 
    /// # Safety
    /// This function calls into native C code but is safe because:
    /// - The input string is properly null-terminated
    /// - The native function only reads the string and returns a boolean
    /// - No memory allocation or complex data structures are involved
    pub fn is_feature_available(feature: &str) -> bool {
        let c_feature = match CString::new(feature) {
            Ok(s) => s,
            Err(_) => return false,
        };
        
        unsafe { IsFeatureAvailable(c_feature.as_ptr()) }
    }
}

#[cfg(feature = "dotnet-interop")]
mod dotnet {
    //! .NET interop module for direct hosting of .NET runtime.
    //! 
    //! This module provides an alternative to the process-based approach
    //! by hosting the .NET runtime directly in the Rust process.
    //! Currently experimental and not fully implemented.
    
    use netcorehost::{nethost, pdcstr};
    use std::ffi::CString;
    use super::CUE4ParseError;
    
    /// .NET runtime host for direct interop
    /// 
    /// This structure manages the .NET runtime lifecycle and provides
    /// methods to call into .NET assemblies directly.
    pub struct DotNetHost {
        // Add fields for .NET hosting
    }
    
    impl DotNetHost {
        /// Create a new .NET runtime host
        /// 
        /// # Returns
        /// A new `DotNetHost` instance or an error if initialization fails
        /// 
        /// # Note
        /// This is currently a placeholder implementation
        pub fn new() -> Result<Self, CUE4ParseError> {
            // Initialize .NET hosting
            Ok(DotNetHost {})
        }
    }
}

#[derive(Error, Debug)]
pub enum CUE4ParseError {
    /// I/O operation failed (file not found, permission denied, etc.)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON parsing or serialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// String contains null bytes or other conversion issues
    #[error("String conversion error")]
    StringConversion,
    
    /// The CLI process execution failed with error output
    #[error("Process execution failed: {0}")]
    ProcessFailed(String),
    
    /// The provided file or directory path is invalid
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    
    /// The requested file or asset could not be found
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    /// Asset parsing failed due to format issues or corruption
    #[error("Parse failed")]
    ParseFailed,
}

/// Result type alias for CUE4Parse operations
/// 
/// This is a convenience alias for `std::result::Result<T, CUE4ParseError>`
/// that's used throughout this crate for consistent error handling.
pub type Result<T> = std::result::Result<T, CUE4ParseError>;

/// Unreal Engine game version identifier
/// 
/// Specifies which version of Unreal Engine the game assets were created with.
/// This affects how assets are parsed and which features are available.
/// 
/// # Examples
/// 
/// ```
/// use cue4parse_rs::GameVersion;
/// 
/// let version = GameVersion::UE5_3;
/// assert_eq!(version.as_str(), "GAME_UE5_3");
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GameVersion {
    /// Unreal Engine 4.0
    #[serde(rename = "GAME_UE4_0")]
    UE4_0,
    /// Unreal Engine 4.27 (final UE4 release)
    #[serde(rename = "GAME_UE4_27")]
    UE4_27,
    /// Unreal Engine 5.0
    #[serde(rename = "GAME_UE5_0")]
    UE5_0,
    /// Unreal Engine 5.1
    #[serde(rename = "GAME_UE5_1")]
    UE5_1,
    /// Unreal Engine 5.2
    #[serde(rename = "GAME_UE5_2")]
    UE5_2,
    /// Unreal Engine 5.3
    #[serde(rename = "GAME_UE5_3")]
    UE5_3,
    /// Unreal Engine 5.4
    #[serde(rename = "GAME_UE5_4")]
    UE5_4,
    /// Unreal Engine 5.5
    #[serde(rename = "GAME_UE5_5")]
    UE5_5,
}

impl GameVersion {
    /// Get the string representation of the game version
    /// 
    /// Returns the string format expected by the CUE4Parse CLI tool.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cue4parse_rs::GameVersion;
    /// 
    /// assert_eq!(GameVersion::UE5_3.as_str(), "GAME_UE5_3");
    /// assert_eq!(GameVersion::UE4_27.as_str(), "GAME_UE4_27");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            GameVersion::UE4_0 => "GAME_UE4_0",
            GameVersion::UE4_27 => "GAME_UE4_27",
            GameVersion::UE5_0 => "GAME_UE5_0",
            GameVersion::UE5_1 => "GAME_UE5_1",
            GameVersion::UE5_2 => "GAME_UE5_2",
            GameVersion::UE5_3 => "GAME_UE5_3",
            GameVersion::UE5_4 => "GAME_UE5_4",
            GameVersion::UE5_5 => "GAME_UE5_5",
        }
    }
}

/// Configuration for a CUE4Parse provider
/// 
/// Contains all the settings needed to initialize and configure
/// a CUE4Parse provider for parsing game assets.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Path to the directory containing game asset files
    pub directory_path: String,
    /// Unreal Engine version used by the game
    pub game_version: GameVersion,
    /// List of AES encryption keys for decrypting assets
    pub aes_keys: Vec<AesKey>,
    /// Optional path to type mappings file (.usmap)
    pub mappings_path: Option<String>,
}

/// AES encryption key for decrypting game assets
/// 
/// Many games encrypt their assets and require specific keys to decrypt them.
/// Keys are usually distributed by the game community or extracted from the game.
#[derive(Debug, Serialize, Deserialize)]
pub struct AesKey {
    /// GUID identifying this key (can be empty for main key)
    pub guid: String,
    /// The actual AES key in hexadecimal format
    pub key: String,
}

/// Information about an exported object within a package
/// 
/// Represents metadata about an object that can be loaded from a package,
/// such as its name, type, and position in the package hierarchy.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportInfo {
    /// The name of the exported object
    pub name: String,
    /// The class/type name of the object
    pub class_name: String,
    /// Index of the outer object (for nested objects)
    pub outer_index: i32,
}

/// Information about a loaded package
/// 
/// Contains metadata about a package file and all the objects it exports.
/// Packages are the fundamental unit of asset organization in Unreal Engine.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    /// The name/path of the package
    pub name: String,
    /// List of all objects exported by this package
    pub exports: Vec<ExportInfo>,
}

/// Main provider for accessing CUE4Parse functionality
/// 
/// The `Provider` is the primary interface for parsing and extracting Unreal Engine assets.
/// It manages configuration, communicates with the CUE4Parse CLI tool, and provides
/// high-level methods for common operations.
/// 
/// # Examples
/// 
/// ```no_run
/// use cue4parse_rs::{Provider, GameVersion};
/// 
/// let mut provider = Provider::new("/path/to/fortnite/paks", GameVersion::UE5_3);
/// provider.add_key("", "0xYOUR_AES_KEY_HERE");
/// provider.set_mappings("/path/to/mappings.usmap");
/// 
/// // List all available packages
/// let packages = provider.list_packages()?;
/// println!("Found {} packages", packages.len());
/// 
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Provider {
    /// Internal configuration for the provider
    pub(crate) config: ProviderConfig,
    /// Path to the CUE4Parse CLI executable
    cue4parse_exe: PathBuf,
}

impl Provider {
    /// Create a new provider instance
    /// 
    /// # Arguments
    /// * `directory_path` - Path to the directory containing game asset files
    /// * `version` - The Unreal Engine version used by the game
    /// 
    /// # Returns
    /// A new `Provider` instance configured with the specified directory and version
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cue4parse_rs::{Provider, GameVersion};
    /// 
    /// let provider = Provider::new("/path/to/game/paks", GameVersion::UE5_3);
    /// ```
    pub fn new(directory_path: impl Into<String>, version: GameVersion) -> Self {
        let config = ProviderConfig {
            directory_path: directory_path.into(),
            game_version: version,
            aes_keys: Vec::new(),
            mappings_path: None,
        };
        
        // Default to looking for CUE4Parse.CLI.exe in the expected location
        let cue4parse_exe = PathBuf::from("../CUE4Parse.CLI/bin/Release/net8.0/CUE4Parse.CLI.exe");
        
        Provider {
            config,
            cue4parse_exe,
        }
    }
    
    /// Set the path to the CUE4Parse executable
    /// 
    /// By default, the provider looks for `CUE4Parse.CLI.exe` in the expected build location.
    /// Use this method to specify a custom path if the executable is located elsewhere.
    /// 
    /// # Arguments
    /// * `path` - Path to the CUE4Parse.CLI executable
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cue4parse_rs::{Provider, GameVersion};
    /// 
    /// let mut provider = Provider::new("/path/to/game", GameVersion::UE5_3);
    /// provider.set_executable_path("C:/tools/CUE4Parse.CLI.exe");
    /// ```
    pub fn set_executable_path(&mut self, path: impl Into<PathBuf>) {
        self.cue4parse_exe = path.into();
    }
    
    /// Add an AES encryption key
    /// 
    /// Many games encrypt their assets and require specific keys to decrypt them.
    /// You can add multiple keys if the game uses different keys for different assets.
    /// 
    /// # Arguments
    /// * `guid` - GUID identifying this key (can be empty string for the main key)
    /// * `key` - The AES key in hexadecimal format (with or without 0x prefix)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cue4parse_rs::{Provider, GameVersion};
    /// 
    /// let mut provider = Provider::new("/path/to/game", GameVersion::UE5_3);
    /// 
    /// // Main encryption key (empty GUID)
    /// provider.add_key("", "0xYOUR_MAIN_AES_KEY_HERE");
    /// 
    /// // Specific key with GUID
    /// provider.add_key("12345678-1234-1234-1234-123456789ABC", "YOUR_SPECIFIC_KEY");
    /// ```
    pub fn add_key(&mut self, guid: impl Into<String>, key: impl Into<String>) {
        self.config.aes_keys.push(AesKey {
            guid: guid.into(),
            key: key.into(),
        });
    }
    
    /// Set the mappings file path
    /// 
    /// Mappings files (.usmap) contain type information that helps CUE4Parse
    /// correctly parse and serialize game assets. They're especially important
    /// for newer games or when you want property names instead of hashes.
    /// 
    /// # Arguments
    /// * `path` - Path to the mappings file (.usmap)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cue4parse_rs::{Provider, GameVersion};
    /// 
    /// let mut provider = Provider::new("/path/to/game", GameVersion::UE5_3);
    /// provider.set_mappings("/path/to/game_mappings.usmap");
    /// ```
    pub fn set_mappings(&mut self, path: impl Into<String>) {
        self.config.mappings_path = Some(path.into());
    }
    
    /// List all packages in the provider
    /// 
    /// Scans the configured directory and returns a list of all available packages
    /// (`.uasset` and `.umap` files). This is useful for discovering what content
    /// is available in the game files.
    /// 
    /// # Returns
    /// A vector of package paths relative to the configured directory
    /// 
    /// # Errors
    /// Returns an error if:
    /// - The CLI tool execution fails
    /// - The configured directory doesn't exist or isn't accessible
    /// - There are permission issues
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use cue4parse_rs::{Provider, GameVersion};
    /// 
    /// let provider = Provider::new("/path/to/game", GameVersion::UE5_3);
    /// let packages = provider.list_packages()?;
    /// 
    /// for package in packages.iter().take(10) {
    ///     println!("Found package: {}", package);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn list_packages(&self) -> Result<Vec<String>> {
        let mut cmd = Command::new(&self.cue4parse_exe);
        cmd.arg("--list-packages");
        cmd.arg("--directory").arg(&self.config.directory_path);
        cmd.arg("--version").arg(self.config.game_version.as_str());
        
        // Add AES keys
        for key in &self.config.aes_keys {
            cmd.arg("--aes-key").arg(format!("{}:{}", key.guid, key.key));
        }
        
        // Add mappings if specified
        if let Some(ref mappings) = self.config.mappings_path {
            cmd.arg("--mappings").arg(mappings);
        }
        
        let output = cmd.output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CUE4ParseError::ProcessFailed(stderr.to_string()));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let packages: Vec<String> = stdout
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect();
        
        Ok(packages)
    }
    
    /// Load package information
    /// 
    /// Loads a specific package and returns information about all the objects
    /// it exports. This is useful for browsing package contents before deciding
    /// which objects to extract.
    /// 
    /// # Arguments
    /// * `package_path` - Path to the package file (relative to configured directory)
    /// 
    /// # Returns
    /// Package information including the list of exported objects
    /// 
    /// # Errors
    /// Returns an error if:
    /// - The package file doesn't exist
    /// - The package is corrupted or can't be parsed
    /// - Required encryption keys are missing
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use cue4parse_rs::{Provider, GameVersion};
    /// 
    /// let provider = Provider::new("/path/to/game", GameVersion::UE5_3);
    /// let package = provider.load_package("GameAssets/Characters/Hero.uasset")?;
    /// 
    /// println!("Package '{}' has {} exports:", package.name, package.exports.len());
    /// for export in &package.exports {
    ///     println!("  {} ({})", export.name, export.class_name);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn load_package(&self, package_path: &str) -> Result<PackageInfo> {
        let mut cmd = Command::new(&self.cue4parse_exe);
        cmd.arg("--package-info");
        cmd.arg("--package").arg(package_path);
        cmd.arg("--directory").arg(&self.config.directory_path);
        cmd.arg("--version").arg(self.config.game_version.as_str());
        cmd.arg("--output-format").arg("json");
        
        // Add AES keys
        for key in &self.config.aes_keys {
            cmd.arg("--aes-key").arg(format!("{}:{}", key.guid, key.key));
        }
        
        // Add mappings if specified
        if let Some(ref mappings) = self.config.mappings_path {
            cmd.arg("--mappings").arg(mappings);
        }
        
        let output = cmd.output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CUE4ParseError::ProcessFailed(stderr.to_string()));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let package_info: PackageInfo = serde_json::from_str(&stdout)?;
        
        Ok(package_info)
    }
    
    /// Export an object to JSON
    /// 
    /// Loads and serializes a specific object to JSON format. This is the most
    /// common way to extract asset data for analysis or conversion to other formats.
    /// 
    /// # Arguments
    /// * `object_path` - Full path to the object (package path + object name)
    /// 
    /// # Returns
    /// JSON representation of the object as a `serde_json::Value`
    /// 
    /// # Errors
    /// Returns an error if:
    /// - The object doesn't exist
    /// - Required encryption keys are missing
    /// - The object can't be serialized to JSON
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use cue4parse_rs::{Provider, GameVersion};
    /// 
    /// let provider = Provider::new("/path/to/game", GameVersion::UE5_3);
    /// let json = provider.export_object_json("GameAssets/Textures/Logo.Logo")?;
    /// 
    /// // Pretty print the JSON
    /// println!("{}", serde_json::to_string_pretty(&json)?);
    /// 
    /// // Access specific fields
    /// if let Some(width) = json["SizeX"].as_u64() {
    ///     println!("Texture width: {}", width);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn export_object_json(&self, object_path: &str) -> Result<serde_json::Value> {
        let mut cmd = Command::new(&self.cue4parse_exe);
        cmd.arg("--export");
        cmd.arg("--object").arg(object_path);
        cmd.arg("--directory").arg(&self.config.directory_path);
        cmd.arg("--version").arg(self.config.game_version.as_str());
        cmd.arg("--output-format").arg("json");
        
        // Add AES keys
        for key in &self.config.aes_keys {
            cmd.arg("--aes-key").arg(format!("{}:{}", key.guid, key.key));
        }
        
        // Add mappings if specified
        if let Some(ref mappings) = self.config.mappings_path {
            cmd.arg("--mappings").arg(mappings);
        }
        
        let output = cmd.output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CUE4ParseError::ProcessFailed(stderr.to_string()));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let json_value: serde_json::Value = serde_json::from_str(&stdout)?;
        
        Ok(json_value)
    }
    
    /// Export an object to a specific file format
    /// 
    /// Exports an object directly to a file in the specified format. This is useful
    /// for extracting assets like textures, meshes, or audio files for use in other tools.
    /// 
    /// # Arguments
    /// * `object_path` - Full path to the object (package path + object name)
    /// * `output_path` - Where to save the exported file
    /// * `format` - Output format ("png", "fbx", "wav", etc.)
    /// 
    /// # Returns
    /// `Ok(())` if the export succeeds
    /// 
    /// # Errors
    /// Returns an error if:
    /// - The object doesn't exist or can't be loaded
    /// - The output format is not supported for this object type
    /// - The output path is not writable
    /// 
    /// # Note
    /// Currently only JSON export is fully implemented. Other formats like PNG, FBX
    /// will be added in future versions through the CUE4Parse-Conversion library.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use cue4parse_rs::{Provider, GameVersion};
    /// 
    /// let provider = Provider::new("/path/to/game", GameVersion::UE5_3);
    /// 
    /// // Export a texture as PNG (when conversion support is added)
    /// // provider.export_object("GameAssets/Textures/Logo.Logo", "logo.png", "png")?;
    /// 
    /// // Export as JSON (currently supported)
    /// provider.export_object("GameAssets/Data/Config.Config", "config.json", "json")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn export_object(&self, object_path: &str, output_path: &str, format: &str) -> Result<()> {
        let mut cmd = Command::new(&self.cue4parse_exe);
        cmd.arg("--export");
        cmd.arg("--object").arg(object_path);
        cmd.arg("--output").arg(output_path);
        cmd.arg("--directory").arg(&self.config.directory_path);
        cmd.arg("--version").arg(self.config.game_version.as_str());
        cmd.arg("--output-format").arg(format);
        
        // Add AES keys
        for key in &self.config.aes_keys {
            cmd.arg("--aes-key").arg(format!("{}:{}", key.guid, key.key));
        }
        
        // Add mappings if specified
        if let Some(ref mappings) = self.config.mappings_path {
            cmd.arg("--mappings").arg(mappings);
        }
        
        let output = cmd.output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CUE4ParseError::ProcessFailed(stderr.to_string()));
        }
        
        Ok(())
    }
}

/// Check if a feature is available in the native library
#[cfg(feature = "native-lib")]
pub fn is_feature_available(feature: &str) -> bool {
    native::is_feature_available(feature)
}

/// Fallback feature check when native library is not available
/// 
/// When the `native-lib` feature is disabled, this function always returns `false`
/// since no native features are available.
/// 
/// # Arguments
/// * `_feature` - The feature name (ignored)
/// 
/// # Returns
/// Always `false` when native library support is disabled
#[cfg(not(feature = "native-lib"))]
pub fn is_feature_available(_feature: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = Provider::new("test/path", GameVersion::UE5_3);
        assert_eq!(provider.config.directory_path, "test/path");
        assert!(matches!(provider.config.game_version, GameVersion::UE5_3));
    }
    
    #[test]
    fn test_adding_keys() {
        let mut provider = Provider::new("test/path", GameVersion::UE5_3);
        provider.add_key("test-guid", "test-key");
        assert_eq!(provider.config.aes_keys.len(), 1);
        assert_eq!(provider.config.aes_keys[0].guid, "test-guid");
        assert_eq!(provider.config.aes_keys[0].key, "test-key");
    }

    #[test]
    fn test_game_version_serialization() {
        // Test that GameVersion serializes correctly to expected strings
        let version = GameVersion::UE5_3;
        let serialized = serde_json::to_string(&version).unwrap();
        assert_eq!(serialized, "\"GAME_UE5_3\"");
        
        // Test deserialization
        let deserialized: GameVersion = serde_json::from_str("\"GAME_UE5_3\"").unwrap();
        assert!(matches!(deserialized, GameVersion::UE5_3));
    }
    
    #[test]
    fn test_game_version_as_str() {
        assert_eq!(GameVersion::UE4_0.as_str(), "GAME_UE4_0");
        assert_eq!(GameVersion::UE4_27.as_str(), "GAME_UE4_27");
        assert_eq!(GameVersion::UE5_0.as_str(), "GAME_UE5_0");
        assert_eq!(GameVersion::UE5_3.as_str(), "GAME_UE5_3");
        assert_eq!(GameVersion::UE5_5.as_str(), "GAME_UE5_5");
    }
    
    #[test]
    fn test_provider_config_serialization() {
        let mut provider = Provider::new("test/path", GameVersion::UE5_3);
        provider.add_key("test-guid", "test-key");
        provider.set_mappings("test.usmap");
        
        // Test that the internal config can be serialized
        let config_json = serde_json::to_string(&provider.config).unwrap();
        assert!(config_json.contains("test/path"));
        assert!(config_json.contains("GAME_UE5_3"));
        assert!(config_json.contains("test-guid"));
        assert!(config_json.contains("test-key"));
        assert!(config_json.contains("test.usmap"));
    }
    
    #[test]
    fn test_multiple_keys() {
        let mut provider = Provider::new("test/path", GameVersion::UE5_3);
        provider.add_key("guid1", "key1");
        provider.add_key("guid2", "key2");
        provider.add_key("", "main-key");
        
        assert_eq!(provider.config.aes_keys.len(), 3);
        assert_eq!(provider.config.aes_keys[0].guid, "guid1");
        assert_eq!(provider.config.aes_keys[1].guid, "guid2");
        assert_eq!(provider.config.aes_keys[2].guid, "");
        assert_eq!(provider.config.aes_keys[2].key, "main-key");
    }
    
    #[test]
    fn test_error_display() {
        let error = CUE4ParseError::InvalidPath("test/path".to_string());
        assert_eq!(format!("{}", error), "Invalid path: test/path");
        
        let error = CUE4ParseError::ProcessFailed("CLI failed".to_string());
        assert_eq!(format!("{}", error), "Process execution failed: CLI failed");
        
        let error = CUE4ParseError::ParseFailed;
        assert_eq!(format!("{}", error), "Parse failed");
    }
    
    #[cfg(feature = "native-lib")]
    #[test]
    fn test_feature_availability() {
        // Test basic feature checking
        println!("ACL available: {}", is_feature_available("ACL"));
        println!("Oodle available: {}", is_feature_available("Oodle"));
        assert!(!is_feature_available("NonExistentFeature"));
    }
    
    #[cfg(not(feature = "native-lib"))]
    #[test]
    fn test_feature_availability_fallback() {
        // When native-lib feature is disabled, should always return false
        assert!(!is_feature_available("ACL"));
        assert!(!is_feature_available("Oodle"));
        assert!(!is_feature_available("NonExistent"));
    }
}
