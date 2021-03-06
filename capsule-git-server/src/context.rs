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

use crate::settings::Settings;

pub struct GitServerContext {
    pub settings: Settings,
}

impl GitServerContext {
    pub fn new() -> Self {
        let config_dir = env::var("CAPSULE_GIT_CTL_CONFIG_DIR").unwrap_or_else(|_| "./config".into());
        Self {
            settings: Settings::new(config_dir.as_str()).unwrap()
        }
    }
}