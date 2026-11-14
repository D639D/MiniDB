use minidb::database::DataBase;
fn main() {
    let mut db = DataBase::new("main").unwrap();
    // db.set("User1", 1).unwrap();
    // db.set("User2", 1.0).unwrap();
    // db.set("User3", "1").unwrap();
    // db.set("User4", 100).unwrap();
    println!("{:?}", db.get(String::from("User4")));
}
