use minidb::core::{functions, structs};

fn main() {
    let mut db = structs::DataBase::create("main_1.db").unwrap();
    db.write_cell(String::from("Hello")).unwrap();
    db.write_cell(932).unwrap();
    db.write_cell(1.0).unwrap();
    db.write_cell(String::from("ABCDEFGHIJKLMNOPQRFSFSKGLFDGKJHKFАJGGKHKGLGFHKHJLFDFSFSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKL")).unwrap();
    let bytes = db.read_row().unwrap();
    println!("{:?}", functions::read_bytes(&bytes));

}
