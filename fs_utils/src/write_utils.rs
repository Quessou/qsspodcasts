use bytes::Bytes;
use itertools::Itertools;
use std::fs::OpenOptions;
use std::fs::{read_to_string, File};
use std::io::BufWriter;
use std::io::{Error as IoError, ErrorKind, Write};
use std::path;

pub fn write_at_end_of_file(file_path: &path::Path, line: &str) -> Result<(), IoError> {
    let mut file = OpenOptions::new().append(true).open(file_path)?;
    writeln!(file, "{}", line)?;
    Ok(())
}

pub fn delete_line_in_file(file_path: &path::Path, line: &str) -> Result<(), IoError> {
    let file = read_to_string(file_path).unwrap();
    let lines = file.lines();
    let lines = lines.filter(|l| !l.contains(line));
    let file_content: String = Itertools::intersperse(lines, "\n").collect();
    let file = File::create(file_path).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write(file_content.as_bytes())?; //file_content.as_bytes())?;
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
    file.write_all(bytes)?;
    file.flush()?;
    Ok(())
}
