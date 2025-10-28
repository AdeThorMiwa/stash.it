pub enum Environment {
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
        if cfg!(feature = "testing") {
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
