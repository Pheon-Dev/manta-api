mod error;

pub use self::error::ClientError;
pub use self::error::{Error, Result};

use crate::crypt::token::generate_token;

use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;
use time::OffsetDateTime;

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: &str) -> Result<()> {
    let token generate_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);

    cookies.add(cookie);

    Ok(())
}

#[derive(Debug, Clone)]
pub struct ReqStamp {
    pub uuid: Uuid,
    pub time_in: OffsetDateTime,
}
