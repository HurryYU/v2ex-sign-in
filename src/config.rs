use anyhow::Context;
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub cookie: String,
    pub email_config: Option<EmailConfig>,
}

#[derive(Debug, Deserialize)]
pub struct EmailConfig {
    pub smtp_url: String,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub notify_from: String,
    pub notify_to: String,
}

impl AppConfig {
    pub fn new() -> anyhow::Result<AppConfig> {
        let settings = Config::builder()
            .add_source(File::with_name("config"))
            .build()
            .context("构建配置失败")?;
        let app_config: AppConfig = settings
            .try_deserialize()
            .context("配置解析失败，请检查config配置")?;
        Ok(app_config)
    }
}
