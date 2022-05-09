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
use isahc::{ReadResponseExt, Request, RequestExt};
use isahc::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Error;

use crate::application::{ApplicationError, GitRepository, GitService};

#[derive(Deserialize, Serialize)]
pub struct CreateGitRepoRequest {
    pub owner: String,
    pub app_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateGitRepoResponse {
    pub uri: String,
}

pub struct DefaultGitService {
    pub host_uri: String,
}

impl From<Error> for ApplicationError {
    fn from(e: Error) -> Self {
        ApplicationError::InternalError { message: e.to_string() }
    }
}

impl From<isahc::http::Error> for ApplicationError {
    fn from(e: isahc::http::Error) -> Self {
        ApplicationError::InternalError { message: e.to_string() }
    }
}

impl From<isahc::Error> for ApplicationError {
    fn from(e: isahc::Error) -> Self {
        ApplicationError::InternalError { message: e.to_string() }
    }
}

impl GitService for DefaultGitService {
    fn create_repo(&self, owner: &str, app_name: &str) -> Result<GitRepository, ApplicationError> {
        let host = &self.host_uri;
        let uri = format!("{}/repositories", host);

        let mut response = Request::post(uri.as_str())
            .header("content-type", "application/json")
            // .timeout(self.timeout)
            .body(serde_json::to_vec(&CreateGitRepoRequest {
                owner: owner.to_string(),
                app_name: app_name.to_string(),
            })?)?
            .send()?;

        if response.status() != StatusCode::CREATED {
            return Err(ApplicationError::GitError { message: format!("response status {}", response.status()) });
        }

        let api_response = response.json::<CreateGitRepoResponse>()?;

        Ok(GitRepository { uri: api_response.uri })
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{body_json, method, path};

    use crate::application::{DefaultGitService, GitService};
    use crate::application::implementation::git_service::{CreateGitRepoRequest, CreateGitRepoResponse};

    #[async_std::test]
    async fn should_send_git_repository_request_to_git_server() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/repositories"))
            .and(body_json(CreateGitRepoRequest {
                owner: "first_capsule_user".to_string(),
                app_name: "first_capsule_application".to_string(),
            }))
            .respond_with(ResponseTemplate::new(201)
                .set_body_json(CreateGitRepoResponse {
                    uri: "https://first_capsule_application.capsuleapp.cyou".to_string()
                }))
            .mount(&mock_server)
            .await;

        let git_service = DefaultGitService { host_uri: mock_server.uri() };

        let git_repo = git_service.create_repo("first_capsule_user", "first_capsule_application").expect("create git repo failed");

        assert_eq!("https://first_capsule_application.capsuleapp.cyou", git_repo.uri)
    }

    #[async_std::test]
    async fn should_get_git_service_error_when_status_code_was_not_201() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/repositories"))
            .and(body_json(CreateGitRepoRequest {
                owner: "first_capsule_user".to_string(),
                app_name: "first_capsule_application".to_string(),
            }))
            .respond_with(ResponseTemplate::new(400)
                .set_body_json(CreateGitRepoResponse {
                    uri: "https://first_capsule_application.capsuleapp.cyou".to_string()
                }))
            .mount(&mock_server)
            .await;

        let git_service = DefaultGitService { host_uri: mock_server.uri() };

        let result = git_service.create_repo("first_capsule_user", "first_capsule_application");

        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!("git service error response status 400 Bad Request", error.to_string())
    }
}