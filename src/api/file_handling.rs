use std::io;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use crate::api::m64_handling::ByteVec;

#[derive(Debug, Clone)]
pub enum DialogError {
    DialogClosed,
    IoError(io::ErrorKind),
}
pub fn read_file(path: &Path) -> io::Result<ByteVec> {

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn save_file(path: &Path, bytes: &ByteVec) -> io::Result<(File)> {
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    Ok(file)
}

