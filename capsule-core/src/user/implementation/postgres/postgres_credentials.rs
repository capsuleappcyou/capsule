use diesel::PgConnection;
use crate::user::credential::Credential;
use crate::user::credentials::Credentials;

pub struct PostgresCredentials<'a> {
    pub(crate) connection: &'a PgConnection,
}

impl<'a> Credentials for PostgresCredentials<'a> {
    fn add(&mut self, _credential: Box<dyn Credential>) {
        todo!()
    }

    fn get_credential_by_name(&self, _name: &str) -> Option<&Box<dyn Credential>> {
        todo!()
    }
}
