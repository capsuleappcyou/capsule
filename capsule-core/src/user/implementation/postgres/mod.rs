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
use diesel::PgConnection;

use crate::user::{User, UserFactory};
use crate::user::implementation::postgres::postgres_credentials::PostgresCredentials;

mod schema;
mod models;
pub mod postgres_repository;
pub(crate) mod postgres_credentials;

pub struct PostgresUserFactory<'a> {
    pub connection: &'a PgConnection,
}

impl<'a> UserFactory for PostgresUserFactory<'a> {
    fn create_user(&self, new_user_name: String) -> User {
        User {
            user_name: new_user_name.clone(),
            credentials: Box::new(
                PostgresCredentials {
                    connection: self.connection,
                    user_name: new_user_name,
                }),
        }
    }
}
