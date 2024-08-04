use serde::de::DeserializeOwned;

pub trait PacketDeserializable: Sized + 'static {
  type DeserializeError: Send;
  fn deserialize_packet<'a>(data: &'a [u8]) -> Result<Self, Self::DeserializeError>
  where
    Self: Sized;
}
impl<T> PacketDeserializable for T
where
  T: DeserializeOwned + 'static,
{
  type DeserializeError = bincode::Error;
  fn deserialize_packet(data: &[u8]) -> Result<Self, Self::DeserializeError>
  where
    Self: Sized,
  {
    bincode::deserialize(data)
  }
}
