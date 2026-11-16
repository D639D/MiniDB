use minidb::database::DataBase;

fn main() {
    let mut db = DataBase::new("main").unwrap();
    // db.set("A", 1).unwrap();
    // db.set("B", 1.0).unwrap();
    // db.delete(String::from("B"));
    // db.set("C", 2.0).unwrap();
    println!("{:?}", db.get(String::from("B")));
}
