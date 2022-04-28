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

use actix_web::{http, post, Responder, web};
use serde::{Deserialize, Serialize};

use capsule_core::application::{Application, GitRepository, GitService};
use capsule_core::CoreError;

use crate::context::CONTEXT;
use crate::settings::GitRepo;

#[derive(Deserialize, Serialize)]
pub struct ApplicationCreateRequest {
    name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ApplicationCreateResponse {
    name: String,
    url: String,
    git_repo_url: String,
}

impl Default for ApplicationCreateRequest {
    fn default() -> Self {
        Self {
            name: None
        }
    }
}

struct DefaultGitService;

impl GitService for DefaultGitService {
    fn create_repo(&self, owner: &str, app_name: &str) -> Result<GitRepository, CoreError> {
        Ok(GitRepository { url: "https://git.capsuleapp.cyou/capsule/first_capsule_application.git".to_string() })
    }
}

#[post("/applications")]
pub async fn create_application(request: web::Json<ApplicationCreateRequest>) -> impl Responder {
    let user_name = "capsule".to_string();

    let app_base_dir = &CONTEXT.settings.git_repo.base_dir;

    let git_server = DefaultGitService;

    let application = Application::new(
        request.name.clone(), user_name, OsString::from(app_base_dir));

    let git_repo_create_result = application.initialize_git_repository(&git_server);

    match git_repo_create_result {
        Ok(repo) => {
            let response = ApplicationCreateResponse {
                name: application.name.clone(),
                url: app_url(&application),
                git_repo_url: repo.url,
            };

            (web::Json(response), http::StatusCode::CREATED)
        }
        Err(e) => {
            let response = ApplicationCreateResponse {
                name: application.name.clone(),
                url: app_url(&application),
                git_repo_url: "".to_string(),
            };

            (web::Json(response), http::StatusCode::CREATED)
        }
    }
}

fn app_url(application: &Application) -> String {
    let template = &CONTEXT.settings.app.url_template;

    template.replace("{app_name}", application.name.as_str())
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::{self}, test};
    use actix_web::dev::Service;

    use super::*;

    #[cfg(test)]
    mod create_application {
        use std::path::Path;

        use tempdir::TempDir;

        use super::*;

        #[actix_web::test]
        async fn should_return_application_information_if_create_successfully() {
            std::env::set_var("CAPSULE_CONFIG_SERVER_DIR", "./_fixture");
            std::env::set_var("CAPSULE_SERVER_CONFIG_FILE", "capsule-server.toml");

            let git_dir = TempDir::new("git").unwrap();
            std::env::set_var("CAPSULE_SERVER__GIT_REPO__BASE_DIR", git_dir.path().to_str().unwrap());

            let app =
                test::init_service(App::new().service(create_application))
                    .await;

            let req = test::TestRequest::post()
                .uri("/applications")
                .set_json(ApplicationCreateRequest { name: Some("first_capsule_application".to_string()) })
                .to_request();

            let resp = app.call(req).await.unwrap();
            assert_eq!(resp.status(), http::StatusCode::CREATED);

            let expect = ApplicationCreateResponse {
                name: "first_capsule_application".to_string(),
                url: "https://first_capsule_application.capsuleapp.cyou".to_string(),
                git_repo_url: "https://git.capsuleapp.cyou/capsule/first_capsule_application.git".to_string(),
            };
            let expect_json = serde_json::to_string(&expect).unwrap();

            let body = test::read_body(resp).await;
            assert_eq!(actix_web::web::Bytes::from(expect_json), body);

            std::env::remove_var("CAPSULE_SERVER__GIT_REPO__BASE_DIR");
        }
    }
}