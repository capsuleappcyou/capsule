use std::time::SystemTime;

use super::schema::capsule_users;

#[derive(Insertable)]
#[table_name = "capsule_users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub create_at: SystemTime,
}

#[derive(Queryable)]
pub struct SavedUser {
    pub id: i32,
    pub name: String,
    pub create_at: SystemTime,
}