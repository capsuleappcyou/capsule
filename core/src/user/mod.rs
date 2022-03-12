use crate::user::credential::Credential;
use crate::user::credentials::Credentials;

pub mod credential;
pub mod credentials;
pub mod repository;

pub struct User {
    pub user_name: String,
    credentials: Box<dyn Credentials>,
}

impl User {
    pub fn add_credential(&mut self, credential: Box<dyn Credential>) {
        self.credentials.add(credential);
    }
}

pub trait UserFactory {
    fn create_user(user_name: String) -> User;
}

#[cfg(test)]
mod tests {
    use crate::user::{User, UserFactory};
    use crate::user::credential::Credential;
    use crate::user::credentials::Credentials;

    struct FakeCredentials {
        credentials: Vec<Box<dyn Credential>>,
    }

    impl FakeCredentials {
        pub fn new() -> Self {
            FakeCredentials { credentials: vec![] }
        }
    }

    impl Credentials for FakeCredentials {
        fn add(&mut self, credential: Box<dyn Credential>) {
            self.credentials.push(credential)
        }
    }

    struct TestUserFactory;

    impl UserFactory for TestUserFactory {
        fn create_user(user_name: String) -> User {
            let credentials = Box::new(FakeCredentials::new());

            User { user_name, credentials }
        }
    }

    #[test]
    fn should_create_user() {
        let user = TestUserFactory::create_user(String::from("test"));

        assert_eq!(user.user_name, String::from("test"))
    }
}