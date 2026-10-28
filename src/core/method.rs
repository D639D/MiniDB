pub impl ToBytes for i32 {
    type Output = Vec<u8>;
    fn to_bytes(&self) -> Self::Output {
        let mut result = Vec::with_capacity(5);
        result.extend_from_slice(&[1]);
        result.extend_from_slice(&self.to_ne_bytes());
        result
    }
}

pub impl ToBytes for f64 {
    type Output = Vec<u8>;
    fn to_bytes(&self) -> Self::Output {
        let mut result = Vec::with_capacity(9);
        result.extend_from_slice(&[1]);
        result.extend_from_slice(&self.to_ne_bytes());
        result
    }
}

pub impl ToBytes for String {
    type Output = Vec<u8>;
    fn to_bytes(&self) -> Self::Output {
        let len = self.len();
        let mut result = Vec::with_capacity(len + 2);
        result.extend_from_slice(&[1]);
        result.extend_from_slice(&self.as_bytes());
        result
    }
}


pub impl FromBytes for i32 {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        let array: [u8; 4] = bytes.try_into().ok()?;
        Some(i32::from_ne_bytes(array))
    }
}

pub impl FromBytes for f64 {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        let array: [u8; 8] = bytes.try_into().ok()?;
        Some(f64::from_ne_bytes(array))
    }
}

pub impl FromBytes for String {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        String::from_utf8(bytes).ok()
    }
}


pub impl Write for Database {
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

pub impl Database {
    // Создание базы данных
    fn create<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::create(path).map_err(|_| Error::OtherErr("Не удалось создать базу данных!".into()))?;

            Ok(Database { file })
    }
    // Запись в базу данных
    fn write_cell<T>(&mut self, bites: T) -> Result<(), Error> {
        self.write_all(bites)
            .map_err(|_| Error::OtherErr("fs".into()))?;
    
        Ok(())
    }
}