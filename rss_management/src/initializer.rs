use std::result::Result;
use std::path::PathBuf;
use std::io::{ErrorKind};
use log::{info, warn, error, debug};

pub struct Initializer {

    feed_dir_path: PathBuf,
    download_dir_path : PathBuf
}


impl Initializer {
    pub fn new() -> Initializer {
        Initializer{
            feed_dir_path : PathBuf::from("/tmp"),
            download_dir_path : PathBuf::from("/tmp")
        }
    }

    pub fn new_from_str(feed_dir_path: &str, download_dir_path: &str) -> Initializer {
        Initializer {
            feed_dir_path : PathBuf::from(feed_dir_path),
            download_dir_path : PathBuf::from(download_dir_path)
        }
    }


    pub fn initialize(&self) -> Result<(), std::io::Error> {
        use std::fs;

        // Handle feed dir path
        Initializer::check_path_integrity(&self.feed_dir_path)?;
        // Honestly not sure to understand why we have to call `to_path_buf` here, whereas we already have a PathBuf object
        fs::create_dir_all(self.feed_dir_path.to_path_buf())?;

        // Handle download dir path
        Initializer::check_path_integrity(&self.download_dir_path)?;
        fs::create_dir_all(self.download_dir_path.to_path_buf())?;

        Ok(())
    }


    fn check_path_integrity(path : &PathBuf) -> Result<(), std::io::Error> {
        if path.exists() && path.is_file() {
            error!("Path points to an already existing file, and not a directory : {}", path.to_str().unwrap());
            return Err(std::io::Error::new(ErrorKind::AlreadyExists, "Path points out to an already existing file"));
        }

        if path.is_relative() {
            warn!("Path is relative, which may lead to unexpected behavior : {}", path.to_str().unwrap());
            return Err(std::io::Error::new(ErrorKind::InvalidInput, "Path must be absolute"));
        }

        if path.is_dir() {
            use std::fs::metadata;
            use std::os::unix::fs::PermissionsExt;

            let dir_permissions = metadata(path.to_str().unwrap())?;
            if (dir_permissions.permissions().mode() & 0xFFF) < 0o700 {
                error!("Bad permissions, user must have rwx rights on dir {}", path.to_str().unwrap());
                return Err(std::io::Error::new(ErrorKind::PermissionDenied, "Bad permissions"));

            }

        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_path_integrity_file_already_exists() -> Result<(), String> {
        use super::Initializer;
        use std::path::PathBuf;
        use std::io::ErrorKind;
        use std::fs;

        let file_path_str : &str = "/tmp/test_already_exists";
        let file_path : PathBuf = PathBuf::from(&file_path_str);
        let _file = fs::File::create(&file_path).unwrap();

        assert_eq!(Initializer::check_path_integrity(&file_path).unwrap_err().kind(), ErrorKind::AlreadyExists);

        let not_existing_file_path_str : &str = "/tmp/test_already_exists_2";
        let not_existing_file_path : PathBuf = PathBuf::from(&not_existing_file_path_str);

        assert!(Initializer::check_path_integrity(&not_existing_file_path).is_ok());

        // Cleanup
        fs::remove_file(file_path).expect("Cleanup of test failed");

        Ok(())
    }

    #[test]
    fn test_path_integrity_relative_path() -> Result<(), String> {
        use super::Initializer;
        use std::path::PathBuf;
        use std::io::ErrorKind;
        use std::env;

        // Init
        let cwd = env::current_dir().unwrap();
        env::set_current_dir("/tmp").expect("Initilization of test failed");


        let relative_file_path_str : &str = "./test_relative_path";
        let relative_file_path : PathBuf = PathBuf::from(&relative_file_path_str);

        assert_eq!(Initializer::check_path_integrity(&relative_file_path).unwrap_err().kind(), ErrorKind::InvalidInput);

        let absolute_file_path_str : &str = "/tmp/test_absolute_path";
        let absolute_file_path : PathBuf = PathBuf::from(&absolute_file_path_str);

        assert!(Initializer::check_path_integrity(&absolute_file_path).is_ok());

        // Cleanup
        env::set_current_dir(&cwd).expect("Cleanup of test failed");

        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_path_integrity_permissions() -> Result<(), String> { // TODO: fix this test
        use super::Initializer;
        use std::path::PathBuf;
        use std::io::ErrorKind;
        use std::fs;
        use std::env;
        use std::fs::Permissions;
        use std::os::unix::fs::PermissionsExt;

        let test_dir_path_str = "/tmp/test_permissions_dir";
        let test_dir_path = PathBuf::from(&test_dir_path_str);

        fs::create_dir(&test_dir_path).expect("Initilization of test failed");
        // println!("Permission : {:#?}", &test_dir_path.metadata().unwrap().permissions());
        let mut permissions = fs::metadata(&test_dir_path).unwrap().permissions();
        permissions.set_mode(0o600);

        assert_eq!(permissions.mode() & 0xFFF, 0o600);

        fs::set_permissions(&test_dir_path, permissions).expect("Initialization of test failed");
        assert_eq!(test_dir_path.metadata().unwrap().permissions().mode() & 0xFFF, 0o600);

        // println!("{:#?}", &test_dir_path.metadata().unwrap().permissions());

        assert_eq!(Initializer::check_path_integrity(&test_dir_path).unwrap_err().kind(), ErrorKind::PermissionDenied);

        fs::set_permissions(&test_dir_path, fs::Permissions::from_mode(0o700)).expect("Initialization of test failed");
        assert!(Initializer::check_path_integrity(&test_dir_path).is_ok());

        // Cleanup
        fs::remove_dir(test_dir_path).expect("Cleanup of test failed");

        Ok(())
    }
}