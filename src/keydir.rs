use crate::errors::{self};
use crate::logfile::LogIndex;
use std::fs::File;
use std::{
    collections::HashMap,
    io::{Read, Seek},
};

use crate::codec::FromBytes;
use crate::codec::{FLOAT_TAG, INT_TAG, KEY_TAG, TEXT_TAG};
#[derive(Debug)]
pub struct KeyDir {
    pub keydir: HashMap<String, KeyDirEntry>,
}

#[derive(Debug)]
pub struct KeyDirEntry {
    pub index: LogIndex,
}

impl KeyDir {
    pub fn new() -> KeyDir {
        let keydir = KeyDir {
            keydir: HashMap::new(),
        };
        keydir
    }
    pub fn set(&mut self, key: String, logindex: LogIndex) -> Option<KeyDirEntry> {
        let keydir_entry = KeyDirEntry { index: logindex };
        self.keydir.insert(key, keydir_entry)
    }

    pub fn get(&mut self, key: String, file: &mut File) -> &KeyDirEntry {
        if self.keydir.is_empty() {
            let _ = populate_keydir_with_data_file(file, self);
            let res = self.keydir.get(&key).unwrap();
            return res;
        }
        let res = self.keydir.get(&key).unwrap();
        res
    }
}

pub fn populate_keydir_with_data_file(
    file: &mut File,
    keydir: &mut KeyDir,
) -> Result<(), errors::KeyDirError> {
    file.seek(std::io::SeekFrom::Start(0))
        .map_err(|err| errors::KeyDirError {
            kind: errors::KeyDirErrorKind::Io(err),
        })?;
    let mut offset = file.stream_position().map_err(|err| errors::KeyDirError {
        kind: errors::KeyDirErrorKind::Io(err),
    })?;
    loop {
        let mut buf = Vec::new();
        buf.clear();
        file.seek(std::io::SeekFrom::Start(offset))
            .map_err(|err| errors::KeyDirError {
                kind: errors::KeyDirErrorKind::Io(err),
            })?;
        file.read_to_end(&mut buf)
            .map_err(|err| errors::KeyDirError {
                kind: errors::KeyDirErrorKind::Io(err),
            })?;
        let slice: &[u8] = &buf;
        match slice {
            [] => return Err(errors::KeyDirError {
                kind: errors::KeyDirErrorKind::Truncated,
            }),
            [KEY_TAG, a, b, c, d, data @ ..] => {
                // Превращаем 4 байта после KEY_TAG в длинну данных

                let len_key = u32::from_ne_bytes([*a, *b, *c, *d]) as usize;
                // Получаем данные из байтов, по длинне данных
                let arr_key = data.get(..len_key).ok_or(errors::KeyDirError {
                    kind: errors::KeyDirErrorKind::Truncated,
                })?;
                let key =
                    String::from_bytes(arr_key.to_vec()).map_err(|err| errors::KeyDirError {
                        kind: errors::KeyDirErrorKind::Codec(err),
                    })?;
                // Переходи на следующий тег данных
                let next_tag = data.get(len_key..).ok_or(errors::KeyDirError {
                    kind: errors::KeyDirErrorKind::Truncated,
                })?;
                match next_tag {
                    [] => return Err(errors::KeyDirError {
                        kind: errors::KeyDirErrorKind::Truncated,
                    }),
                    [INT_TAG, ..] => {
                        let index = LogIndex {
                            offset: 1 + 4 + len_key + offset as usize,
                            len: 5,
                        };
                        let log = KeyDirEntry { index: index };
                        keydir.keydir.insert(key, log);
                        offset += 1 + 4 + len_key as u64 + 5;
                        continue;
                    }
                    [FLOAT_TAG, ..] => {
                        let index = LogIndex {
                            offset: 1 + 4 + len_key + offset as usize,
                            len: 9,
                        };
                        let log = KeyDirEntry { index: index };
                        keydir.keydir.insert(key, log);
                        offset += 1 + 4 + len_key as u64 + 9;
                        continue;

                    }
                    [TEXT_TAG, a, b, c, d, ..] => {
                        let len = u32::from_ne_bytes([*a, *b, *c, *d]) as usize;
                        let index = LogIndex {
                            offset: 1 + 4 + len_key + offset as usize,
                            len: 1 + 4 + len,
                        };
                        let log = KeyDirEntry { index: index };
                        keydir.keydir.insert(key, log);
                        offset += 1 + 4 + len as u64 + 1 + 4 + len_key as u64;
                        continue;
                    }
                    [tag, ..] => return Err(errors::KeyDirError {
                        kind: errors::KeyDirErrorKind::UnknowTag(*tag),
                    }),
                }
            }
            [tag, ..] => return Err(errors::KeyDirError {
                kind: errors::KeyDirErrorKind::UnknowTag(*tag),
            }),
        };
    }
}
