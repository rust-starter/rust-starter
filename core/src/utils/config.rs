use config::{Config, Environment};

use super::error::Result;

#[derive(Debug, Deserialize)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    debug: bool,
    database: Database,
}

impl AppConfig {
    pub fn new(config_file: Option<&str>) -> Result<Self> {
        let contents = include_str!("../resources/default_config.toml");

        let mut settings = Config::new();

        settings
            .merge(config::File::from_str(&contents, config::FileFormat::Toml))?
            .merge(Environment::with_prefix("APP"))?;

        if let Some(config_file_path) = config_file {
            settings.merge(config::File::with_name(config_file_path))?;
        }

        // TODO: Replace this unwrap with a ? implementation
        Ok(settings.try_into().unwrap())
    }
}
