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

use capsule_core::application::Application;

use crate::context::CONTEXT;

#[derive(Deserialize, Serialize)]
pub struct ApplicationCreateRequest {
    name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ApplicationCreateResponse {
    name: String,
    url: String,
    git_repo: String,
}

impl Default for ApplicationCreateRequest {
    fn default() -> Self {
        Self {
            name: None
        }
    }
}

#[post("/applications")]
pub async fn create_application(request: web::Json<ApplicationCreateRequest>) -> impl Responder {
    let user_name = "capsule".to_string();

    let app_base_dir = &CONTEXT.settings.git_repo.base_dir;

    let application = Application::new(
        request.name.clone(), user_name, OsString::from(app_base_dir));

    application.initialize_git_repository();

    let response = ApplicationCreateResponse {
        name: application.name.clone(),
        url: app_url(&application),
        git_repo: git_repo_url(&application),
    };

    (web::Json(response), http::StatusCode::CREATED)
}

fn app_url(application: &Application) -> String {
    let template = &CONTEXT.settings.app.url_template;

    template.replace("{app_name}", application.name.as_str())
}

fn git_repo_url(application: &Application) -> String {
    let template = &CONTEXT.settings.git_repo.url_template;

    let app_name = application.name.as_str();
    let owner_name = application.owner.as_str();

    template.replace("{app_name}", app_name).replace("{user_name}", owner_name)
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
        async fn should_201_if_create_application_successfully() {
            std::env::set_var("CAPSULE_CONFIG_SERVER_DIR", "./_fixture/");
            std::env::set_var("CAPSULE_SERVER_CONFIG_FILE", "capsule-server.toml");

            let git_dir = TempDir::new("git").unwrap();
            std::env::set_var("CAPSULE_SERVER__GIT_REPO__BASE_DIR", git_dir.path().to_str().unwrap());

            let app =
                test::init_service(App::new().service(create_application))
                    .await;

            let req = test::TestRequest::post()
                .uri("/applications")
                .set_json(ApplicationCreateRequest::default())
                .to_request();

            let resp = app.call(req).await.unwrap();
            assert_eq!(resp.status(), http::StatusCode::CREATED);

            std::env::remove_var("CAPSULE_SERVER__GIT_REPO__BASE_DIR");
        }

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
            let expect = ApplicationCreateResponse {
                name: "first_capsule_application".to_string(),
                url: "https://first_capsule_application.capsuleapp.cyou".to_string(),
                git_repo: "https://git.capsuleapp.cyou/capsule/first_capsule_application.git".to_string(),
            };
            let expect_json = serde_json::to_string(&expect).unwrap();

            let body = test::read_body(resp).await;
            assert_eq!(actix_web::web::Bytes::from(expect_json), body);

            std::env::remove_var("CAPSULE_SERVER__GIT_REPO__BASE_DIR");
        }

        #[actix_web::test]
        async fn should_create_application_git_bare_repo() {
            std::env::set_var("CAPSULE_CONFIG_SERVER_DIR", "./_fixture");
            std::env::set_var("CAPSULE_SERVER_CONFIG_FILE", "capsule-server-test.toml");

            let git_dir = TempDir::new("git").unwrap();
            std::env::set_var("CAPSULE_SERVER__GIT_REPO__BASE_DIR", git_dir.path().to_str().unwrap());

            let app =
                test::init_service(App::new().service(create_application))
                    .await;

            let req = test::TestRequest::post()
                .uri("/applications")
                .set_json(ApplicationCreateRequest::default())
                .to_request();

            let resp = app.call(req).await.unwrap();
            let body = test::read_body(resp).await;

            let json_string = String::from_utf8(body.to_vec()).unwrap();
            let response: ApplicationCreateResponse = serde_json::from_str(json_string.as_str()).unwrap();

            let application_git_repo_path = &CONTEXT.settings.git_repo.base_dir.as_str();

            assert!(Path::new(application_git_repo_path).join("capsule").join(format!("{}.git", response.name)).join("hooks").exists());

            std::env::remove_var("CAPSULE_SERVER__GIT_REPO__BASE_DIR");
        }
    }
}