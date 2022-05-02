use capsule_core::application::{GitRepository, GitService};
use capsule_core::CoreError;

pub struct DefaultGitService;

impl GitService for DefaultGitService {
    fn create_repo(&self, _owner: &str, _app_name: &str) -> Result<GitRepository, CoreError> {
        todo!()
    }
}