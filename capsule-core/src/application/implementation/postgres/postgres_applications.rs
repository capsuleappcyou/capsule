use std::thread::sleep;
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

use diesel::{insert_into, PgConnection, RunQueryDsl};
use diesel::associations::HasTable;
use diesel::pg::Pg;

use crate::application::{Application, ApplicationVisitor, Updater};
use crate::application::applications::Applications;
use crate::application::implementation::postgres::models::{NewApplication, SavedApplication};
use crate::application::implementation::postgres::schema::capsule_applications;
use crate::application::implementation::postgres::schema::capsule_applications::dsl::*;
use crate::CoreError;

struct PostgresApplications<'a> {
    connection: &'a PgConnection,
}

impl<'a> PostgresApplications<'a> {
    pub fn new(connection: &'a PgConnection) -> PostgresApplications {
        PostgresApplications { connection }
    }
}

impl<'a> Applications for PostgresApplications<'a> {
    fn add(&self, application: &Application) -> Result<Application, CoreError> {
        application.accept(persist).save(self.connection)
    }
}

fn persist(name: &str, app_owner: &str) -> PgSaver {
    PgSaver { name: name.to_string(), owner: app_owner.to_string() }
}

struct PgUpdater {}

impl Updater for PgUpdater {
    fn update(&self, application: &Application) {
        todo!()
    }
}

struct PgSaver {
    name: String,
    owner: String,
}

impl PgSaver {
    pub fn save(&self, connection: &PgConnection) -> Result<Application, CoreError> {
        let new_application = NewApplication {
            application_name: self.name.clone(),
            owner: self.owner.clone(),
            create_at: SystemTime::now(),
        };

        insert_into(capsule_applications::table)
            .values(&new_application)
            .execute(connection)?;

        Ok(Application {
            name: self.name.to_string(),
            owner: self.owner.to_string(),
            updater: Some(Box::new(PgUpdater {})),
        })
    }
}

#[cfg(test)]
mod tests {
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

    use test_tool::get_test_db_connection;

    use crate::application::Application;
    use crate::application::applications::Applications;
    use crate::application::implementation::postgres::models::SavedApplication;
    use crate::application::implementation::postgres::postgres_applications::PostgresApplications;
    use crate::application::implementation::postgres::schema::capsule_applications::application_name;
    use crate::application::implementation::postgres::schema::capsule_applications::dsl::capsule_applications;

    #[test]
    fn should_save_application_to_db() {
        let connection = &get_test_db_connection();

        let applications = PostgresApplications::new(connection);

        let application = Application::new(Some("first_capsule_application".to_string()), "first_capsule_user".to_string());

        applications.add(&application);

        let query_result = capsule_applications
            .filter(application_name.eq("first_capsule_application"))
            .first::<SavedApplication>(connection);

        let saved_application = query_result.unwrap();
        assert_eq!(saved_application.application_name, "first_capsule_application".to_string());
        assert_eq!(saved_application.owner, "first_capsule_user".to_string());
    }

    #[test]
    fn should_update_db_if_application_rename() {
        let connection = &get_test_db_connection();

        let applications = PostgresApplications::new(connection);

        let application = Application::new(Some("first_capsule_application".to_string()), "first_capsule_user".to_string());

        let mut application = applications.add(&application).expect("save application failed");

        // application.rename("new_name");

        // let query_result = capsule_applications
        //     .filter(application_name.eq("new_name"))
        //     .first::<SavedApplication>(connection);
        //
        // let saved_application = query_result.unwrap();
        // assert_eq!(saved_application.application_name, "new_name".to_string());
        // assert_eq!(saved_application.owner, "first_capsule_user".to_string());
    }
}