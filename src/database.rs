use std::collections::HashMap;
use std::fs::OpenOptions;

use crate::keydir::KeyDir;
use crate::writer::Writer;

pub struct DataBase {
    writer: Writer,
    keydir: KeyDir,
}

impl DataBase {
    pub fn new(name: &str) -> DataBase {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(name)
            .unwrap();

        let keydir = KeyDir {
            keydir: HashMap::new(),
        };

        let writer = Writer {
            active_data_file: file,
        };

        let res = DataBase {
            writer: writer,
            keydir: keydir,
        };
        res
    }
}
