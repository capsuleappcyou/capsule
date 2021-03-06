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
use derive_more::{Display, Error};
#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(Debug, Error, Display)]
pub struct GitError;

#[cfg_attr(test, automock)]
pub trait GitService {
    fn create_repo(&self, owner: &str, app_name: &str) -> Result<GitRepository, GitError>;
}

pub struct GitRepository {
    pub uri: String,
}


