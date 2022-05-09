use std::collections::HashMap;
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
use std::fmt::{Debug, Display, Formatter};

use actix_web::{error, HttpResponse};
use actix_web::http::StatusCode;
use derive_more::Error;

pub mod application;

#[derive(Debug, Error)]
pub enum ApiError {
    ValidationFailed { message: String },
    InternalError { message: String },
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let resp_json = match self {
            ApiError::ValidationFailed { message } => {
                let mut response = HashMap::new();
                response.insert("message", message);

                serde_json::to_string(&response).unwrap()
            }
            ApiError::InternalError { message } => {
                let mut response = HashMap::new();
                response.insert("message", message);

                serde_json::to_string(&response).unwrap()
            }
        };

        write!(f, "{}", resp_json)
    }
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::ValidationFailed { message: _ } => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::InternalError { message: _ } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .body(self.to_string())
    }
}