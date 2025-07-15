
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ancestry {
    pub ancestry: Vec<String>,
}

impl Ancestry {
    pub fn new() -> Self {
        Self { ancestry: Vec::new() }
    }
    
    pub fn from_vec(ancestry: Vec<String>) -> Self {
        Self { ancestry }
    }
    
    pub fn get_version(&self) -> i32 {
        self.ancestry.len() as i32
    }
} 