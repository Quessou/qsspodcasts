use std::io;

use rss::Channel;

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
            self.url_storer.write_url(&url)?;
        }
        Ok(())
    }

    pub async fn get_feed<'a>(&'a self, url: &'a str) -> ChannelTuple<'a> {
        self.feed_downloader.download_feed(url).await.unwrap()
    }

    pub async fn get_all_feeds(&mut self) -> Vec<ChannelTuple> {
        // Note : This is bad AF
        let rss_feeds = &self.rss_feeds;

        let mut feeds: Vec<ChannelTuple> = vec![];
        for f in rss_feeds {
            feeds.push(self.get_feed(&f).await);
        }
        //self.rss_feeds.iter().fold(vec![], async |accum, f| {
        //    accum.push(self.get_feed(f).await);
        //    return accum;
        //    //for feed_url in rss_feeds {
        //    //    feeds.push(self.get_feed(&feed_url).await)
        //    //}
        //});
        feeds
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
