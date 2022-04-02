#[macro_use]
extern crate diesel_migrations;

use diesel::{Connection, PgConnection};
use diesel_migrations::embed_migrations;

#[cfg(feature = "pg")]
embed_migrations!("../capsule-core/migrations/postgres");

#[cfg(feature = "pg")]
pub fn get_test_db_connection() -> PgConnection {
    let connection = establish_connection();

    let _ = connection.begin_test_transaction();

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
