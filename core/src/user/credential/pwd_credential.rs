use crate::user::credential::{Credential, CredentialError};

#[derive(Debug, PartialEq)]
pub struct PwdCredential {
    plaintext: String,
}

impl Credential for PwdCredential {
    fn verify(&self, input_credential: Box<dyn Credential>) -> Result<(), CredentialError> {
        let pwd_credential = input_credential.downcast_ref::<PwdCredential>();

        match pwd_credential {
            Some(pwd) =>
                if pwd.plaintext == self.plaintext {
                    Ok(())
                } else {
                    Err(CredentialError)
                },
            _ => Err(CredentialError)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::user::credential::{Credential, CredentialError};
    use crate::user::credential::pwd_credential::PwdCredential;

    #[test]
    fn should_ok_given_correct_password_credential() {
        let pwd = PwdCredential { plaintext: "password".to_string() };
        let input_pwd = PwdCredential { plaintext: "password".to_string() };

        let result = pwd.verify(Box::new(input_pwd));

        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn should_error_given_incorrect_password_credential() {
        let pwd = PwdCredential { plaintext: "password".to_string() };
        let input_pwd = PwdCredential { plaintext: "wrong".to_string() };

        let result = pwd.verify(Box::new(input_pwd));

        assert_eq!(result.is_ok(), false);
    }
}