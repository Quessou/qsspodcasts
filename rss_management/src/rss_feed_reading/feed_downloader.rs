use std::error::Error;

use crate::channel_tuple::ChannelTuple;

use super::utils::get_feed;

pub struct FeedDownloader {}

impl FeedDownloader {
    pub async fn download_feed<'a>(&'a self, url: &'a str) -> Result<ChannelTuple, Box<dyn Error>> {
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
