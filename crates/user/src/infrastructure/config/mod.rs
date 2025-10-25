use macros::inject;

pub struct JWTConfig {
    pub secret: String,
}

#[inject]
pub struct Config {
    pub jwt: JWTConfig,
}
