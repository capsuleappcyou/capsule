use std::fmt::Debug;
use std::marker::PhantomData;

use crate::user::credential::Credential;
use crate::user::credentials::Credentials;

pub mod credential;
pub mod credentials;
pub mod repository;

#[derive(Debug)]
pub struct User<A, B>
    where A: Credentials<B>,
          B: Credential {
    pub user_name: String,
    credentials: Box<A>,
    phantom: PhantomData<B>,
}

impl<A, B> User<A, B>
    where A: Credentials<B>,
          B: Credential {
    pub fn add_credential(self, credential: Box<B>) {
        self.credentials.add(credential)
    }
}