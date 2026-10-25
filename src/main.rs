fn main() {
    let one_1: i32 = 1;
    let two_1: f64 = 1.0;
    let three_1: String = String::from("1");

    let one_1 = one_1.to_le_bytes();
    let two_1 = two_1.to_le_bytes();
    let three_1 = three_1.as_bytes();

    let one_2: i64 = 2;
    let two_2: f64 = 2.0;
    let three_2: String = String::from("Hello");

    let one_2 = one_2.to_le_bytes();
    let two_2 = two_2.to_le_bytes();
    let three_2 = three_2.as_bytes();

    println!("{:?} || {:?} || {:?}", one_1, two_1, three_1);
    println!("{:?} || {:?} || {:?}", one_2, two_2, three_2);
}
