use crate::core::enums::DataBaseErr;

use super::enums;

pub trait ToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>, DataBaseErr>;
}

pub trait FromBytes {
    type Output;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self::Output, enums::DataBaseErr>;
}
