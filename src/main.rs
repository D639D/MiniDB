use std::collections::HashMap;

fn main() {
    let number = String::from("10");

    let mut db = Vec::new();
    db.push(HashMap::new());
    db[0].insert(String::from("User_text"), number);

    for item in &db {
        for (_, value) in item {
            println!("{:?}", value.parse::<i32>().unwrap());
        }
    }
}
