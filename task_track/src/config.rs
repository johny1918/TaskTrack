use config::{Config, Environment};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub jwt_exp_seconds: i64,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        let config = Config::builder()
            .add_source(Environment::default())
            .build()?;
        let cfg = config.try_deserialize()?;
        Ok(cfg)
    }
}
