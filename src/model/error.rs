use crate::model::store;
use crate::crypt;

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: i64 },
    UserAlreadyExists { username: String },

    // Modules
    // TODO: crypt
    Store(store::Error),
    Crypt(crypt::Error),

    // Externals
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// impl From<store::Error> for Error {
//     fn from(val: store::Error) -> Self {
//         Self::Store(val)
//     }
// }
// store
impl From<store::Error> for Error {
    fn from(val: store::Error) -> Self {
        Self::Store(val)
    }
}
// crypt
impl From<crypt::Error> for Error {
    fn from(val: crypt::Error) -> Self {
        Error::Crypt(val)
    }
}

// sqlx
impl From<sqlx::Error> for Error {
    fn from(val: sqlx::Error) -> Self {
        Error::Sqlx(val)
    }
}

// Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
