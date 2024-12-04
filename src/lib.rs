#![deny(clippy::all)]

pub mod archive;
mod error;
pub mod write;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
