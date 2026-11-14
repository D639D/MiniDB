use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};


use crate::codec::Types;
use crate::codec::{EncodeKey, ToBytes, read_cell};
use crate::errors::{self};
use crate::keydir::KeyDir;
use crate::logfile::LogIndex;
use crate::writer::Writer;

#[derive(Debug)]
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
            .append(true)
            .open(name)
            .map_err(|err| errors::DataBaseError {
                kind: errors::DataBaseErrorKind::Io(err),
            })?;
        let keydir = KeyDir::new();

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

        let logindex = self.write_to_active_data_file(&encode_key, &encode_data)?;

        self.keydir.set(key.to_string(), logindex);

        Ok(())
    }

    pub fn write_to_active_data_file(
        &mut self,
        encode_key: &[u8],
        encode_data: &[u8],
    ) -> Result<LogIndex, errors::DataBaseError> {
        let len = encode_data.len();
        self.writer
            .active_data_file
            .write_all(&encode_key)
            .map_err(|err| errors::DataBaseError {
                kind: errors::DataBaseErrorKind::Io(err),
            })?;

        let offset = self
            .writer
            .active_data_file
            .stream_position()
            .map_err(|err| errors::DataBaseError {
                kind: errors::DataBaseErrorKind::Io(err),
            })? as usize;
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
        let index = self.keydir.get(key, &mut self.writer.active_data_file);
        let offset = index.index.offset as u64;
        let len = index.index.len;
        let mut buf = vec![0; len];
        self.writer
            .active_data_file
            .seek(std::io::SeekFrom::Start(offset))
            .map_err(|err| errors::DataBaseError {
                kind: errors::DataBaseErrorKind::Io(err),
            })?;
        self.writer
            .active_data_file
            .read(&mut buf)
            .map_err(|err| errors::DataBaseError {
                kind: errors::DataBaseErrorKind::Io(err),
            })?;

        let res = read_cell(&buf).map_err(|err| errors::DataBaseError {
            kind: errors::DataBaseErrorKind::Codec(err),
        })?;
        Ok(res)
    }
}
