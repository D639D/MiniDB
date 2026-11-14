use crate::logfile::LogFileIterator;
use crate::{errors, logfile::LogIndex};
use std::{
    collections::HashMap,
    io::{Read, Seek},
};
use std::{fs::File, path::Path};

use crate::utils;

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
    pub fn new(dbdir: &Path) -> KeyDir {
        let mut keydir = KeyDir {
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
            println!("KeyDir | Get -> 1. KeyDir is empty!");
            populate_keydir_with_data_file(file, self);
            println!("KeyDir | Get -> 2.1. Is ok?");
            let res = self.keydir.get(&key).unwrap(); // <--- Крашит тут 
            println!("KeyDir | Get -> 3.1. Res: {:?}?", &res);
            return res;
        }
        println!("KeyDir | Get -> 2.2. Is ok?");
        let res = self.keydir.get(&key).unwrap();
        println!("KeyDir | Get -> 3.2. Res: {:?}?", &res);
        res
    }
}

pub fn populate_keydir_with_data_file(file: &mut File, keydir: &mut KeyDir) {
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    let mut offset = file.stream_position().unwrap();
    println!("KeyDir | Populate_keydir -> 1. Offset -> {:?}", &offset);
    loop {
        println!("Начало цикла");
        let mut buf = Vec::new();
        buf.clear();
        file.seek(std::io::SeekFrom::Start(offset)).unwrap();
        file.read_to_end(&mut buf).unwrap();
        println!("KeyDir | Populate_keydir -> 2. Buf -> {:?}", &buf);
        let slice: &[u8] = &buf;
        println!("KeyDir | Populate_keydir -> 3. Slice -> {:?}", &slice);
        match slice {
            [] => {
                println!("None_1");
                break;
            }
            [KEY_TAG, a, b, c, d, data @ ..] => {
                // Превращаем 4 байта после KEY_TAG в длинну данных

                let len_key = u32::from_ne_bytes([*a, *b, *c, *d]) as usize;
                println!("KeyDir | Populate_keydir -> 4. Len_key -> {:?}", &len_key);
                // Получаем данные из байтов, по длинне данных
                let arr_key = data.get(..len_key).unwrap();
                println!("KeyDir | Populate_keydir -> 5. Arr_key -> {:?}", &arr_key);
                let key = String::from_bytes(arr_key.to_vec()).unwrap();
                println!("KeyDir | Populate_keydir -> 6. Key -> {:?}", &key);
                // Переходи на следующий тег данных
                let next_tag = data.get(len_key..).unwrap();
                println!("KeyDir | Populate_keydir -> 7. Next_tag -> {:?}", &next_tag);
                match next_tag {
                    [] => {
                        //
                    },
                    [INT_TAG, ..] => {
                        println!("KeyDir | Populate_keydir -> 8. INT_TAG");
                        let index = LogIndex {
                            offset: 1 + 4 + len_key + offset as usize,
                            len: 5,
                        };
                        println!("KeyDir | Populate_keydir -> 9. Index -> {:?}", &index);
                        let log = KeyDirEntry { index: index };
                        println!("KeyDir | Populate_keydir -> 10. Log -> {:?}", &log);
                        keydir.keydir.insert(key, log);
                        println!(
                            "KeyDir | Populate_keydir -> 11. Keydir.insert -> {:?}",
                            &keydir.keydir
                        );
                        offset += 1 + 4 + len_key as u64 + 5;
                        println!("KeyDir | Populate_keydir -> 12. Offset -> {:?}\n", &offset);
                    },
                    [FLOAT_TAG, ..] => {
                        println!("KeyDir | Populate_keydir -> 8. FLOAT_TAG");
                        let index = LogIndex {
                            offset: 1 + 4 + len_key + offset as usize,
                            len: 9,
                        };
                        println!("KeyDir | Populate_keydir -> 9. Index -> {:?}", &index);
                        let log = KeyDirEntry { index: index };
                        println!("KeyDir | Populate_keydir -> 10. Log -> {:?}", &log);
                        keydir.keydir.insert(key, log);
                        println!(
                            "KeyDir | Populate_keydir -> 11. Keydir.insert -> {:?}",
                            &keydir.keydir
                        );
                        offset += 1 + 4 + len_key as u64 + 9;
                        println!("KeyDir | Populate_keydir -> 12. Offset -> {:?}\n", &offset);
                    },
                    [TEXT_TAG, a, b, c, d, ..] => {
                        println!("KeyDir | Populate_keydir -> 8. TEXT_TAG");
                        let len = u32::from_ne_bytes([*a, *b, *c, *d]) as usize;
                        println!("KeyDir | Populate_keydir -> 9. Len -> {:?}", &len);
                        let index = LogIndex {
                            offset: 1 + 4 + len_key + offset as usize,
                            len: 1 + 4 + len,
                        };
                        println!("KeyDir | Populate_keydir -> 10. Index -> {:?}", &index);
                        let log = KeyDirEntry { index: index };
                        println!("KeyDir | Populate_keydir -> 11. Log -> {:?}", &log);
                        keydir.keydir.insert(key, log);
                        println!(
                            "KeyDir | Populate_keydir -> 12. Keydir.insert -> {:?}",
                            &keydir.keydir
                        );
                        offset += 1 + 4 + len as u64 + 1 + 4 + len_key as u64;
                        println!("KeyDir | Populate_keydir -> 13. Offset -> {:?}\n", &offset);
                    },
                    _ => {
                        //
                    }
                };
            }
            _ => {
                println!("None_2");
                break;
            }
        };
    };
    println!("Keydir. {:?}", keydir.keydir);
}
