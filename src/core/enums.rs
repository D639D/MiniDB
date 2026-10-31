#[derive(Debug)]
pub enum Types {
    Int(i32),
    Float(f64),
    Text(String),
}

#[derive(Debug)]
pub enum DataBaseErr {
    Int(String),
    Float(String),
    Str(String),
    Other(String),
}