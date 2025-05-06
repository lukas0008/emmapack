mod packet_deserializable;
mod packet_readable;
mod packet_sendable;
mod packet_serializable;

pub use packet_deserializable::*;
pub use packet_readable::*;
pub use packet_sendable::*;
pub use packet_serializable::*;

use thiserror::Error;
#[derive(Debug, Error)]
pub enum DefaultError {
  #[error("error while serializing/deserializing: {0}")]
  SerializeError(#[from] bincode::Error),
  #[error("io error: {0}")]
  IoError(#[from] std::io::Error),
}

pub const CURENT_PROTOCOL_VERSION: u32 = 0;
