use std::{collections::HashMap, path::PathBuf};

use tokio::fs::DirEntry;

use crate::caches::PodcastStateCache;

/// # TODO
///   - Add an explicit error type instead of just unit type
pub async fn build_podcast_state_cache<P: PathProvider>(path: P) -> Result<PodcastStateCache, ()> {
    let dir = tokio::fs::read_dir(path)
        .await
        .expect("Opening of directory containing list of finished podcasts failed");
    let cache = PodcastStateCache::new(HashMap::default());
    Err(())
}
