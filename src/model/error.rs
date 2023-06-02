use crate::model::store;

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: i64 },
    UserAlreadyExists { username: String },

    // Modules
    Store(store::Error),

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
