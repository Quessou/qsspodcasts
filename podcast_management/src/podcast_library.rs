use crate::data_objects::hashable::Hashable;
use crate::data_objects::podcast::Podcast;
use crate::data_objects::podcast_episode::PodcastEpisode;

pub struct PodcastLibrary {
    pub podcasts: Vec<Podcast>,
}

impl PodcastLibrary {
    pub fn new() -> PodcastLibrary {
        PodcastLibrary { podcasts: vec![] }
    }

    pub fn clear(&mut self) {
        self.podcasts = vec![];
    }

    pub fn push(&mut self, podcasts: impl Into<Vec<Podcast>>) {
        self.podcasts.append(&mut podcasts.into());
    }

    pub fn search_episode(&self, hash: &str) -> Option<PodcastEpisode> {
        for p in &self.podcasts {
            for e in &p.episodes {
                if e.hash() == hash {
                    return Some(e.clone());
                }
            }
        }
        None
    }
}

impl Default for PodcastLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::builders::podcast_builder::PodcastBuilder;

    use super::PodcastLibrary;

    #[test]
    fn test_push() -> Result<(), String> {
        let podcast_builder = PodcastBuilder::new();
        let mut library = PodcastLibrary::new();
        assert_eq!(library.podcasts.len(), 0);
        let mut podcasts = vec![podcast_builder.build(&rss::Channel::default())];
        library.push(podcasts);
        assert_eq!(library.podcasts.len(), 1);
        Ok(())
    }

    #[test]
    fn test_clear() -> Result<(), String> {
        let podcast_builder = PodcastBuilder::new();
        let mut library = PodcastLibrary::new();
        let mut podcasts = vec![podcast_builder.build(&rss::Channel::default())];
        library.push(podcasts);
        assert_eq!(library.podcasts.len(), 1);
        library.clear();
        assert_eq!(library.podcasts.len(), 0);
        Ok(())
    }
}
