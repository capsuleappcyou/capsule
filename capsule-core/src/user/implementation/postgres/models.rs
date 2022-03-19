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

use super::schema::capsule_user_credentials;
use super::schema::capsule_users;

#[derive(Insertable)]
#[table_name = "capsule_users"]
pub struct NewUser {
    pub user_name: String,
    pub create_at: SystemTime,
}

#[derive(Queryable)]
pub struct SavedUser {
    pub id: i32,
    pub user_name: String,
    pub create_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "capsule_user_credentials"]
pub struct NewCapsuleUserCredential {
    pub user_name: String,
    pub credential_name: String,
    pub flat_data: String,
    pub create_at: SystemTime,
}

#[derive(Queryable)]
pub struct SavedCapsuleUserCredential {
    pub id: i32,
    pub user_name: String,
    pub credential_name: String,
    pub flat_data: String,
    pub create_at: SystemTime,
}