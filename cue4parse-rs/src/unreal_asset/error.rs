
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UnrealAssetError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(String),
    #[error("Invalid index: {0}")]
    InvalidIndex(String),
    #[error("Custom error: {0}")]
    Custom(String),
}

impl UnrealAssetError {
    pub fn new(message: &str) -> Self {
        UnrealAssetError::Custom(message.to_string())
    }
}

pub type UnrealAssetResult<T> = std::result::Result<T, UnrealAssetError>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No data available")]
    NoData,
    #[error("Invalid package index: {0}")]
    InvalidIndex(i32),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("CUE4Parse error: {0}")]
    CUE4Parse(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

impl Error {
    
    pub fn no_data(message: String) -> Self {
        Error::InvalidData(message)
    }

    
    pub fn invalid_package_index(index: i32) -> Self {
        Error::InvalidIndex(index)
    }
}


impl From<Error> for UnrealAssetError {
    fn from(err: Error) -> Self {
        match err {
            Error::Io(e) => UnrealAssetError::Io(e),
            _ => UnrealAssetError::Parse(err.to_string()),
        }
    }
}


impl From<UnrealAssetError> for Error {
    fn from(err: UnrealAssetError) -> Self {
        match err {
            UnrealAssetError::Io(e) => Error::Io(e),
            _ => Error::CUE4Parse(err.to_string()),
        }
    }
} 