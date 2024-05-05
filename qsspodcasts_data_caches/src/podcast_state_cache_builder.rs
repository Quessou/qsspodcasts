use std::collections::HashMap;
use std::error::Error;

use log;

use path_providing::path_provider::PathProvider;
use podcast_management::data_objects::podcast_state::PodcastState;

use super::PodcastStateCache;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    ReadingDirectoryFailed,
    ReadingOfFileTypeFailed,
}

#[derive(Debug)]
pub struct CommandError {
    source: Option<Box<dyn Error>>,
    kind: ErrorKind,
}

fn is_hash(hash: &str) -> bool {
    i64::from_str_radix(hash, 16).is_ok()
}

/// # TODO
///   - Add an explicit error type instead of just unit type
pub async fn build_podcast_state_cache<P: PathProvider>(
    path_provider: P,
) -> Result<PodcastStateCache, ()> {
    let finished_podcasts_dir = path_provider.finished_podcasts_dir_path();
    let mut dir = if let Ok(d) = tokio::fs::read_dir(&finished_podcasts_dir).await {
        d
    } else {
        log::error!(
            "Failed to read content of dir {}",
            &finished_podcasts_dir.to_str().unwrap()
        );
        return Err(());
    };
    let mut states: HashMap<String, PodcastState> = HashMap::with_capacity(100);
    while let Some(entry) = dir.next_entry().await.unwrap() {
        let file_type = entry
            .file_type()
            .await
            .expect("Reading of some file type on the filesystem failed");
        if file_type.is_file() {
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap();
            if is_hash(file_name) {
                states.insert(file_name.to_owned(), PodcastState::Finished);
            }
        } else {
            log::info!("Unknown file type in finished podcasts' directory");
        }
    }

    let cache = PodcastStateCache::new(states);
    Ok(cache)
}
