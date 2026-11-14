use minidb::database::DataBase;

fn main() {
    let mut db = DataBase::new("main").unwrap();
    // db.set("User1", 1).unwrap();
    // db.set("User2", true).unwrap();
    println!("{:?}", db.get(String::from("User2")));
}
