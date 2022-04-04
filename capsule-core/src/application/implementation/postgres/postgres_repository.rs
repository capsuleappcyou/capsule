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
use std::ffi::OsString;
use std::time::SystemTime;

use diesel::*;
use diesel::PgConnection;
use diesel::result::Error;

use crate::application::Application;
use crate::application::implementation::postgres::models::{NewApplication, SavedApplication};
use crate::application::implementation::postgres::schema::capsule_applications;
use crate::application::implementation::postgres::schema::capsule_applications::dsl::*;
use crate::application::repository::ApplicationRepository;
use crate::CoreError;

impl From<diesel::result::Error> for CoreError {
    fn from(e: Error) -> Self {
        CoreError { message: e.to_string() }
    }
}

pub struct PostgresApplicationRepository<'a> {
    pub connection: &'a PgConnection,
}

impl<'a> ApplicationRepository for PostgresApplicationRepository<'a> {
    fn add(&self, application: &Application) -> Result<(), CoreError> {
        let new_application = NewApplication {
            application_name: application.name.clone(),
            owner: application.owner.clone(),
            application_directory: application.application_directory.to_str().unwrap().to_string(),
            create_at: SystemTime::now(),
        };

        diesel::insert_into(capsule_applications::table)
            .values(&new_application)
            .execute(*&self.connection)?;

        Ok(())
    }

    fn find_by_name(&self, name: &str) -> Result<Option<Application>, CoreError> {
        let query_result = capsule_applications
            .filter(application_name.eq(name))
            .first::<SavedApplication>(*&self.connection)
            .optional()?
            .or_else(|| None);

        match query_result {
            Some(saved_application) => {
                let application = Application {
                    name: saved_application.application_name,
                    owner: saved_application.owner,
                    application_directory: OsString::from(saved_application.application_directory),
                };
                Ok(Some(application))
            }
            None => Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use diesel::*;
    use tempdir::TempDir;

    use test_tool::get_test_db_connection;

    use crate::application::Application;
    use crate::application::implementation::postgres::models::SavedApplication;
    use crate::application::implementation::postgres::postgres_repository::PostgresApplicationRepository;
    use crate::application::implementation::postgres::schema::capsule_applications::dsl::*;
    use crate::application::repository::ApplicationRepository;

    #[test]
    fn should_save_application_to_db() {
        let connection = &get_test_db_connection();

        let repository: Box<dyn ApplicationRepository> = Box::new(PostgresApplicationRepository { connection });

        let application = create_application();

        let result = repository.add(&application);

        assert_eq!(result.is_ok(), true);

        let query_result = capsule_applications
            .filter(application_name.eq("first_capsule_application"))
            .first::<SavedApplication>(connection);

        let saved_application = query_result.unwrap();
        assert_eq!(saved_application.application_name, "first_capsule_application".to_string());
        assert_eq!(saved_application.owner, "first_capsule_user".to_string());
        assert_eq!(saved_application.application_directory, application.application_directory.as_os_str().to_str().unwrap().to_string());
    }

    #[test]
    fn should_find_application_by_name() {
        let connection = &get_test_db_connection();

        let repository: Box<dyn ApplicationRepository> = Box::new(PostgresApplicationRepository { connection });

        let new_application = create_application();

        let _ = repository.add(&new_application);

        let application = repository.find_by_name("first_capsule_application").unwrap().unwrap();

        assert_eq!(application.name, "first_capsule_application".to_string());
        assert_eq!(application.owner, "first_capsule_user".to_string());
        assert_eq!(application.application_directory, new_application.application_directory);
    }

    #[test]
    fn should_not_find_application_if_application_present() {
        let connection = &get_test_db_connection();

        let repository: Box<dyn ApplicationRepository> = Box::new(PostgresApplicationRepository { connection });

        let application = repository.find_by_name("first_capsule_application");

        assert_eq!(application.unwrap().is_none(), true);
    }

    fn create_application() -> Application {
        let temp_dir = TempDir::new("").unwrap();

        Application {
            name: "first_capsule_application".to_string(),
            owner: "first_capsule_user".to_string(),
            application_directory: OsString::from(temp_dir.path().as_os_str()),
        }
    }
}