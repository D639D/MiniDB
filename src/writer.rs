use std::fs::File;
#[derive(Debug)]
pub struct Writer {
    pub active_data_file: File
}

// pub struct DataBase {
//     file: File,
// }

// impl DataBase {
//     pub fn create(name: &str) -> Result<Self, errors::DataBaseError> {
//         let file = OpenOptions::new()
//             .write(true)
//             .read(true)
//             .create(true)
//             .truncate(true)
//             .open(name)
//             .map_err(|error| errors::DataBaseError {
//                 kind: errors::DataBaseErrorKind::Io(error),
//             })?;

//         Ok(DataBase { file })
//     }

//     pub fn set<K: EncodeKey, V: ToBytes>(&mut self, key: K, value: V) -> io::Result<()> {
//         self.file.write_all(&key.encode_key())?;
//         self.file.write_all(&value.to_bytes())?;

//         Ok(())
//     }

//     pub fn get(&mut self, key: &str) -> Result<Types, errors::DataBaseError> {
//         let encode_key = key.as_bytes();
//         // 1. Поставить указатель в начало файла
//         self.file
//             .seek(io::SeekFrom::Start(0))
//             .map_err(|_| errors::DataBaseError {
//                 kind: errors::DataBaseErrorKind::FailGetSeekStart,
//             })?;

//         // 2. Получить смещение, изначально 0
//         let mut offset = self
//             .file
//             .stream_position()
//             .map_err(|_| errors::DataBaseError {
//                 kind: errors::DataBaseErrorKind::FailGetStartPosition,
//             })?;
//         // Создать буфер
//         let mut buf = Vec::new();

//         loop {
//             println!("1. Получаем смещение: {:?}", offset);
//             buf.clear();
//             // Смещаем указатель на смещение
//             self.file
//                 .seek(io::SeekFrom::Start(offset))
//                 .map_err(|_| errors::DataBaseError {
//                     kind: errors::DataBaseErrorKind::FailSetSeekPosition,
//                 })?;
//             // Читаем файл и записываем байты в буфер
//             self.file
//                 .read_to_end(&mut buf)
//                 .map_err(|_| errors::DataBaseError {
//                     kind: errors::DataBaseErrorKind::FailReadToEnd,
//                 })?;

//             let slice: &[u8] = &buf;
//             println!("2. Полученные байты: {:?}", slice);
//             match slice {
//                 [] => {
//                     return Err(errors::DataBaseError {
//                         kind: errors::DataBaseErrorKind::EmptyArray,
//                     });
//                 }
//                 [KEY_TAG, a, b, c, d, data @ ..] => {
//                     // Превращаем 4 байта после KEY_TAG в длинну данных

//                     let len_key = u32::from_ne_bytes([*a, *b, *c, *d]) as usize;
//                     println!("3. Длина данных Key: {:?}", &len_key);
//                     // Получаем данные из байтов, по длинне данных
//                     let arr_key = data.get(..len_key).ok_or_else(|| errors::DataBaseError {
//                         kind: errors::DataBaseErrorKind::FailGetLen,
//                     })?;
//                     println!("4. Данные из байтов Key: {:?}", &arr_key);

//                     // Переходи на следующий тег данных
//                     let next_tag = data.get(len_key..).ok_or_else(|| errors::DataBaseError {
//                         kind: errors::DataBaseErrorKind::FailGetLen,
//                     })?;
//                     println!("5. Следующие данные: {:?}", next_tag);
//                     if arr_key == encode_key {
//                         // Проверка тега
//                         match next_tag {
//                             [] => {
//                                 return Err(errors::DataBaseError {
//                                     kind: errors::DataBaseErrorKind::EmptyArray,
//                                 });
//                             }
//                             [INT_TAG, a, b, c, d, ..] => {
//                                 let arr: [u8; 4] = [*a, *b, *c, *d];
//                                 let res = i32::from_bytes(arr.to_vec()).map(Types::Int).map_err(
//                                     |_| errors::DataBaseError {
//                                         kind: errors::DataBaseErrorKind::FailFromBytes,
//                                     },
//                                 )?;
//                                 println!("6. Байты данных Int: {:?}", &arr);
//                                 println!("7. Значение данных Int: {:?}", &res);
//                                 return Ok(res);
//                             }
//                             [FLOAT_TAG, a, b, c, d, e, f, g, h, ..] => {
//                                 let arr: [u8; 8] = [*a, *b, *c, *d, *e, *f, *g, *h];

//                                 let res = f64::from_bytes(arr.to_vec()).map(Types::Float).map_err(
//                                     |_| errors::DataBaseError {
//                                         kind: errors::DataBaseErrorKind::FailFromBytes,
//                                     },
//                                 )?;
//                                 println!("6. Байты данных Float: {:?}", &arr);
//                                 println!("7. Значение данных Float: {:?}", &res);
//                                 return Ok(res);
//                             }
//                             [TEXT_TAG, a, b, c, d, data @ ..] => {
//                                 let len = u32::from_ne_bytes([*a, *b, *c, *d]) as usize;
//                                 let arr = data.get(..len).ok_or_else(|| errors::DataBaseError {
//                                     kind: errors::DataBaseErrorKind::FailGetArr,
//                                 })?;
//                                 let res = String::from_bytes(arr.to_vec())
//                                     .map(Types::Text)
//                                     .map_err(|_| errors::DataBaseError {
//                                         kind: errors::DataBaseErrorKind::FailFromBytes,
//                                     })?;
//                                 println!("6. Байты данных TEXT: {:?}", &arr);
//                                 println!("7. Значение данных TEXT: {:?}", &res);
//                                 println!("8. Байты данных TEXT: {:?}", &len);
//                                 return Ok(res);
//                             }
//                             _ => {
//                                 return Err(errors::DataBaseError {
//                                     kind: errors::DataBaseErrorKind::UnknownTypeKey,
//                                 });
//                             }
//                         }
//                     } else {
//                         match next_tag {
//                             [] => {
//                                 return Err(errors::DataBaseError {
//                                     kind: errors::DataBaseErrorKind::EmptyArray,
//                                 });
//                             }
//                             [INT_TAG, ..] => {
//                                 let tmp = 1 + 4 + len_key + 5;
//                                 offset += tmp as u64;
//                                 println!("9. Длинна смещения: {:?}\n", &offset);
//                             }
//                             [FLOAT_TAG, ..] => {
//                                 let tmp = 1 + 4 + len_key + 9;
//                                 offset += tmp as u64;
//                                 println!("9. Длинна смещения: {:?}\n", &offset);
//                             }
//                             [TEXT_TAG, a, b, c, d, ..] => {
//                                 let len = u32::from_ne_bytes([*a, *b, *c, *d]) as usize;
//                                 let tmp = 1 + 4 + len + 1 + 4 + len_key;
//                                 offset += tmp as u64;
//                                 println!("9. Длинна смещения: {:?}\n", &offset);
//                             }
//                             _ => {
//                                 return Err(errors::DataBaseError {
//                                     kind: errors::DataBaseErrorKind::UnknownTypeData,
//                                 });
//                             }
//                         }
//                     }
//                 }
//                 _ => {
//                     return Err(errors::DataBaseError {
//                         kind: errors::DataBaseErrorKind::UnknownTypeKey,
//                     });
//                 }
//             };
//         }
//     }
// }
