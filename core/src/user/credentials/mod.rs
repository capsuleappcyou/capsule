use crate::user::credential::Credential;

pub trait Credentials {
    fn add(&mut self, credential: Box<dyn Credential>);

    fn get_credential_by_name(&self, name: String) -> Option<&Box<dyn Credential>>;
}