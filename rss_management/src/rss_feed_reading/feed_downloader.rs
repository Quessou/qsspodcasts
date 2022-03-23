use rss::Channel;
use std::error::Error;

use super::utils::get_feed;

pub struct FeedDownloader {
}

impl FeedDownloader {
    pub async fn download_feed(&self, url: &str) -> Result<Channel, Box<dyn Error>> {
        get_feed(url).await
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn test_get_channel() -> Result<(), String> {
        // TODO
        Ok(())
    }
}