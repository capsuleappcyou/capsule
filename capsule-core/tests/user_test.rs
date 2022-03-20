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
#[macro_use]
#[cfg(test)]
extern crate diesel_migrations;

use diesel::{Connection, PgConnection};

use capsule_core::user::{PlaintextCredential, PostgresUserFactory, PostgresUserRepository, UserFactory, UserRepository};

embed_migrations!("./migrations/postgres");

#[cfg(test)]
pub(crate) fn establish_connection() -> PgConnection {
    PgConnection::establish("postgres://postgres:123456@localhost/capsule")
        .expect(&format!("Error connecting to {}", "postgres://postgres:123456@localhost/capsule"))
}

#[cfg(test)]
pub(crate) fn get_test_db_connection() -> PgConnection {
    let connection = establish_connection();

    let _ = connection.begin_test_transaction();

    let migration_result = embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    if let Err(e) = migration_result {
        eprintln!("migration error: {}", e);
    }

    connection
}

#[test]
fn should_verify_correct_password() {
    let connection = &get_test_db_connection();
    let user_factory = PostgresUserFactory { connection };
    let user_repository = PostgresUserRepository { connection };

    let mut user = user_factory.create_user("first_capsule_user".to_string());
    let _ = user_repository.add(&user);

    let password = PlaintextCredential { plaintext: "capsule_password".to_string() };
    let _ = user.add_credential(Box::new(password));

    let correct_password = PlaintextCredential { plaintext: "capsule_password".to_string() };
    let password_verify_result = user.verify_credential(Box::new(correct_password));

    assert_eq!(password_verify_result.is_ok(), true);
}

#[test]
fn should_verify_incorrect_password() {
    let connection = &get_test_db_connection();
    let user_factory = PostgresUserFactory { connection };
    let user_repository = PostgresUserRepository { connection };

    let mut user = user_factory.create_user("first_capsule_user".to_string());
    let _ = user_repository.add(&user);

    let password = PlaintextCredential { plaintext: "capsule_password".to_string() };
    let _ = user.add_credential(Box::new(password));

    let correct_password = PlaintextCredential { plaintext: "wrong_capsule_password".to_string() };
    let password_verify_result = user.verify_credential(Box::new(correct_password));

    assert_eq!(password_verify_result.is_ok(), false);
}
