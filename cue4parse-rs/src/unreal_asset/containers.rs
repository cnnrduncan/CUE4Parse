
use serde::{Deserialize, Serialize};
use crate::unreal_asset::versions::ObjectVersion;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedResource<T> {
    pub data: T,
    pub is_loaded: bool,
}

impl<T> SharedResource<T> {
    pub fn new(data: T) -> Self {
        Self { data, is_loaded: true }
    }
    
    pub fn get(&self) -> &T {
        &self.data
    }
    
    pub fn get_ref(&self) -> &T {
        &self.data
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

pub type NameMap = Vec<String>;

pub trait Container {
    fn get_version(&self) -> ObjectVersion;
} 