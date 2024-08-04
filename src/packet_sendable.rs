use std::io::Write;

use super::PacketSerializable;

#[cfg(feature = "tokio")]
use std::future::Future;
#[cfg(feature = "tokio")]
use tokio::io::AsyncWriteExt;

#[cfg(feature = "tokio")]
pub trait PacketSendable: AsyncWriteExt + Send + Sync + Unpin {
  type SendError;
  fn send_packet<D: PacketSerializable + Send + Clone + Sync + 'static>(
    &mut self,
    packet: D,
  ) -> impl Future<Output = Result<(), Self::SendError>>;
}

#[cfg(feature = "tokio")]
impl<T> PacketSendable for T
where
  T: AsyncWriteExt + Send + Sync + Unpin,
{
  type SendError = ();
  async fn send_packet<D: PacketSerializable>(&mut self, packet: D) -> Result<(), Self::SendError> {
    let data = packet.serialize_packet().map_err(|_| ())?;
    let mut send = (data.len() as u64).to_be_bytes().to_vec();
    send.extend(data);
    let res = <T as AsyncWriteExt>::write_all(self, &send)
      .await
      .map_err(|_| ())?;
    Ok(res)
  }
}

pub trait PacketSendableSync: Write {
  type SendError;
  fn send_packet_sync<D: PacketSerializable + Send + Clone + Sync + 'static>(
    &mut self,
    packet: D,
  ) -> Result<(), Self::SendError>;
}

impl<T> PacketSendableSync for T
where
  T: Write,
{
  type SendError = ();
  fn send_packet_sync<D: PacketSerializable>(&mut self, packet: D) -> Result<(), Self::SendError> {
    let data = packet.serialize_packet().map_err(|_| ())?;
    let mut send = (data.len() as u64).to_be_bytes().to_vec();
    send.extend(data);
    let res = T::write_all(self, &send).map_err(|_| ())?;
    Ok(res)
  }
}
