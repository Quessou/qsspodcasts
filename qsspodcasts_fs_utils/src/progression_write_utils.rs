use std::path::PathBuf;
use std::time::Duration;

use tokio::io::AsyncWriteExt;

pub async fn write_progression_in_file(
    progression: &Duration,
    file_path: PathBuf,
) -> Result<(), ()> {
    // If the parent directory does not exist, we're screwed
    assert!(file_path.parent().unwrap().exists());
    let file = tokio::fs::File::create(file_path)
        .await
        .expect("Opening of file containing progression failed");
    let mut file_writer = tokio::io::BufWriter::new(file);

    let progression_s = progression.as_secs();
    file_writer
        .write_u64(progression_s)
        .await
        .expect("Writing of progression failed");
    file_writer
        .flush()
        .await
        .expect("Flushing of file containing progression failed");
    file_writer
        .shutdown()
        .await
        .expect("Closing of file writer failed");
    Ok(())
}
