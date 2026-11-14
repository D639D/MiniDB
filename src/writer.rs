use std::fs::File;
#[derive(Debug)]
pub struct Writer {
    pub active_data_file: File,
}
