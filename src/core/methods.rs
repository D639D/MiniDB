use std::fmt::{self};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::core::enums::DataBaseErr;
use crate::core::structs::DataBase;

use super::enums;
use super::functions;
use super::structs;
use super::traits;

impl fmt::Display for enums::Types {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            enums::Types::Int(v) => write!(f, "{v}"),
            enums::Types::Float(v) => write!(f, "{v}"),
            enums::Types::Text(v) => write!(f, "{v}"),
        }
    }
}

impl fmt::Display for enums::DataBaseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            enums::DataBaseErr::Int(m) => write!(f, "ошибка Int: {m}"),
            enums::DataBaseErr::Float(m) => write!(f, "ошибка Float: {m}"),
            enums::DataBaseErr::Str(m) => write!(f, "ошибка String: {m}"),
            enums::DataBaseErr::Other(m) => write!(f, "ошибка: {m}"),
        }
    }
}

impl std::error::Error for enums::DataBaseErr {}

impl traits::ToBytes for i32 {
    fn to_bytes(&self) -> Result<Vec<u8>, DataBaseErr> {
        if *self > u8::MAX as i32 {
            return Err(DataBaseErr::Str("Число слишком большое!".into()));
        }
        let mut res = Vec::with_capacity(5);
        res.extend_from_slice(&[1]);
        res.extend_from_slice(&self.to_ne_bytes());
        Ok(res)
    }
}

impl traits::ToBytes for f64 {
    fn to_bytes(&self) -> Result<Vec<u8>, DataBaseErr> {
        if *self > u8::MAX as f64 {
            return Err(DataBaseErr::Str("Число слишком большое!".into()));
        }
        let mut res = Vec::with_capacity(9);
        res.extend_from_slice(&[2]);
        res.extend_from_slice(&self.to_ne_bytes());
        Ok(res)
    }
}

impl traits::ToBytes for String {
    fn to_bytes(&self) -> Result<Vec<u8>, DataBaseErr> {
        if self.len() > u8::MAX as usize {
            return Err(DataBaseErr::Str("Строка слишком длинная!".into()));
        }
        let mut res = Vec::new();
        res.extend_from_slice(&[3]);
        res.extend_from_slice(&[self.len() as u8]);
        res.extend_from_slice(self.as_bytes());
        Ok(res)
    }
}

impl traits::FromBytes for i32 {
    type Output = i32;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self::Output, enums::DataBaseErr> {
        let array: [u8; 4] = bytes
            .try_into()
            .map_err(|_| enums::DataBaseErr::Int("Ошибка преобразования байтов в i32".into()))?;
        Ok(i32::from_ne_bytes(array))
    }
}

impl traits::FromBytes for f64 {
    type Output = f64;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self::Output, enums::DataBaseErr> {
        let array: [u8; 8] = bytes
            .try_into()
            .map_err(|_| enums::DataBaseErr::Float("Ошибка преобразования байтов в f64".into()))?;
        Ok(f64::from_ne_bytes(array))
    }
}

impl traits::FromBytes for String {
    type Output = String;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self::Output, enums::DataBaseErr> {
        String::from_utf8(bytes)
            .map_err(|_| enums::DataBaseErr::Str("Ошибка преобразования байтов в String".into()))
    }
}

impl structs::DataBase {
    // Создать базу данных
    pub fn create(name: &str) -> Result<structs::DataBase, enums::DataBaseErr> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(name)
            .map_err(|_| enums::DataBaseErr::Other("Ошибка создания базы данных!".into()))?;

        Ok(structs::DataBase { file })
    }
    // Записать байты
    pub fn write_cell<T: traits::ToBytes>(
        &mut self,
        value: T,
    ) -> Result<usize, enums::DataBaseErr> {
        let bytes = functions::variable(value)?;
        self.file
            .write_all(&bytes)
            .map_err(|_| enums::DataBaseErr::Other("Ошибка записи байтов в файл".into()))?;
        Ok(bytes.len())
    }
    // Прочитать файл
    pub fn read_row(&mut self) -> Result<Vec<u8>, enums::DataBaseErr> {
        let _ = self
            .file
            .seek(SeekFrom::Start(0))
            .map_err(|_| enums::DataBaseErr::Other("Не удалось прочитать файл".into()));
        let mut buf = Vec::new();
        self.file
            .read_to_end(&mut buf)
            .map_err(|_| enums::DataBaseErr::Other("Ошибка чтения файла".into()))?;

        Ok(buf)
    }
}
