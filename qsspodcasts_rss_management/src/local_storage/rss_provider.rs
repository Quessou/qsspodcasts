use std::io;

use futures::future::try_join_all;
use log;
use tokio::sync::{Mutex, RwLock};

use crate::channel_tuple::ChannelTuple;
use crate::rss_feed_reading::feed_downloader::{self, FeedDownloader};
use crate::rss_feed_reading::utils::get_feed;
use crate::url_storage::url_storer::UrlStorer;

pub struct RssProvider<T: UrlStorer> {
    rss_feeds: tokio::sync::RwLock<Vec<String>>,
    url_storer: T,
    feed_downloader: feed_downloader::FeedDownloader,
}

impl<T: UrlStorer> RssProvider<T> {
    pub fn new(mut url_storer: T) -> RssProvider<T> {
        RssProvider {
            rss_feeds: RwLock::new(url_storer.get_urls().unwrap()),
            url_storer,
            feed_downloader: FeedDownloader {},
        }
    }

    pub async fn add_url(&mut self, url: &str) -> Result<(), io::Error> {
        let url_string = String::from(url);
        if !self.rss_feeds.read().await.contains(&url_string) {
            self.rss_feeds.get_mut().push(url_string);
            self.url_storer.write_url(url)?;
        } else {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists));
        }
        Ok(())
    }
    pub fn delete_url(&mut self, url: &str) -> Result<(), io::Error> {
        self.rss_feeds.get_mut().retain(|u| u != url);
        self.url_storer.delete_url(url)?;
        Ok(())
    }

    /*
    pub async fn get_feed<'a>(&'a self, url: &'a str) -> Option<ChannelTuple<'a>> {
        /*
        match get_feed(url).await {
            Ok(t) => Some(t),
            Err(e) => {
                log::error!("Could not load rss feed : {}", e);
                None
            }
        }
        */
    }*/

    pub async fn get_all_feeds(&mut self) -> (Vec<ChannelTuple>, Vec<String>) {
        // Note : This is bad AF
        let rss_feeds = &mut self.rss_feeds;
        let locked_rss_feeds = rss_feeds.get_mut();

        let mut feeds: tokio::sync::Mutex<Vec<ChannelTuple>> = tokio::sync::Mutex::new(vec![]);
        let mut faulty_feeds: tokio::sync::Mutex<Vec<String>> = tokio::sync::Mutex::new(vec![]);

        let mut get_feed_futures = vec![];

        for f in locked_rss_feeds {
            get_feed_futures.push(async {
                let feed = get_feed(f).await;
                if let Some(f) = feed {
                    feeds.lock().await.push(f);
                } else {
                    faulty_feeds.lock().await.push(f.clone());
                }
                /* Lol ??? */
                if false {
                    return Err(());
                }
                Ok(())
            });
        }
        let handle = tokio::runtime::Handle::current();
        try_join_all(get_feed_futures).await;

        let feeds: Vec<ChannelTuple> = feeds.into_inner();
        let faulty_feeds: Vec<String> = faulty_feeds.into_inner();
        (feeds, faulty_feeds)
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use tokio::test;

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
    async fn test_add_url() -> Result<(), String> {
        let mut rss_provider = RssProvider::new(DummyUrlStorer {});
        if let Err(e) = rss_provider.add_url("https://www.toto.com").await {
            return Err(e.to_string());
        }
        assert_eq!(rss_provider.rss_feeds.read().await.len(), 1);
        Ok(())
    }
}

unsafe impl<T: UrlStorer> Send for RssProvider<T> {}
