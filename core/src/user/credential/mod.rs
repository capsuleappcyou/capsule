use std::{error, fmt};
use std::fmt::Debug;

pub mod pwd_credential;

#[derive(Debug, Clone)]
pub struct CredentialError;

impl fmt::Display for CredentialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid credential.")
    }
}

impl error::Error for CredentialError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub trait Credential {
    fn verify(&self, credential: Box<dyn Credential>) -> Result<(), CredentialError>;
}
