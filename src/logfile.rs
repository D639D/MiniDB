use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub struct LogIndex {
    pub offset: usize,
    pub len: usize,
}

pub struct LogFileIterator {
    pub log_path: PathBuf,
    pub reader: File,
}

impl LogFileIterator {
    pub fn new() -> File {
        let log_path = File::open("main").unwrap();
        log_path
    }
}
