use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StashStatus {
    ACTIVE,
    PAUSED,
    CLOSED,
}

impl FromStr for StashStatus {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ACTIVE" => Self::ACTIVE,
            "PAUSED" => Self::PAUSED,
            "CLOSED" => Self::CLOSED,
            _ => return Err("invalid stash status"),
        })
    }
}
