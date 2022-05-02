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
use anarchist_readable_name_generator_lib::readable_name;
use rand::Rng;

pub use implementation::postgres::postgres_repository::PostgresApplicationRepository;

pub use crate::application::git::{GitRepository, GitService};
use crate::CoreError;

mod repository;
mod implementation;
mod git;

pub struct Application {
    pub name: String,
    pub owner: String,
}

impl Application {
    pub fn new(new_app_name: Option<String>, owner: String) -> Self {
        let name = match new_app_name {
            Some(app_name) => app_name,
            _ => Self::random_name(),
        };

        Self { name, owner }
    }

    fn random_name() -> String {
        let random_name = readable_name();
        let random_number: u32 = rand::thread_rng().gen();

        format!("{}_{}", random_name, random_number)
    }

    pub fn create_git_repository(&self, git_service: &dyn GitService) -> Result<GitRepository, CoreError> {
        Ok(git_service.create_repo(self.owner.as_str(), self.name.as_str())?)
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::application::{Application, GitRepository};
    use crate::application::git::MockGitService;

    #[test]
    fn should_generate_application_if_not_given_application_name() {
        let name = None;
        let application = Application::new(name, "first_capsule_user".to_string());

        println!("{}", &application.name);
        assert_eq!(application.name.is_empty(), false);
    }

    #[test]
    fn should_use_given_application_name_if_give_application_name() {
        let name = Some("first_capsule_application".to_string());
        let application = Application::new(name, "first_capsule_user".to_string());

        assert_eq!(application.name, "first_capsule_application");
    }

    #[test]
    fn should_call_git_service_to_create_git_repo() {
        let application = Application::new(Some("first_capsule_application".to_string()), "first_capsule_user".to_string());
        let mut git_service = MockGitService::new();

        git_service.expect_create_repo()
            .with(eq("first_capsule_user"), eq("first_capsule_application"))
            .times(1)
            .returning(|_, _| Ok(GitRepository { url: "".to_string() }));

        application.create_git_repository(&git_service).expect("create git repo failed.");
    }
}
