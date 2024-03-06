use std::path::PathBuf;
use std::time::Duration;

use tokio::io::AsyncWriteExt;

pub async fn write_progression_in_file(
    progression: &Duration,
    file_path: PathBuf,
) -> Result<(), ()> {
    let file = tokio::fs::File::create(file_path)
        .await
        .expect("Opening of file containing progression failed");
    let mut file_writer = tokio::io::BufWriter::new(file);

    let progression_s = progression.as_secs();
    file_writer
        .write_u64(progression_s)
        .await
        .expect("Writing of progression failed");
    Ok(())
}
