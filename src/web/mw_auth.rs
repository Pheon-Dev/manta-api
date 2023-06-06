use crate::crypt::token::{validate_token_sign_and_exp, Token};
// TODO: use crate::model::user::{UserBmc, UserForAuth};
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web;
use crate::web::AUTH_TOKEN;
use crate::web::{Error, Result};

use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::middleware::Next;
use axum::http::request::Parts;
use axum::http::Request;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookies, Cookie};
use tracing::debug;

pub async fn mw_ctx_require<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

// TODO: middleware context resolve
pub async fn mw_ctx_resolve<B>(
    mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>
) -> Result<Response>{
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let token = token
    .ok_or(CtxAuthError::TokenNotInCookie)
        .and_then(|t| Token::parse(&t).map_err(|_| CtxAuthError::TokenWrongFormat));

    // let result_user = match &token{
    //     Ok(token) => {
    //         
    //     }
    // }

    Ok()
}

// endregion: --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "ETRACTOR");

        parts
            .extensions
            .get::<CtxAuthResult<Ctx>>()
            .ok_or(Error::CtxAuth(CtxAuthError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxAuth)
    }
}

// endregion: --- Ctx Extractor

// region: --- Ctx Extractor Result/Error
type CtxAuthResult<T> = core::result::Result<T, CtxAuthError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxAuthError {
    TokenNotInCookie,
    TokenWrongFormat,
    FailUserNotFound(String),
    FailValidate(String),
    CtxNotInRequestExt,
    CtxCreateFail(String),
}
// endregion: --- Ctx Extractor Result/Error
