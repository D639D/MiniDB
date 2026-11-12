use minidb::database::DataBase;
use std::path::PathBuf;
fn main() {
    // let mut db = DataBase::create("main").unwrap();
    // db.set("User", "1").unwrap();
    // println!("{:?}", db.get("User").unwrap());
    let path = PathBuf::new();
    let mut db = DataBase::new("main", &path).unwrap();

    println!("{:?}", db.get(String::from("User")));
}
