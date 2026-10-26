trait ToBytes {
    type Output;
    fn to_bytes(&self) -> Self::Output;
}
trait FromBytes: Sized {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self>;
}

impl ToBytes for i32 {
    type Output = Vec<u8>;
    fn to_bytes(&self) -> Self::Output {
        self.to_ne_bytes().to_vec()
    }
}
impl FromBytes for i32 {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        let array: [u8; 4] = bytes.try_into().ok()?;
        Some(i32::from_ne_bytes(array))
    }
}

impl ToBytes for f64 {
    type Output = Vec<u8>;
    fn to_bytes(&self) -> Self::Output {
        self.to_ne_bytes().to_vec()
    }
}
impl FromBytes for f64 {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        let array: [u8; 8] = bytes.try_into().ok()?;
        Some(f64::from_ne_bytes(array))
    }
}

impl ToBytes for String {
    type Output = Vec<u8>;
    fn to_bytes(&self) -> Self::Output {
        self.as_bytes().to_vec()
    }
}
impl FromBytes for String {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        String::from_utf8(bytes).ok()
    }
}

fn main() {
    // let num_int = 42;
    // let bytes_int = num_int.to_bytes();
    // println!("{:?}", &bytes_int);
    // if let Some(i) = i32::from_bytes(bytes_int) {
    //     println!("{:?}", i)
    // }

    // let num_float = 42.0;
    // let bytes_float = num_float.to_bytes();
    // println!("{:?}", &bytes_float);
    // if let Some(i) = f64::from_bytes(bytes_float) {
    //     println!("{:?}", i)
    // }

    let num_int = String::from("HELLO");
    let bytes_str = num_int.to_bytes();
    let len = bytes_str.len();
    let prefix: [u8; 2] = [3, len as u8];
    let mut result = Vec::with_capacity(len + 1);
    result.extend_from_slice(&prefix);
    result.extend_from_slice(&bytes_str);
    read_cell(&result);
}

fn read_cell(bytes: &[u8]) {
    match bytes[0] {
        1 => {
            let tmp = bytes[1..=4].to_vec();
            if let Some(i) = i32::from_bytes(tmp) {
                println!("{:?}", i)
            }
        }
        2 => {
            let tmp = bytes[1..=8].to_vec();
            if let Some(i) = f64::from_bytes(tmp) {
                println!("{:?}", i)
            }
        }
        3 => {
            let len = bytes[1];
            let tmp = bytes[2..2 + len as usize].to_vec();
            if let Some(i) = String::from_bytes(tmp) {
                println!("{:?}", i)
            }
        }
        _ => {}
    }
}
