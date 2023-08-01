use crate::crypt::password::{self};
use crate::crypt::EncryptContent;
use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::{Error, ModelManager, Result};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

// region: --- User Types
#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User {
	pub id: i64,
	pub username: String,
	pub email: String,

	pub cid: i64,

	// -- Timestamps
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,

	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct UserForCreate {
	pub username: String,
	pub email: String,
	pub password_clear: String,
}

#[derive(Deserialize, Fields)]
pub struct UserForUpdate {
	pub username: Option<String>,
	pub email: Option<String>,
	pub password_clear: Option<String>,
}

#[derive(Fields)]
pub struct UserForInsert {
	username: String,
	email: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
	pub id: i64,
	pub username: String,
	pub email: String,

	// -- password and token info
	pub password: Option<String>,
	pub password_salt: Uuid,
	pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
	pub id: i64,
	pub username: String,
	pub email: String,

	// -- token info
	pub token_salt: Uuid,
}

// Marker Trait
pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

// endregion: -- User types

pub struct UserBmc;

impl DbBmc for UserBmc {
	const TABLE: &'static str = "user";
	const HAS_TIMESTAMPS: bool = true;
}

impl UserBmc {
	#[allow(unused)]
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		user_c: UserForCreate,
	) -> Result<i64> {
		let UserForCreate { username, email, password_clear } = user_c;

		let user_fi = UserForInsert {
			username: username.to_string(),
			email: email.to_string(),
		};

		let user_id = base::create::<Self, _>(ctx, mm, user_fi).await.map_err(
			|model_error| match model_error {
				Error::Sqlx(sqlx_error) => {
					if let Some((code, constraint)) =
						sqlx_error.as_database_error().and_then(|db_error| {
							db_error.code().zip(db_error.constraint())
						}) {
						if code == "23505"
							&& (constraint == "user_username_key"
								|| constraint == "user_username_norm_key")
						{
							return Error::UserAlreadyExists { username };
						}
					}
					Error::Sqlx(sqlx_error)
				}
				_ => model_error,
			},
		)?;
		Self::update_password(ctx, mm, user_id, &password_clear).await?;

		Ok(user_id)
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		user_u: UserForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, user_u).await
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<User>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}

	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
	where
		E: UserBy,
	{
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_username<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		username: &str,
	) -> Result<Option<E>>
	where
		E: UserBy,
	{
		let db = mm.db();

		let user = sqlb::select()
			.table(Self::TABLE)
			.and_where("username", "=", username)
			.fetch_optional::<_, E>(db)
			.await?;

		Ok(user)
	}

	pub async fn first_by_email<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		email: &str,
	) -> Result<Option<E>>
	where
		E: UserBy,
	{
		let db = mm.db();

		let user = sqlb::select()
			.table(Self::TABLE)
			.and_where("email", "=", email)
			.fetch_optional::<_, E>(db)
			.await?;

		Ok(user)
	}

	pub async fn update_password(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		password_clear: &str,
	) -> Result<()> {
		let db = mm.db();

		let user: UserForLogin = Self::get(ctx, mm, id).await?;

		let password = password::encrypt_password(&EncryptContent {
			content: password_clear.to_string(),
			salt: user.password_salt.to_string(),
		})?;

		sqlb::update()
			.table(Self::TABLE)
			.and_where("id", "=", id)
			.data(vec![("password", password.to_string()).into()])
			.exec(db)
			.await?;

		Ok(())
	}

	pub async fn update_email(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		email: &str,
	) -> Result<()> {
		let db = mm.db();

		let _user: UserForLogin = Self::get(ctx, mm, id).await?;

		sqlb::update()
			.table(Self::TABLE)
			.and_where("id", "=", id)
			.data(vec![("email", email.to_string()).into()])
			.exec(db)
			.await?;

		Ok(())
	}
}

// TODO: Tests
