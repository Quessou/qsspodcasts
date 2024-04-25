use crate::data_objects::podcast_state::PodcastState;
use std::collections::HashMap;

type Hash = String;

pub struct PodcastStateCache {
    states: HashMap<Hash, PodcastState>,
}

impl PodcastStateCache {
    pub fn new(podcast_states: HashMap<Hash, PodcastState>) -> Self {
        Self {
            states: podcast_states,
        }
    }

    pub fn get_podcast_state(&self, hash: &Hash) -> Option<&PodcastState> {
        self.states.get(hash)
    }
}
