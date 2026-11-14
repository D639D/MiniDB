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

pub enum KeyDirErrorKind {
    Truncated,
    UnknowTag(u8),
    Io(std::io::Error),
    Codec(CodecError),
}
pub struct KeyDirError {
    pub kind: KeyDirErrorKind,
}

#[derive(Debug)]
pub enum DataBaseErrorKind {
    Io(std::io::Error),
    Codec(CodecError),
    KeyDir(CodecError),
}
#[derive(Debug)]
pub struct DataBaseError {
    pub kind: DataBaseErrorKind,
}
