use std::io::Error as IoError;

use tokio::io::{self as tokioIo, AsyncBufReadExt, AsyncWriteExt};

pub async fn show_prompt() {
    // TODO : define this as static global (or equivalent) to prevent useless instanciations at each call
    let mut writer = tokioIo::BufWriter::new(tokioIo::stdout());
    writer
        .write_all(b">>> ")
        .await
        .expect("Writing prompt failed");
    writer.flush().await;
    //writer.wr
}

pub async fn read_command() -> Result<String, IoError> {
    let mut s = vec![];

    show_prompt().await;

    let mut reader = tokioIo::BufReader::new(tokioIo::stdin());
    reader
        .read_until(b'\n', &mut s)
        .await
        .expect("Reading from stdin failed");
    Ok(std::str::from_utf8(&s).unwrap().to_string())
}
