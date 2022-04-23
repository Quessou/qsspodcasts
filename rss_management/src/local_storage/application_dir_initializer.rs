use std::result::Result;
use std::path::PathBuf;
use std::io::{ErrorKind};
use log::{warn, error};
use fs_utils::permissions::are_permissions_fulfilled;
//use home::home_dir;
use std::path::Path;

use path_providing::path_provider::PathProvider;

pub struct ApplicationDirInitializer {
    pub path_provider: Box<dyn PathProvider>
}


impl ApplicationDirInitializer {

    pub fn is_app_dir_created(&self, path: PathBuf) -> bool {
        // TODO : What to do with this ?
        Path::new(&path).exists() && Path::new(&(path.to_str().unwrap().to_owned() + "/" + self.path_provider.rss_feed_list_file_name())).exists() &&
        Path::new(&(path.to_str().unwrap().to_owned() + "/" + self.path_provider.download_dir_name())).exists()
    }

    pub fn new(path_provider: Box<dyn PathProvider>) -> ApplicationDirInitializer {
        ApplicationDirInitializer{path_provider}
    }


    pub fn initialize_application_dir(&self, app_dir_path: &str) -> Result<(), std::io::Error> {
        use std::fs;

        assert!(! PathBuf::from(app_dir_path).is_dir());

        let app_dir_path = PathBuf::from(app_dir_path);
        // Create app configuration dir
        ApplicationDirInitializer::is_path_valid(&app_dir_path)?;
        fs::create_dir_all(&app_dir_path.to_path_buf())?;

        let rss_feed_list_file_path: PathBuf = app_dir_path.join(self.path_provider.rss_feed_list_file_name());
        fs::File::create(rss_feed_list_file_path)?;
        let download_dir_path: PathBuf = app_dir_path.join(self.path_provider.download_dir_name());
        fs::create_dir_all(download_dir_path)?;

        Ok(())
    }


    fn is_path_valid(path : &PathBuf) -> Result<(), std::io::Error> {
        if path.exists() && path.is_file() {
            error!("Path points to an already existing file, and not a directory : {}", path.to_str().unwrap());
            return Err(std::io::Error::new(ErrorKind::AlreadyExists, "Path points out to an already existing file"));
        }

        if path.is_relative() {
            warn!("Path is relative, which may lead to unexpected behavior : {}", path.to_str().unwrap());
            return Err(std::io::Error::new(ErrorKind::InvalidInput, "Path must be absolute"));
        }

        if path.is_dir() {
            if ! are_permissions_fulfilled(path, 0o700).unwrap() {
                error!("Bad permissions, user must have rwx rights on dir {}", path.to_str().unwrap());
                return Err(std::io::Error::new(ErrorKind::PermissionDenied, "Bad permissions"));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;


    #[test]
    fn test_path_integrity_file_already_exists() -> Result<(), String> {
        use super::ApplicationDirInitializer;
        use std::path::PathBuf;
        use std::io::ErrorKind;
        use std::fs;

        let file_path_str : &str = "/tmp/test_already_exists";
        let file_path : PathBuf = PathBuf::from(&file_path_str);
        let _file = fs::File::create(&file_path).unwrap();

        assert_eq!(ApplicationDirInitializer::is_path_valid(&file_path).unwrap_err().kind(), ErrorKind::AlreadyExists);

        let not_existing_file_path_str : &str = "/tmp/test_does_not_exist";
        let not_existing_file_path : PathBuf = PathBuf::from(&not_existing_file_path_str);

        assert!(ApplicationDirInitializer::is_path_valid(&not_existing_file_path).is_ok());

        // Cleanup
        fs::remove_file(file_path).expect("Cleanup of test failed");

        Ok(())
    }

    #[test]
    fn test_path_integrity_relative_path() -> Result<(), String> {
        use super::ApplicationDirInitializer;
        use std::path::PathBuf;
        use std::io::ErrorKind;
        use std::env;

        // Init
        let cwd = env::current_dir().unwrap();
        env::set_current_dir("/tmp").expect("Initilization of test failed");


        let relative_file_path_str : &str = "./test_relative_path";
        let relative_file_path : PathBuf = PathBuf::from(&relative_file_path_str);

        assert_eq!(ApplicationDirInitializer::is_path_valid(&relative_file_path).unwrap_err().kind(), ErrorKind::InvalidInput);

        let absolute_file_path_str : &str = "/tmp/test_absolute_path";
        let absolute_file_path : PathBuf = PathBuf::from(&absolute_file_path_str);

        assert!(ApplicationDirInitializer::is_path_valid(&absolute_file_path).is_ok());

        // Cleanup
        env::set_current_dir(&cwd).expect("Cleanup of test failed");

        Ok(())
    }

    #[test]
    fn test_path_integrity_permissions() -> Result<(), String> {
        use super::ApplicationDirInitializer;
        use std::path::PathBuf;
        use std::io::ErrorKind;
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let test_dir_path_str = "/tmp/test_permissions_dir";
        let test_dir_path = PathBuf::from(&test_dir_path_str);

        fs::create_dir(&test_dir_path).expect("Initilization of test failed");
        let mut permissions = fs::metadata(&test_dir_path).unwrap().permissions();
        permissions.set_mode(0o600);
        // Here we just check that the set_mode call worked.
        assert_eq!(permissions.mode() & 0xFFF, 0o600);

        fs::set_permissions(&test_dir_path, permissions).expect("Set of permissions failed");
        assert_eq!(test_dir_path.metadata().unwrap().permissions().mode() & 0xFFF, 0o600);
        assert_eq!(ApplicationDirInitializer::is_path_valid(&test_dir_path).unwrap_err().kind(), ErrorKind::PermissionDenied);

        fs::set_permissions(&test_dir_path, fs::Permissions::from_mode(0o700)).expect("Initialization of test failed");
        assert!(ApplicationDirInitializer::is_path_valid(&test_dir_path).is_ok());

        // Cleanup
        fs::remove_dir(test_dir_path).expect("Cleanup of test failed");

        Ok(())
    }

    #[test]
    fn test_initialize_application_dir() -> Result<(), String> {
        use super::ApplicationDirInitializer;
        use std::path::PathBuf;
        use path_providing::dummy_path_provider::DummyPathProvider;

        let dummy_app_dir = "/tmp/.qsspodcasts";
        let app_dir_initializer = ApplicationDirInitializer{path_provider: Box::new(DummyPathProvider::new(dummy_app_dir))};
        assert!(! PathBuf::from(dummy_app_dir).is_dir());
        app_dir_initializer.initialize_application_dir(&dummy_app_dir).expect("Initialization application dir failed");
        assert!(PathBuf::from(dummy_app_dir).is_dir());

        fs::remove_dir_all(PathBuf::from(dummy_app_dir).to_path_buf()).expect("Cleanup of test failed");

        Ok(())
    }
}