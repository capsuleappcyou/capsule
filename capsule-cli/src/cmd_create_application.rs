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

pub fn handle<P, A>(application_directory: P, api: A) -> Result<ApplicationCreateResponse, CliError>
    where P: AsRef<Path>,
          A: AsRef<dyn CapsuleApi> {
    unimplemented!()
}

fn is_git_repository<P: AsRef<Path>>(application_directory: P) -> bool {
    PathBuf::new().join(application_directory).join(".git").exists()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use tempdir::TempDir;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};

    use crate::cmd_create_application::{handle, is_git_repository};

    use super::*;

    // TODO create application if current directory is not a git repository.
    #[async_std::test]
    async fn should_create_application_if_application_directory_is_not_a_git_repository() {
        // let mock_server = MockServer::start().await;
        //
        // Mock::given(method("POST"))
        //     .and(path("/applications"))
        //     .respond_with(ResponseTemplate::new(201)
        //         .set_body_json(ApplicationCreateResult { name: "first_capsule_application".to_string() }))
        //     .mount(&mock_server)
        //     .await;
        //
        // let application_directory = PathBuf::new().join(".");
        // let result = handle(application_directory.as_path());
        //
        // assert_eq!(result.is_ok(), true);
        //
        // let application_create_result = result.ok().unwrap();
        // assert_eq!(application_create_result.name, "first_capsule_application")
    }

    // TODO add remote git repository if current directory is a git repository.
}