use std::time::SystemTime;

use super::schema::capsule_applications;

#[derive(Queryable)]
pub struct SavedApplication {
    pub id: i32,
    pub application_name: String,
    pub owner: String,
    pub create_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "capsule_applications"]
pub struct NewApplication {
    pub application_name: String,
    pub owner: String,
    pub create_at: SystemTime,
}
