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
use std::sync::Arc;
use std::time::SystemTime;

use diesel::{ExpressionMethods, insert_into, PgConnection, QueryDsl, RunQueryDsl};
use diesel::associations::HasTable;
use diesel::pg::Pg;
use diesel::result::Error;

use crate::application::{Application, ApplicationVisitor, Updater};
use crate::application::applications::Applications;
use crate::application::implementation::postgres::models::{NewApplication, SavedApplication};
use crate::application::implementation::postgres::schema::capsule_applications;
use crate::application::implementation::postgres::schema::capsule_applications::dsl::*;
use crate::CoreError;

struct PostgresApplications {
    connection: Arc<PgConnection>,
}

impl PostgresApplications {
    pub fn new(connection: Arc<PgConnection>) -> PostgresApplications {
        PostgresApplications { connection }
    }
}

impl From<Error> for CoreError {
    fn from(_: Error) -> Self {
        todo!()
    }
}

impl Applications for PostgresApplications {
    fn add(&self, application: &Application) -> Result<Application, CoreError> {
        let new_application = application.accept(new_application);

        insert_into(capsule_applications::table)
            .values(&new_application)
            .execute(self.connection.as_ref())?;

        Ok(Application {
            id: new_application.application_id,
            name: new_application.application_name.clone(),
            owner: new_application.owner.clone(),
            updater: Some(Box::new(PgUpdater { connection: self.connection.clone() })),
        })
    }
}

fn new_application(app_id: i64, name: &str, app_owner: &str) -> NewApplication {
    NewApplication {
        application_id: app_id,
        application_name: name.to_string(),
        owner: app_owner.to_string(),
        create_at: SystemTime::now(),
    }
}

struct PgUpdater {
    connection: Arc<PgConnection>,
}

impl Updater for PgUpdater {
    fn update(&self, application: &Application) {
        let app_name = application.accept(PgUpdater::get_name);

        diesel::update(capsule_applications.filter(application_name.eq("first_capsule_application".to_string())))
            .set(application_name.eq(app_name))
            .execute(self.connection.as_ref())
            .expect("");
    }
}

impl PgUpdater {
    fn get_name(_:i64, name: &str, _: &str) -> String {
        name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

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
        let connection = Arc::new(get_test_db_connection());

        let applications = PostgresApplications::new(connection.clone());

        let application = Application::new(Some("first_capsule_application".to_string()), "first_capsule_user".to_string());

        applications.add(&application);

        let query_result = capsule_applications
            .filter(application_name.eq("first_capsule_application"))
            .first::<SavedApplication>(connection.as_ref());

        let saved_application = query_result.unwrap();
        assert_eq!(saved_application.application_name, "first_capsule_application".to_string());
        assert_eq!(saved_application.owner, "first_capsule_user".to_string());
    }

    #[test]
    fn should_update_db_if_application_rename() {
        let connection = Arc::new(get_test_db_connection());

        let applications = PostgresApplications::new(connection.clone());

        let application = Application::new(Some("first_capsule_application".to_string()), "first_capsule_user".to_string());

        let mut application = applications.add(&application).expect("save application failed");

        application.rename("new_name");

        let query_result = capsule_applications
            .filter(application_name.eq("new_name"))
            .first::<SavedApplication>(connection.clone().as_ref());

        let saved_application = query_result.unwrap();
        assert_eq!(saved_application.application_name, "new_name".to_string());
        assert_eq!(saved_application.owner, "first_capsule_user".to_string());
    }
}