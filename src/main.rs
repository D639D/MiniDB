use minidb::database::DataBase;
fn main() {
    // let mut db = DataBase::create("main").unwrap();
    // db.set("User", "1").unwrap();
    // println!("{:?}", db.get("User").unwrap());

    let mut db = DataBase::new("main").unwrap();
    db.set("User1", 1);
    db.set("User2", 1.0);
    db.set("User3", "1");
    db.set("User1", 2);
    println!("{:?}", db.get(String::from("User3")));
}
