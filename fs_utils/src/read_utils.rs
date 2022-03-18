use std::fs;
use std::path;
use std::io;
use std::io::BufRead;

pub fn read_lines(path: &path::Path) -> Result<Vec<String>, io::Error> {
    let file_to_read = fs::File::open(&path)?;
    let file_content = io::BufReader::new(file_to_read).lines();
    let mut lines = Vec::new();
    for line in file_content {
        if let Ok(l) = line {
            lines.push(l);
        }
    }
    Ok(lines)
}