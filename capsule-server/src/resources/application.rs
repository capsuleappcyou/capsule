use std::time::SystemTime;

// Copyright 2022 the original author or authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// // http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use actix_web::{body::BoxBody, http::header::ContentType};
use serde::{Deserialize, Serialize};

use capsule_core::application::{Application, ApplicationError};

use crate::context::ServerContext;
use crate::resources::ApiError;

#[derive(Deserialize, Serialize)]
pub struct ApplicationCreateRequest {
    name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ApplicationCreateResponse {
    name: String,
    application_uri: String,
    git_repo_uri: String,
}

impl Responder for ApplicationCreateResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Created()
            .content_type(ContentType::json())
            .body(body)
    }
}

impl Default for ApplicationCreateRequest {
    fn default() -> Self {
        Self {
            name: None
        }
    }
}

impl From<ApplicationError> for ApiError {
    fn from(e: ApplicationError) -> Self {
        match e {
            ApplicationError::GitError { message } => {
                ApiError::ValidationFailed { message }
            }
            ApplicationError::DomainNameError { message } => {
                ApiError::ValidationFailed { message }
            }
            ApplicationError::InternalError { message } => {
                ApiError::InternalError { message }
            }
        }
    }
}

#[post("/applications")]
pub async fn create_application(request: web::Json<ApplicationCreateRequest>, context: web::Data<ServerContext>) -> Result<ApplicationCreateResponse, ApiError> {
    let user_name = "capsule".to_string();
    let git_service = context.git_service();
    let application = Application::new(1, request.name.clone(), user_name);
    let git_repo = application.create_git_repository(git_service.as_ref())?;
    let cname_record = application.add_cname_record(context.domain_name_service().as_ref())?;

    Ok(ApplicationCreateResponse {
        name: application.accept(get_application_name),
        application_uri: format!("https://{}", cname_record.domain_name),
        git_repo_uri: git_repo.uri,
    })
}

