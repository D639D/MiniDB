use super::*;
use rand::{Rng, distributions::Alphanumeric};

fn temp_path() -> structs::DataBase {
    let random_text: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let mut p = std::env::temp_dir();
    p.push(format!(
        "minidb_int_{}_{}.db",
        std::process::id(),
        random_text
    ));
    let s = p.to_string_lossy().into_owned();
    let _ = std::fs::remove_file(&s);
    let db = structs::DataBase::create(&s).unwrap();
    db
}

#[cfg(test)]
mod tests {
    use crate::core::{functions, tests::temp_path};

    #[test]
    fn overflow_u8_int() {
        let mut db = temp_path();
        let input = 255;
        db.write_cell(input).expect("writ succed");
        let row = db.read_file().expect("read row succeed");
        let cells = functions::read_bytes(&row).expect("parse row");
        assert_eq!(cells.len(), 1);
    }

    #[test]
    fn overflow_u8_float() {
        let mut db = temp_path();
        let input = 255.0;
        db.write_cell(input).expect("writ succed");
        let row = db.read_file().expect("read row succeed");
        let cells = functions::read_bytes(&row).expect("parse row");
        assert_eq!(cells.len(), 1);
    }

    #[test]
    fn overflow_u8_string() {
        let mut db = temp_path();
        let input = String::from("fabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstu");
        db.write_cell(input).expect("writ succed");
        let row = db.read_file().expect("read row succeed");
        let cells = functions::read_bytes(&row).expect("parse row");
        assert_eq!(cells.len(), 1);
    }
}
