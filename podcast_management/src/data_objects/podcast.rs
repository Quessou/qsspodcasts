use super::hashable::Hashable;
use super::podcast_episode::PodcastEpisode;
use hex;
use html2text;
use rss::Image;
use sha1::{Digest, Sha1};

#[derive(Debug, Clone)]
pub struct Podcast {
    pub title: String,
    pub link: String,
    pub description: String,
    copyright: Option<String>,
    pub_date: Option<String>,
    image: Option<Image>,
    pub episodes: Vec<PodcastEpisode>,
}

impl Podcast {
    pub fn new(
        title: &str,
        link: &str,
        description: &str,
        copyright: Option<String>,
        pub_date: Option<String>,
        image: Option<Image>,
        episodes: Vec<PodcastEpisode>,
    ) -> Podcast {
        let description = html2text::from_read(description.as_bytes(), usize::max_value());
        Podcast {
            title: title.to_string(),
            link: link.to_string(),
            description,
            copyright,
            pub_date,
            image,
            episodes,
        }
    }

    pub fn shallow_copy(&self) -> Podcast {
        Podcast::new(
            &self.title,
            &self.link,
            &self.description,
            self.copyright.clone(),
            self.pub_date.clone(),
            self.image.clone(),
            vec![],
        )
    }
}

impl Hashable for Podcast {
    fn hash(&self) -> String {
        let mut hasher = Sha1::new();
        hasher.update(self.title.as_bytes());
        hasher.update(self.description.as_bytes());
        let d: [u8; 3] = TryFrom::try_from(&hasher.finalize()[17..]).unwrap();
        let hash: String = hex::encode(d);
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_dummy_podcast(title: &str, link: &str, description: &str) -> Podcast {
        Podcast::new(title, link, description, None, None, None, vec![])
    }

    #[test]
    fn test_hash_equals() {
        let p1 = get_dummy_podcast("title", "https://www.google.com", "description");
        let p2 = p1.clone();
        assert_eq!(p1.hash(), p2.hash())
    }

    #[test]
    fn test_hash_not_equals() {
        let p1 = get_dummy_podcast("title", "https://www.google.com", "description");
        let p2 = get_dummy_podcast("title2", "https://www.google.com", "description");
        assert_ne!(p1.hash(), p2.hash())
    }
}

impl From<Podcast> for Vec<Podcast> {
    fn from(p: Podcast) -> Self {
        vec![p]
    }
}