fn get_application_name(_: i64, name: &str, _: &str, _: SystemTime) -> String {
    name.to_string()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{App, http::{self}, test};
    use actix_web::dev::Service;
    use actix_web::middleware;
    use actix_web::web::Bytes;

    use capsule_core::application::{ApplicationError, GitRepository, GitService};
    use capsule_core::application::{CnameRecord, DomainNameService};

    use crate::context::ServerContext;
    use crate::Settings;

    use super::*;

    #[actix_web::test]
    async fn should_return_application_information_if_create_successfully() {
        struct GitServiceStub;
        impl GitService for GitServiceStub {
            fn create_repo(&self, _owner: &str, _app_name: &str) -> Result<GitRepository, ApplicationError> {
                Ok(GitRepository { uri: "https://git.capsuleapp.cyou/capsule/first_capsule_application.git".to_string() })
            }
        }

        struct DomainNameServiceStub;
        impl DomainNameService for DomainNameServiceStub {
            fn add_cname_record(&self, cname: &str) -> Result<CnameRecord, ApplicationError> {
                Ok(CnameRecord { domain_name: format!("{}.capsuleapp.cyou", cname) })
            }
        }

        let app =
            test::init_service(App::new()
                .app_data(web::Data::new(context(GitServiceStub, DomainNameServiceStub)))
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
            application_uri: "https://first_capsule_application.capsuleapp.cyou".to_string(),
            git_repo_uri: "https://git.capsuleapp.cyou/capsule/first_capsule_application.git".to_string(),
        };
        let expect_json = serde_json::to_string(&expect).unwrap();

        let body = test::read_body(resp).await;
        assert_eq!(actix_web::web::Bytes::from(expect_json), body);
    }

    #[actix_web::test]
    async fn should_return_error_message_if_create_git_repository_failed() {
        struct GitServiceStub;
        impl GitService for GitServiceStub {
            fn create_repo(&self, _owner: &str, _app_name: &str) -> Result<GitRepository, ApplicationError> {
                Err(ApplicationError::GitError { message: "create git repository failed.".to_string() })
            }
        }

        struct DomainNameServiceStub;
        impl DomainNameService for DomainNameServiceStub {
            fn add_cname_record(&self, cname: &str) -> Result<CnameRecord, ApplicationError> {
                Ok(CnameRecord { domain_name: format!("{}.capsuleapp.cyou", cname) })
            }
        }

        let app =
            test::init_service(App::new()
                .app_data(web::Data::new(context(GitServiceStub, DomainNameServiceStub)))
                .wrap(middleware::Logger::default())
                .service(create_application))
                .await;

        let req = test::TestRequest::post()
            .uri("/applications")
            .set_json(ApplicationCreateRequest { name: Some("first_capsule_application".to_string()) })
            .to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY);

        let body = test::read_body(resp).await;

        let expect = Bytes::from(r#"{"message":"create git repository failed."}"#);
        assert_eq!(expect, body);
    }

    #[actix_web::test]
    async fn should_return_message_if_add_cname_record_failed() {
        struct GitServiceStub;
        impl GitService for GitServiceStub {
            fn create_repo(&self, _owner: &str, _app_name: &str) -> Result<GitRepository, ApplicationError> {
                Ok(GitRepository { uri: "https://git.capsuleapp.cyou/capsule/first_capsule_application.git".to_string() })
            }
        }

        struct DomainNameServiceStub;
        impl DomainNameService for DomainNameServiceStub {
            fn add_cname_record(&self, _cname: &str) -> Result<CnameRecord, ApplicationError> {
                Err(ApplicationError::DomainNameError { message: "add application domain record failed.".to_string() })
            }
        }

        let app =
            test::init_service(App::new()
                .app_data(web::Data::new(context(GitServiceStub, DomainNameServiceStub)))
                .wrap(middleware::Logger::default())
                .service(create_application))
                .await;

        let req = test::TestRequest::post()
            .uri("/applications")
            .set_json(ApplicationCreateRequest { name: Some("first_capsule_application".to_string()) })
            .to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY);

        let body = test::read_body(resp).await;

        let expect = Bytes::from(r#"{"message":"add application domain record failed."}"#);
        assert_eq!(expect, body);
    }

    #[actix_web::test]
    async fn should_500_if_internal_error_happened() {
        struct GitServiceStub;
        impl GitService for GitServiceStub {
            fn create_repo(&self, _owner: &str, _app_name: &str) -> Result<GitRepository, ApplicationError> {
                Ok(GitRepository { uri: "https://git.capsuleapp.cyou/capsule/first_capsule_application.git".to_string() })
            }
        }

        struct DomainNameServiceStub;
        impl DomainNameService for DomainNameServiceStub {
            fn add_cname_record(&self, _cname: &str) -> Result<CnameRecord, ApplicationError> {
                Err(ApplicationError::InternalError { message: "internal error.".to_string() })
            }
        }

        let app =
            test::init_service(App::new()
                .app_data(web::Data::new(context(GitServiceStub, DomainNameServiceStub)))
                .wrap(middleware::Logger::default())
                .service(create_application))
                .await;

        let req = test::TestRequest::post()
            .uri("/applications")
            .set_json(ApplicationCreateRequest { name: Some("first_capsule_application".to_string()) })
            .to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);

        let body = test::read_body(resp).await;

        let expect = Bytes::from(r#"{"message":"internal error."}"#);
        assert_eq!(expect, body);
    }

    fn context(git_service: impl GitService + 'static, domain_service: impl DomainNameService + 'static) -> ServerContext {
        std::env::set_var("CAPSULE_CONFIG_SERVER_DIR", "./_fixture");
        std::env::set_var("CAPSULE_SERVER_CONFIG_FILE", "capsule-server.toml");

        ServerContext {
            settings: Arc::new(Settings::new()),
            git_service: Arc::new(git_service),
            domain_name_service: Arc::new(domain_service),
        }
    }
}