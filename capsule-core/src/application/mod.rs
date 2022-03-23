use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use crate::CoreErr;

mod repository;

struct Application {
    name: String,
}

impl Application {
    fn create_directory(&self, project_base_dir: &OsStr) -> Result<Box<Path>, CoreErr> {
        let project_dir = PathBuf::new()
            .join(Path::new(project_base_dir))
            .join(Path::new(self.name.as_str()));

        let result = create_dir_all(project_dir.as_path());

        match result {
            Ok(()) => Ok(project_dir.into_boxed_path()),
            Err(e) => Err(CoreErr { message: e.to_string() })
        }
    }
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;

    use crate::application::Application;

    #[test]
    fn should_create_application_directory() {
        let application = Application { name: "first application".to_string() };

        let project_base_dir = TempDir::new("").unwrap();

        let result = application.create_directory(project_base_dir.path().as_os_str());

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap().exists(), true);
    }

    // TODO init application with git and install git hooks
}

