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
use std::net::IpAddr;
use std::str::FromStr;

use actix_web::{App, HttpServer};

use resources::application;
use settings::Settings;

use crate::context::CONTEXT;

mod resources;
mod settings;
mod context;
mod implementation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = &CONTEXT.settings.server.listen_addr;
    let bind_port = &CONTEXT.settings.server.listen_port;

    HttpServer::new(|| App::new()
        .service(application::create_application))
        .bind((IpAddr::from_str(bind_addr.as_str()).unwrap(), *bind_port))?
        .run()
        .await
}
