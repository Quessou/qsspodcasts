use std::io;

use rss::Channel;

use crate::url_storer::url_storer::UrlStorer;
use crate::rss_feed_reading::feed_downloader::{self, FeedDownloader};

pub struct RssProvider<T: UrlStorer> {
    rss_feeds: Vec<String>,
    url_storer: T,
    feed_downloader: feed_downloader::FeedDownloader
}

impl<T: UrlStorer> RssProvider<T> {
    pub fn new(mut url_storer : T) -> RssProvider<T> {
        RssProvider { rss_feeds: url_storer.get_urls().unwrap(), url_storer: url_storer, feed_downloader: FeedDownloader{} }

    }

    pub fn add_url(&mut self, url: &str) -> Result<(), io::Error> {
        // TODO : Add check on the fact that the str given in parameter is actually a URL
        self.rss_feeds.push(String::from(url));
        self.url_storer.write_url(url)?;
        Ok(())
    }

    pub async fn get_feed(&mut self, url: &str) -> Channel {
        let hu = self.feed_downloader.download_feed(url);
        self.feed_downloader.download_feed(url).await.unwrap()
    }

    pub async fn get_all_feeds(&mut self) -> Vec<Channel> {
        use tokio::join;
        use std::future::Future;
        let toto :Vec<dyn Future<Output=Channel>> = self.rss_feeds.iter().map(|f| self.get_feed(f)).collect();
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use std::io;

    struct DummyUrlStorer;
    impl UrlStorer for DummyUrlStorer {
        fn write_url(&mut self, _url: & str) -> Result<(), io::Error> {
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
        let mut rss_provider = RssProvider::new(DummyUrlStorer{});
        if let Err(e) = rss_provider.add_url("https://www.toto.com") {
            return Err(e.kind().to_string());
        }
        assert_eq!(rss_provider.rss_feeds.len(), 1);
        Ok(())
    }
}