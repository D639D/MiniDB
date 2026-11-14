#[derive(Debug)]
pub enum CodecErrorKind {
    Truncated,
    InvalidUtf8,
    UnknowTag(u8),
    InvalidLength { expected: usize, actual: usize },
}
#[derive(Debug)]
pub struct CodecError {
    pub kind: CodecErrorKind,
}
#[derive(Debug)]
pub enum DataBaseErrorKind {
    Io(std::io::Error),
    Codec(CodecError),
}
#[derive(Debug)]
pub struct DataBaseError {
    pub kind: DataBaseErrorKind,
}
