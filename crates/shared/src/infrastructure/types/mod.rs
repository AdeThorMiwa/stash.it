use crate::infrastructure::types::error::Error;

pub mod error;

pub type Result<T, E = Error> = std::result::Result<T, E>;
