use crate::errors::{self};

pub const INT_TAG: u8 = 1;
pub const FLOAT_TAG: u8 = 2;
pub const BOOL_TAG: u8 = 3;
pub const TEXT_TAG: u8 = 4;
pub const KEY_TAG: u8 = 5;
pub const TOMBSTONE_TAG: u8 = 6;

#[derive(Debug)]
pub enum Types {
    Int(i32),
    Float(f64),
    Bool(bool),
    Text(String),
}

//------------------------------------------------------------------------------//

pub trait FromBytes {
    type Output;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self::Output, errors::CodecError>;
}

impl FromBytes for i32 {
    type Output = i32;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self::Output, errors::CodecError> {
        let actual = bytes.len();
        let array: [u8; 4] = bytes.try_into().map_err(|_| errors::CodecError {
            kind: errors::CodecErrorKind::InvalidLength {
                expected: 8,
                actual,
            },
        })?;
        Ok(i32::from_ne_bytes(array))
    }
}

impl FromBytes for f64 {
    type Output = f64;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self::Output, errors::CodecError> {
        let actual = bytes.len();
        let array: [u8; 8] = bytes.try_into().map_err(|_| errors::CodecError {
            kind: errors::CodecErrorKind::InvalidLength {
                expected: 8,
                actual,
            },
        })?;
        Ok(f64::from_ne_bytes(array))
    }
}

impl FromBytes for String {
    type Output = String;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self::Output, errors::CodecError> {
        let actual = bytes.len();
        String::from_utf8(bytes).map_err(|_| errors::CodecError {
            kind: errors::CodecErrorKind::InvalidLength { expected: 2, actual },
        })
    }
}

impl FromBytes for bool {
    type Output = bool;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self::Output, errors::CodecError> {
        let actual = bytes.len();
        let arr: [u8; 1] = bytes.try_into().map_err(|_| errors::CodecError {
            kind: errors::CodecErrorKind::InvalidLength { expected: 1, actual }
        })?;
        let res = arr[0] != 0;
        Ok(res)
    }
}

//------------------------------------------------------------------------------//

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(5);
        res.extend_from_slice(&[INT_TAG]);
        res.extend_from_slice(&self.to_ne_bytes());
        res
    }
}

impl ToBytes for f64 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(9);
        res.extend_from_slice(&[FLOAT_TAG]);
        res.extend_from_slice(&self.to_ne_bytes());
        res
    }
}

impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&[TEXT_TAG]);
        res.extend_from_slice(&(self.len() as u32).to_ne_bytes());
        res.extend_from_slice(self.as_bytes());
        res
    }
}

impl ToBytes for &str {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&[TEXT_TAG]);
        res.extend_from_slice(&(self.len() as u32).to_ne_bytes());
        res.extend_from_slice(self.as_bytes());
        res
    }
}

impl ToBytes for bool {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(2);
        res.extend_from_slice(&[BOOL_TAG]);
        res.extend_from_slice(&[*self as u8]);
        res
    }
}

//------------------------------------------------------------------------------//

pub trait EncodeKey {
    fn encode_key(&self) -> Vec<u8>;
}

impl EncodeKey for &str {
    fn encode_key(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&[KEY_TAG]);
        res.extend_from_slice(&(self.len() as u32).to_ne_bytes());
        res.extend_from_slice(self.as_bytes());
        res
    }
}

//------------------------------------------------------------------------------//

pub fn read_cell(value: &[u8]) -> Result<Types, errors::CodecError> {

    match value {
        [] => Err(errors::CodecError {
            kind: errors::CodecErrorKind::Truncated,
        }),

        [INT_TAG, a, b, c, d, ..] => {
            let arr: [u8; 4] = [*a, *b, *c, *d];
            i32::from_bytes(arr.to_vec()).map(Types::Int)
        }
        [FLOAT_TAG, a, b, c, d, e, f, g, h, ..] => {
            let arr: [u8; 8] = [*a, *b, *c, *d, *e, *f, *g, *h];
            f64::from_bytes(arr.to_vec()).map(Types::Float)
        }
        [BOOL_TAG, a, ..] => {
            let arr: [u8; 1] = [*a];
            bool::from_bytes(arr.to_vec()).map(Types::Bool)
        }
        [TEXT_TAG, a, b, c, d, data @ ..] => {
            let len = u32::from_ne_bytes([*a, *b, *c, *d]) as usize;
            let arr = data.get(..len).ok_or(errors::CodecError {
                kind: errors::CodecErrorKind::Truncated,
            })?;
            String::from_bytes(arr.to_vec()).map(Types::Text)
        }
        [tag, ..] => Err(errors::CodecError {
            kind: errors::CodecErrorKind::UnknowTag(*tag),
        }),
    }
}
