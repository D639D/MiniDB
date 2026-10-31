use super::enums;

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait FromBytes {
    type Output;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self::Output, enums::DataBaseErr>;
}
