use rss::Image;
use super::podcast_episode::PodcastEpisode;

#[derive(Debug)]
pub struct Podcast {
    title: String,
    link: String,
    description: String,
    copyright: Option<String>,
    pub_date: Option<String>,
    image: Option<Image>,
    episodes: Vec<PodcastEpisode>,
}

impl Podcast {
    pub fn new(title : &str, link : &str, description: & str, copyright: Option<String>, pub_date: Option<String>, image: Option<Image>, episodes: Vec<PodcastEpisode>) -> Podcast {
        Podcast { title : title.to_string(), link: link.to_string(), description: description.to_string(), copyright: copyright, pub_date: pub_date, image: image, episodes : episodes }
    }
}