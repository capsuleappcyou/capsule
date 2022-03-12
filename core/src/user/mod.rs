use std::marker::PhantomData;

use crate::user::credential::Credential;
use crate::user::credentials::Credentials;

pub mod credential;
pub mod credentials;
pub mod repository;

struct User<T> {
    pub user_name: String,
    credentials: Box<dyn Credentials<T>>,
    phantom: PhantomData<T>,
}

impl<T> User<T> {
    pub fn new(user_name: String, credentials: Box<dyn Credentials<T>>) -> Self {
        User {
            user_name,
            credentials,
            phantom: Default::default(),
        }
    }
}

// impl<T> User<T>
//     where T: Credential {
//     pub fn add_credential(&self, credential: T) {
//         // self.credentials.add(credential)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::user::credential::Credential;
    use crate::user::credential::pwd_credential::PwdCredential;
    use crate::user::credentials::Credentials;
    use crate::user::User;

    struct FakeCredentials {
        credentials: Vec<Box<dyn Credential>>,
    }

    impl FakeCredentials {
        pub fn new() -> Self {
            FakeCredentials { credentials: vec![] }
        }
    }

    impl<T: Credential> Credentials<T> for FakeCredentials {
        fn add(mut self, credential: T) {
            self.credentials.push(Box::new(credential))
        }
    }

    #[test]
    fn should_create_user() {
        let user = User::<PwdCredential>::new(
            String::from("test"),
            Box::new(FakeCredentials::new()));

        assert_eq!(user.user_name, String::from("test"))
    }
}