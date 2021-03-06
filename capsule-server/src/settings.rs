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
use std::env;

use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Server {
    pub listen_addr: String,
    pub listen_port: u16,
}

#[derive(Deserialize)]
pub struct GitService {
    pub uri: String,
}

#[derive(Deserialize)]
pub struct Settings {
    pub server: Server,
    pub git_service: GitService,
}

impl Settings {
    pub fn new() -> Settings {
        let config_dir = env::var("CAPSULE_CONFIG_SERVER_DIR").unwrap_or_else(|_| "./config".into());
        let config_file = env::var("CAPSULE_SERVER_CONFIG_FILE").unwrap_or_else(|_| "capsule-server.toml".into());

        let config_result = Config::builder()
            .add_source(File::with_name(&format!("{}/{}", config_dir, config_file)).required(false))
            .add_source(Environment::with_prefix("capsule_server").separator("__"))
            .build();

        let config = match config_result {
            Ok(c) => c,
            Err(e) => panic!("read capsule config error: {:?}", e)
        };

        match config.try_deserialize() {
            Ok(settings) => settings,
            Err(e) => panic!("read capsule config error: {:?}", e)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn should_read_server_config() {
        let settings = settings();

        assert_eq!(80, settings.server.listen_port);
        assert_eq!("::", settings.server.listen_addr);
    }

    #[test]
    fn should_read_git_service_uri() {
        let settings = settings();

        assert_eq!("https://git-ctl.capsuleapp.cyou:7892", settings.git_service.uri);
    }

    fn settings() -> Settings {
        env::set_var("CAPSULE_CONFIG_SERVER_DIR", "./_fixture");

        Settings::new()
    }
}