use std::io;

use log;

use crate::channel_tuple::ChannelTuple;
use crate::rss_feed_reading::feed_downloader::{self, FeedDownloader};
use crate::url_storage::url_storer::UrlStorer;

pub struct RssProvider<T: UrlStorer> {
    rss_feeds: Vec<String>,
    url_storer: T,
    feed_downloader: feed_downloader::FeedDownloader,
}

impl<T: UrlStorer> RssProvider<T> {
    pub fn new(mut url_storer: T) -> RssProvider<T> {
        RssProvider {
            rss_feeds: url_storer.get_urls().unwrap(),
            url_storer,
            feed_downloader: FeedDownloader {},
        }
    }

    pub fn add_url(&mut self, url: &str) -> Result<(), io::Error> {
        let url_string = String::from(url);
        if !self.rss_feeds.contains(&url_string) {
            self.rss_feeds.push(url_string);
            self.url_storer.write_url(url)?;
        } else {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists));
        }
        Ok(())
    }
    pub fn delete_url(&mut self, url: &str) -> Result<(), io::Error> {
        self.rss_feeds.retain(|u| u != url);
        self.url_storer.delete_url(url)?;
        Ok(())
    }

    pub async fn get_feed<'a>(&'a self, url: &'a str) -> Option<ChannelTuple<'a>> {
        match self.feed_downloader.download_feed(url).await {
            Ok(t) => Some(t),
            Err(e) => {
                log::error!("Could not load rss feed : {}", e);
                None
            }
        }
    }

    pub async fn get_all_feeds(&mut self) -> (Vec<ChannelTuple>, Vec<String>) {
        // Note : This is bad AF
        let rss_feeds = &self.rss_feeds;

        let mut feeds: Vec<ChannelTuple> = vec![];
        let mut faulty_feeds = vec![];
        for f in rss_feeds {
            let feed = self.get_feed(f).await;
            if feed.is_some() {
                feeds.push(feed.unwrap());
            } else {
                faulty_feeds.push(f.clone());
            }
        }
        (feeds, faulty_feeds)
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    struct DummyUrlStorer;
    impl UrlStorer for DummyUrlStorer {
        fn write_url(&mut self, _url: &str) -> Result<(), io::Error> {
            Ok(())
        }
        fn delete_url(&mut self, _url: &str) -> Result<(), io::Error> {
            Ok(())
        }

        fn get_urls(&mut self) -> Result<Vec<String>, io::Error> {
            Ok(Vec::new())
        }
    }

    use crate::url_storage::file_url_storer::UrlStorer;

    use super::RssProvider;
    #[test]
    fn test_add_url() -> Result<(), String> {
        let mut rss_provider = RssProvider::new(DummyUrlStorer {});
        if let Err(e) = rss_provider.add_url("https://www.toto.com") {
            return Err(e.to_string());
        }
        assert_eq!(rss_provider.rss_feeds.len(), 1);
        Ok(())
    }
}
