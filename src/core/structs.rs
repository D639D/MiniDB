pub struct Database {
    pub file: File,
}

pub trait ToBytes {
    pub type Output;
    pub fn to_bytes(&self) -> Self::Output;
}
pub trait FromBytes: Sized {
    pub fn from_bytes(bytes: Vec<u8>) -> Option<Self>;
}
