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

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub listen_addr: String,
    pub listen_port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct App {
    pub url_template: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GitRepo {
    pub base_dir: String,
    pub domain_name: String,
    pub port: u16,
    pub scheme: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub app: App,
    pub server: Server,
    pub git_repo: GitRepo,
}

impl Settings {
    pub fn new(config_dir: &str) -> Result<Settings, ConfigError> {
        let capsule_env = env::var("CAPSULE_ENV").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name(&format!("{}/default", config_dir)))
            .add_source(File::with_name(&format!("{}/{}", config_dir, capsule_env)).required(false))
            .add_source(Environment::with_prefix("CAPSULE_"))
            .build()?;

        config.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use crate::Settings;

    #[test]
    fn should_read_server_config() {
        let settings = Settings::new("./_fixture").unwrap();

        assert_eq!(80, settings.server.listen_port);
        assert_eq!("::", settings.server.listen_addr);
    }

    #[test]
    fn should_read_git_repo_config() {
        let settings = Settings::new("./_fixture").unwrap();

        assert_eq!(80, settings.git_repo.port);
        assert_eq!("git.capsuleapp.cyou", settings.git_repo.domain_name);
        assert_eq!("https", settings.git_repo.scheme);
        assert_eq!("/tmp/capsule/git", settings.git_repo.base_dir);
    }

    #[test]
    fn should_read_app_config() {
        let settings = Settings::new("./_fixture").unwrap();

        assert_eq!("https://{app_name}.capsuleapp.cyou", settings.app.url_template);
    }
}