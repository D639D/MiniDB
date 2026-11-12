use crate::logfile::LogFileIterator;
use crate::logfile::LogIndex;
use std::path::Path;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    io::{Read, Seek, Write},
};

use crate::codec::FromBytes;
use crate::codec::{BOOL_TAG, FLOAT_TAG, INT_TAG, KEY_TAG, TEXT_TAG};
#[derive(Debug)]
pub struct KeyDir {
    pub keydir: HashMap<String, KeyDirEntry>,
}

#[derive(Debug)]
pub struct KeyDirEntry {
    pub index: LogIndex,
}

impl KeyDir {
    pub fn new(minidbdir: &PathBuf) -> KeyDir {
        let mut keydir = KeyDir {
            keydir: HashMap::new(),
        };

        populate_keydir_with_data_file(minidbdir, &mut keydir);
        keydir
    }
    pub fn set(&mut self, key: String, logindex: LogIndex) -> Option<KeyDirEntry> {
        let keydir_entry = KeyDirEntry { index: logindex };
        self.keydir.insert(key, keydir_entry)
    }

    pub fn get(&self, key: String) -> &KeyDirEntry {
        self.keydir.get(&key).unwrap()
    }
}

pub fn populate_keydir_with_data_file(logfile: &Path, keydir: &mut KeyDir) {
    let mut log_file = LogFileIterator::new(logfile);
    log_file.seek(std::io::SeekFrom::Start(0)).unwrap();
    let mut offset = log_file.stream_position().unwrap();
    loop {
        let mut buf = Vec::new();
        log_file
            .seek(std::io::SeekFrom::Start(offset))
            .unwrap();
        log_file.read_to_end(&mut buf).unwrap();
        let slice: &[u8] = &buf;
        match slice {
            [] => println!("None"),
            [KEY_TAG, a, b, c, d, data @ ..] => {
                // Превращаем 4 байта после KEY_TAG в длинну данных

                let len_key = u32::from_ne_bytes([*a, *b, *c, *d]) as usize;

                // Получаем данные из байтов, по длинне данных
                let arr_key = data.get(..len_key).unwrap();
                let key = String::from_bytes(arr_key.to_vec()).unwrap();
                // Переходи на следующий тег данных
                let next_tag = data.get(len_key..).unwrap();

                match next_tag {
                    [] => {
                        //
                    }
                    [INT_TAG, ..] => {
                        let index = LogIndex {
                            offset: 1 + len_key + offset as usize,
                            len: 5,
                        };
                        let log = KeyDirEntry { index: index };
                        keydir.keydir.insert(key, log);
                        let tmp = 1 + 4 + len_key + 5;
                        offset += tmp as u64;
                    }
                    [FLOAT_TAG, ..] => {
                        let index = LogIndex {
                            offset: 1 + len_key + offset as usize,
                            len: 9,
                        };
                        let log = KeyDirEntry { index: index };
                        keydir.keydir.insert(key, log);
                        let tmp = 1 + 4 + len_key + 9;
                        offset += tmp as u64;
                    }
                    [TEXT_TAG, a, b, c, d, ..] => {
                        let len = u32::from_ne_bytes([*a, *b, *c, *d]) as usize;
                        let index = LogIndex {
                            offset: 1 + len_key + offset as usize,
                            len: len,
                        };
                        let log = KeyDirEntry { index: index };
                        keydir.keydir.insert(key, log);
                        let tmp = 1 + 4 + len + 1 + 4 + len_key;
                        offset += tmp as u64;
                    }
                    _ => {
                        //
                    }
                }
            }
            _ => println!("None"),
        };
    }
}
