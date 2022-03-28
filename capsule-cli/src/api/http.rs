use std::time::Duration;

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
use async_std::prelude::FutureExt;
use isahc::http::{Error, StatusCode, Uri};
use isahc::prelude::*;
use isahc::Request;
use serde::{Deserialize, Serialize};
use wiremock::matchers::body_json;

use crate::api::{ApiError, ApplicationCreateResponse, CapsuleApi};

pub struct HttpCapsuleApi {
    uri: String,
    timeout: Duration,
}

#[derive(Serialize, Deserialize)]
struct CreateApplicationRequest {
    name: Option<String>,
}

impl From<isahc::http::Error> for ApiError {
    fn from(e: isahc::http::Error) -> Self {
        Self { message: e.to_string() }
    }
}

impl From<isahc::Error> for ApiError {
    fn from(e: isahc::Error) -> Self {
        Self { message: e.to_string() }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        Self { message: e.to_string() }
    }
}

impl CapsuleApi for HttpCapsuleApi {
    fn create_application(&self, name: Option<String>) -> Result<ApplicationCreateResponse, ApiError> {
        let base_uri = &self.uri;
        let uri_str = format!("{base_uri}/applications");

        let mut response = Request::post(uri_str.as_str())
            .header("content-type", "application/json")
            .timeout(self.timeout)
            .body(serde_json::to_vec(&CreateApplicationRequest { name })?)?
            .send()?;

        if response.status() != StatusCode::CREATED {
            let status_code = response.status().as_u16();
            return Err(ApiError { message: format!("The server response status {status_code}.") });
        }

        let api_response = response.json::<ApplicationCreateResponse>()?;

        Ok(ApplicationCreateResponse { name: api_response.name })
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{body_json, method, path};

    use crate::api::{ApplicationCreateResponse, CapsuleApi};
    use crate::api::http::HttpCapsuleApi;

    use super::*;

    #[async_std::test]
    async fn should_send_create_application_request_to_api_server() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/applications"))
            .and(body_json(CreateApplicationRequest { name: Some("first_capsule_application".to_string()) }))
            .respond_with(ResponseTemplate::new(201)
                .set_body_json(ApplicationCreateResponse { name: "first_capsule_application".to_string() }))
            .mount(&mock_server)
            .await;

        let api = HttpCapsuleApi { uri: mock_server.uri(), timeout: Duration::from_secs(5) };
        let result = api.create_application(Some("first_capsule_application".to_string()));

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap().name, "first_capsule_application");
    }

    #[async_std::test]
    async fn should_return_error_if_server_response_500() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/applications"))
            .respond_with(ResponseTemplate::new(500)
                .set_body_json(ApplicationCreateResponse { name: "first_capsule_application".to_string() }))
            .mount(&mock_server)
            .await;

        let api = HttpCapsuleApi { uri: mock_server.uri(), timeout: Duration::from_secs(5) };
        let result = api.create_application(Some("first_capsule_application".to_string()));

        assert_eq!(result.is_ok(), false);
        assert_eq!(result.err().unwrap().message, "The server response status 500.");
    }

    #[async_std::test]
    async fn should_return_error_if_server_time_out() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/applications"))
            .and(body_json(CreateApplicationRequest { name: Some("first_capsule_application".to_string()) }))
            .respond_with(ResponseTemplate::new(201)
                .set_delay(Duration::from_secs(60))
                .set_body_json(ApplicationCreateResponse { name: "first_capsule_application".to_string() }))
            .mount(&mock_server)
            .await;

        let api = HttpCapsuleApi { uri: mock_server.uri(), timeout: Duration::from_secs(1) };
        let result = api.create_application(Some("first_capsule_application".to_string()));

        assert_eq!(result.is_ok(), false);
        assert_eq!(result.err().unwrap().message, "request or operation took longer than the configured timeout time");
    }
}