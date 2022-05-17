use rss::Channel;
use std::error::Error;

pub async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
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
    use rss::Channel;

    use super::get_feed;
    #[test]
    fn test_get_channel() -> Result<(), String> {
        // TODO : When I'll be motivated, prefer launching a webserver locally in order to make these tests independant from any online third-party
        let url: &str = "https://www.lemonde.fr/rss/une.xml";
        let channel = aw!(get_feed(url));
        if let Err(_) = channel {
            return Err(String::from("Test failed"));
        }
        let channel: Channel = channel.unwrap();
        // WTF was that assert_eq ?
        //assert_eq!(channel.items().len(), 0);
        assert_eq!(channel.link(), url);
        Ok(())
    }
}
