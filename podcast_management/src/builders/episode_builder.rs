use rss;
use crate::data_objects::podcast_episode::PodcastEpisode;

pub struct EpisodeBuilder {   
}

impl EpisodeBuilder {
    pub fn build(&self, item: &rss::Item) -> Result<PodcastEpisode,String> {
        Ok(PodcastEpisode{})
    }
}


#[cfg(test)]
mod test {
    fn build_dummy_category(name: &str, domain: &str) -> rss::Category {
        let category = rss::Category::default();
        category.set_name(name);
        category.set_domain(domain.to_string());
        category
    }

    fn build_dummy_item(title: &str, link: &str, description: &str, author: &str, categories: Vec<rss::Category>, guid: rss::Guid, pub_date: &str, source: rss::Source, content: &str) -> rss::Item {
        let mut item = rss::Item::default();
        item.set_title(title.to_string());
        item.set_link(link.to_string());
        item.set_description(description.to_string());
        item.set_author(author.to_string());
        item.set_categories(categories);
        item.set_guid(guid);
        item.set_pub_date(pub_date.to_string());
        item.set_source(source);
        item.set_content(content.to_string());
        item
    }

    #[test]
    fn test_build_episode() -> Result<(), String> {
        let title: &str = ""; 
        let link: &str = ""; 
        let description: &str = ""; 
        let author: &str = ""; 
        let category_name: &str = "";
        let category_domain: &str = "";
        let categories: Vec<rss::Category> = vec::new(build_dummy_category(category_name, category_domain));
        let guid: rss::Guid = rss::Guid::default();
        let pub_date: &str = ""; 
        let source: rss::Source = rss::Source::default();
        let content: &str = "";
        let item = build_dummy_item(title, link, description, author, categories, guid, pub_date, source, content);
        let episode_builder = super::EpisodeBuilder{};
        let episode = episode_builder.build(&item);
        Ok(())
    } 
}