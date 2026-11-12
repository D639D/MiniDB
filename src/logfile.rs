use std::{fs::File, path::{Path, PathBuf}};

#[derive(Debug)]
pub struct LogIndex {
    pub offset: usize,
    pub len: usize,
}

pub struct LogFileIterator {
    pub lot_path: PathBuf,
    pub reader: File
}

impl LogFileIterator {
    pub fn new(patch: &Path) -> File {
        let reader = File::open(patch).unwrap();
        reader
    }
}