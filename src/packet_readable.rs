use std::io::Read;

#[cfg(feature = "tokio")]
use std::future::Future;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead, AsyncReadExt};

use super::PacketDeserializable;

#[cfg(feature = "tokio")]
pub trait PacketReadable {
  type ReadError;
  fn read_packet<D: PacketDeserializable + Send + Sync + 'static>(
    &mut self,
  ) -> impl Future<Output = Result<D, Self::ReadError>>;
}

#[cfg(feature = "tokio")]
impl<T> PacketReadable for T
where
  T: AsyncRead + Send + Sync + Unpin,
{
  type ReadError = ();
  async fn read_packet<D: PacketDeserializable + Send + Sync + 'static>(
    &mut self,
  ) -> Result<D, Self::ReadError> {
    let mut len = vec![0u8; 8];
    self.read_exact(&mut len).await.map_err(|_| ())?;
    let len = u64::from_be_bytes(len.try_into().unwrap());
    let mut data = vec![0u8; len as usize];
    self.read_exact(&mut data).await.map_err(|_| ())?;
    D::deserialize_packet(&data).map_err(|_| ())
  }
}

pub trait PacketReadableSync: Read {
  type ReadError;
  fn read_packet_sync<D: PacketDeserializable>(&mut self) -> Result<D, Self::ReadError>;
}

impl<T> PacketReadableSync for T
where
  T: Read,
{
  type ReadError = ();
  fn read_packet_sync<D: PacketDeserializable>(&mut self) -> Result<D, Self::ReadError> {
    let mut len = vec![0u8; 8];
    self.read_exact(&mut len).map_err(|_| ())?;
    let len = u64::from_be_bytes(len.try_into().unwrap());
    let mut data = vec![0u8; len as usize];
    self.read_exact(&mut data).map_err(|_| ())?;
    D::deserialize_packet(&data).map_err(|_| ())
  }
}
