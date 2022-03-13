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