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

// Standard library imports
use std::collections::HashMap;
use std::io::{Read, Write, Seek};
use std::fmt;
use std::marker::PhantomData;

// Third-party imports
use uuid::Uuid;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

// Conditional serde_json import for JSON handling
#[cfg(feature = "unrealmodding-compat")]
use serde_json;

// Hash algorithm imports for Phase 3
#[cfg(feature = "unrealmodding-compat")]
use fnv::FnvHasher;
#[cfg(feature = "unrealmodding-compat")]
use std::hash::{Hash, Hasher};
#[cfg(feature = "unrealmodding-compat")]
use lru::LruCache;
#[cfg(feature = "unrealmodding-compat")]
use std::num::NonZeroUsize;

// Bitflags for UE5 features
#[cfg(feature = "unrealmodding-compat")]
use bitflags::bitflags;

// Conditional imports for unrealmodding-compat feature
#[cfg(feature = "unrealmodding-compat")]
use crate::{Provider, Result};

pub mod containers;
pub mod error;
pub mod exports;
pub mod properties;
pub mod reader;
pub mod types;
pub mod unversioned;
pub mod versions;

pub use containers::*;
pub use error::*;
pub use exports::*;
pub use properties::*;
pub use reader::*;
pub use types::*;
pub use unversioned::*;
pub use versions::*;

// ============================================================================
// CAST MACRO - Essential for Stove compatibility
// ============================================================================

/// Cast macro for property type conversions
/// 
/// This macro provides safe casting between different property types,
/// essential for Stove's property manipulation needs.
#[cfg(feature = "unrealmodding-compat")]
#[macro_export]
macro_rules! cast {
    ($property:expr, $variant:ident) => {
        match $property {
            Property::$variant(value) => Some(value),
            _ => None,
        }
    };
    ($property:expr, $variant:ident as $target:ty) => {
        match $property {
            Property::$variant(value) => Some(value as $target),
            _ => None,
        }
    };
}

// Re-export the macro for compatibility
#[cfg(feature = "unrealmodding-compat")]
pub use crate::cast; 