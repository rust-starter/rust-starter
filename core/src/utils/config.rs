use config::{Config, Environment};
use std::ops::Deref;
use std::sync::RwLock;

use super::error::Result;

// CONFIG static variable. It's actually an AppConfig
// inside an RwLock.
lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::new());
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub debug: bool,
    pub database: Database,
}

impl AppConfig {
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

        // TODO: Merge settings with Clap Settings Arguments

        // Save Config to RwLoc
        {
            let mut w = CONFIG.write()?;
            *w = settings;
        }

        Ok(())
    }

    // Set CONFIG
    pub fn set(key: &str, value: &str) -> Result<()> {
        {
            // Set Property
            CONFIG.write()?.set(key, value)?;
        }

        Ok(())
    }

    // Get a single value
    pub fn get<'de, T>(key: &'de str) -> Result<T> where T: serde::Deserialize<'de> {
        Ok(CONFIG.read()?.get::<T>(key)?)
    }

    // Get CONFIG
    // This clones Config (from RwLock<Config>) into a new AppConfig object.
    // This means you have to fetch this again if you changed the configuration.
    pub fn fetch() -> Result<AppConfig> {
        // Get a Read Lock from RwLock
        let r = CONFIG.read()?;

        // Clone the Config object
        let config_clone = r.deref().clone();

        // Coerce Config into AppConfig
        Ok(config_clone.try_into()?)
    }
}
