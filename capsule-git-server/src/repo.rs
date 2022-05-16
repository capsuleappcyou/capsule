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
use std::fs::copy;
use std::path::{Path, PathBuf};

use git2::{Error, Repository};

use crate::repo::ErrorKind::{GitRepoAlreadyExists, GitRepoNotInitialized};

pub struct GitRepository {
    pub user: String,
    pub name: String,
    pub directory: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    GitRepoAlreadyExists,
    GitRepoNotInitialized,
    GitError(String),
}

#[derive(Debug, Clone)]
pub struct GitRepoErr {
    pub error_kind: ErrorKind,
}

impl From<Error> for GitRepoErr {
    fn from(e: Error) -> Self {
        Self {
            error_kind: ErrorKind::GitError(e.message().to_string())
        }
    }
}

impl From<std::io::Error> for GitRepoErr {
    fn from(e: std::io::Error) -> Self {
        Self {
            error_kind: ErrorKind::GitError(e.to_string())
        }
    }
}

impl GitRepository {
    pub fn new<P: AsRef<Path>>(user: &str, name: &str, dir: P) -> Self {
        let dir = dir.as_ref().to_str().unwrap();

        Self {
            user: user.to_string(),
            name: name.to_string(),
            directory: dir.to_string(),
        }
    }

    pub fn init_bare_repository(&self) -> Result<(), GitRepoErr> {
        if self.repo_path().exists() {
            return Err(GitRepoErr { error_kind: GitRepoAlreadyExists });
        }

        Repository::init_bare(self.repo_path())?;

        Ok(())
    }

    pub fn repo_path(&self) -> PathBuf {
        let repo_path = format!("{}/{}/{}.git", self.directory, self.user, self.name);

        PathBuf::new().join(repo_path)
    }

    pub fn install_git_hooks<P: AsRef<Path>>(&self, hooks_dir: P, hook_file_names: &Vec<&str>) -> Result<(), GitRepoErr> {
        for hook_file in hook_file_names {
            let from = PathBuf::new().join(&hooks_dir).join(hook_file);
            let to = self.repo_path().join("hooks").join(hook_file);

            if !self.repo_path().join("hooks").exists() {
                return Err(GitRepoErr { error_kind: GitRepoNotInitialized });
            }

            copy(from, to)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use tempdir::TempDir;

    use crate::repo::ErrorKind::{GitRepoAlreadyExists, GitRepoNotInitialized};
    use crate::repo::GitRepository;

    #[test]
    fn should_init_git_bare_repo() {
        let repo_dir = TempDir::new("test").unwrap();

        let git_repo = GitRepository::new("first_capsule_user", "first_capsule_application", repo_dir.path());

        git_repo.init_bare_repository().expect("init bare repo failed");

        let repo_hooks_path = &repo_dir.path().join("first_capsule_user").join("first_capsule_application.git").join("hooks");
        assert!(repo_hooks_path.exists());
    }

    #[test]
    fn should_install_git_hooks() {
        let repo_dir = TempDir::new("test").unwrap();

        let git_repo = GitRepository::new("first_capsule_user", "first_capsule_application", repo_dir.path());
        git_repo.init_bare_repository().expect("init bare repo failed");

        let result = git_repo.install_git_hooks("./_fixture/git_hooks/", &vec!["TEST_HOOKS"]);
        assert!(result.is_ok());

        let path = git_repo.repo_path().join("hooks").join("TEST_HOOKS");
        assert_eq!(read_to_string(path).unwrap(), "this is a test hook file.")
    }

    #[test]
    fn should_return_error_when_install_hooks_on_uninitialized_git_repo() {
        let repo_dir = TempDir::new("test").unwrap();

        let git_repo = GitRepository::new("first_capsule_user", "first_capsule_application", repo_dir.path());

        let result = git_repo.install_git_hooks("./_fixture/git_hooks/", &vec!["TEST_HOOKS"]);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().error_kind, GitRepoNotInitialized);
    }

    #[test]
    fn should_return_error_when_git_repo_already_exists() {
        let repo_dir = TempDir::new("test").unwrap();

        let git_repo = GitRepository::new("first_capsule_user", "first_capsule_application", repo_dir.path());
        git_repo.init_bare_repository().expect("init bare repo failed");

        let result = git_repo.init_bare_repository();

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().error_kind, GitRepoAlreadyExists);
    }
}