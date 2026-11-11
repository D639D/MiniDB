use std::collections::HashMap;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{Error, Read, Seek, Write};

use crate::keydir::{KeyDir, LogIndex};
use crate::writer::Writer;

use crate::codec::Types;
use crate::codec::{EncodeKey, ToBytes, read_cell};

use crate::errors::{self};

pub struct DataBase {
    writer: Writer,
    keydir: KeyDir,
}

impl DataBase {
    pub fn new(name: &str) -> Result<DataBase, errors::DataBaseError> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(name)
            .map_err(|err| errors::DataBaseError {
                kind: errors::DataBaseErrorKind::Io(err),
            })?;

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
        Ok(res)
    }

    pub fn set<K: EncodeKey + Display, V: ToBytes>(
        &mut self,
        key: K,
        data: V,
    ) -> Result<(), errors::DataBaseError> {
        let encode_key = key.encode_key();
        let encode_data = data.to_bytes();

        let logindex = self.wtire_to_active_data_file(&encode_key, &encode_data)?;

        self.keydir.set(key.to_string(), logindex);

        Ok(())
    }

    pub fn wtire_to_active_data_file(
        &mut self,
        encode_key: &[u8],
        encode_data: &[u8],
    ) -> Result<LogIndex, errors::DataBaseError> {
        let offset = self
            .writer
            .active_data_file
            .stream_position()
            .map_err(|err| errors::DataBaseError {
                kind: errors::DataBaseErrorKind::Io(err),
            })? as usize;

        let len = encode_data.len();
        self.writer
            .active_data_file
            .write_all(&encode_key)
            .map_err(|err| errors::DataBaseError {
                kind: errors::DataBaseErrorKind::Io(err),
            })?;
        self.writer
            .active_data_file
            .write_all(&encode_data)
            .map_err(|err| errors::DataBaseError {
                kind: errors::DataBaseErrorKind::Io(err),
            })?;

        Ok(LogIndex {
            offset: offset,
            len: len,
        })
    }

    pub fn get(&mut self, key: String) -> Result<(Types, usize), errors::DataBaseError> {
        let index = self.keydir.get(key);
        println!("1");
        let offset = index.index.offset as u64;
        println!("2");
        let len = index.index.len;
        println!("3");
        let mut buf = vec![0; len];
        println!("4");
        self.writer
            .active_data_file
            .seek(std::io::SeekFrom::Start(0))
            .map_err(|err| errors::DataBaseError {
                kind: errors::DataBaseErrorKind::Io(err),
            })?;
        println!("5");
        self.writer
            .active_data_file
            .read(&mut buf)
            .map_err(|err| errors::DataBaseError {
                kind: errors::DataBaseErrorKind::Io(err),
            })?;
        println!("6");

        let res = read_cell(&buf).map_err(|err| errors::DataBaseError {
            kind: errors::DataBaseErrorKind::Codec(err),
        })?;
        println!("7");
        Ok(res)
    }
}
