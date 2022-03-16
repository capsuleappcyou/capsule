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
use std::{error, fmt};
use std::fmt::Debug;

use downcast_rs::DowncastSync;

pub mod pwd_credential;

#[derive(Debug, Clone)]
pub struct CredentialError;

impl fmt::Display for CredentialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid credential.")
    }
}

impl error::Error for CredentialError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub trait Credential: DowncastSync {
    fn verify(&self, credential: &dyn Credential) -> Result<(), CredentialError>;
    fn name(&self) -> String;
}

impl_downcast!(Credential);