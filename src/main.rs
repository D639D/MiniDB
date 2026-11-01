use minidb::core::{functions, structs};

fn main() {
    let mut db = structs::DataBase::create("main_1.db").unwrap();
    // db.write_cell(String::from("Hello")).unwrap();
    // db.write_cell(255).unwrap();
    println!("1. {:?}", db.write_cell(256).unwrap());
    // db.write_cell(String::from("ABDFАGHIJKLMNOPQRSFSKGLFDGKJHKFАJGGKHKGLFHKHJLFDFSFSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKL")).unwrap();
    let bytes = db.read_file().unwrap();
    println!("2. {:?}", &bytes);
    println!("3. {:?}", functions::read_bytes(&bytes));
}
