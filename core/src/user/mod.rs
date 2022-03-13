use std::ops::Deref;

use crate::user::credential::{Credential, CredentialError};
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

    pub fn verify_credential(&self, input_credential: Box<dyn Credential>) -> Result<(), CredentialError> {
        let credential = self.credentials.get_credential_by_name(input_credential.name());

        match credential {
            Some(c) => c.verify(input_credential.deref()),
            _ => Err(CredentialError)
        }
    }
}

pub trait UserFactory {
    fn create_user(user_name: String) -> User;
}

#[cfg(test)]
mod tests {
    use crate::user::{User, UserFactory};
    use crate::user::credential::Credential;
    use crate::user::credential::pwd_credential::PwdCredential;
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

        fn get_credential_by_name(&self, _name: String) -> Option<&Box<dyn Credential>> {
            Some(self.credentials.get(0).unwrap())
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

    #[test]
    fn should_verify_password_credential() {
        let mut user = TestUserFactory::create_user(String::from("test"));

        let password = Box::new(PwdCredential { plaintext: String::from("password") });
        user.add_credential(password);

        let input_password = Box::new(PwdCredential { plaintext: String::from("password") });
        let verify_result = user.verify_credential(input_password);

        assert_eq!(verify_result.is_ok(), true);
    }
}