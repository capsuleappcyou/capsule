use crate::user::credential::Credential;

pub trait Credentials {
    fn add(&mut self, credential: Box<dyn Credential>);
}