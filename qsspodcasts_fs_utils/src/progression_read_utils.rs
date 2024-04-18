use std::{path::PathBuf, time::Duration};

use tokio::io::{AsyncReadExt, BufReader};

pub async fn read_progression_in_file(path: PathBuf) -> Option<std::time::Duration> {
    let file = tokio::fs::File::open(path)
        .await
        .expect("Opening of file containing progression failed");
    let mut file_reader = BufReader::new(file);
    let duration_s = file_reader.read_u64().await.unwrap();
    Some(Duration::from_secs(duration_s))
}
