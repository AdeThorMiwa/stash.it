use rand::{Rng, rng};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OtpCode(usize);

impl OtpCode {
    pub fn six_digit() -> Self {
        let mut rng = rng();
        let six_digit_code = rng.random_range(100000..999999);
        Self(six_digit_code)
    }
}
