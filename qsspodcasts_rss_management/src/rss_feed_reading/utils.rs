use crate::channel_tuple::ChannelTuple;
use log::info;
use rss::Channel;
use std::error::Error;

pub async fn get_feed(url: &str) -> Option<ChannelTuple> {
    match get_feed_inner(url).await {
        Ok(t) => Some(t),
        Err(e) => {
            log::error!("Could not load rss feed : {}", e);
            None
        }
    }
}
async fn get_feed_inner(url: &str) -> Result<ChannelTuple, Box<dyn Error>> {
    info!("Downloading feed on URL {}", url);
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    info!("Building of channel for URL {} done", url);
    Ok((url, channel))
}

#[cfg(test)]
mod tests {
    // Allows to test async functions
    use tokio_test;
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    use super::*;

    #[test]
    fn test_get_channel() -> Result<(), String> {
        // TODO : When I'll be motivated, prefer launching a webserver locally in order to make these tests independant from any online third-party
        let url: &str = "https://www.lemonde.fr/rss/une.xml";
        let channel = aw!(get_feed_inner(url));
        if let Err(_) = channel {
            return Err(String::from("Test failed"));
        }
        let channel: ChannelTuple = channel.unwrap();
        assert_eq!(channel.1.link(), url);
        Ok(())
    }
}
