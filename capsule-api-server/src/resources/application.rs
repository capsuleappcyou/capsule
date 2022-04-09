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
use actix_web::{HttpResponse, post, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ApplicationCreateRequest {
    name: Option<String>,
}

impl Default for ApplicationCreateRequest {
    fn default() -> Self {
        Self {
            name: None
        }
    }
}

#[post("/applications")]
pub async fn create_application(_request: web::Json<ApplicationCreateRequest>) -> HttpResponse {
    HttpResponse::Created().finish()
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::{self}, test};
    use actix_web::dev::Service;

    use super::*;

    #[actix_web::test]
    async fn create_application_should_ok() {
        let app =
            test::init_service(App::new().service(create_application))
                .await;

        let req = test::TestRequest::post()
            .uri("/applications")
            .set_json(ApplicationCreateRequest::default())
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::CREATED);
    }
}