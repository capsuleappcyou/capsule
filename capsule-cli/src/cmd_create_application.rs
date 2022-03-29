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
use std::path::{Path, PathBuf};

use git2::Repository;

use crate::api::{ApplicationCreateResponse, CapsuleApi};
use crate::CliError;

pub fn handle<P, A>(_application_directory: P, application_name: Option<String>, api: &A) -> Result<ApplicationCreateResponse, CliError>
    where P: AsRef<Path>, A: CapsuleApi {
    api.create_application(application_name)
}

fn is_git_repository<P: AsRef<Path>>(application_directory: P) -> bool {
    PathBuf::new().join(application_directory).join(".git").exists()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use mockall::{automock, mock, predicate::*};
    use tempdir::TempDir;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};

    use crate::api::MockCapsuleApi;
    use crate::cmd_create_application::{handle, is_git_repository};

    use super::*;

    #[test]
    fn should_create_application_if_application_directory_is_not_a_git_repository() {
        let mut mock_api = MockCapsuleApi::new();

        let application_directory = PathBuf::new().join(".");

        let directory_path = application_directory.as_path();

        mock_api
            .expect_create_application()
            .with(eq(None))
            .times(1)
            .returning(|name| Ok(ApplicationCreateResponse { name: "first_capsule_application".to_string() }));

        let result = handle(directory_path, None, &mock_api);

        assert_eq!(result.is_ok(), true);

        assert_eq!(result.ok().unwrap().name, "first_capsule_application");
        assert_eq!(is_git_repository(application_directory), false);
    }

    // TODO add remote git repository if current directory is a git repository.
}