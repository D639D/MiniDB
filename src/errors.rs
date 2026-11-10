use std::io;

// -------------------------------- //
#[derive(Debug)]
pub struct DataBaseError {
    pub kind: DataBaseErrorKind,
}
#[derive(Debug)]
pub enum DataBaseErrorKind {
    Io(io::Error),
    OutsideDataBaseCreate,
    FailGetSeekStart,
    FailGetStartPosition,
    FailSetSeekPosition,
    FailGetLen,
    FailReadToEnd,
    FailGetArr,
    UnknownTypeData,
    UnknownTypeKey,
    UnknownType,
    EmptyArray,
    FailFromBytes,
}

// -------------------------------- //

#[derive(Debug)]
pub struct ReadCellError {
    pub kind: ReadErrorKind,
}

#[derive(Debug)]
pub enum ReadErrorKind {
    Io(io::Error),
    FailDecode,
    EmptyArray,
    UnknownType,
}

// -------------------------------- //
#[derive(Debug)]
pub struct FromBytesError {
    pub kind: FromBytesErrorKind,
}
#[derive(Debug)]
pub enum FromBytesErrorKind {
    Io(io::Error),
    FailDecodeFromBytes,
}

#[derive(Debug)]
pub struct LogFile {
    pub kind: LogFileKind,
}
#[derive(Debug)]
pub enum LogFileKind {
    Io(io::Error),
    FailCreateLogIndex,
}