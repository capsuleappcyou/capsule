use crate::user::credential::Credential;

pub trait Credentials<T>
    where T: Credential {
    fn add(self, credential: Box<T>);
}

// impl <T> Debug for dyn UserCredentials<T> {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "UNIMPLEMENTED")
//     }
// }