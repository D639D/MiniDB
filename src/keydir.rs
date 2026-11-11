use std::collections::HashMap;

#[derive(Debug)]
pub struct KeyDir {
    pub keydir: HashMap<String, KeyDirEntry>,
}

#[derive(Debug)]
pub struct KeyDirEntry {
    pub index: LogIndex,
}

#[derive(Debug)]
pub struct LogIndex {
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
    pub fn set(&mut self, key: String, logindex: LogIndex) -> Option<KeyDirEntry> {
        let keydir_entry = KeyDirEntry { index: logindex };
        self.keydir.insert(key, keydir_entry)
    }

    pub fn get(&self, key: String) -> &KeyDirEntry {
        self.keydir.get(&key).unwrap()
    }
}
