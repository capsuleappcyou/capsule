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

use capsule_core::application::{DefaultGitService, DomainNameService, GitService};

use crate::implementation::domain_name_service::DefaultDomainNameService;
use crate::settings::Settings;

pub struct ServerContext {
    pub settings: Arc<Settings>,
    pub git_service: Arc<dyn GitService>,
    pub domain_name_service: Arc<dyn DomainNameService>,
}

impl ServerContext {
    pub fn new() -> Self {
        let settings = Arc::new(Settings::new());
        let git_service = Arc::new(DefaultGitService { host_uri: "".to_string() });
        let domain_name_service = Arc::new(DefaultDomainNameService);

        Self { settings, git_service, domain_name_service }
    }

    pub fn settings(&self) -> Arc<Settings> {
        self.settings.clone()
    }

    pub fn git_service(&self) -> Arc<dyn GitService> {
        self.git_service.clone()
    }

    pub fn domain_name_service(&self) -> Arc<dyn DomainNameService> {
        self.domain_name_service.clone()
    }
}
