use std::net::IpAddr;
use std::str::FromStr;

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
use actix_web::{App, HttpServer};
use lazy_static::lazy_static;

use resources::application;
use settings::Settings;

mod resources;
pub mod settings;
lazy_static! {
    // static ref SETTINGS: Settings = Settings::new("./config").unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let bind_addr = &SETTINGS.server.listen_addr;
    // let bind_port = &SETTINGS.server.listen_port;
    HttpServer::new(|| App::new()
        .service(application::create_application))
        .bind((IpAddr::from_str("::").unwrap(), 80))?
        .run()
        .await
}
