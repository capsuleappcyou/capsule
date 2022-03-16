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
        let credential = self.credentials.get_credential_by_name(input_credential.name().as_str());

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
    use crate::user::credential::{Credential, CredentialError};
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

        fn get_credential_by_name(&self, _name: &str) -> Option<&Box<dyn Credential>> {
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
        let user = TestUserFactory::create_user(String::from("test"));

        assert_eq!(user.user_name, String::from("test"))
    }

    #[test]
    fn should_verify_password_given_correct_credential() {
        let mut user = TestUserFactory::create_user(String::from("test"));

        let password = Box::new(PwdCredential { plaintext: String::from("password") });
        user.add_credential(password);

        let input_password = Box::new(PwdCredential { plaintext: String::from("password") });
        let verify_result = user.verify_credential(input_password);

        assert_eq!(verify_result.is_ok(), true);
    }

    #[test]
    fn should_verify_password_given_incorrect_credential() {
        let mut user = TestUserFactory::create_user(String::from("test"));

        let password = Box::new(PwdCredential { plaintext: String::from("password") });
        user.add_credential(password);

        let wrong_password = Box::new(PwdCredential { plaintext: String::from("wrong password") });
        let verify_result = user.verify_credential(wrong_password);

        assert_eq!(verify_result.is_err(), true);
    }

    #[test]
    fn should_verify_password_given_unsupported_credential() {
        let mut user = TestUserFactory::create_user(String::from("test"));

        let password = Box::new(PwdCredential { plaintext: String::from("password") });
        user.add_credential(password);

        let unsupported_credential = Box::new(UnSupportedCredential {});
        let verify_result = user.verify_credential(unsupported_credential);

        assert_eq!(verify_result.is_err(), true);
    }
}