mod as_bytes;
mod error;
mod expansion;

pub mod cmd;
pub mod db;

pub use error::Error;
pub use expansion::Expand;

pub type Result<T> = std::result::Result<T, Error>;
