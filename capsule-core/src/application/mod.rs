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
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use crate::CoreErr;

mod repository;

struct Application {
    name: String,
    application_directory: OsString,
}

impl Application {
    fn create_directory(&self) -> Result<Box<Path>, CoreErr> {
        let project_dir = PathBuf::new()
            .join(Path::new(self.application_directory.as_os_str()))
            .join(Path::new(self.name.as_str()));

        let result = create_dir_all(project_dir.as_path());

        match result {
            Ok(()) => Ok(project_dir.into_boxed_path()),
            Err(e) => Err(CoreErr { message: e.to_string() })
        }
    }
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;

    use crate::application::Application;

    #[test]
    fn should_create_application_directory() {
        let project_base_dir = TempDir::new("").unwrap();

        let application = Application { name: "first_application".to_string(), application_directory: project_base_dir.path().as_os_str().to_os_string() };

        let result = application.create_directory();

        assert_eq!(result.is_ok(), true);

        let project_path = result.ok().unwrap();
        assert_eq!(project_path.exists(), true);
        assert_eq!(project_path.to_str().unwrap().to_string().ends_with("first_application"), true);
    }

    // TODO init application with git and install git hooks
}

