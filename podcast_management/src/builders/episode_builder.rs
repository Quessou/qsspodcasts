use crate::data_objects::podcast_episode::PodcastEpisode;
use rss;

use chrono;

pub struct EpisodeBuilder {}

impl EpisodeBuilder {
    pub fn build(&self, item: &rss::Item) -> Result<PodcastEpisode, String> {
        Ok(PodcastEpisode::from_item(item))
    }
}

#[cfg(test)]
mod test {
    use chrono::DateTime;

    fn build_dummy_category(name: &str, domain: &str) -> rss::Category {
        let mut category = rss::Category::default();
        category.set_name(name);
        category.set_domain(domain.to_string());
        category
    }

    fn build_dummy_item(
        title: &str,
        link: &str,
        description: &str,
        author: &str,
        categories: Vec<rss::Category>,
        guid: &rss::Guid,
        pub_date: &str,
        source: rss::Source,
        content: &str,
        enclosure: &rss::Enclosure,
    ) -> rss::Item {
        let mut item = rss::Item::default();
        item.set_title(title.to_string());
        item.set_link(link.to_string());
        item.set_description(description.to_string());
        item.set_author(author.to_string());
        item.set_categories(categories);
        item.set_guid(guid.clone());
        item.set_pub_date(pub_date.to_string());
        item.set_source(source);
        item.set_content(content.to_string());
        item.set_enclosure(enclosure.clone());
        item
    }
    /*
    #[test]
    fn test_build_episode() -> Result<(), String> {
        let title: &str = "";
        let link: &str = "";
        let description: &str = "";
        let author: &str = "";
        let category_name: &str = "";
        let category_domain: &str = "";
        let categories: Vec<rss::Category> =
            vec![build_dummy_category(category_name, category_domain)];
        let guid: rss::Guid = rss::Guid::default();
        let date = chrono::Utc::now();
        let pub_date: chrono::DateTime<chrono::FixedOffset> =
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc2822(&date.to_rfc2822())
                .unwrap();
        let mut source: rss::Source = rss::Source::default();
        source.set_title(Some("title".to_string()));
        source.set_url("https://www.google.com");
        let content: &str = "";
        let enclosure = rss::Enclosure::default();
        let item = build_dummy_item(
            title,
            link,
            description,
            author,
            categories,
            &guid,
            pub_date,
            source,
            content,
            &enclosure,
        );
        let episode_builder = super::EpisodeBuilder {};
        let episode = episode_builder.build(&item).unwrap();

        assert_eq!(episode.title, title);
        assert_eq!(episode.link, link);
        assert_eq!(episode.description, description);
        assert_eq!(episode.author, author);
        assert_eq!(episode.categories[0].name, category_name);
        assert_eq!(
            episode.categories[0].domain.as_ref().unwrap(),
            category_domain
        );
        assert_eq!(&episode.guid, &guid);
        assert_eq!(episode.pub_date, pub_date);

        Ok(())
    }
     */
}
