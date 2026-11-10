use std::collections::HashMap;

#[derive(Debug)]
pub struct KeyDir {
    pub keydir: HashMap<String, KeyDirEntry>,
}
#[derive(Debug)]
pub struct KeyDirEntry {
    pub offset: usize,
    pub len: usize,
}

impl KeyDir {
    pub fn new() -> KeyDir {
        let keydir = KeyDir {
            keydir: HashMap::new(),
        };
        keydir
    }
    pub fn set(&mut self, key: String, offset: usize, len: usize) -> Option<KeyDirEntry> {
        let keydir_entry = KeyDirEntry {
            offset: offset,
            len: len,
        };
        self.keydir.insert(key, keydir_entry)
    }

    pub fn get(&self, key: String) -> &KeyDirEntry {
        self.keydir.get(&key).unwrap()
    }
}
