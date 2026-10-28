use std::fs::File;
use std::io::{self, Write};
use std::path::Path;


#[derive(PartialEq, Debug)]
enum Types {
    Int(i32),
    Float(f64),
    Text(String),
}
#[derive(Debug)]
enum Error {
    IntErr(String),
    FloatErr(String),
    TextErr(String),
    OtherErr(String),
}


struct Database {
    file: File,
}

trait ToBytes {
    type Output;
    fn to_bytes(&self) -> Self::Output;
}
trait FromBytes: Sized {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self>;
}

impl ToBytes for i32 {
    type Output = Vec<u8>;
    fn to_bytes(&self) -> Self::Output {
        let mut result = Vec::with_capacity(5);
        result.extend_from_slice(&[1]);
        result.extend_from_slice(&self.to_ne_bytes());
        result
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
        let mut result = Vec::with_capacity(9);
        result.extend_from_slice(&[1]);
        result.extend_from_slice(&self.to_ne_bytes());
        result
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
        let len = self.len();
        let mut result = Vec::with_capacity(len + 2);
        result.extend_from_slice(&[1]);
        result.extend_from_slice(&self.as_bytes());
        result
    }
}
impl FromBytes for String {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        String::from_utf8(bytes).ok()
    }
}

impl Write for Database {
    // Метод записи, для крейта "write_all"
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {

        Database::write_all(self, buf)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        Ok(buf.len())
    }
    // Хз что это, вроде как буфер
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Database {
    // Создание базы данных
    fn create<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::create(path).map_err(|_| Error::OtherErr("Не удалось создать базу данных!".into()))?;

            Ok(Database { file })
    }
    // Запись в базу данных
    fn write_cell(&mut self, bites: &[u8]) -> Result<(), Error> {
        self.write_all(bites)
            .map_err(|_| Error::OtherErr("fs".into()))?;
    
        Ok(())
    }
}

fn main(){
    let mut db = Database::create("main").unwrap();
}
// Конвертация в байты
fn convert_to_cell(cell: &Types) -> Vec<u8> {
    match cell {
        Types::Int(i) => i.to_bytes(),
        Types::Float(f) => f.to_bytes(),
        Types::Text(s) => s.to_bytes(),
    }
}
// Чтение байтов
fn read_cell(bytes: &[u8]) -> Result<(Types, usize), Error> {
    match bytes {
        // пустой массив
        [] => Err(Error::OtherErr("Массив пустой".into())),

        // Int: 1 байт тега + ровно 4 байта данных
        [1, a, b, c, d, ..] => {
            let arr: [u8; 4] = [*a, *b, *c, *d];
            i32::from_bytes(arr.to_vec())
                .map(|i| (Types::Int(i), 5))
                .ok_or_else(|| Error::IntErr("Не удалось прочитать Int".into()))
        }

        // Float: 1 + 8 = 9 байт
        [2, a, b, c, d, e, f, g, h, ..] => {
            let arr: [u8; 8] = [*a, *b, *c, *d, *e, *f, *g, *h];
            f64::from_bytes(arr.to_vec())
                .map(|f| (Types::Float(f), 9))
                .ok_or_else(|| Error::FloatErr("Не удалось прочитать Float".into()))
        }

        // Text: 1 байт тега + 1 байт длины + данные
        [3, len, data @ ..] => {
            let len = *len as usize;
            let slice = data
                .get(..len)
                .ok_or_else(|| Error::TextErr(format!("Массив короче заявленной длины ({len})")))?;
            String::from_bytes(slice.to_vec())
                .map(|s| (Types::Text(s), 2 + len))
                .ok_or_else(|| Error::TextErr("Не удалось прочитать Text".into()))
        }

        // любой другой тег
        _ => Err(Error::OtherErr("Неизвестный тип ячейки".into())),
    }
}