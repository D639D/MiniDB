use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug)]
enum Types {
    Int(i32),
    Float(f64),
    Text(String),
}
#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    IntErr(String),
    FloatErr(String),
    TextErr(String),
    OtherErr(String),
}

trait ToBytes {
    type Output;
    fn to_bytes(&self) -> Self::Output;
}
trait FromBytes: Sized {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self>;
}

trait CreateDb {
    fn new_db(&self) -> Self;
}


impl ToBytes for i32 {
    type Output = Vec<u8>;
    fn to_bytes(&self) -> Self::Output {
        self.to_ne_bytes().to_vec()
    }
}
impl FromBytes for i32 {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        let array: [u8; 4] = bytes.try_into().ok()?;
        Some(i32::from_ne_bytes(array))
    }
}

impl ToBytes for f64 {
    type Output = Vec<u8>;
    fn to_bytes(&self) -> Self::Output {
        self.to_ne_bytes().to_vec()
    }
}
impl FromBytes for f64 {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        let array: [u8; 8] = bytes.try_into().ok()?;
        Some(f64::from_ne_bytes(array))
    }
}

impl ToBytes for String {
    type Output = Vec<u8>;
    fn to_bytes(&self) -> Self::Output {
        self.as_bytes().to_vec()
    }
}
impl FromBytes for String {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        String::from_utf8(bytes).ok()
    }
}

pub trait NewDB: Sized {
    fn new(name: impl AsRef<Path>) -> io::Result<Self>;
}

pub struct Db(File);

impl NewDB for Db {
    fn new(name: impl AsRef<Path>) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(name)?;
        Ok(Db(file))
    }
}

fn main() {

    let num_int = 32;
    let bytes_str = num_int.to_bytes();
    let len = bytes_str.len();
    let prefix: [u8; 1] = [1];
    let mut result = Vec::with_capacity(len + 1);
    result.extend_from_slice(&prefix);
    result.extend_from_slice(&bytes_str);
    println!("{:?}", read_cell(&result));
}

fn read_cell(bytes: &[u8]) -> Result<(Types, usize), Error> {
    match bytes {
        // пустой массив
        [] => Err(Error::OtherErr("Массив пустой".into())),

        // Int: 1 байт тега + ровно 4 байта данных
        [1, data @ ..] => {
            let arr: [u8; 4] = data.try_into()
                .map_err(|_| Error::IntErr("Ожидалось 5 байт".into()))?;
            i32::from_bytes(arr.to_vec())
                .map(|f|(Types::Int(f), 5))
                .ok_or_else(|| Error::IntErr("Не удалось прочитать Int".into()))
        }

        // Float: 1 + 8 = 9 байт
        [2, data @ ..] => {
            let arr: [u8; 8] = data.try_into()
                .map_err(|_| Error::FloatErr("Ожидалось 9 байт".into()))?;
            f64::from_bytes(arr.to_vec())
                .map(|f| (Types::Float(f), 9))
                .ok_or_else(|| Error::FloatErr("Не удалось прочитать Float".into()))
        }

        // Text: 1 байт тега + 1 байт длины + данные
        [3, len, data @ ..] => {
            let len = *len as usize;
            let slice = data.get(..len)
                .ok_or_else(|| Error::TextErr(
                    format!("Массив короче заявленной длины ({len})")
                ))?;
            String::from_bytes(slice.to_vec())
                .map(|f|(Types::Text(f), 2 + len))
                .ok_or_else(|| Error::TextErr("Не удалось прочитать Text".into()))
        }

        // любой другой тег
        _ => Err(Error::OtherErr("Неизвестный тип ячейки".into())),
    }
}