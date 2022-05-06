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
use actix_web::{http, post, Responder, web};
use serde::{Deserialize, Serialize};

use capsule_core::application::Application;

use crate::context::ServerContext;

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

#[post("/applications")]
pub async fn create_application(request: web::Json<ApplicationCreateRequest>, context: web::Data<ServerContext>) -> impl Responder {
    let user_name = "capsule".to_string();

    let git_service = context.git_service();

    let application = Application::new(request.name.clone(), user_name);

    let git_repo = application.create_git_repository(git_service.as_ref()).unwrap();
    let cname_record = application.add_cname_record(context.domain_name_service().as_ref()).unwrap();

    let response = ApplicationCreateResponse {
        name: application.name.clone(),
        url: format!("https://{}", cname_record.domain_name),
        git_repo_url: git_repo.url,
    };

    (web::Json(response), http::StatusCode::CREATED)
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::{self}, test};
    use actix_web::dev::Service;

    use super::*;

    #[cfg(test)]
    mod create_application {
        use std::sync::Arc;

        use actix_web::middleware;

        use capsule_core::application::{GitRepository, GitService};
        use capsule_core::application::{CnameRecord, DomainNameService};
        use capsule_core::CoreError;

        use crate::context::ServerContext;
        use crate::Settings;

        use super::*;

        #[actix_web::test]
        async fn should_return_application_information_if_create_successfully() {
            std::env::set_var("CAPSULE_CONFIG_SERVER_DIR", "./_fixture");
            std::env::set_var("CAPSULE_SERVER_CONFIG_FILE", "capsule-server.toml");

            let app =
                test::init_service(App::new()
                    .app_data(web::Data::new(context()))
                    .wrap(middleware::Logger::default())
                    .service(create_application))
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

        fn context() -> ServerContext {
            struct GitServiceStub;
            impl GitService for GitServiceStub {
                fn create_repo(&self, _owner: &str, _app_name: &str) -> Result<GitRepository, CoreError> {
                    Ok(GitRepository { url: "https://git.capsuleapp.cyou/capsule/first_capsule_application.git".to_string() })
                }
            }

            struct DomainNameServiceStub;
            impl DomainNameService for DomainNameServiceStub {
                fn add_cname_record(&self, cname: &str) -> Result<CnameRecord, CoreError> {
                    Ok(CnameRecord { domain_name: format!("{}.capsuleapp.cyou",cname) })
                }
            }

            ServerContext {
                settings: Arc::new(Settings::new()),
                git_service: Arc::new(GitServiceStub),
                domain_name_service: Arc::new(DomainNameServiceStub),
            }
        }
    }
}