use std::fs::OpenOptions;
use std::path;
use std::io::{Write, Error as IoError};

pub fn write_at_end_of_file(file_path : &path::Path, line: &str) -> Result<(), IoError> {
    let mut file = OpenOptions::new().append(true).open(&file_path)?;
    writeln!(file, "{}", line)?;
    Ok(())
}