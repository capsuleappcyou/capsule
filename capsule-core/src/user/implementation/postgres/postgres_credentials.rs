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
use std::time::SystemTime;

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::PersistenceError;
use crate::user::credential::Credential;
use crate::user::credential::pwd_credential::{Password, PasswordCredential, PlaintextCredential};
use crate::user::credentials::Credentials;
use crate::user::implementation::postgres::models::{NewCapsuleUserCredential, SavedCapsuleUserCredential};
use crate::user::implementation::postgres::postgres_credentials::capsule_user_credentials::dsl::*;
use crate::user::implementation::postgres::schema::capsule_user_credentials;
use crate::user::implementation::postgres::schema::capsule_user_credentials::user_name;

pub(crate) struct PostgresCredentials<'a> {
    pub connection: &'a PgConnection,
    pub user_name: String,
}

impl<'a> Credentials for PostgresCredentials<'a> {
    fn add(&mut self, input_credential: Box<dyn Credential>) -> Result<(), PersistenceError> {
        let credential = input_credential.downcast_ref::<PlaintextCredential>();

        if let Some(c) = credential {
            let password = c.gen_password();

            let new_credential = NewCapsuleUserCredential {
                user_name: self.user_name.clone(),
                credential_name: c.name(),
                flat_data: serde_json::to_string(&password).unwrap(),
                create_at: SystemTime::now(),
            };

            let result = diesel::insert_into(capsule_user_credentials::table)
                .values(&new_credential)
                .execute(*&self.connection);

            return match result {
                Ok(_) => Ok(()),
                Err(e) => Err(PersistenceError { message: e.to_string() })
            };
        }

        Err(PersistenceError { message: "Unsupported credential.".to_string() })
    }

    fn get_credential_by_credential_name(&self, target_name: &str) -> Option<Box<dyn Credential>> {
        match target_name {
            "password" => {
                let saved_credential = capsule_user_credentials
                    .filter(user_name.eq(self.user_name.as_str()))
                    .filter(credential_name.eq(target_name))
                    .first::<SavedCapsuleUserCredential>(self.connection);

                match saved_credential {
                    Ok(c) => {
                        let password = serde_json::from_str::<Password>(c.flat_data.as_str()).unwrap();
                        Some(Box::new(PasswordCredential { password }))
                    }
                    Err(_) => None
                }
            }
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use diesel::{ExpressionMethods, QueryDsl};

    use crate::user::credential::pwd_credential::{PasswordCredential, PlaintextCredential};
    use crate::user::implementation::postgres::get_test_db_connection;
    use crate::user::implementation::postgres::models::SavedCapsuleUserCredential;
    use crate::user::implementation::postgres::postgres_credentials::tests::dsl::capsule_user_credentials;
    use crate::user::implementation::postgres::schema::capsule_user_credentials::*;

    use super::*;

    #[test]
    fn should_add_credential() {
        let connection = &get_test_db_connection();

        let mut credentials = PostgresCredentials {
            connection,
            user_name: String::from("first_capsule_user"),
        };

        let pwd_credential = PlaintextCredential { plaintext: String::from("password") };
        let result = credentials.add(Box::new(pwd_credential));

        assert_eq!(result.is_ok(), true);

        let results: Vec<SavedCapsuleUserCredential> = capsule_user_credentials
            .filter(user_name.eq("first_capsule_user"))
            .load::<SavedCapsuleUserCredential>(connection)
            .expect("Error loading user credential");

        assert_eq!(results.len(), 1);

        let saved_credential = results.get(0).unwrap();

        println!("{}", saved_credential.flat_data);
        assert_eq!(saved_credential.credential_name, String::from("password"));
        assert_eq!(saved_credential.flat_data.len() > 0, true);
        assert_eq!(saved_credential.user_name, String::from("first_capsule_user"));
    }

    #[test]
    fn should_get_by_credential_name() {
        let connection = &get_test_db_connection();

        let mut credentials = PostgresCredentials {
            connection,
            user_name: String::from("first_capsule_user"),
        };

        let pwd_credential = PlaintextCredential { plaintext: String::from("password") };
        let _ = credentials.add(Box::new(pwd_credential));

        let saved_credential = credentials.get_credential_by_credential_name("password").unwrap();
        let credential = saved_credential.downcast_ref::<PasswordCredential>().unwrap();

        assert_ne!(credential.password.salt, 0);
        assert_ne!(credential.password.digest.len(), 0);
    }

    #[test]
    fn should_not_found_unsupported_credential() {
        let connection = &get_test_db_connection();

        let credentials = PostgresCredentials {
            connection,
            user_name: String::from("first_capsule_user"),
        };

        let credential = credentials.get_credential_by_credential_name("not_supported");

        assert_eq!(credential.is_none(), true);
    }

    #[test]
    fn should_not_found_not_exists_credential() {
        let connection = &get_test_db_connection();

        let mut credentials = PostgresCredentials {
            connection,
            user_name: String::from("first_capsule_user"),
        };

        let pwd_credential = PlaintextCredential { plaintext: String::from("password") };
        let _ = credentials.add(Box::new(pwd_credential));

        let credential = credentials.get_credential_by_credential_name("not_exists");

        assert_eq!(credential.is_none(), true);
    }

    #[test]
    fn should_not_add_duplicate_credential() {
        let connection = &get_test_db_connection();

        let mut credentials = PostgresCredentials {
            connection,
            user_name: String::from("first_capsule_user"),
        };

        let pwd_credential = PlaintextCredential { plaintext: String::from("password") };

        let _ = credentials.add(Box::new(pwd_credential));

        let duplicated_credential = PlaintextCredential { plaintext: String::from("password") };
        let result = credentials.add(Box::new(duplicated_credential));

        assert_eq!(result.is_ok(), false);
    }
}