use std::{path::PathBuf, time::Duration};

use tokio::io::{AsyncReadExt, BufReader};

pub async fn read_progression_in_file(path: PathBuf) -> Option<std::time::Duration> {
    if !tokio::fs::try_exists(&path).await.expect(
        "Interacting with the filesystem while trying to retrieve progression of podcast failed",
    ) {
        return None;
    }
    let file = tokio::fs::File::open(path)
        .await
        .expect("Opening of file containing progression failed");
    let mut file_reader = BufReader::new(file);
    let duration_s = file_reader.read_u64().await.unwrap();
    Some(Duration::from_secs(duration_s))
}
