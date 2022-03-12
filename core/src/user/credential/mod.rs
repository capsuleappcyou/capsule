use std::{error, fmt};
use std::fmt::Debug;

use downcast_rs::DowncastSync;

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

pub trait Credential: DowncastSync {
    fn verify(&self, credential: &dyn Credential) -> Result<(), CredentialError>;
}

impl_downcast!(Credential);
