use anyhow::Context;
use config::{Config, Environment};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub cookie: String,
}

impl AppConfig {
    pub fn new() -> anyhow::Result<AppConfig> {
        dotenv().ok();
        let settings = Config::builder()
            .add_source(
                Environment::with_prefix("SIGN")
                    .prefix_separator("_")
                    .try_parsing(true)
                    .separator("__"),
            )
            .build()
            .context("构建配置失败")?;
        let app_config: AppConfig = settings
            .try_deserialize()
            .context("配置解析失败，请检查 .env 变量名称类型是否匹配")?;
        Ok(app_config)
    }
}
