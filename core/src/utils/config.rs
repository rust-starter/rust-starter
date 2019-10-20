use config::{Config, Environment};
use std::ops::Deref;
use std::sync::RwLock;

use super::error::Result;

// CONFIG static variable. It's actually an AppConfig
// inside an RwLock.
lazy_static! {
    static ref CONFIG: RwLock<AppConfig<'static>> = RwLock::new(AppConfig {
        debug: true,
        database: Database { url: "Database URL", }
    });
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Database<'s> {
    url: &'s str,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct AppConfig<'s> {
    debug: bool,
    #[serde(borrow)]
    database: Database<'s>,
}

impl AppConfig<'_> {
    pub fn init(config_file: Option<&str>) -> Result<()> {
        let mut settings = Config::new();

        // Embed file into executable
        // This macro will embed the configuration file into the
        // executable. Check include_str! for more info.
        let contents = include_str!("../resources/default_config.toml");

        // Merge settings with default file and env variables
        settings
            .merge(config::File::from_str(&contents, config::FileFormat::Toml))?
            .merge(Environment::with_prefix("APP"))?;

        // Merge settings with Clap config file if there is one
        if let Some(config_file_path) = config_file {
            settings.merge(config::File::with_name(config_file_path))?;
        }

        // Convert Config into AppConfig
        let app_config = settings.try_into()?;

        // Save config to RwLoc
        {
            let mut w = CONFIG.write()?;
            *w = app_config;
        }

        Ok(())
    }

    // Set CONFIG
    pub fn set(app_config: AppConfig<'static>) -> Result<()> {
        // Save config to RwLoc
        {
            let mut w = CONFIG.write()?;
            *w = app_config;
        }

        Ok(())
    }

    // Get CONFIG
    // This clones CONFIG into a new AppConfig object.
    // This means you have to fetch this again if you changed the configuration.
    pub fn get() -> Result<AppConfig<'static>> {
        // Get a Read Lock from RwLock
        let r = CONFIG.read()?;

        // Deref and Clone the object
        let config = r.deref().clone();

        Ok(config)
    }
}
