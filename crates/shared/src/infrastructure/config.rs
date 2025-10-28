use serde::Deserialize;

use crate::infrastructure::env::Environment;

pub fn get_config<'de, C>() -> Result<C, config::ConfigError>
where
    C: Deserialize<'de>,
{
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let config_dir = base_path.join("config");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .ok()
        .and_then(|v| v.try_into().ok())
        .unwrap_or_else(Environment::detect);

    let environment_filename = format!("{}.yaml", environment.as_str());

    // Init config reader
    let config = config::Config::builder()
        .add_source(config::File::from(config_dir.join(environment_filename)))
        .add_source(config::Environment::with_prefix("APP").prefix_separator("_").separator("__"))
        .build()?;

    config.try_deserialize::<C>()
}
