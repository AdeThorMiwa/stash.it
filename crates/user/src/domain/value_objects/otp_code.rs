use rand::{Rng, rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OtpCode(usize);

impl OtpCode {
    pub fn six_digit() -> Self {
        let six_digit_code = if cfg!(feature = "testing") {
            112358 // fixed code for testability
        } else {
            let mut rng = rng();
            rng.random_range(100000..999999)
        };

        Self(six_digit_code)
    }
}

impl ToString for OtpCode {
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }
}
