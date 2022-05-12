use std::io;
pub trait UrlStorer {
    fn write_url(&mut self, url: &str) -> Result<(), io::Error>;
    fn get_urls(&mut self) -> Result<Vec<String>, io::Error>;
}
