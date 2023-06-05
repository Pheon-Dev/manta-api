use crate::{crypt, model, web};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // TODO: RPC
    // login
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPwd {user_id: i64},
    LoginFailPwdNotMatching {user_id: i64},

    // TODO: middleware/extractor
    // TODO: ctxAuthError
    // TODO: modules
    Model(model::Error),
    Crypt(crypt::Error),

    // TODO: external modules
}

impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Error::Model(val)
    }
}

impl core::fmt::Display for Error {
    fn fmt (
        &self,
        fmt: &mut core::fmt::Formatter,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use web::Error::*;

        match self {

            LoginFailUsernameNotFound
            | LoginFailUserHasNoPwd { .. }
            | LoginFailPwdNotMatching { .. } => {
                    (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
                }

            // Fallback
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),
        }
    }
}

/// This is the ClientError use to be serialised in the 
/// json-rpc error body.
/// Only used in the mw_res_mapper
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    USER_ALREADY_EXISTS,
    LOGIN_FAIL,
    NO_AUTH,
    EntityNotFound { entity: &'static str, id: i64},
    SERVICE_ERROR,
}
