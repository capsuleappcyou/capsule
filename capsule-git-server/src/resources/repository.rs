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
use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use actix_web::{body::BoxBody, http::header::ContentType};
use serde::{Deserialize, Serialize};

use crate::context::GitServerContext;
use crate::repo::GitRepository;
use crate::resources::ApiError;

#[derive(Deserialize, Serialize)]
pub struct GitRepoCreateRequest {
    pub user: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct GitRepositoryCreateResponse {
    git_repo_uri: String,
}

impl Responder for GitRepositoryCreateResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Created()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/repositories")]
pub async fn create_repository(request: web::Json<GitRepoCreateRequest>, context: web::Data<GitServerContext>) -> Result<GitRepositoryCreateResponse, ApiError> {
    let git_repo = GitRepository {
        name: request.name.clone(),
        user: request.user.clone(),
        directory: context.settings.git_repo.directory.clone(),
    };

    git_repo.init_bare_repository();

    let git_repo_uri = context.settings.git_repo.url_template
        .replace("{user_name}", &request.user)
        .replace("{app_name}", &request.name);

    Ok(GitRepositoryCreateResponse { git_repo_uri })
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::remove_dir_all;
    use std::path::PathBuf;

    use actix_web::{App, http::{self}, middleware, test, web};
    use actix_web::dev::{Path, Service};
    use tempdir::TempDir;

    use crate::context::GitServerContext;
    use crate::resources::repository::{create_repository, GitRepoCreateRequest, GitRepositoryCreateResponse};

    #[actix_web::test]
    async fn should_return_git_repository_information_if_create_successfully() {
        let app =
            test::init_service(App::new()
                .app_data(web::Data::new(context()))
                .wrap(middleware::Logger::default())
                .service(create_repository))
                .await;

        let req = test::TestRequest::post()
            .uri("/repositories")
            .set_json(GitRepoCreateRequest {
                name: "first_capsule_application".to_string(),
                user: "capsule".to_string(),
            })
            .to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::CREATED);

        let body = test::read_body(resp).await;

        let expect = GitRepositoryCreateResponse {
            git_repo_uri: "https://git.capsuleapp.cyou/capsule/first_capsule_application.git".to_string(),
        };
        let expect_json = serde_json::to_string(&expect).unwrap();

        assert_eq!(expect_json, body);
    }

    #[actix_web::test]
    async fn should_create_git_repo() {
        let app =
            test::init_service(App::new()
                .app_data(web::Data::new(context()))
                .wrap(middleware::Logger::default())
                .service(create_repository))
                .await;

        let req = test::TestRequest::post()
            .uri("/repositories")
            .set_json(GitRepoCreateRequest {
                name: "first_capsule_application".to_string(),
                user: "capsule".to_string(),
            })
            .to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::CREATED);

        let repo_hooks_path = PathBuf::from("/tmp/git/").join("capsule").join("first_capsule_application.git").join("hooks");
        assert!(repo_hooks_path.exists());
    }

    fn context() -> GitServerContext {
        env::set_var("CAPSULE_GIT_CTL_CONFIG_DIR", "./_fixture");

        let context = GitServerContext::new();

        remove_dir_all(context.settings.git_repo.directory.clone().as_str());

        context
    }
}