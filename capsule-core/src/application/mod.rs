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

use anarchist_readable_name_generator_lib::readable_name;
use git2::Repository;
use rand::Rng;

pub use implementation::postgres::postgres_repository::PostgresApplicationRepository;

pub use crate::application::git::{GitRepository, GitService};
use crate::CoreError;

mod repository;
mod implementation;
mod git;

pub struct Application {
    pub name: String,
    pub owner: String,
    pub application_directory: OsString,
}

impl Application {
    pub fn new(new_app_name: Option<String>, owner: String, application_base_directory: OsString) -> Self {
        let name = match new_app_name {
            Some(app_name) => app_name,
            _ => Self::random_name(),
        };

        let app_path = PathBuf::new().join(application_base_directory).join(&owner).join(format!("{}.git", &name));
        let application_directory = OsString::from(app_path.as_path().to_str().unwrap());

        Self {
            name,
            owner,
            application_directory,
        }
    }

    fn random_name() -> String {
        let random_name = readable_name();
        let random_number: u32 = rand::thread_rng().gen();

        format!("{}_{}", random_name, random_number)
    }

    pub fn initialize_git_repository(&self, git_service: &dyn GitService) -> Result<GitRepository, CoreError> {
        let git_repo = git_service.create_repo(self.owner.as_str(), self.name.as_str())?;

        Ok(git_repo)
    }

    fn get_application_dir(&self) -> PathBuf {
        return PathBuf::new().join(self.application_directory.as_os_str());
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::path::{Path, PathBuf};

    use tempdir::TempDir;

    use crate::application::Application;
    use crate::application::git::{GitRepository, GitService};
    use crate::CoreError;

    struct DummyGitService;

    impl GitService for DummyGitService {
        fn create_repo(&self, owner: &str, app_name: &str) -> Result<GitRepository, CoreError> {
            todo!()
        }
    }

    #[test]
    fn should_generate_application_if_not_given_application_name() {
        let application = create_application(None);

        println!("{}", &application.name);
        assert_eq!(application.name.is_empty(), false);
    }

    #[test]
    fn should_use_given_application_name_if_give_application_name() {
        let application = create_application(Some("first_capsule_application".to_string()));

        assert_eq!(application.name, "first_capsule_application");
    }

    fn create_application(name: Option<String>) -> Application {
        let project_base_dir = TempDir::new("").unwrap();
        Application::new(
            name,
            "first_capsule_user".to_string(),
            project_base_dir.path().as_os_str().to_os_string(),
        )
    }
}
