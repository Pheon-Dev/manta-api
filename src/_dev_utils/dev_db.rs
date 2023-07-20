// NOTE: Harcode to prevent deployed system db update.
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_MANTA_APP_URL: &str =
	"postgres://manta_app_user:dev_only_password@localhost/manta_app_db";

// sql files
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const DEMO_PASSWORD: &str = "welcome";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
	Ok(())
}
