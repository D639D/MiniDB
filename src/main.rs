use minidb::database::DataBase;

fn main() {
    let mut db = DataBase::create("main").unwrap();
    db.set("User", 1).unwrap();
    println!("{:?}", db.get("User").unwrap());
}
