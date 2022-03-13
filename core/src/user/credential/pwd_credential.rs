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
use crate::user::credential::{Credential, CredentialError};

#[derive(Debug, PartialEq)]
pub struct PwdCredential {
    pub plaintext: String,
}

impl Credential for PwdCredential {
    fn verify(&self, input_credential: &dyn Credential) -> Result<(), CredentialError> {
        let pwd_credential = input_credential.downcast_ref::<PwdCredential>();

        match pwd_credential {
            Some(p) if p.plaintext == self.plaintext => {
                Ok(())
            }
            _ => Err(CredentialError)
        }
    }

    fn name(&self) -> String {
        String::from("password")
    }
}

#[cfg(test)]
mod tests {
    use crate::user::credential::Credential;
    use crate::user::credential::pwd_credential::PwdCredential;

    #[test]
    fn should_ok_given_correct_password_credential() {
        let pwd = PwdCredential { plaintext: "password".to_string() };
        let input_pwd = PwdCredential { plaintext: "password".to_string() };

        let result = pwd.verify(&input_pwd);

        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn should_error_given_incorrect_password_credential() {
        let pwd = PwdCredential { plaintext: "password".to_string() };
        let input_pwd = PwdCredential { plaintext: "wrong".to_string() };

        let result = pwd.verify(&input_pwd);

        assert_eq!(result.is_ok(), false);
    }
}