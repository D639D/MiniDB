use super::enums;
use super::traits::FromBytes;

// Преобразовать в байты

// Преобразование байтов в значение
pub fn read_cell(value: &[u8]) -> Result<(enums::Types, usize), enums::DataBaseErr> {
    match value {
        [] => Err(enums::DataBaseErr::Other("Пустой массив".into())),

        [1, a, b, c, d, ..] => {
            let arr: [u8; 4] = [*a, *b, *c, *d];
            i32::from_bytes(arr.to_vec()).map(|i| (enums::Types::Int(i), arr.len() + 1))
        }
        [2, a, b, c, d, e, f, g, h, ..] => {
            let arr: [u8; 8] = [*a, *b, *c, *d, *e, *f, *g, *h];
            f64::from_bytes(arr.to_vec()).map(|f| (enums::Types::Float(f), arr.len() + 1))
        }
        [3, len, data @ ..] => {
            let len = *len as usize;
            let arr = data.get(..len).ok_or_else(|| {
                enums::DataBaseErr::Str("Не удалось получить срез по длинне массива".into())
            })?;
            String::from_bytes(arr.to_vec()).map(|s| (enums::Types::Text(s), arr.len() + 2))
        }
        _ => Err(enums::DataBaseErr::Other("Тип не найден".into())),
    }
}

pub fn read_bytes(bytes: &[u8]) -> Result<Vec<enums::Types>, enums::DataBaseErr> {
    let mut offset = 0;
    let mut res = Vec::new();
    while bytes.len() > offset {
        let (num, tmp) = read_cell(&bytes[offset..])?;
        offset += tmp;
        res.push(num);
    }
    Ok(res)
}
