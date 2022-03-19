use std::time::SystemTime;

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
use diesel::{PgConnection, RunQueryDsl};

use crate::PersistenceError;
use crate::user::credential::Credential;
use crate::user::credentials::Credentials;
use crate::user::implementation::postgres::models::NewCapsuleUserPasswordCredential;
use crate::user::implementation::postgres::schema::capsule_user_password_credentials;

pub struct PostgresCredentials<'a> {
    pub(crate) connection: &'a PgConnection,
}

impl<'a> Credentials for PostgresCredentials<'a> {
    fn add(&mut self, _credential: Box<dyn Credential>) -> Result<(), PersistenceError> {
        let new_credential = NewCapsuleUserPasswordCredential {
            user_name: "test".to_string(),
            hash_value: String::from("dummy"),
            salt: 12,
            create_at: SystemTime::now(),
        };

        let result = diesel::insert_into(capsule_user_password_credentials::table)
            .values(&new_credential)
            .execute(*&self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PersistenceError { message: e.to_string() })
        }
    }

    fn get_credential_by_credential_name(&self, _name: &str) -> Option<&Box<dyn Credential>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use diesel::{ExpressionMethods, QueryDsl};

    use crate::user::credential::pwd_credential::PwdCredential;
    use crate::user::implementation::postgres::get_test_db_connection;
    use crate::user::implementation::postgres::models::SavedCapsuleUserPasswordCredential;
    use crate::user::implementation::postgres::postgres_credentials::tests::dsl::capsule_user_password_credentials;
    use crate::user::implementation::postgres::schema::capsule_user_password_credentials::*;

    use super::*;

    #[test]
    fn should_add_credential() {
        let connection = &get_test_db_connection();

        let mut credentials = PostgresCredentials { connection };

        let pwd_credential = PwdCredential { plaintext: String::from("password") };
        let result = credentials.add(Box::new(pwd_credential));

        assert_eq!(result.is_ok(), true);

        let results: Vec<SavedCapsuleUserPasswordCredential> = capsule_user_password_credentials
            .filter(user_name.eq("test"))
            .load::<SavedCapsuleUserPasswordCredential>(connection)
            .expect("Error loading user credential");

        assert_eq!(results.len(), 1);

        let saved_credential = results.get(0).unwrap();
        assert_eq!(saved_credential.hash_value, String::from("dummy"));
        assert_eq!(saved_credential.salt, 12);
        assert_eq!(saved_credential.user_name, String::from("test"));
    }
}