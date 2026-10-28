#[derive(PartialEq, Debug)]
pub enum Types {
    Int(i32),
    Float(f64),
    Text(String),
}
#[derive(Debug)]
pub enum Error {
    IntErr(String),
    FloatErr(String),
    TextErr(String),
    OtherErr(String),
}