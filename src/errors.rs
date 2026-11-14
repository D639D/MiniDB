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
pub enum KeyDirErrorKind {
    Truncated,
    UnknowTag(u8),
    KeyNotFound,
    Io(std::io::Error),
    Codec(CodecError),
}
#[derive(Debug)]
pub struct KeyDirError {
    pub kind: KeyDirErrorKind,
}

#[derive(Debug)]
pub enum DataBaseErrorKind {
    Io(std::io::Error),
    Codec(CodecError),
    KeyDir(KeyDirError),
}
#[derive(Debug)]
pub struct DataBaseError {
    pub kind: DataBaseErrorKind,
}
