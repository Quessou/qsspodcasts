use super::podcast_episode::PodcastEpisode;
use rss::Image;

#[derive(Debug)]
pub struct Podcast {
    _title: String,
    _link: String,
    _description: String,
    _copyright: Option<String>,
    _pub_date: Option<String>,
    _image: Option<Image>,
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
            _title: title.to_string(),
            _link: link.to_string(),
            _description: description.to_string(),
            _copyright: copyright,
            _pub_date: pub_date,
            _image: image,
            episodes: episodes,
        }
    }
}
