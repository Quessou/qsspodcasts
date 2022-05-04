use bytes::Bytes;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Error as IoError, ErrorKind, Write};
use std::path;

pub fn write_at_end_of_file(file_path: &path::Path, line: &str) -> Result<(), IoError> {
    let mut file = OpenOptions::new().append(true).open(&file_path)?;
    writeln!(file, "{}", line)?;
    Ok(())
}

pub fn open_or_create_file(file_path: &str) -> Result<File, std::io::Error> {
    let file = File::open(file_path);
    let file = match file {
        Ok(file) => file, // nominal case
        Err(error) => match error.kind() {
            // Handling the error depending of its content
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(fc) => fc, // Create the file if it's just a matter of not finding it
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    Ok(file)
}

pub fn write_bytes_in_file(file_path: &str, bytes: &Bytes) -> Result<(), std::io::Error> {
    let mut file = open_or_create_file(file_path)?;
    file.write(&bytes.to_vec())?;
    file.flush()?;
    Ok(())
}
