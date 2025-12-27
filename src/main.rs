use crate::config::AppConfig;
use crate::sign_in::auto_sign_in;

mod config;
mod sign_in;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::new()?;
    auto_sign_in(&config).await?;
    Ok(())
}
