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

use diesel::*;

use crate::user::{User, UserFactory};
use crate::user::credential::Credential;
use crate::user::credentials::Credentials;
use crate::user::repository::postgres::models::NewUser;
use crate::user::repository::UserRepository;

use super::schema::*;

struct ProgressUserRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> UserRepository for ProgressUserRepository<'a> {
    fn add(&self, user: &User) {
        let new_user = NewUser {
            name: &user.user_name.as_str(),
            create_at: SystemTime::now(),
        };

        diesel::insert_into(capsule_users::table)
            .values(&new_user)
            .execute(*&self.connection)
            .unwrap();
    }
}

pub struct PostgresUserFactory;

impl UserFactory for PostgresUserFactory {
    fn create_user(user_name: String) -> User {
        User { user_name, credentials: Box::new(PostgresCredentials {}) }
    }
}

struct PostgresCredentials;

impl Credentials for PostgresCredentials {
    fn add(&mut self, _credential: Box<dyn Credential>) {
        todo!()
    }

    fn get_credential_by_name(&self, _name: &str) -> Option<&Box<dyn Credential>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use diesel::result::Error;

    use crate::diesel::*;
    use crate::user::repository::postgres::models::SavedUser;
    use crate::user::repository::postgres::schema::capsule_users::dsl::*;

    use super::*;

    #[test]
    fn should_add_user() {
        let user = PostgresUserFactory::create_user(String::from("first_capsule_user"));
        let connection = &establish_connection();

        let repository: Box<dyn UserRepository> = Box::new(ProgressUserRepository { connection });

        connection.test_transaction::<_, Error, _>(|| {
            repository.add(&user);

            let results: Vec<SavedUser> = capsule_users
                .filter(name.eq("first_capsule_user"))
                .limit(1)
                .load::<SavedUser>(connection)
                .expect("Error loading users");

            let first_capsule_user: &SavedUser = results.get(0).unwrap();
            assert_eq!(first_capsule_user.name, "first_capsule_user");

            Ok(())
        });
    }

    fn establish_connection() -> PgConnection {
        PgConnection::establish("postgres://postgres:123456@postgres/capsule")
            .expect(&format!("Error connecting to {}", "postgres://postgres:123456@postgres/capsule"))
    }
}