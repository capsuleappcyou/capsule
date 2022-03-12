use crate::user::credential::Credential;

pub trait Credentials<T: Credential> {
    fn add(self, credential: T);
}