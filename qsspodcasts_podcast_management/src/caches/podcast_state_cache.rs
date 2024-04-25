use crate::data_objects::podcast_state::PodcastState;
use std::{collections::HashMap, path::PathBuf};

type Hash = String;

struct PodcastStateCache {
    states: HashMap<Hash, PodcastState>,
}

impl PodcastStateCache {
    pub fn new(app_dir_path: PathBuf) {}

    pub fn get_podcast_state(&self, hash: &Hash) -> Option<&PodcastState> {
        self.states.get(hash)
    }
}
