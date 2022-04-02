// Copyright 2022 the original author or authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::ffi::OsString;
use std::fs::copy;
use std::path::{Path, PathBuf};

use git2::Repository;

use crate::CoreError;

mod repository;
mod implementation;

pub struct Application {
    name: String,
    owner: String,
    application_directory: OsString,
}

impl Application {
    pub fn initialize_git_repository(&self) -> Result<Box<Path>, CoreError> {
        let application_dir = self.get_application_dir();

        let result = Repository::init_bare(application_dir.as_path());

        match result {
            Ok(_) => Ok(application_dir.into_boxed_path()),
            Err(e) => Err(CoreError { message: e.to_string() })
        }
    }

    pub fn install_git_hooks<P: AsRef<Path>>(&self, hooks_dir: P, hook_file_names: &Vec<&str>) -> Result<(), CoreError> {
        for hook_file in hook_file_names {
            let from = PathBuf::new().join(&hooks_dir).join(hook_file);
            let to = self.get_application_dir().join("hooks").join(hook_file);

            let result = copy(from, to);

            if let Err(e) = result {
                return Err(CoreError { message: e.to_string() });
            }
        }

        Ok(())
    }

    fn get_application_dir(&self) -> PathBuf {
        return PathBuf::new()
            .join(self.application_directory.as_os_str())
            .join(self.name.as_str());
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::path::{Path, PathBuf};

    use tempdir::TempDir;

    use crate::application::Application;

    #[test]
    fn should_initialize_git_repository() {
        let application = create_application();

        let result = application.initialize_git_repository();
        assert_eq!(result.is_ok(), true);

        let project_path = result.ok().unwrap();
        assert_eq!(PathBuf::new().join(project_path).join("objects").exists(), true);
    }

    #[test]
    fn should_install_git_hooks_to_application() {
        let application = create_application();

        let _ = application.initialize_git_repository();

        let result = application.install_git_hooks("./_fixture/git_hooks/", &vec!["TEST_HOOKS"]);
        assert_eq!(result.is_ok(), true);

        let path = application.get_application_dir().join(Path::new("hooks")).join(Path::new("TEST_HOOKS"));
        assert_eq!(read_to_string(path).unwrap(), "this is a test hook file.")
    }

    #[test]
    fn should_error_when_install_git_hooks_to_application_if_application_not_initialized() {
        let application = create_application();

        let result = application.install_git_hooks("./_fixture/git_hooks/", &vec!["TEST_HOOKS"]);
        assert_eq!(result.is_ok(), false);
    }

    fn create_application() -> Application {
        let project_base_dir = TempDir::new("").unwrap();
        Application {
            name: "first_application".to_string(),
            owner: "first_capsule_user".to_string(),
            application_directory: project_base_dir.path().as_os_str().to_os_string(),
        }
    }
}

