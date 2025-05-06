use std::io::Read;

#[cfg(feature = "tokio")]
use std::future::Future;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::DefaultError;

use super::PacketDeserializable;

#[cfg(feature = "tokio")]
pub trait PacketReadable {
  type ReadError;
  type DeserializeError;
  fn read_packet<
    D: PacketDeserializable<DeserializeError = bincode::Error> + Send + Sync + 'static,
  >(
    &mut self,
  ) -> impl Future<Output = Result<D, Self::ReadError>>;
}

#[cfg(feature = "tokio")]
impl<T> PacketReadable for T
where
  T: AsyncRead + Send + Sync + Unpin,
{
  type ReadError = DefaultError;
  type DeserializeError = bincode::Error;
  async fn read_packet<
    D: PacketDeserializable<DeserializeError = bincode::Error> + Send + Sync + 'static,
  >(
    &mut self,
  ) -> Result<D, Self::ReadError> {
    let mut len = vec![0u8; 8];
    self.read_exact(&mut len).await?;
    let len = u64::from_be_bytes(len.try_into().unwrap());
    let mut data = vec![0u8; len as usize];
    self.read_exact(&mut data).await?;
    Ok(D::deserialize_packet(&data)?)
  }
}

pub trait PacketReadableSync: Read {
  type ReadError;
  type DeserializeError;
  fn read_packet_sync<D: PacketDeserializable<DeserializeError = Self::DeserializeError>>(
    &mut self,
  ) -> Result<D, Self::ReadError>;
}

impl<T> PacketReadableSync for T
where
  T: Read,
{
  type ReadError = DefaultError;
  type DeserializeError = bincode::Error;
  fn read_packet_sync<D: PacketDeserializable<DeserializeError = Self::DeserializeError>>(
    &mut self,
  ) -> Result<D, Self::ReadError> {
    let mut len = vec![0u8; 8];
    self.read_exact(&mut len)?;
    let len = u64::from_be_bytes(len.try_into().unwrap());
    let mut data = vec![0u8; len as usize];
    self.read_exact(&mut data)?;
    Ok(D::deserialize_packet(&data)?)
  }
}
