use std::io::Error as IoError;

use tokio::io::{self as tokioIo, AsyncBufReadExt};

pub async fn read_command() -> Result<String, IoError> {
    let mut s = vec![];

    let mut reader = tokioIo::BufReader::new(tokioIo::stdin());
    reader
        .read_until(b'\n', &mut s)
        .await
        .expect("Reading from stdin failed");
    s.pop();
    Ok(std::str::from_utf8(&s).unwrap().to_string())
}
