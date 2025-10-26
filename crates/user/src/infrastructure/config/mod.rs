use di::injectable;
use serde::Deserialize;

enum Environment {
    Development,
    Production,
    Test,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
            Environment::Test => "test",
        }
    }

    pub fn detect() -> Self {
        // Highest priority: cargo test
        if cfg!(test) {
            return Environment::Test;
        }

        // Detect by build mode
        if cfg!(debug_assertions) {
            Environment::Development
        } else {
            Environment::Production
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            "test" => Ok(Self::Test),
            other => Err(format!(
                "{} is not a supported environment. \
                Use either `local` or `production`.",
                other
            )),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct JWTConfig {
    pub secret: String,
}

#[injectable]
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    pub jwt: JWTConfig,
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let config_dir = base_path.join("config");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .ok()
        .and_then(|v| v.try_into().ok())
        .unwrap_or_else(Environment::detect);

    println!("config_dir: {:?} {}", config_dir, environment.as_str());
    let environment_filename = format!("{}.yaml", environment.as_str());

    // Init config reader
    let config = config::Config::builder()
        .add_source(config::File::from(config_dir.join(environment_filename)))
        .add_source(config::Environment::with_prefix("APP").prefix_separator("_").separator("__"))
        .build()?;

    config.try_deserialize::<Config>()
}
