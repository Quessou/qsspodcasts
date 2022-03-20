use std::io;
use crate::url_storer::url_storer::UrlStorer;

pub struct RssProvider<T: UrlStorer> {
    rss_feeds: Vec<String>,
    url_storer: T
}

impl<T: UrlStorer> RssProvider<T> {
    pub fn new(mut url_storer : T) -> RssProvider<T> {
        RssProvider { rss_feeds: url_storer.get_urls().unwrap(), url_storer: url_storer }
    }

    pub fn add_url(&mut self, url: &str) -> Result<(), io::Error> {
        // TODO : Add check on the fact that the str given in parameter is actually a URL
        self.rss_feeds.push(String::from(url));
        self.url_storer.write_url(url)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;

    struct DummyUrlStorer;
    impl UrlStorer for DummyUrlStorer {
        fn write_url(&mut self, url: & str) -> Result<(), io::Error> {
            Ok(())
        }
    
        fn get_urls(&mut self) -> Result<Vec<String>, io::Error> {
            Ok(Vec::new())
        }
    }

    use crate::url_storer::file_url_storer::UrlStorer;

    use super::RssProvider;
    #[test]
    fn test_add_url() -> Result<(), String> {
        // TODO : FIXME
        let mut rss_provider = RssProvider::new(DummyUrlStorer{});
        rss_provider.add_url("https://www.toto.com");
        assert_eq!(rss_provider.rss_feeds.len(), 1);
        Ok(())
    }
}