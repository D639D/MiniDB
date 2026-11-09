use crate::codec::{EncodeKey, FromBytes, ToBytes};
use crate::codec::{FLOAT_TAG, INT_TAG, KEY_TAG, TEXT_TAG};
use std::collections::HashMap;
use std::io::{Read, Seek, Write};

pub struct LogFile<W: Write + Read + Seek + Copy> {
    pub file: W,
    pub index: String,
    pub offset: usize,
}

impl<W: Write + Read + Seek + Copy> LogFile<W> {
    fn write_in_mem(&mut self) {
        let mut in_mem_log = Vec::new();
        let _ = self.file.seek(std::io::SeekFrom::Start(0));
        let mut offset = self.file.stream_position().unwrap() as usize;
        let mut buf = Vec::new();
        let _ = self.file.read_to_end(&mut buf);

        let slice: &[u8] = &buf;
        loop {
            let mut index_offset = HashMap::new();
            match slice {
                [] => println!("Пусто"),
                [KEY_TAG, a, b, c, d, data @ ..] => {
                    let len_key = u32::from_ne_bytes([*a, *b, *c, *d]) as usize;
                    let arr_key = data.get(..len_key).unwrap();
                    let res_key = String::from_bytes(arr_key.to_vec()).unwrap();
                    let off = 1 + len_key;
                    offset += off;
                    let res = LogFile {
                        file: self.file,
                        index: res_key,
                        offset: off,
                    };
                    index_offset.insert(res.index, res.offset);
                    in_mem_log.push(index_offset);
                }
                _ => println!("Нету значений"),
            }
        }
    }
}
