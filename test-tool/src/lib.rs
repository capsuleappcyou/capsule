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
extern crate diesel_migrations;

use diesel::{Connection, PgConnection};
use diesel_migrations::embed_migrations;

#[cfg(feature = "pg")]
embed_migrations!("../capsule-core/migrations/postgres");

#[cfg(feature = "pg")]
pub fn get_test_db_connection() -> PgConnection {
    let connection = establish_connection();

    connection.begin_test_transaction().expect("could not begin test transaction");

    let migration_result = embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    if let Err(e) = migration_result {
        eprintln!("migration error: {}", e);
    }

    connection
}

#[cfg(feature = "pg")]
fn establish_connection() -> PgConnection {
    PgConnection::establish("postgres://postgres:123456@localhost/capsule")
        .expect(&format!("Error connecting to {}", "postgres://postgres:123456@localhost/capsule"))
}
