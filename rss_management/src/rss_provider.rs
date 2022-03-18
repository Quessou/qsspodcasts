use std::{path::{Path, PathBuf}, io, fs::OpenOptions};
use fs_utils::{read_utils, write_utils};

pub struct RssProvider {
    rss_feeds: Vec<String>,
    rss_list_file_path: Option<PathBuf>
}

impl RssProvider {
    pub fn new(rss_list_file_path: Option<&str>) -> RssProvider {
        match rss_list_file_path {
            Option::None => RssProvider { rss_feeds: Vec::new(), rss_list_file_path: Option::None },
            Option::Some(path) => {
                let rss_feeds = read_utils::read_lines(Path::new(path)).expect("Reading of configuration file failed");
                RssProvider { rss_feeds : rss_feeds, rss_list_file_path: Option::Some(PathBuf::from(path)) }
            },
        }
    }

    pub fn add_url(&mut self, url: &str) -> Result<(), io::Error> {
        self.rss_feeds.push(String::from(url));
        match &self.rss_list_file_path {
            Option::None => Ok(()),
            Option::Some(path) => {
                write_utils::write_at_end_of_file(path, url);
                Ok(())
            }
        }
    }
}