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
use crate::user::repository::postgres::models::{NewUser, SavedUser};
use crate::user::repository::postgres::schema::capsule_users;
use crate::user::repository::postgres::schema::capsule_users::dsl::*;
use crate::user::repository::postgres::schema::capsule_users::name;
use crate::user::repository::UserRepository;

struct PostgresUserRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> UserRepository for PostgresUserRepository<'a> {
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

    fn find_by_user_name(&self, user_name: &str) -> Option<User> {
        let query_result = capsule_users
            .filter(name.eq(user_name))
            .first::<SavedUser>(*&self.connection);

        match query_result {
            Ok(saved_user) => {
                let user = User { user_name: saved_user.name, credentials: Box::new(PostgresCredentials { connection: self.connection }) };

                Some(user)
            }
            Err(_) => None
        }
    }
}

pub struct PostgresUserFactory<'a> {
    connection: &'a PgConnection,
}

impl<'a> UserFactory for PostgresUserFactory<'a> {
    fn create_user(&self, user_name: String) -> User {
        User { user_name, credentials: Box::new(PostgresCredentials { connection: self.connection }) }
    }
}

struct PostgresCredentials<'a> {
    connection: &'a PgConnection,
}

impl<'a> Credentials for PostgresCredentials<'a> {
    fn add(&mut self, _credential: Box<dyn Credential>) {
        todo!()
    }

    fn get_credential_by_name(&self, _name: &str) -> Option<&Box<dyn Credential>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use diesel_migrations::embed_migrations;

    use crate::diesel::*;
    use crate::user::repository::postgres::models::SavedUser;
    use crate::user::repository::postgres::schema::capsule_users::dsl::*;

    use super::*;

    embed_migrations!("./migrations");

    fn get_test_db_connection() -> PgConnection {
        let connection = establish_connection();

        let _ = connection.begin_test_transaction();

        let _ = embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

        connection
    }

    fn establish_connection() -> PgConnection {
        PgConnection::establish("postgres://postgres:123456@localhost/capsule")
            .expect(&format!("Error connecting to {}", "postgres://postgres:123456@localhost/capsule"))
    }

    #[test]
    fn should_add_user() {
        let connection = &get_test_db_connection();

        let user_factory = PostgresUserFactory { connection };

        let user = user_factory.create_user(String::from("first_capsule_user"));

        let repository: Box<dyn UserRepository> = Box::new(PostgresUserRepository { connection });

        repository.add(&user);

        let results: Vec<SavedUser> = capsule_users
            .filter(name.eq("first_capsule_user"))
            .limit(1)
            .load::<SavedUser>(connection)
            .expect("Error loading users");

        let first_capsule_user: &SavedUser = results.get(0).unwrap();
        assert_eq!(first_capsule_user.name, "first_capsule_user");
    }

    #[test]
    fn should_find_user_by_user_name() {
        let connection = &get_test_db_connection();

        let user_factory = PostgresUserFactory { connection };

        let user = user_factory.create_user(String::from("first_capsule_user"));

        let repository: Box<dyn UserRepository> = Box::new(PostgresUserRepository { connection });

        repository.add(&user);

        let first_capsule_user = repository.find_by_user_name("first_capsule_user").unwrap();

        assert_eq!(first_capsule_user.user_name, "first_capsule_user");
    }
}