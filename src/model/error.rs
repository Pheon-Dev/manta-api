use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
	EntityNotFound { entity: &'static str, id: i64 },
	UserAlreadyExists { username: String },

	// -- Externals
	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

impl From<sqlx::Error> for Error {
	fn from(val: sqlx::Error) -> Self {
		Error::Sqlx(val)
	}
}

impl std::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut std::fmt::Formatter,
	) -> core::result::Result<(), std::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
