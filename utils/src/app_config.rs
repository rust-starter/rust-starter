use config::{Config, Environment};
use lazy_static::{__Deref, lazy_static};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::RwLock;

use super::error::Result;
use crate::types::LogLevel;

// CONFIG static variable. It's actually an AppConfig
// inside an RwLock.
lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::new());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub debug: bool,
    pub log_level: LogLevel,
    pub database: Database,
}

impl AppConfig {
    /// Initialize AppConfig.
    pub fn init(default_config: Option<&str>) -> Result<()> {
        let mut settings = Config::new();

        // Embed file into executable
        // This macro will embed the configuration file into the
        // executable. Check include_str! for more info.
        if let Some(config_contents) = default_config {
            //let contents = include_str!(config_file_path);
            settings.merge(config::File::from_str(
                config_contents,
                config::FileFormat::Toml,
            ))?;
        }

        // Merge settings with env variables
        settings.merge(Environment::with_prefix("APP"))?; // TODO: Merge settings with Clap Settings Arguments

        // Save Config to RwLoc
        {
            let mut w = CONFIG.write()?;
            *w = settings;
        }

        Ok(())
    }

    pub fn merge_args(app: clap::App) -> Result<()> {
        let args = app.get_matches();

        if args.is_present("debug") {
            AppConfig::set("debug", args.value_of("debug").unwrap())?;
        }

        if args.is_present("log_level") {
            AppConfig::set("log_level", args.value_of("log_level").unwrap())?;
        }

        Ok(())
    }

    pub fn merge_config(config_file: Option<&Path>) -> Result<()> {
        // Merge settings with config file if there is one
        if let Some(config_file_path) = config_file {
            {
                CONFIG
                    .write()?
                    .merge(config::File::with_name(config_file_path.to_str().unwrap()))?;
            }
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
    pub fn get<'de, T>(key: &'de str) -> Result<T>
    where
        T: serde::Deserialize<'de>,
    {
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
