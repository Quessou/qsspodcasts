use std::io::Error as IoError;

pub fn read_command() -> Result<String, IoError> {
    let mut s = String::from("");
    std::io::stdin().read_line(&mut s)?;
    Ok(s)
}
