use utils::app_config::*;

#[test]
fn fetch_config() {
    // Initialize configuration
    let config_contents = include_str!("resources/test_config.toml");
    AppConfig::init(Some(config_contents)).unwrap();

    // Fetch an instance of Config
    let config = AppConfig::fetch().unwrap();

    // Check the values
    assert_eq!(config.debug, false);
    assert_eq!(config.database.url, "custom database url");
}

#[test]
fn verify_get() {
    // Initialize configuration
    let config_contents = include_str!("resources/test_config.toml");
    AppConfig::init(Some(config_contents)).unwrap();

    // Check value with get
    assert_eq!(AppConfig::get::<bool>("debug").unwrap(), false);
    assert_eq!(
        AppConfig::get::<String>("database.url").unwrap(),
        "new url"
    );
}

#[test]
fn verify_set() {
    // Initialize configuration
    let config_contents = include_str!("resources/test_config.toml");
    AppConfig::init(Some(config_contents)).unwrap();

    // Set a field
    AppConfig::set("database.url", "new url").unwrap();

    // Fetch a new instance of Config
    let config = AppConfig::fetch().unwrap();

    // Check value was modified
    assert_eq!(config.database.url, "new url");
}
