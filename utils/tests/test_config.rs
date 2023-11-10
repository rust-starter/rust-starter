use std::sync::Once;
use utils::app_config::*;

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        // Initialize configuration
        let config_contents = include_str!("resources/test_config.toml");
        AppConfig::init(Some(config_contents)).unwrap();
    });
}

#[test]
fn fetch_config() {
    initialize();

    // Fetch an instance of Config
    let config = AppConfig::fetch().unwrap();

    // Check the values
    assert_eq!(config.debug, false);
    //dbg!(&config.database.url);
    assert_eq!(config.database.url, "custom database url");
}

#[test]
fn verify_get() {
    initialize();

    // Check value with get
    assert_eq!(AppConfig::get::<bool>("debug").unwrap(), false);
    assert_eq!(
        AppConfig::get::<String>("database.url").unwrap(),
        "custom database url"
    );
}

#[test]
fn verify_set() {
    initialize();

    // Set a field
    AppConfig::set("database.variable", "new value").unwrap();

    // Fetch a new instance of Config
    let config = AppConfig::fetch().unwrap();

    // Check value was modified
    assert_eq!(config.database.variable, "new value");
}
