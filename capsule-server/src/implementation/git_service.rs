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
use capsule_core::application::{GitRepository, GitService};
use capsule_core::CoreError;

pub struct DefaultGitService;

impl GitService for DefaultGitService {
    fn create_repo(&self, _owner: &str, _app_name: &str) -> Result<GitRepository, CoreError> {
        todo!()
    }
}