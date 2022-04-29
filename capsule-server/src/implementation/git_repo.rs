use capsule_core::application::{GitRepository, GitService};
use capsule_core::CoreError;

pub struct DefaultGitService;

impl GitService for DefaultGitService {
    fn create_repo(&self, owner: &str, app_name: &str) -> Result<GitRepository, CoreError> {
        Ok(GitRepository { url: "https://git.capsuleapp.cyou/capsule/first_capsule_application.git".to_string() })
    }
}