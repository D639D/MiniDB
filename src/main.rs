use minidb::database::DataBase;
fn main() {
    // let mut db = DataBase::create("main").unwrap();
    // db.set("User", "1").unwrap();
    // println!("{:?}", db.get("User").unwrap());

    let mut db = DataBase::new("main").unwrap();
    println!("{:?}", db.set("User", 1));
    println!("{:?}", db.get(String::from("User")));
}
