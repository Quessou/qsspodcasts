use rss::Guid;

#[derive(Debug)]
pub struct PodcastEpisode {
    pub title: String,
    pub link: String,
    pub description: String,
    pub author: String,
    pub categories: Vec<rss::Category>,
    pub guid: rss::Guid,
    pub pub_date: String,
    pub source: rss::Source,
    pub content: String,
    pub url: String,
    pub download_path: Option<String>,
}

impl PodcastEpisode {
    pub fn new(
        title: &str,
        link: &str,
        description: &str,
        author: &str,
        categories: &[rss::Category],
        guid: &rss::Guid,
        pub_date: &str,
        source: &rss::Source,
        content: &str,
        url: &str,
        download_path: &Option<String>,
    ) -> PodcastEpisode {
        PodcastEpisode {
            title: title.to_string(),
            link: link.to_string(),
            description: description.to_string(),
            author: author.to_string(),
            categories: categories.to_vec(),
            guid: rss::Guid {
                permalink: guid.permalink,
                value: guid.value.to_string(),
            },
            pub_date: pub_date.to_string(),
            source: rss::Source {
                title: Some(
                    source
                        .title
                        .as_ref()
                        .unwrap_or(&String::from(""))
                        .to_string(),
                ),
                url: source.url.to_string(),
            },
            content: content.to_string(),
            url: url.to_string(),
            download_path: download_path.clone(),
        }
    }

    pub fn from_item(item: &rss::Item) -> PodcastEpisode {
        PodcastEpisode::new(
            item.title.as_ref().unwrap(),
            item.link.as_ref().unwrap(),
            item.description.as_ref().unwrap(),
            item.author.as_ref().unwrap_or(&String::from("Unknown")),
            item.categories.as_ref(),
            item.guid.as_ref().unwrap(),
            item.pub_date.as_ref().unwrap(),
            item.source.as_ref().unwrap_or(&rss::Source::default()),
            item.content.as_ref().unwrap_or(&String::from("")),
            &item.enclosure.as_ref().unwrap().url,
            &None,
        )
    }
}

impl Default for PodcastEpisode {
    fn default() -> PodcastEpisode {
        PodcastEpisode::new(
            "",
            "",
            "",
            "",
            &[],
            &Guid::default(),
            "",
            &rss::Source::default(),
            "",
            "",
            &None,
        )
    }
}
