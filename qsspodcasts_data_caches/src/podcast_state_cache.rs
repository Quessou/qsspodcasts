use podcast_management::data_objects::podcast_state::PodcastState;
use std::collections::HashMap;

type Hash = String;

#[derive(Default)]
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

    pub fn set_podcast_state(&mut self, hash: &Hash, state: &PodcastState) {
        self.states.insert(hash.clone(), *state);
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_state_change() {
        let states: HashMap<Hash, PodcastState> = HashMap::with_capacity(10);
        let mut cache = PodcastStateCache::new(states);
        let hash = "111111".to_owned();

        cache.set_podcast_state(&hash, &PodcastState::Finished);

        assert_eq!(
            cache.get_podcast_state(&hash).unwrap(),
            &PodcastState::Finished
        )
    }
}
