use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::result::Result;

const PERMISSIONS_MASK: u32 = 0xFFF;

/// Checks permission of file whose path is given in parameter
///
/// # Arguments
///
/// * `path` - A PathBuf containing a path to the file whose permissions has to be checked
/// * `permissions` - Permissions we need on the file
pub fn are_permissions_fulfilled(path: &PathBuf, permissions: u32) -> Result<bool, std::io::Error> {
    let tested_metadata = fs::metadata(path.to_str().unwrap())?;
    Ok((tested_metadata.permissions().mode() & PERMISSIONS_MASK) >= permissions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::warn;
    use std::fs;
    use test_case::test_case;

    struct FilePermissionsTestData {
        file_path: PathBuf,
    }

    impl FilePermissionsTestData {
        pub fn new(file_path: &str, permission_to_set: u32) -> FilePermissionsTestData {
            let file_path = PathBuf::from(file_path);

            let f = fs::File::create(&file_path).expect("File creation for test failed");

            let mut permissions = fs::metadata(&file_path).unwrap().permissions();
            permissions.set_mode(permission_to_set);
            fs::set_permissions(&file_path, permissions).expect("Set of permissions failed");
            f.sync_all().expect("Sync failed");

            FilePermissionsTestData {
                file_path: file_path,
            }
        }
    }

    impl Drop for FilePermissionsTestData {
        fn drop(&mut self) {
            if !(self.file_path.exists() && self.file_path.is_file()) {
                warn!("Trying to delete something that either does not exist or isn't a file");
            }
            fs::remove_file(&self.file_path).expect("Cleanup of test failed");
        }
    }

    #[test_case("/tmp/file_path1", 0o600, 0o700 => false; "False if tested permission is higher than the file permission")]
    #[test_case("/tmp/file_path2", 0o700, 0o700 => true; "True if permissions are equal")]
    #[test_case("/tmp/file_path3", 0o700, 0o600 => true; "True if tested permission is lower than the file permission")]
    fn test_are_permissions_fulfilled(
        file_path: &str,
        file_permission: u32,
        tested_permission: u32,
    ) -> bool {
        let test_data = FilePermissionsTestData::new(file_path, file_permission);

        let mut _permissions = fs::metadata(&test_data.file_path).unwrap().permissions();

        are_permissions_fulfilled(&test_data.file_path, tested_permission).unwrap()
    }
}
