use super::podcast_episode::PodcastEpisode;
use rss::Image;

use crate::style::{
    color::Color,
    stylized::{Style, Stylized},
};

#[derive(Debug)]
pub struct Podcast {
    title: String,
    link: String,
    description: String,
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
}

impl Stylized for Podcast {
    fn to_stylized(&self) -> Vec<(&str, Option<Vec<crate::style::stylized::Style>>)> {
        // TODO : See if there isn't a more fancy way of designing that code (call to "map" ?)
        return vec![
            (
                &self.title,
                Some(vec![
                    Style::Bold,
                    Style::Underlined,
                    Style::Color(Color::Red),
                ]),
            ),
            ("\n", None),
            (
                &self.description,
                Some(vec![Style::Italic, Style::Color(Color::Blue)]),
            ),
        ];
    }
}
