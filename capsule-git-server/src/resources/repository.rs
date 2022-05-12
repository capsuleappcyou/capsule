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
use crate::resources::ApiError;

#[derive(Deserialize, Serialize)]
pub struct ApplicationCreateRequest {
    name: Option<String>,
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
pub async fn create_application(_request: web::Json<ApplicationCreateRequest>, _context: web::Data<GitServerContext>) -> Result<GitRepositoryCreateResponse, ApiError> {
    todo!()
}