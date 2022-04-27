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
pub struct CtlServer {
    pub listen_addr: String,
    pub listen_port: u16,
}


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GitRepo {
    pub url_template: String,
    pub directory: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub ctl_server: CtlServer,
    pub git_repo: GitRepo,
}

impl Settings {
    pub fn new(config_dir: &str) -> Result<Settings, ConfigError> {
        let capsule_git_env = env::var("CAPSULE_GIT_ENV").unwrap_or_else(|_| "default".into());

        let config = Config::builder()
            .add_source(File::with_name(&format!("{}/{}", config_dir, capsule_git_env)).required(false))
            .add_source(Environment::with_prefix("capsule_git").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::Settings;

    #[test]
    fn should_read_ctl_server_config() {
        let settings = Settings::new("./_fixture").unwrap();

        assert_eq!(settings.ctl_server.listen_addr, "::");
        assert_eq!(settings.ctl_server.listen_port, 7892);
    }

    #[test]
    fn should_read_git_repo_config() {
        let settings = Settings::new("./_fixture").unwrap();

        assert_eq!(settings.git_repo.url_template, "https://git.capsuleapp.cyou/{user_name}/{app_name}.git");
        assert_eq!(settings.git_repo.directory, "/tmp/capsule/git");
    }
}