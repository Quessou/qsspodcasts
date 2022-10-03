use super::podcast_episode::PodcastEpisode;
use rss::Image;

//use crate::style::{
//    color::Color,
//    stylized::{Style, Stylized, StylizedContent},
//};

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
        Podcast {
            title: title.to_string(),
            link: link.to_string(),
            description: description.to_string(),
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
