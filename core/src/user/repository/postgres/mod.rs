use diesel::pg::PgConnection;
use diesel::prelude::*;

mod schema;
mod models;

pub fn _establish_connection() -> PgConnection {
    PgConnection::establish("postgres://postgres:123456@127.0.0.1/capsule")
        .expect(&format!("Error connecting to {}", "postgres://postgres:123456@127.0.0.1/capsule"))
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use chrono::{DateTime, FixedOffset, NaiveDateTime};
    use diesel::debug_query;
    use diesel::pg::Pg;

    use crate::user::repository::postgres::models::NewUser;

    use super::schema::capsule_users;

    #[test]
    fn should_execute_insert_sql() {
        let new_user = NewUser {
            name: "test",
            create_at: get_system_time_from_str("2022-03-15 22:05:04"),
        };

        let insert_statement = diesel::insert_into(capsule_users::table).values(&new_user);

        let expect_sql = r#"INSERT INTO "capsule_users" ("name", "create_at") VALUES ($1, $2) -- binds: ["test", SystemTime { tv_sec: 1647381904, tv_nsec: 0 }]"#;
        assert_eq!(expect_sql, debug_query::<Pg, _>(&insert_statement).to_string());
    }

    fn get_system_time_from_str(s: &str) -> SystemTime {
        let datetime = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap();
        SystemTime::from(DateTime::<FixedOffset>::from_utc(datetime, FixedOffset::east(8)))
    }
}