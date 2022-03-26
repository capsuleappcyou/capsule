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
use std::path::Path;

use git2::Repository;

use crate::CommandError;

pub fn handle<P: AsRef<Path>>(application_directory: P) -> Result<(), CommandError> {
    let result = Repository::init(application_directory);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(CommandError { message: e.to_string() })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use tempdir::TempDir;

    #[test]
    fn should_init_git_repository_if_application_is_not_a_git_repository() {
        let application_dir = TempDir::new("").unwrap();

        let result = super::handle(&application_dir);

        assert_eq!(result.is_ok(), true);
        assert_eq!(PathBuf::from(application_dir.path()).join(".git").exists(), true);
    }
    //TODO create application on capsule if application not present
    //TODO add git remote to application repository
}