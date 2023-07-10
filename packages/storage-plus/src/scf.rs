use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};



pub trait SerializeForBasicType {
    fn serialize(&self, buf: &mut Vec<u8>);
    fn deserialize(buf: &[u8]) -> Option<Self> where Self: Sized;
}

impl SerializeForBasicType for u32 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
    fn deserialize(buf: &[u8]) -> Option<Self> {
        if buf.len() < 4 {
            return None;
        }
        Some(u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]))
    }
}

impl SerializeForBasicType for u64 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
    fn deserialize(buf: &[u8]) -> Option<Self> {
        if buf.len() < 8 {
            return None;
        }
        Some(u64::from_be_bytes([buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]))
    }
}

pub fn serialize_to_bytes<T: SerializeForBasicType>(value: &T) -> Vec<u8> {
    let mut buf = Vec::new();
    value.serialize(&mut buf);
    buf
}

pub fn deserialize_from_bytes<T: SerializeForBasicType>(bytes: Vec<u8>) -> Option<T> {
    T::deserialize(&bytes)
}
