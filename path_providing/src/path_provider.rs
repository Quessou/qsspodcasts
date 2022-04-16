pub use std::path::PathBuf;

pub trait PathProvider {
    fn app_dir_path(&self) -> PathBuf;
    fn rss_feed_list_file_path(&self) -> PathBuf;
    fn download_dir_path(&self) -> PathBuf;
    fn rss_feed_list_file_name(&self) -> &'static str;
    fn download_dir_name(&self) -> &'static str;
}