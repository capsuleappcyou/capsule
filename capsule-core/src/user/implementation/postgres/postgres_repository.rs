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
use crate::user::implementation::postgres::models::{NewUser, SavedUser};
use crate::user::implementation::postgres::postgres_credentials::PostgresCredentials;
use crate::user::implementation::postgres::schema::capsule_users;
use crate::user::implementation::postgres::schema::capsule_users::dsl::*;
use crate::user::implementation::postgres::schema::capsule_users::user_name;
use crate::user::repository::UserRepository;

struct PostgresUserRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> UserRepository for PostgresUserRepository<'a> {
    fn add(&self, user: &User) {
        let new_user = NewUser {
            user_name: user.user_name.clone(),
            create_at: SystemTime::now(),
        };

        diesel::insert_into(capsule_users::table)
            .values(&new_user)
            .execute(*&self.connection)
            .unwrap();
    }

    fn find_by_user_name(&self, target_user_name: &str) -> Option<User> {
        let query_result = capsule_users
            .filter(user_name.eq(&target_user_name.to_string()))
            .first::<SavedUser>(*&self.connection);

        match query_result {
            Ok(saved_user) => {
                let user = User { user_name: saved_user.user_name, credentials: Box::new(PostgresCredentials { connection: self.connection }) };

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
    fn create_user(&self, new_user_name: String) -> User {
        User { user_name: new_user_name, credentials: Box::new(PostgresCredentials { connection: self.connection }) }
    }
}

#[cfg(test)]
mod tests {
    use crate::diesel::*;
    use crate::user::implementation::postgres::get_test_db_connection;
    use crate::user::implementation::postgres::models::SavedUser;
    use crate::user::implementation::postgres::schema::capsule_users::dsl::*;

    use super::*;

    #[test]
    fn should_add_user() {
        let connection = &get_test_db_connection();

        let user_factory = PostgresUserFactory { connection };

        let user = user_factory.create_user(String::from("first_capsule_user"));

        let repository: Box<dyn UserRepository> = Box::new(PostgresUserRepository { connection });

        repository.add(&user);

        let results: Vec<SavedUser> = capsule_users
            .filter(user_name.eq("first_capsule_user"))
            .limit(1)
            .load::<SavedUser>(connection)
            .expect("Error loading users");

        let first_capsule_user: &SavedUser = results.get(0).unwrap();
        assert_eq!(first_capsule_user.user_name, "first_capsule_user");
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

    #[test]
    fn should_not_find_user() {
        let connection = &get_test_db_connection();

        let repository: Box<dyn UserRepository> = Box::new(PostgresUserRepository { connection });

        let first_capsule_user = repository.find_by_user_name("first_capsule_user");

        assert_eq!(first_capsule_user.is_none(), true);
    }
}