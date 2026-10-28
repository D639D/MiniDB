use minidb::core::{enums, method, structs};

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let mut db = Database::create("main").unwrap();
    db.write_cell(1).unwrap();
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
