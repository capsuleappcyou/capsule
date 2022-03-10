use mockall::Any;

use crate::user::credential::{Credential, CredentialError};

#[derive(Debug, PartialEq)]
pub struct PwdCredential {
    plaintext: String,
}

impl Credential for PwdCredential {
    fn verify(&self, input_credential: Box<dyn Credential>) -> Result<(), CredentialError> {
        let pwd_credential = input_credential.as_any().downcast_ref::<PwdCredential>();

        match pwd_credential {
            Some(pwd) => {
                return if pwd.plaintext == self.plaintext {
                    Ok(())
                } else {
                    Err(CredentialError)
                };
            }
            _ => Err(CredentialError)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::user::credential::Credential;
    use crate::user::credential::pwd_credential::PwdCredential;

    #[test]
    fn should_ok_given_correct_password_credential() {
        let pwd1 = PwdCredential { plaintext: "password".to_string() };
        let pwd2 = PwdCredential { plaintext: "password".to_string() };

        let result = pwd1.verify(Box::new(pwd2));

        assert_eq!(result.is_ok(), true)
    }
}