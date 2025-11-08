use di::injectable;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct JWTConfig {
    pub secret: String,
}

#[injectable]
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    pub jwt: JWTConfig,
}
