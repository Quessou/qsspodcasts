pub use super::url_storer::UrlStorer;
use std::{io, path::PathBuf};
use fs_utils::{read_utils, write_utils};


pub struct FileUrlStorer {
    file_path: PathBuf
}

impl FileUrlStorer {
    pub fn new(file_path: PathBuf) -> FileUrlStorer {
        FileUrlStorer { file_path : file_path}
    }
}

impl UrlStorer for FileUrlStorer {
    fn write_url(&mut self, url: & str) -> Result<(), io::Error> {
        // TODO
        write_utils::write_at_end_of_file(&self.file_path, url)?;
        Ok(())
    }

    fn get_urls(&mut self) -> Result<Vec<String>, io::Error> {
        read_utils::read_lines(&self.file_path)
    }
}