mod config;
mod error;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}
