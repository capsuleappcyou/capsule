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
use std::sync::Arc;
use lazy_static::lazy_static;

use capsule_core::application::GitService;

use crate::implementation::git_service::DefaultGitService;
use crate::settings::Settings;

lazy_static! {
    pub static ref CONTEXT: ServerContext = ServerContext::new();
}

pub struct ServerContext {
    pub settings: Settings,
}

impl ServerContext {
    pub fn new() -> Self {
        let settings = Settings::new();

        Self { settings }
    }

    pub fn git_service(&self) -> impl GitService {
        DefaultGitService
    }
}
