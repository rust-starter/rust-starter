use config::{Config, Environment};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};

use super::error::Result;

// CONFIG static variable. It's actually an AppConfig
// inside an RwLock.
lazy_static! {
    pub static ref CONFIG: Arc<Mutex<Config>> =
        Arc::new(Mutex::new(Config::builder().build().unwrap()));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub debug: bool,
    pub database: Database,
}

impl AppConfig {
    /// Initialize AppConfig.
    pub fn init(default_config: Option<&str>) -> Result<()> {
        let mut settings = Config::builder().add_source(Environment::with_prefix("APP"));

        // Embed file into executable
        // This macro will embed the configuration file into the
        // executable. Check include_str! for more info.
        if let Some(config_contents) = default_config {
            //let contents = include_str!(config_file_path);
            let default_source = config::File::from_str(config_contents, config::FileFormat::Toml);
            settings = settings.add_source(default_source);
        }

        let config_build = settings.build()?;

        // TODO: Merge settings with Clap Settings Arguments

        // Save Config to RwLoc
        let mut w = CONFIG.lock()?;
        *w = config_build;
        drop(w);

        Ok(())
    }

    pub fn merge_args(app: &mut clap::App) -> Result<()> {
        dbg!(CONFIG.lock()?.clone().deserialize::<Vec<String>>());

        Ok(())
    }

    pub fn merge_config(config_file: Option<&Path>) -> Result<()> {
        // Merge settings with config file if there is one
        if let Some(config_file_path) = config_file {
            {
                CONFIG
                    .lock()?
                    .merge(config::File::with_name(config_file_path.to_str().unwrap()))?;
            }
        }
        Ok(())
    }

    // Set CONFIG
    pub fn set(key: &str, value: &str) -> Result<()> {
        // Set Property
        let mut guard = CONFIG.lock()?;
        guard.set(key, value)?;
        drop(guard);

        Ok(())
    }

    // Get a single value
    pub fn get<'de, T>(key: &'de str) -> Result<T>
    where
        T: serde::Deserialize<'de>,
    {
        let guard = CONFIG.lock()?;
        let value = guard.get::<T>(key)?;
        drop(guard);
        Ok(value)
    }

    // Get CONFIG
    // This clones Config (from RwLock<Config>) into a new AppConfig object.
    // This means you have to fetch this again if you changed the configuration.
    pub fn fetch() -> Result<AppConfig> {
        // Get a Read Lock from RwLock
        let r = CONFIG.lock()?;

        // Clone the Config object
        let config_clone = r.deref().clone();

        drop(r);

        // Coerce Config into AppConfig
        Ok(config_clone.try_deserialize().unwrap())
    }
}
