use serde::Serialize;

pub trait PacketSerializable: Sized {
  type SerializeError;
  fn serialize_packet(&self) -> Result<Vec<u8>, Self::SerializeError>;
}

impl<T> PacketSerializable for T
where
  T: Serialize,
{
  type SerializeError = bincode::Error;
  fn serialize_packet(&self) -> Result<Vec<u8>, Self::SerializeError> {
    bincode::serialize(self)
  }
}
