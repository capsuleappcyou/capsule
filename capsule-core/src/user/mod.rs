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
use std::fs::create_dir_all;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use crate::CoreError;
use crate::user::credential::{Credential, CredentialError};
pub use crate::user::credential::pwd_credential::PlaintextCredential;
use crate::user::credentials::Credentials;
pub use crate::user::implementation::postgres::postgres_repository::PostgresUserRepository;
pub use crate::user::implementation::postgres::PostgresUserFactory;
pub use crate::user::repository::UserRepository;

pub mod credential;
pub mod repository;
pub(crate) mod credentials;
pub(crate) mod implementation;

pub struct User<'a> {
    pub user_name: String,
    credentials: Box<dyn Credentials + 'a>,
}

impl<'a> User<'a> {
    pub fn add_credential(&mut self, credential: Box<dyn Credential>) -> Result<(), CredentialError> {
        match self.credentials.add(credential) {
            Ok(_) => Ok(()),
            Err(e) => Err(CredentialError { message: e.message })
        }
    }

    pub fn verify_credential(&self, input_credential: Box<dyn Credential>) -> Result<(), CredentialError> {
        let credential = self.credentials.get_credential_by_credential_name(input_credential.name().as_str());

        match credential {
            Some(c) => c.verify(input_credential.deref()),
            _ => Err(CredentialError { message: String::from("unsupported credential") })
        }
    }

    pub fn create_home_dir<P: AsRef<Path>>(&self, base_dir: P) -> Result<Box<Path>, CoreError> {
        let home_dir = PathBuf::new()
            .join(base_dir)
            .join(self.user_name.as_str());

        let result = create_dir_all(&home_dir);

        match result {
            Ok(_) => Ok(home_dir.into_boxed_path()),
            Err(e) => Err(CoreError { message: e.to_string() })
        }
    }
}

pub trait UserFactory {
    fn create_user(&self, user_name: String) -> User;
}

#[cfg(test)]
mod tests {
    use std::fs::create_dir;
    use std::path::{Path, PathBuf};

    use tempdir::TempDir;

    use crate::CoreError;
    use crate::user::{User, UserFactory};
    use crate::user::credential::{Credential, CredentialError};
    use crate::user::credential::pwd_credential::{Password, PasswordCredential, PlaintextCredential};
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
        fn add(&mut self, credential: Box<dyn Credential>) -> Result<(), CoreError> {
            self.credentials.push(credential);
            Ok(())
        }

        fn get_credential_by_credential_name(&self, _name: &str) -> Option<Box<dyn Credential>> {
            let credential = PasswordCredential {
                password: Password {
                    salt: 3129932827,
                    digest: "def98520a0b3cb13c0b96ade9c8a02a2".to_string(),
                }
            };

            Some(Box::new(credential))
        }
    }

    struct TestUserFactory;

    impl UserFactory for TestUserFactory {
        fn create_user(&self, user_name: String) -> User {
            let credentials = Box::new(FakeCredentials::new());

            User { user_name, credentials }
        }
    }

    struct UnSupportedCredential;

    impl Credential for UnSupportedCredential {
        fn verify(&self, _credential: &dyn Credential) -> Result<(), CredentialError> {
            Ok(())
        }

        fn name(&self) -> String {
            String::from("unsupported")
        }
    }

    #[test]
    fn should_create_user() {
        let user_factory = TestUserFactory;

        let user = user_factory.create_user(String::from("test"));

        assert_eq!(user.user_name, String::from("test"))
    }

    #[test]
    fn should_verify_password_given_correct_credential() {
        let user_factory = TestUserFactory;

        let mut user = user_factory.create_user(String::from("test"));

        let password = Box::new(PlaintextCredential { plaintext: String::from("password") });
        let _ = user.add_credential(password);

        let correct_password = Box::new(PlaintextCredential { plaintext: String::from("password") });
        let verify_result = user.verify_credential(correct_password);

        assert_eq!(verify_result.is_ok(), true);
    }

    #[test]
    fn should_verify_password_given_incorrect_credential() {
        let user_factory = TestUserFactory;

        let mut user = user_factory.create_user(String::from("test"));

        let password = Box::new(PlaintextCredential { plaintext: String::from("password") });
        let _ = user.add_credential(password);

        let wrong_password = Box::new(PlaintextCredential { plaintext: String::from("wrong password") });
        let verify_result = user.verify_credential(wrong_password);

        assert_eq!(verify_result.is_err(), true);
    }

    #[test]
    fn should_verify_password_given_unsupported_credential() {
        let user_factory = TestUserFactory;

        let mut user = user_factory.create_user(String::from("test"));

        let password = Box::new(PlaintextCredential { plaintext: String::from("password") });
        let _ = user.add_credential(password);

        let unsupported_credential = Box::new(UnSupportedCredential {});
        let verify_result = user.verify_credential(unsupported_credential);

        assert_eq!(verify_result.is_err(), true);
    }

    #[test]
    fn should_create_home_directory() {
        let user_factory = TestUserFactory;

        let user = user_factory.create_user(String::from("first_capsule_user"));

        let home_base_dir = TempDir::new("capsule_users").unwrap();

        let result = user.create_home_dir(home_base_dir.path());

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap().exists(), true);
    }

    #[test]
    fn should_not_create_home_directory_multi_time() {
        let user_factory = TestUserFactory;

        let user = user_factory.create_user(String::from("first_capsule_user"));

        let home_base_dir = TempDir::new("capsule_users").unwrap();

        let _ = user.create_home_dir(home_base_dir.path().as_os_str());

        let test_path = PathBuf::new()
            .join(Path::new(home_base_dir.path()))
            .join(Path::new("first_capsule_user"))
            .join(Path::new("test_path"));
        let _ = create_dir(test_path.as_path());
        let _ = user.create_home_dir(home_base_dir.path().as_os_str());

        assert_eq!(test_path.as_path().exists(), true);
    }
}