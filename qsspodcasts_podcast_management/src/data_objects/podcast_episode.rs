use chrono::DateTime;
use chrono::Days;
use chrono::FixedOffset;
use html2text;
use rss::Guid;
use sha1::Digest;
use sha1::Sha1;

use super::hashable::Hashable;

#[derive(Debug, Clone, PartialEq)]
pub struct PodcastEpisode {
    pub title: String,
    pub link: String,
    pub description: String,
    pub author: String,
    pub categories: Vec<rss::Category>,
    pub guid: rss::Guid,
    pub pub_date: DateTime<FixedOffset>,
    pub source: rss::Source,
    pub content: String,
    pub url: String,
    pub download_path: Option<String>,
    pub podcast_name: String,
}

impl PodcastEpisode {
    #[allow(clippy::too_many_arguments)]
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
        let description = html2text::from_read(description.as_bytes(), usize::MAX);
        PodcastEpisode {
            title: title.to_string(),
            link: link.to_string(),
            description,
            author: author.to_string(),
            categories: categories.to_vec(),
            guid: rss::Guid {
                permalink: guid.permalink,
                value: guid.value.to_string(),
            },
            pub_date: DateTime::parse_from_rfc2822(pub_date).unwrap(),
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
            podcast_name: "".to_string(),
        }
    }

    pub fn from_item(item: &rss::Item) -> Option<PodcastEpisode> {
        if item.title.is_none()
            || item.link.is_none()
            || item.description.is_none()
            || item.enclosure.is_none()
        {
            return None;
        }
        Some(PodcastEpisode::new(
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
        ))
    }

    pub fn set_podcast_name(&mut self, name: &str) {
        self.podcast_name = name.to_string();
    }

    pub fn get_file_name(&self) -> String {
        let mut file_name = self.podcast_name.clone();
        file_name.push('_');
        file_name.push_str(&self.hash());
        file_name
    }

    pub fn was_published_recently(&self) -> bool {
        let todays_date = chrono::Local::now().date_naive();
        let yesterdays_date = chrono::Local::now()
            //.naive_local()
            .date_naive()
            .checked_sub_days(Days::new(1))
            .expect("Yesterday is an invalid date for some reason ??");
        let relevant_dates = [todays_date, yesterdays_date];
        relevant_dates.contains(&self.pub_date.date_naive())
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

impl Hashable for PodcastEpisode {
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

    use test_case::test_case;

    pub fn build_episode_for_date_tests(date: chrono::DateTime<FixedOffset>) -> PodcastEpisode {
        PodcastEpisode::new(
            "",
            "",
            "",
            "",
            &vec![],
            &rss::Guid::default(),
            &date.to_rfc2822(),
            &rss::Source::default(),
            "",
            "",
            &None,
        )
    }

    #[test_case(chrono::Local::now() => true; "Ok if the podcast was published today")]
    #[test_case(chrono::Local::now().checked_sub_days(Days::new(1)).unwrap() => true; "Ok if the podcast was published yesterday")]
    #[test_case(chrono::Local::now().checked_sub_days(Days::new(7)).unwrap() => false; "NOK if the podcast was published one week ago")]
    pub fn test_match_episode_publication_date(date: DateTime<chrono::Local>) -> bool {
        let date = date.with_timezone(date.offset());
        let episode = build_episode_for_date_tests(date);
        episode.was_published_recently()
    }
}
